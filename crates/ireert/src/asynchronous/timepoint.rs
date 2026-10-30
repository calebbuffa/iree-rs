//! Callbacks on semaphore timeline values, relays between semaphores, and
//! platform fence interop.
//!
//! HAL semaphores share the async semaphore layout, so these operations serve
//! both [`super::Semaphore`] and [`crate::hal::Semaphore`].
use super::{Primitive, Proactor};
use crate::{Result, check};
use core::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};
use iree_native as n;

type Callback = Box<dyn FnOnce(Result<()>) + Send>;

// The timepoint is the first field so native callbacks can recover the state.
#[repr(C)]
struct State {
    timepoint: n::iree_async_semaphore_timepoint_t,
    semaphore: *mut n::iree_async_semaphore_t,
    // Set by whichever of the callback and the guard finishes second-to-last;
    // the other side then frees the state.
    claimed: AtomicBool,
    callback: Option<Callback>,
}
impl Drop for State {
    fn drop(&mut self) {
        // SAFETY: the state holds one semaphore reference.
        unsafe { n::iree_async_semaphore_release(self.semaphore) }
    }
}

/// Frees `state` if the other side has already finished with it.
///
/// # Safety
/// `state` must come from [`Timepoint::register`] and be released at most once per side.
unsafe fn finish(state: NonNull<State>) {
    // SAFETY: the state stays allocated until both sides finish.
    if unsafe { state.as_ref() }
        .claimed
        .swap(true, Ordering::AcqRel)
    {
        // SAFETY: both sides are done; this side frees the allocation.
        drop(unsafe { Box::from_raw(state.as_ptr()) });
    }
}

unsafe extern "C" fn fire(
    user_data: *mut core::ffi::c_void,
    _timepoint: *mut n::iree_async_semaphore_timepoint_t,
    status: n::iree_status_t,
) {
    let Some(state) = NonNull::new(user_data.cast::<State>()) else {
        return;
    };
    // SAFETY: the status is owned by this callback.
    let result = unsafe { n::consume(status) };
    // SAFETY: only this callback touches the closure once registered.
    let callback = unsafe { (*state.as_ptr()).callback.take() };
    if let Some(callback) = callback {
        let _completed = n::contain(|| callback(result));
    }
    // SAFETY: the callback side finishes exactly once.
    unsafe { finish(state) }
}

/// A pending callback on a semaphore value.
///
/// Dropping it before the callback runs cancels the callback; use
/// [`detach`](Self::detach) to let it run regardless.
#[must_use = "dropping a timepoint cancels its callback"]
pub struct Timepoint {
    state: NonNull<State>,
}
impl Timepoint {
    /// Registers `callback` on a borrowed async-compatible semaphore.
    ///
    /// # Safety
    /// `semaphore` must be a live async or HAL semaphore.
    pub(crate) unsafe fn register(
        semaphore: *mut n::iree_async_semaphore_t,
        value: u64,
        callback: Callback,
    ) -> Result<Self> {
        // SAFETY: the semaphore is live; the state keeps its own reference.
        unsafe { n::iree_async_semaphore_retain(semaphore) };
        let state = NonNull::from(Box::leak(Box::new(State {
            timepoint: n::iree_async_semaphore_timepoint_t::default(),
            semaphore,
            claimed: AtomicBool::new(false),
            callback: Some(callback),
        })));
        // SAFETY: the state is uniquely owned until registration.
        unsafe {
            let timepoint = &raw mut (*state.as_ptr()).timepoint;
            (*timepoint).callback = Some(fire);
            (*timepoint).user_data = state.as_ptr().cast();
        }
        // SAFETY: the timepoint stays at a stable heap address until both sides finish.
        let status = check(unsafe {
            n::iree_async_semaphore_acquire_timepoint(
                semaphore,
                value,
                &raw mut (*state.as_ptr()).timepoint,
            )
        });
        if let Err(error) = status {
            // SAFETY: a failed registration never invokes the callback.
            drop(unsafe { Box::from_raw(state.as_ptr()) });
            return Err(error);
        }
        Ok(Self { state })
    }

    /// Lets the callback run whenever the value is reached or the semaphore fails.
    pub fn detach(self) {
        let state = core::mem::ManuallyDrop::new(self).state;
        // SAFETY: the guard side finishes exactly once.
        unsafe { finish(state) }
    }

    /// Cancels the callback, returning true if it had not started.
    pub fn cancel(self) -> bool {
        let state = core::mem::ManuallyDrop::new(self).state;
        // SAFETY: the guard side finishes exactly once.
        unsafe { cancel(state) }
    }
}
impl Drop for Timepoint {
    fn drop(&mut self) {
        // SAFETY: the guard side finishes exactly once.
        let _cancelled = unsafe { cancel(self.state) };
    }
}

/// Cancels a pending callback and finishes the guard side.
///
/// # Safety
/// `state` must come from [`Timepoint::register`]; the guard side must not have finished.
unsafe fn cancel(state: NonNull<State>) -> bool {
    // SAFETY: the semaphore is retained by the still-allocated state.
    let cancelled = unsafe {
        let raw = state.as_ptr();
        n::iree_async_semaphore_cancel_timepoint((*raw).semaphore, &raw mut (*raw).timepoint)
    };
    if cancelled {
        // SAFETY: the callback will never run; the guard frees the state.
        drop(unsafe { Box::from_raw(state.as_ptr()) });
    } else {
        // SAFETY: the guard side finishes exactly once.
        unsafe { finish(state) }
    }
    cancelled
}
// SAFETY: the guard only cancels or releases thread-safe native state.
unsafe impl Send for Timepoint {}
// SAFETY: the guard exposes no shared access.
unsafe impl Sync for Timepoint {}

/// A retained semaphore handle that can move into a callback.
struct Retained(*mut n::iree_async_semaphore_t);
impl Drop for Retained {
    fn drop(&mut self) {
        // SAFETY: this handle owns one reference.
        unsafe { n::iree_async_semaphore_release(self.0) }
    }
}
// SAFETY: async and HAL semaphore operations are thread-safe.
unsafe impl Send for Retained {}

/// Signals `target` to `target_value` once `source` reaches `value`, or fails
/// `target` with the source failure.
///
/// # Safety
/// `source` and `target` must be live async-compatible semaphores.
pub(crate) unsafe fn link(
    source: *mut n::iree_async_semaphore_t,
    value: u64,
    target: *mut n::iree_async_semaphore_t,
    target_value: u64,
) -> Result<Timepoint> {
    // SAFETY: the target is live; `Retained` owns the new reference.
    unsafe { n::iree_async_semaphore_retain(target) };
    let target = Retained(target);
    let relay = move |result: Result<()>| {
        let target = target;
        let result = result.and_then(|()| {
            // SAFETY: the retained target is live.
            check(unsafe {
                n::iree_async_semaphore_signal(target.0, target_value, core::ptr::null())
            })
        });
        if let Err(error) = result {
            // SAFETY: the retained target is live and takes ownership of the status.
            unsafe { n::iree_async_semaphore_fail(target.0, error.into_status()) }
        }
    };
    // SAFETY: the source is live.
    unsafe { Timepoint::register(source, value, Box::new(relay)) }
}

/// Signals `semaphore` to `value` once the platform `fence` signals.
///
/// # Safety
/// `semaphore` must be live. Signal-path dependencies must remain valid until
/// completion or cancellation by proactor shutdown.
pub(crate) unsafe fn import_fence(
    semaphore: *mut n::iree_async_semaphore_t,
    proactor: &Proactor,
    fence: Primitive,
    value: u64,
) -> Result<()> {
    // Upstream requires the semaphore to outlive the pending fence.
    // SAFETY: the semaphore is live.
    let keep_alive = unsafe { Timepoint::register(semaphore, value, Box::new(|_| {})) }?;
    // SAFETY: the proactor takes ownership of the fence only on success.
    let status = check(unsafe {
        n::iree_async_semaphore_import_fence(proactor.as_raw(), fence.as_raw(), semaphore, value)
    });
    match status {
        Ok(()) => {
            core::mem::forget(fence);
            keep_alive.detach();
            Ok(())
        }
        Err(error) => Err(error),
    }
}

/// Exports a platform fence that signals once `semaphore` reaches `value`.
///
/// # Safety
/// `semaphore` must be live. The returned primitive must remain open until
/// `value` is reached or the semaphore fails, including when native callbacks
/// retain the same OS handle rather than duplicating it.
pub(crate) unsafe fn export_fence(
    semaphore: *mut n::iree_async_semaphore_t,
    proactor: &Proactor,
    value: u64,
) -> Result<Primitive> {
    let mut raw = Primitive::none().into_raw();
    // SAFETY: the semaphore and proactor are live; the output is writable.
    check(unsafe {
        n::iree_async_semaphore_export_fence(proactor.as_raw(), semaphore, value, &mut raw)
    })?;
    // SAFETY: the caller owns the exported handle.
    Ok(unsafe { Primitive::from_raw(raw) })
}
