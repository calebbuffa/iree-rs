//! VM loops: the cooperative scheduler interface asynchronous invocations run on.
//!
//! A [`Loop`] is a borrowed `iree_vm_loop_t`. IREE ships the [`InlineLoop`];
//! hosts integrate the VM into their own scheduler (an async runtime, ALooper,
//! io_uring, a browser event loop) by implementing [`LoopDriver`].
use crate::base::WaitSource;
use crate::base::time::{deadline_from_instant, instant_from_deadline};
use crate::{Code, Error, Result, check};
use core::{cell::UnsafeCell, ffi::c_void, marker::PhantomData, mem::ManuallyDrop, ptr};
use iree_native as n;
use std::time::{Duration, Instant};

/// A borrowed VM loop.
///
/// Loops run one operation at a time, possibly on different threads, so every
/// callback is `Send`. Callbacks must not block: schedule waits instead. An
/// error returned by a callback goes to the loop's error handler.
#[derive(Clone, Copy)]
pub struct Loop<'a> {
    raw: n::iree_vm_loop_t,
    _owner: PhantomData<&'a ()>,
}

impl Loop<'static> {
    /// A loop that can do no work; every operation fails.
    pub fn null() -> Self {
        // SAFETY: returns a plain value.
        Self::adopt(unsafe { n::iree_vm_loop_null() })
    }
}

impl<'a> Loop<'a> {
    fn adopt(raw: n::iree_vm_loop_t) -> Self {
        Self {
            raw,
            _owner: PhantomData,
        }
    }

    /// Borrows a native loop whose state belongs to `owner`.
    ///
    /// # Safety
    /// `raw` must be a valid loop while `owner` is borrowed.
    pub unsafe fn from_raw<O: ?Sized>(raw: n::iree_vm_loop_t, _owner: &'a O) -> Self {
        Self::adopt(raw)
    }

    /// Returns a loop that dispatches every operation to `driver`.
    pub fn new<D: LoopDriver>(driver: &'a D) -> Self {
        Self::adopt(n::iree_vm_loop_t {
            self_: ptr::from_ref(driver).cast_mut().cast(),
            ctl: Some(driver_ctl::<D>),
        })
    }

    /// Returns the native loop value.
    pub fn as_raw(&self) -> n::iree_vm_loop_t {
        self.raw
    }

    /// Runs `callback` from the loop at some point in the future.
    pub fn call<F>(self, callback: F) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
    {
        schedule(callback, Box::default(), |function, user_data, _| {
            // SAFETY: the loop is valid for `'a`; `user_data` lives until the callback.
            unsafe {
                n::iree_vm_loop_call(
                    self.raw,
                    n::IREE_VM_LOOP_PRIORITY_DEFAULT,
                    function,
                    user_data,
                )
            }
        })
    }

    /// Runs `callback` once `deadline` passes.
    pub fn wait_until<F>(self, deadline: Instant, callback: F) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
    {
        // SAFETY: converts a plain value.
        let timeout = unsafe { n::iree_make_deadline(deadline_from_instant(Some(deadline))) };
        schedule(callback, Box::default(), |function, user_data, _| {
            // SAFETY: as in `call`.
            unsafe { n::iree_vm_loop_wait_until(self.raw, timeout, function, user_data) }
        })
    }

    /// Runs `callback` once `source` resolves, fails, or `timeout` elapses.
    ///
    /// # Safety
    /// The native owner of `source` must stay alive until the callback runs.
    pub unsafe fn wait_one<F>(
        self,
        source: WaitSource<'_>,
        timeout: Option<Duration>,
        callback: F,
    ) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
    {
        let timeout = n::timeout(timeout)?;
        schedule(callback, Box::default(), |function, user_data, _| {
            // SAFETY: as in `call`; the caller keeps the source alive.
            unsafe {
                n::iree_vm_loop_wait_one(self.raw, source.as_raw(), timeout, function, user_data)
            }
        })
    }

    /// Runs `callback` once any of `sources` resolves, or `timeout` elapses.
    ///
    /// # Safety
    /// The native owners of `sources` must stay alive until the callback runs.
    pub unsafe fn wait_any<F>(
        self,
        sources: &[WaitSource<'_>],
        timeout: Option<Duration>,
        callback: F,
    ) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
    {
        // SAFETY: forwarded to the caller.
        unsafe { self.wait_multi(n::iree_vm_loop_wait_any, sources, timeout, callback) }
    }

    /// Runs `callback` once all of `sources` resolve, or `timeout` elapses.
    ///
    /// # Safety
    /// The native owners of `sources` must stay alive until the callback runs.
    pub unsafe fn wait_all<F>(
        self,
        sources: &[WaitSource<'_>],
        timeout: Option<Duration>,
        callback: F,
    ) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
    {
        // SAFETY: forwarded to the caller.
        unsafe { self.wait_multi(n::iree_vm_loop_wait_all, sources, timeout, callback) }
    }

    unsafe fn wait_multi<F>(
        self,
        wait: unsafe extern "C" fn(
            n::iree_vm_loop_t,
            n::iree_host_size_t,
            *mut n::iree_wait_source_t,
            n::iree_timeout_t,
            n::iree_vm_loop_callback_fn_t,
            *mut c_void,
        ) -> n::iree_status_t,
        sources: &[WaitSource<'_>],
        timeout: Option<Duration>,
        callback: F,
    ) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
    {
        let timeout = n::timeout(timeout)?;
        // The loop borrows the array until the callback, so it travels with it.
        let sources = sources.iter().map(WaitSource::as_raw).collect();
        schedule(callback, sources, |function, user_data, (data, len)| {
            // SAFETY: as in `call`; the array is owned by `user_data`.
            unsafe { wait(self.raw, len, data, timeout, function, user_data) }
        })
    }

    /// Blocks until the loop is idle or `timeout` elapses.
    ///
    /// Loops that cannot drain return [`Code::DeadlineExceeded`].
    pub fn drain(self, timeout: Option<Duration>) -> Result<()> {
        let timeout = n::timeout(timeout)?;
        // SAFETY: the loop is valid for `'a`.
        check(unsafe { n::iree_vm_loop_drain(self.raw, timeout) })
    }
}

/// One scheduled callback plus the wait sources the loop borrows until it runs.
struct Pending<F> {
    callback: F,
    sources: Box<[n::iree_wait_source_t]>,
}

fn schedule<F>(
    callback: F,
    sources: Box<[n::iree_wait_source_t]>,
    enqueue: impl FnOnce(
        n::iree_vm_loop_callback_fn_t,
        *mut c_void,
        (*mut n::iree_wait_source_t, usize),
    ) -> n::iree_status_t,
) -> Result<()>
where
    F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
{
    let mut pending = Box::new(Pending { callback, sources });
    // Raw parts only: an inline loop may run and free the state during enqueue.
    let sources = (pending.sources.as_mut_ptr(), pending.sources.len());
    let pending = Box::into_raw(pending);
    let result = check(enqueue(Some(run_pending::<F>), pending.cast(), sources));
    if result.is_err() {
        // SAFETY: a failed enqueue neither issues the callback nor retains it.
        let pending = unsafe { Box::from_raw(pending) };
        n::contain(|| drop(pending));
    }
    result
}

unsafe extern "C" fn run_pending<F>(
    user_data: *mut c_void,
    raw: n::iree_vm_loop_t,
    status: n::iree_status_t,
) -> n::iree_status_t
where
    F: FnOnce(Loop<'_>, Result<()>) -> Result<()> + Send + 'static,
{
    // SAFETY: `schedule` hands each loop operation one boxed state, issued once.
    let pending = unsafe { Box::from_raw(user_data.cast::<Pending<F>>()) };
    // SAFETY: the loop transfers ownership of `status` to the callback.
    let result = unsafe { n::consume(status) };
    n::callback_status(|| (pending.callback)(Loop::adopt(raw), result))
}

/// IREE's inline loop: every operation runs on the calling thread as it is
/// enqueued, using no heap memory beyond this storage.
///
/// Suited to the smallest environments: reentrant scheduling is limited to a
/// few operations, waits block the calling thread, and the first callback
/// error aborts pending work and sticks until [`take_error`](Self::take_error).
/// IREE's inline `WAIT_ANY` implementation may wait only on the first source,
/// even if a later source is already ready; use a host [`LoopDriver`] when
/// readiness-racing wait-any semantics are required.
///
/// Callbacks schedule follow-up work on the [`Loop`] they are handed. Using
/// this loop's own [`as_loop`](Self::as_loop) while it is running fails with
/// [`Code::FailedPrecondition`]: IREE reinitializes the storage on every
/// top-level operation.
pub struct InlineLoop {
    state: Box<InlineState>,
}

struct InlineState {
    storage: UnsafeCell<n::iree_vm_loop_inline_storage_t>,
    running: core::cell::Cell<bool>,
}

// SAFETY: the inline loop is thread-compatible; it only runs on the thread
// that calls into it and `&self` access is limited to that thread by `!Sync`.
unsafe impl Send for InlineLoop {}

impl InlineLoop {
    /// Creates an inline loop with its own storage.
    pub fn new() -> Self {
        let state = Box::new(InlineState {
            storage: UnsafeCell::new(n::iree_vm_loop_inline_storage_t::default()),
            running: core::cell::Cell::new(false),
        });
        // SAFETY: the storage is boxed, so its address is stable for the loop's
        // life; this only resets its status.
        let _ = unsafe { n::iree_vm_loop_inline_initialize(state.storage.get()) };
        Self { state }
    }

    /// Borrows the loop.
    pub fn as_loop(&self) -> Loop<'_> {
        Loop::adopt(n::iree_vm_loop_t {
            self_: ptr::from_ref::<InlineState>(&self.state).cast_mut().cast(),
            ctl: Some(inline_ctl),
        })
    }

    /// Returns and clears the first error a callback raised, re-enabling the loop.
    pub fn take_error(&mut self) -> Result<()> {
        let status = core::mem::replace(&mut self.state.storage.get_mut().status, ptr::null_mut());
        // SAFETY: the loop transferred the error to its storage; this takes it once.
        unsafe { n::consume(status) }
    }
}

/// Forwards top-level operations to IREE's inline loop, rejecting re-entry.
unsafe extern "C" fn inline_ctl(
    self_: *mut c_void,
    command: n::iree_vm_loop_command_t,
    params: *const c_void,
    inout: *mut *mut c_void,
) -> n::iree_status_t {
    // SAFETY: `InlineLoop::as_loop` stores its boxed state, borrowed for the loop's life.
    let state = unsafe { &*self_.cast::<InlineState>() };
    if command == n::IREE_VM_LOOP_COMMAND_DRAIN as n::iree_vm_loop_command_t {
        // Inline loops finish all work before returning, so there is nothing to drain.
        return ptr::null_mut();
    }
    if state.running.replace(true) {
        return Error::new(
            Code::FailedPrecondition,
            "inline loop re-entered; schedule work on the loop passed to the callback",
        )
        .into_status();
    }
    // SAFETY: the storage is valid and not in use by an outer operation;
    // callbacks contain their panics, so this returns normally.
    let status = unsafe {
        n::iree_vm_loop_inline_using_storage_ctl(state.storage.get().cast(), command, params, inout)
    };
    state.running.set(false);
    status
}

impl Default for InlineLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for InlineLoop {
    fn drop(&mut self) {
        // SAFETY: no loop borrows remain; this frees any sticky status.
        unsafe { n::iree_vm_loop_inline_deinitialize(self.state.storage.get()) }
    }
}

/// A host scheduler that VM loops dispatch to; see [`Loop::new`].
///
/// Each operation hands over a [`Continuation`] the driver must eventually
/// complete, from any thread but one at a time. Dropping a continuation
/// completes it as aborted, so no work is ever lost or leaked.
pub trait LoopDriver: Send + Sync {
    /// Runs `continuation` at some point in the future.
    fn call(&self, continuation: Continuation);

    /// Runs `continuation` once `deadline` passes; `None` is never.
    fn wait_until(&self, deadline: Option<Instant>, continuation: Continuation);

    /// Runs the request's continuation once its wait condition is met, it
    /// fails, or its deadline passes.
    fn wait(&self, request: WaitRequest);

    /// Blocks until no work is pending or `deadline` passes.
    ///
    /// The default reports that draining is unsupported.
    fn drain(&self, deadline: Option<Instant>) -> Result<()> {
        let _ = deadline;
        Err(Error::new(Code::DeadlineExceeded, "VM loop cannot drain"))
    }
}

/// A pending loop callback that must run exactly once.
pub struct Continuation(n::iree_vm_loop_callback_t);

// SAFETY: loop callbacks may be issued from any thread.
unsafe impl Send for Continuation {}

impl Continuation {
    /// Runs the continuation with the operation's outcome. `loop_` is where
    /// it may schedule more work; the returned error is the callback's own.
    pub fn complete(self, loop_: Loop<'_>, outcome: Result<()>) -> Result<()> {
        let callback = ManuallyDrop::new(self).0;
        let status = match outcome {
            Ok(()) => ptr::null_mut(),
            Err(error) => error.into_status(),
        };
        let Some(function) = callback.fn_ else {
            // SAFETY: with no callback the status is ours to free.
            let _ = unsafe { n::consume(status) };
            return Ok(());
        };
        // SAFETY: the loop operation handed us this callback to issue once,
        // transferring ownership of `status` to it.
        check(unsafe { function(callback.user_data, loop_.raw, status) })
    }
}

impl Drop for Continuation {
    fn drop(&mut self) {
        let Some(function) = self.0.fn_ else { return };
        let aborted = n::abi::status::from_code(n::IREE_STATUS_ABORTED);
        // SAFETY: as in `complete`; aborted callbacks get the null loop so they
        // cannot schedule more work.
        let status = unsafe { function(self.0.user_data, Loop::null().raw, aborted) };
        // SAFETY: the callback returned an owned status.
        let _ = unsafe { n::consume(status) };
    }
}

/// How a [`WaitRequest`] combines its sources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaitMode {
    /// One source.
    One,
    /// Any one of the sources.
    Any,
    /// All of the sources.
    All,
}

/// A wait operation handed to a [`LoopDriver`].
pub struct WaitRequest {
    mode: WaitMode,
    deadline: Option<Instant>,
    one: n::iree_wait_source_t,
    many: *const n::iree_wait_source_t,
    count: usize,
    continuation: Continuation,
}

// SAFETY: the wait sources are plain values the issuer keeps alive until the
// continuation runs, which may be on any thread.
unsafe impl Send for WaitRequest {}

impl WaitRequest {
    /// How the sources combine.
    pub fn mode(&self) -> WaitMode {
        self.mode
    }

    /// When the wait fails with [`Code::DeadlineExceeded`]; `None` is never.
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// The sources, valid until the continuation runs.
    pub fn sources(&self) -> &[WaitSource<'_>] {
        let (data, len) = match self.mode {
            WaitMode::One => (ptr::from_ref(&self.one), 1),
            WaitMode::Any | WaitMode::All => (self.many, self.count),
        };
        // SAFETY: `WaitSource` is a transparent wrapper; the issuer keeps the
        // array alive until the continuation runs, which needs `self`.
        unsafe { n::slice(data.cast::<WaitSource<'_>>(), len) }
    }

    /// Completes the wait with its outcome; see [`Continuation::complete`].
    pub fn complete(self, loop_: Loop<'_>, outcome: Result<()>) -> Result<()> {
        self.continuation.complete(loop_, outcome)
    }

    /// Detaches the continuation, ending the borrow of the sources.
    pub fn into_continuation(self) -> Continuation {
        self.continuation
    }
}

unsafe extern "C" fn driver_ctl<D: LoopDriver>(
    self_: *mut c_void,
    command: n::iree_vm_loop_command_t,
    params: *const c_void,
    _inout: *mut *mut c_void,
) -> n::iree_status_t {
    // SAFETY: `Loop::new` stored a `&D` that outlives the loop.
    let driver = unsafe { &*self_.cast::<D>() };
    // Operations always issue their callback, by the driver or by a dropped
    // continuation, so they report success even when the driver panics.
    fn operation(run: impl FnOnce()) -> n::iree_status_t {
        n::contain(run);
        ptr::null_mut()
    }
    // SAFETY: IREE passes the parameter struct matching each command.
    unsafe {
        let command = command as n::iree_vm_loop_command_e;
        match command {
            n::IREE_VM_LOOP_COMMAND_CALL => {
                let params = *params.cast::<n::iree_vm_loop_call_params_t>();
                operation(|| driver.call(Continuation(params.callback)))
            }
            n::IREE_VM_LOOP_COMMAND_WAIT_UNTIL => {
                let params = *params.cast::<n::iree_vm_loop_wait_until_params_t>();
                operation(|| {
                    let deadline = instant_from_deadline(params.deadline_ns);
                    driver.wait_until(deadline, Continuation(params.callback))
                })
            }
            n::IREE_VM_LOOP_COMMAND_WAIT_ONE => {
                let params = *params.cast::<n::iree_vm_loop_wait_one_params_t>();
                operation(|| {
                    driver.wait(WaitRequest {
                        mode: WaitMode::One,
                        deadline: instant_from_deadline(params.deadline_ns),
                        one: params.wait_source,
                        many: ptr::null(),
                        count: 0,
                        continuation: Continuation(params.callback),
                    })
                })
            }
            n::IREE_VM_LOOP_COMMAND_WAIT_ANY | n::IREE_VM_LOOP_COMMAND_WAIT_ALL => {
                let params = *params.cast::<n::iree_vm_loop_wait_multi_params_t>();
                let mode = if command == n::IREE_VM_LOOP_COMMAND_WAIT_ANY {
                    WaitMode::Any
                } else {
                    WaitMode::All
                };
                operation(|| {
                    driver.wait(WaitRequest {
                        mode,
                        deadline: instant_from_deadline(params.deadline_ns),
                        // SAFETY: an unused immediate source fills the single slot.
                        one: n::iree_wait_source_immediate(),
                        many: params.wait_sources,
                        count: params.count,
                        continuation: Continuation(params.callback),
                    })
                })
            }
            n::IREE_VM_LOOP_COMMAND_DRAIN => {
                let params = *params.cast::<n::iree_vm_loop_drain_params_t>();
                n::callback_status(|| driver.drain(instant_from_deadline(params.deadline_ns)))
            }
            _ => Error::new(Code::Unimplemented, "unknown VM loop command").into_status(),
        }
    }
}
