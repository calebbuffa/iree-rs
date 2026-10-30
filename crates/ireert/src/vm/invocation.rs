//! VM invocation: synchronous, loop-based asynchronous, and stepwise.
use super::{Context, Function, List, Loop};
use crate::base::WaitSource;
use crate::base::time::{deadline_from_instant, instant_from_deadline};
use crate::hal::bitfield::bitfield;
use crate::{Code, Error, Result, check};
use core::{ffi::c_void, mem::ManuallyDrop, ptr};
use iree_native as n;
use std::time::Instant;

bitfield! {
    /// Invocation behavior flags.
    InvocationFlags: n::iree_vm_invocation_flags_t {
        /// Enables execution tracing.
        TRACE_EXECUTION = n::IREE_VM_INVOCATION_FLAG_TRACE_EXECUTION,
        /// Traces inline into the caller's stack.
        TRACE_INLINE = n::IREE_VM_INVOCATION_FLAG_TRACE_INLINE,
    }
}

impl Context {
    /// Invokes `function` with `inputs`, appending its results to `outputs`.
    ///
    /// Blocks on any waits the function performs.
    pub fn invoke(
        &self,
        function: &Function,
        flags: InvocationFlags,
        inputs: &List,
        outputs: &mut List,
    ) -> Result<()> {
        // SAFETY: the context, function, and lists are live for the call.
        check(unsafe {
            n::iree_vm_invoke(
                self.as_raw(),
                *function.as_raw(),
                flags.bits(),
                ptr::null(),
                inputs.as_raw(),
                outputs.as_raw(),
                crate::system(),
            )
        })
    }

    /// Invokes `function` on `loop_` without blocking; waits the function
    /// performs are scheduled on the loop.
    ///
    /// `callback` always runs exactly once, possibly before this returns,
    /// with `outputs` holding the results on success. If scheduling fails it
    /// never runs and the error is returned here. Invocations on one context
    /// may only overlap if it was created with `ContextFlags::CONCURRENT`.
    pub fn invoke_async<F>(
        &self,
        loop_: Loop<'_>,
        function: &Function,
        flags: InvocationFlags,
        inputs: &List,
        outputs: List,
        callback: F,
    ) -> Result<()>
    where
        F: FnOnce(Loop<'_>, Result<List>) -> Result<()> + Send + 'static,
    {
        let pending = Box::into_raw(Box::new(AsyncInvoke {
            // SAFETY: plain C storage; zero is a valid initial state.
            state: unsafe { core::mem::zeroed() },
            outputs,
            callback,
        }));
        // SAFETY: the state lives in `pending` until the callback reclaims it;
        // the call retains the context and lists itself.
        let result = check(unsafe {
            n::iree_vm_async_invoke(
                loop_.as_raw(),
                ptr::addr_of_mut!((*pending).state),
                self.as_raw(),
                *function.as_raw(),
                flags.bits(),
                ptr::null(),
                inputs.as_raw(),
                (*pending).outputs.as_raw(),
                crate::system(),
                Some(complete_async::<F>),
                pending.cast(),
            )
        });
        if result.is_err() {
            // SAFETY: a failed launch released its references and will not
            // issue the callback.
            let pending = unsafe { Box::from_raw(pending) };
            n::contain(|| drop(pending));
        }
        result
    }

    /// Begins a stepwise invocation, for hosts that schedule VM execution
    /// themselves; see [`Invocation`].
    pub fn begin_invoke(
        &self,
        function: &Function,
        flags: InvocationFlags,
        inputs: &List,
    ) -> Result<Invocation> {
        let mut invocation = Invocation {
            // SAFETY: plain C storage; zero is the state abort accepts.
            state: Box::new(unsafe { core::mem::zeroed() }),
            pending: false,
        };
        // SAFETY: the boxed state never moves; the context, function, and
        // inputs are live for the call, and the state retains what it needs.
        let status = unsafe {
            n::iree_vm_begin_invoke(
                &mut *invocation.state,
                self.as_raw(),
                *function.as_raw(),
                flags.bits(),
                ptr::null(),
                inputs.as_raw(),
                crate::system(),
            )
        };
        invocation.pending = invocation.step(status)?;
        Ok(invocation)
    }
}

struct AsyncInvoke<F> {
    state: n::iree_vm_async_invoke_state_t,
    outputs: List,
    callback: F,
}

unsafe extern "C" fn complete_async<F>(
    user_data: *mut c_void,
    loop_: n::iree_vm_loop_t,
    status: n::iree_status_t,
    outputs: *mut n::iree_vm_list_t,
) -> n::iree_status_t
where
    F: FnOnce(Loop<'_>, Result<List>) -> Result<()> + Send + 'static,
{
    // SAFETY: `invoke_async` passed one boxed state, completed exactly once;
    // the invocation no longer touches it.
    let pending = unsafe { Box::from_raw(user_data.cast::<AsyncInvoke<F>>()) };
    // SAFETY: the callback owns `status` and the invocation's output reference.
    let result = unsafe {
        if !outputs.is_null() {
            n::iree_vm_list_release(outputs);
        }
        n::consume(status)
    };
    let AsyncInvoke {
        outputs, callback, ..
    } = *pending;
    // SAFETY: the loop is valid for the duration of the callback.
    let loop_ = unsafe { Loop::from_raw(loop_, &()) };
    n::callback_status(|| callback(loop_, result.map(|()| outputs)))
}

/// A stepwise invocation (`iree_vm_begin_invoke` and friends).
///
/// While [`is_pending`](Self::is_pending), either perform the pending
/// [`wait_frame`](Self::wait_frame) (or block on it with
/// [`wait`](Self::wait)) and then [`resume`](Self::resume), or just resume a
/// cooperative yield. Finish with [`end`](Self::end); dropping an unfinished
/// invocation aborts it. Steps may run on any thread, one at a time.
pub struct Invocation {
    state: Box<n::iree_vm_invoke_state_t>,
    pending: bool,
}

// SAFETY: IREE permits an invocation to be resumed on any thread, one at a
// time; this owner uniquely controls the state and keeps all retained handles alive.
unsafe impl Send for Invocation {}

impl Invocation {
    /// Maps a step status to whether the invocation is still pending.
    fn step(&mut self, status: n::iree_status_t) -> Result<bool> {
        if n::abi::status::code(status) == n::IREE_STATUS_DEFERRED {
            // SAFETY: the code-only deferred status is ours to free.
            let _ = unsafe { n::consume(status) };
            return Ok(true);
        }
        check(status).map(|()| false)
    }

    /// Whether the invocation yielded or is waiting, rather than finished.
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// The wait the invocation is blocked on, or `None` after a plain yield
    /// or once finished.
    pub fn wait_frame(&mut self) -> Option<WaitFrame<'_>> {
        if !self.pending {
            return None;
        }
        // SAFETY: a pending invocation has a live stack.
        let frame = unsafe { n::iree_vm_stack_current_frame(self.state.stack) };
        // SAFETY: a non-null frame is live until the invocation resumes.
        if frame.is_null() || unsafe { (*frame).type_ } != n::IREE_VM_STACK_FRAME_WAIT {
            return None;
        }
        // SAFETY: wait frames store an `iree_vm_wait_frame_t`.
        let raw = unsafe { n::iree_vm_stack_frame_storage(frame) }.cast();
        Some(WaitFrame {
            raw,
            state: &mut self.state,
        })
    }

    /// Blocks on the pending wait until it resolves or `deadline` passes
    /// (`None` uses the wait's own deadline). Resume afterwards.
    pub fn wait(&mut self, deadline: Option<Instant>) -> Result<()> {
        let frame = self
            .wait_frame()
            .ok_or_else(|| Error::new(Code::FailedPrecondition, "invocation is not waiting"))?
            .raw;
        // SAFETY: the frame belongs to this live, waiting invocation.
        check(unsafe {
            n::iree_vm_wait_invoke(&mut *self.state, frame, deadline_from_instant(deadline))
        })
    }

    /// Continues execution until the next yield, wait, or completion.
    pub fn resume(&mut self) -> Result<()> {
        let cooperative_yield = if self.pending {
            // SAFETY: pending invocations have a live stack; a non-wait frame
            // represents a cooperative yield.
            unsafe {
                let frame = n::iree_vm_stack_current_frame(self.state.stack);
                !frame.is_null() && (*frame).type_ != n::IREE_VM_STACK_FRAME_WAIT
            }
        } else {
            false
        };
        let status = if cooperative_yield {
            // The upstream resume API treats all deferred statuses as waits,
            // but yieldable bytecode imports need their bytecode caller to
            // resume the import with its own result storage. This mirrors the
            // upstream async-wake status reset and resumes that caller first.
            // SAFETY: the pending invocation owns its deferred status and live stack.
            unsafe {
                n::iree_status_free(self.state.status);
                self.state.status = n::abi::status::ok();
            }
            // SAFETY: the pending invocation owns a live stack.
            let (top, parent) = unsafe {
                (
                    n::iree_vm_stack_top(self.state.stack),
                    n::iree_vm_stack_parent_frame(self.state.stack),
                )
            };
            // SAFETY: a non-null top frame belongs to the pending stack.
            let native_top =
                !top.is_null() && unsafe { (*top).type_ == n::IREE_VM_STACK_FRAME_NATIVE };
            if native_top && !parent.is_null() {
                // SAFETY: the parent frame is live on this stack.
                let function = unsafe { (*parent).function };
                let module = function.module;
                // SAFETY: the parent function retains its module for this frame.
                let resume = unsafe { (*module).resume_call };
                if let Some(resume) = resume {
                    // SAFETY: the parent frame owns this module; the call
                    // results span belongs to the live invocation state.
                    let status =
                        unsafe { resume((*module).self_, self.state.stack, self.state.results) };
                    self.state.status = status;
                    let code = n::abi::status::code(status);
                    if code == n::IREE_STATUS_DEFERRED {
                        n::abi::status::from_code(code)
                    } else if code != n::IREE_STATUS_OK {
                        n::abi::status::ok()
                    } else {
                        // The caller may have returned into an enclosing VM frame.
                        // SAFETY: this invocation still owns its stack.
                        let current = unsafe { n::iree_vm_stack_current_frame(self.state.stack) };
                        if current.is_null() {
                            n::abi::status::ok()
                        } else {
                            // SAFETY: the caller returned successfully and left
                            // a live frame for the standard unwinder.
                            unsafe { n::iree_vm_resume_invoke(&mut *self.state) }
                        }
                    }
                } else {
                    n::abi::status::from_code(n::IREE_STATUS_FAILED_PRECONDITION)
                }
            } else {
                // Top-level native functions and internal bytecode yields use
                // the standard resume path.
                // SAFETY: the state is live and its deferred status was reset.
                unsafe { n::iree_vm_resume_invoke(&mut *self.state) }
            }
        } else {
            // SAFETY: the state is live; resuming a finished invocation is a no-op.
            unsafe { n::iree_vm_resume_invoke(&mut *self.state) }
        };
        self.pending = self.step(status)?;
        Ok(())
    }

    /// Finishes the invocation, appending its results to `outputs`, and
    /// returns its status.
    pub fn end(self, outputs: &mut List) -> Result<()> {
        let mut this = ManuallyDrop::new(self);
        let mut invoke_status = ptr::null_mut();
        // SAFETY: the state is live; on success it is released and only
        // `invoke_status` remains ours.
        let status = unsafe {
            n::iree_vm_end_invoke(&mut *this.state, outputs.as_raw(), &mut invoke_status)
        };
        if !n::abi::status::is_ok(status) {
            // SAFETY: a failed end leaves the state for abort to release.
            unsafe { n::iree_vm_abort_invoke(&mut *this.state) };
        }
        // SAFETY: the state holds no more resources and is never used again.
        let state = unsafe { ptr::read(&this.state) };
        drop(state);
        check(status)?;
        // SAFETY: `end` transferred the invocation status to us.
        unsafe { n::consume(invoke_status) }
    }
}

impl Drop for Invocation {
    fn drop(&mut self) {
        // SAFETY: abort releases any resources, including from zeroed state.
        unsafe { n::iree_vm_abort_invoke(&mut *self.state) }
    }
}

/// The wait a stepwise [`Invocation`] is blocked on.
pub struct WaitFrame<'a> {
    raw: *mut n::iree_vm_wait_frame_t,
    state: &'a mut n::iree_vm_invoke_state_t,
}

/// What a [`WaitFrame`] waits for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaitFrameKind {
    /// Only the deadline.
    Until,
    /// Any one of the sources.
    Any,
    /// All of the sources.
    All,
}

impl WaitFrame<'_> {
    /// What the frame waits for.
    pub fn kind(&self) -> WaitFrameKind {
        // SAFETY: the frame is live while the invocation is borrowed.
        match n::iree_vm_wait_type_e::from(unsafe { (*self.raw).wait_type }) {
            n::IREE_VM_WAIT_ANY => WaitFrameKind::Any,
            n::IREE_VM_WAIT_ALL => WaitFrameKind::All,
            _ => WaitFrameKind::Until,
        }
    }

    /// When the wait fails with [`Code::DeadlineExceeded`]; `None` is never.
    pub fn deadline(&self) -> Option<Instant> {
        // SAFETY: as in `kind`.
        instant_from_deadline(unsafe { (*self.raw).deadline_ns })
    }

    /// The sources to wait on.
    pub fn sources(&self) -> &[WaitSource<'_>] {
        // SAFETY: as in `kind`; `WaitSource` is a transparent wrapper over the
        // frame's trailing array of `count` sources.
        unsafe {
            n::slice(
                (*self.raw).wait_sources.as_ptr().cast::<WaitSource<'_>>(),
                (*self.raw).count,
            )
        }
    }

    /// Records the outcome of a wait the host performed itself; resume next.
    pub fn complete(self, outcome: Result<()>) {
        // SAFETY: as in `kind`; this mirrors IREE's own asynchronous wake-up:
        // the waiter reads `wait_status`, and the deferred invocation status is
        // replaced by OK so the next resume continues.
        unsafe {
            (*self.raw).wait_status = match outcome {
                Ok(()) => ptr::null_mut(),
                Err(error) => error.into_status(),
            };
            let _ = n::consume(core::mem::replace(&mut self.state.status, ptr::null_mut()));
        }
    }
}
