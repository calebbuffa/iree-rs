use crate::{Device, Error, Result, StatusCode, runtime::check, sys};
use std::{
    cell::UnsafeCell,
    ffi::c_void,
    future::Future,
    pin::Pin,
    ptr::{self, NonNull},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    time::Duration,
};

#[cfg(not(feature = "task"))]
use std::rc::Rc as TimelineOwnership;
#[cfg(feature = "task")]
use std::sync::Arc as TimelineOwnership;

#[cfg(feature = "task")]
const _: () = assert!(sys::THREADING_ENABLED);

fn cancelled_status() -> sys::iree_status_t {
    // IREE's code-only status representation is the status integer with no
    // diagnostic storage. No allocation or pointer dereference is involved.
    ptr::without_provenance_mut(sys::IREE_STATUS_CANCELLED as usize)
}

/// A relative wait timeout, represented without platform-specific clock units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeout {
    /// Return immediately if completion has not occurred.
    Immediate,
    /// Wait for at most this duration.
    After(Duration),
    /// Wait without a deadline.
    Infinite,
}

impl Timeout {
    pub(crate) fn native(self) -> Result<sys::iree_timeout_t> {
        let nanos = match self {
            Self::Infinite => return Ok(unsafe { sys::iree_infinite_timeout() }),
            Self::Immediate => 0,
            Self::After(duration) => i64::try_from(duration.as_nanos())
                .map_err(|_| Error::invalid("timeout exceeds native nanosecond range"))?,
        };
        Ok(unsafe { sys::iree_make_timeout_ns(nanos) })
    }
}

fn validate_value(value: u64) -> Result<()> {
    if value > sys::IREE_HAL_SEMAPHORE_MAX_VALUE as u64 {
        return Err(Error::invalid(
            "timeline value is reserved for native failure signaling",
        ));
    }

    Ok(())
}

fn validate_software_value(value: u64) -> Result<()> {
    if value > i64::MAX as u64 {
        return Err(Error::invalid(
            "software timeline value exceeds the native signed timeline range",
        ));
    }
    Ok(())
}

struct TimelineInner(NonNull<sys::iree_async_semaphore_t>);

// Native software semaphores provide atomic reference counts and serialize
// timeline/timepoint operations. The task feature requires native threading.
#[cfg(feature = "task")]
unsafe impl Send for TimelineInner {}
#[cfg(feature = "task")]
unsafe impl Sync for TimelineInner {}

impl Drop for TimelineInner {
    fn drop(&mut self) {
        unsafe { sys::iree_async_semaphore_release(self.0.as_ptr()) };
    }
}

/// An owned native software timeline for asynchronous cross-thread coordination.
///
/// Timepoints are completed by native callbacks, without polling threads or a
/// Rust async-runtime dependency. With `task`, clones are `Send + Sync`.
#[derive(Clone)]
pub struct Timeline(TimelineOwnership<TimelineInner>);

impl Timeline {
    /// Create a software timeline at an initial value.
    pub fn new(initial_value: u64) -> Result<Self> {
        validate_software_value(initial_value)?;
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_async_semaphore_create(
                ptr::null_mut(),
                initial_value,
                sys::IREE_ASYNC_SEMAPHORE_DEFAULT_FRONTIER_CAPACITY as u8,
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Self(TimelineOwnership::new(TimelineInner(
            NonNull::new(raw).ok_or_else(|| Error::invalid("IREE returned a null timeline"))?,
        ))))
    }

    /// Query the current value, propagating a sticky failure.
    pub fn value(&self) -> Result<u64> {
        let code = unsafe { sys::iree_async_semaphore_query_status(self.as_raw()) };
        if code != 0 {
            return Err(Error::new(
                StatusCode::from_native(code),
                "native timeline failed",
            ));
        }
        Ok(unsafe { sys::iree_async_semaphore_query(self.as_raw()) })
    }

    /// Advance the timeline. The value must be strictly increasing.
    pub fn signal(&self, value: u64) -> Result<()> {
        validate_software_value(value)?;
        check(unsafe { sys::iree_async_semaphore_signal(self.as_raw(), value, ptr::null()) })
    }

    /// Permanently cancel this timeline and complete all pending waits.
    pub fn cancel(&self) {
        unsafe { sys::iree_async_semaphore_fail(self.as_raw(), cancelled_status()) };
    }

    /// Register a native asynchronous timepoint.
    ///
    /// Dropping the future unregisters its timepoint. It never blocks on device
    /// work. A callback already in flight retains its own completion storage.
    pub fn wait(&self, value: u64) -> Result<Wait<Self>> {
        validate_software_value(value)?;
        Wait::new(self.clone(), self.as_raw(), value)
    }

    /// Borrow the native software semaphore without transferring ownership.
    pub fn as_raw(&self) -> *mut sys::iree_async_semaphore_t {
        self.0.0.as_ptr()
    }
}

/// An owned HAL timeline semaphore bound to a particular device.
///
/// Device and external graphics ownership are retained. Unlike a standalone
/// software timeline, this handle preserves the owning device's thread rules.
pub struct Semaphore {
    raw: NonNull<sys::iree_hal_semaphore_t>,
    pub(crate) device: Device,
}

impl Device {
    /// Create a device-compatible timeline semaphore.
    pub fn semaphore(&self, initial_value: u64) -> Result<Semaphore> {
        validate_value(initial_value)?;
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_semaphore_create(self.as_raw(), u64::MAX, initial_value, 0, &mut raw)
        })?;
        Ok(Semaphore {
            raw: NonNull::new(raw)
                .ok_or_else(|| Error::invalid("IREE returned a null semaphore"))?,
            device: self.clone(),
        })
    }
}

impl Semaphore {
    /// Query the timeline value and propagate any native failure.
    pub fn value(&self) -> Result<u64> {
        let mut value = 0;
        check(unsafe { sys::iree_hal_semaphore_query(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    /// Advance the native device timeline.
    pub fn signal(&self, value: u64) -> Result<()> {
        validate_value(value)?;
        check(unsafe { sys::iree_hal_semaphore_signal(self.as_raw(), value, ptr::null()) })
    }

    /// Block until the timeline reaches a value, with an explicit timeout.
    pub fn wait_blocking(&self, value: u64, timeout: Timeout) -> Result<()> {
        validate_value(value)?;
        check(unsafe { sys::iree_hal_semaphore_wait(self.as_raw(), value, timeout.native()?, 0) })
    }

    /// Await native completion without a blocking worker or periodic polling.
    pub fn wait(&self, value: u64) -> Result<Wait<Self>> {
        validate_value(value)?;
        // HAL semaphores embed the async semaphore at offset zero by contract.
        Wait::new(self.clone(), self.as_raw().cast(), value)
    }

    /// Fail this semaphore and all pending waits with cancellation.
    pub fn cancel(&self) {
        unsafe { sys::iree_hal_semaphore_fail(self.as_raw(), cancelled_status()) };
    }

    /// Borrow the native HAL semaphore without transferring its reference.
    pub fn as_raw(&self) -> *mut sys::iree_hal_semaphore_t {
        self.raw.as_ptr()
    }
}

impl Clone for Semaphore {
    fn clone(&self) -> Self {
        unsafe { sys::iree_hal_semaphore_retain(self.as_raw()) };
        Self {
            raw: self.raw,
            device: self.device.clone(),
        }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe { sys::iree_hal_semaphore_release(self.as_raw()) };
    }
}

#[derive(Default)]
struct Completion {
    result: Option<Result<()>>,
    waker: Option<Waker>,
}

struct Registration {
    timepoint: UnsafeCell<sys::iree_async_semaphore_timepoint_t>,
    completion: Mutex<Completion>,
}

// Only native acquire/cancel/dispatch access the intrusive timepoint, under the
// native semaphore's lock. Rust accesses completion exclusively under its mutex.
unsafe impl Send for Registration {}
unsafe impl Sync for Registration {}

unsafe extern "C" fn complete(
    user_data: *mut c_void,
    _timepoint: *mut sys::iree_async_semaphore_timepoint_t,
    status: sys::iree_status_t,
) {
    let registration = unsafe { Arc::from_raw(user_data.cast::<Registration>()) };
    let result = check(status);
    let waker = {
        let mut completion = registration
            .completion
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        completion.result = Some(result);
        completion.waker.take()
    };
    if let Some(waker) = waker {
        waker.wake();
    }
}

/// A cancellation-safe Rust future driven directly by an IREE timepoint.
///
/// `S` retains the source semaphore and its native/device lifetime. Software
/// timeline futures can move between threads when `task` is enabled; HAL
/// futures retain the device's more restrictive ownership model.
pub struct Wait<S> {
    registration: Arc<Registration>,
    semaphore: NonNull<sys::iree_async_semaphore_t>,
    _source: S,
    completed: bool,
}

#[cfg(feature = "task")]
unsafe impl<S: Send> Send for Wait<S> {}

impl<S> Wait<S> {
    fn new(source: S, semaphore: *mut sys::iree_async_semaphore_t, value: u64) -> Result<Self> {
        let registration = Arc::new(Registration {
            timepoint: UnsafeCell::new(sys::iree_async_semaphore_timepoint_t::default()),
            completion: Mutex::new(Completion::default()),
        });
        let callback_owner = Arc::into_raw(registration.clone());
        unsafe {
            (*registration.timepoint.get()).callback = Some(complete);
            (*registration.timepoint.get()).user_data = callback_owner.cast_mut().cast();
        }
        let status = unsafe {
            sys::iree_async_semaphore_acquire_timepoint(
                semaphore,
                value,
                registration.timepoint.get(),
            )
        };
        if let Err(error) = check(status) {
            // This pin only returns errors before registering a callback.
            unsafe {
                drop(Arc::from_raw(callback_owner));
            }
            return Err(error);
        }
        Ok(Self {
            registration,
            semaphore: NonNull::new(semaphore).unwrap(),
            _source: source,
            completed: false,
        })
    }
}

impl<S> Unpin for Wait<S> {}

impl<S> Future for Wait<S> {
    type Output = Result<()>;
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            return Poll::Ready(Err(Error::invalid(
                "completion future was already consumed",
            )));
        }
        // Waker clone/drop may execute arbitrary executor code. Neither runs
        // while holding the completion lock, including when replacing a waker.
        let mut next_waker = Some(context.waker().clone());
        let result = {
            let mut completion = self
                .registration
                .completion
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            match completion.result.take() {
                Some(result) => Some(result),
                None => {
                    std::mem::swap(&mut completion.waker, &mut next_waker);
                    None
                }
            }
        };
        drop(next_waker);
        if let Some(result) = result {
            self.completed = true;
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}

impl<S> Drop for Wait<S> {
    fn drop(&mut self) {
        if unsafe {
            sys::iree_async_semaphore_cancel_timepoint(
                self.semaphore.as_ptr(),
                self.registration.timepoint.get(),
            )
        } {
            // Cancellation won: native code guarantees the callback cannot run.
            unsafe {
                drop(Arc::from_raw(Arc::as_ptr(&self.registration)));
            }
        }
    }
}
