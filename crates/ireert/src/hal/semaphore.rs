//! HAL timeline semaphores.
//!
//! A semaphore's payload is monotonic: after `signal(v)`, `query() >= v`.
//! Creation flags whose violation is undefined behavior are only reachable
//! through `unsafe` constructors.

use super::bitfield::bitfield;
use super::{QueueAffinity, SemaphorePoint, WaitFlags, device::Device};
use crate::{Code, Error, Result, check};
use core::{marker::PhantomData, ptr, time::Duration};
use iree_native as n;

bitfield! {
    /// Semaphore creation behavior (`iree_hal_semaphore_flags_t`).
    SemaphoreFlags: n::iree_hal_semaphore_flags_t {
        /// Only used on the creating device and its selected queues. Caller contract.
        DEVICE_LOCAL = n::IREE_HAL_SEMAPHORE_FLAG_DEVICE_LOCAL,
        /// Host waits use platform interrupts instead of spinning.
        HOST_INTERRUPT = n::IREE_HAL_SEMAPHORE_FLAG_HOST_INTERRUPT,
        /// Hint that the whole semaphore may be exported to another API.
        EXPORTABLE = n::IREE_HAL_SEMAPHORE_FLAG_EXPORTABLE,
        /// Individual timepoints may be exported (see [`crate::hal::timepoint`]).
        EXPORTABLE_TIMEPOINTS = n::IREE_HAL_SEMAPHORE_FLAG_EXPORTABLE_TIMEPOINTS,
        /// Every signal comes from one logical producer. Caller contract.
        SINGLE_PRODUCER = n::IREE_HAL_SEMAPHORE_FLAG_SINGLE_PRODUCER,
        /// The runtime default: host interrupts and exportable timepoints.
        DEFAULT = n::IREE_HAL_SEMAPHORE_FLAG_DEFAULT,
    }
}

bitfield! {
    /// Operations a device or external timepoint may perform on a semaphore
    /// (`iree_hal_semaphore_compatibility_t`).
    SemaphoreCompatibility: n::iree_hal_semaphore_compatibility_t {
        /// The host may wait.
        HOST_WAIT = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_HOST_WAIT,
        /// The device may wait.
        DEVICE_WAIT = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_DEVICE_WAIT,
        /// The host may signal.
        HOST_SIGNAL = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_HOST_SIGNAL,
        /// The device may signal.
        DEVICE_SIGNAL = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_DEVICE_SIGNAL,
        /// Host wait and signal.
        HOST_ONLY = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_HOST_ONLY,
        /// Device wait and signal.
        DEVICE_ONLY = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_DEVICE_ONLY,
        /// Every operation.
        ALL = n::IREE_HAL_SEMAPHORE_COMPATIBILITY_ALL,
    }
}

/// A retained timeline semaphore.
#[repr(transparent)]
pub struct Semaphore(*mut n::iree_hal_semaphore_t);
impl Semaphore {
    /// Native failure payload sentinel (`IREE_HAL_SEMAPHORE_FAILURE_VALUE`).
    pub const FAILURE_VALUE: u64 = n::IREE_HAL_SEMAPHORE_MAX_VALUE as u64 + 1;
    /// Creates a semaphore with [`SemaphoreFlags::DEFAULT`].
    pub fn new(device: &Device, affinity: QueueAffinity, initial_value: u64) -> Result<Self> {
        Self::with_flags(device, affinity, initial_value, SemaphoreFlags::DEFAULT)
    }

    /// Creates a semaphore with explicit behavior flags.
    ///
    /// Rejects [`SemaphoreFlags::DEVICE_LOCAL`] and [`SemaphoreFlags::SINGLE_PRODUCER`]:
    /// both move an undefined-behavior contract onto the caller; use
    /// [`Semaphore::with_flags_unchecked`].
    pub fn with_flags(
        device: &Device,
        affinity: QueueAffinity,
        initial_value: u64,
        flags: SemaphoreFlags,
    ) -> Result<Self> {
        if flags.intersects(SemaphoreFlags::DEVICE_LOCAL | SemaphoreFlags::SINGLE_PRODUCER) {
            return Err(Error::new(
                Code::InvalidArgument,
                "DEVICE_LOCAL and SINGLE_PRODUCER semaphores require with_flags_unchecked",
            ));
        }
        // SAFETY: the flags carrying caller contracts were rejected above.
        unsafe { Self::with_flags_unchecked(device, affinity, initial_value, flags) }
    }

    /// Creates a semaphore with any behavior flags.
    ///
    /// # Safety
    /// With [`SemaphoreFlags::DEVICE_LOCAL`] the semaphore must only be used on
    /// `device` (and only on `affinity` queues when it names specific queues).
    /// With [`SemaphoreFlags::SINGLE_PRODUCER`] every signal, host or device,
    /// must come from one logical producer timeline.
    pub unsafe fn with_flags_unchecked(
        device: &Device,
        affinity: QueueAffinity,
        initial_value: u64,
        flags: SemaphoreFlags,
    ) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: the device is live and `out` is writable.
        check(unsafe {
            n::iree_hal_semaphore_create(
                device.as_raw(),
                affinity.bits(),
                initial_value,
                flags.bits(),
                &mut out,
            )
        })?;
        // SAFETY: successful creation returns one owned semaphore reference.
        Ok(unsafe { Self::from_raw(out) })
    }

    /// Returns the ways `device` may use this semaphore.
    pub fn compatibility_with(&self, device: &Device) -> SemaphoreCompatibility {
        // SAFETY: both objects are live.
        SemaphoreCompatibility::from_bits_retain(unsafe {
            n::iree_hal_device_query_semaphore_compatibility(device.as_raw(), self.0)
        })
    }

    /// Adopts one owned semaphore reference.
    ///
    /// # Safety
    /// `raw` must be a live semaphore pointer whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_semaphore_t) -> Self {
        Self(raw)
    }

    /// Borrows the native semaphore pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_semaphore_t {
        self.0
    }

    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_semaphore_t {
        core::mem::ManuallyDrop::new(self).0
    }
    /// Runs `callback` once this semaphore reaches `value` or fails.
    ///
    /// The callback may run immediately, or later on any thread, and must not block.
    pub fn on_value<F>(&self, value: u64, callback: F) -> Result<crate::asynchronous::Timepoint>
    where
        F: FnOnce(Result<()>) + Send + 'static,
    {
        // SAFETY: the semaphore is live.
        unsafe {
            crate::asynchronous::Timepoint::register(self.0.cast(), value, Box::new(callback))
        }
    }

    /// Signals `target` to `target_value` once this semaphore reaches `value`,
    /// propagating failure. Dropping the returned timepoint cancels the relay.
    pub fn link(
        &self,
        value: u64,
        target: &Self,
        target_value: u64,
    ) -> Result<crate::asynchronous::Timepoint> {
        // SAFETY: both semaphores are live.
        unsafe {
            crate::asynchronous::timepoint::link(
                self.0.cast(),
                value,
                target.0.cast(),
                target_value,
            )
        }
    }

    /// Signals this semaphore to `value` once the platform `fence` signals;
    /// `proactor` takes ownership of the fence on success.
    ///
    /// # Safety
    /// Keep the proactor, the device group and every signal-path dependency
    /// alive until completion or cancellation by proactor shutdown.
    pub unsafe fn import_fence(
        &self,
        proactor: &crate::asynchronous::Proactor,
        fence: crate::asynchronous::Primitive,
        value: u64,
    ) -> Result<()> {
        // SAFETY: the semaphore is live; the caller preserves signal dependencies.
        unsafe {
            crate::asynchronous::timepoint::import_fence(self.0.cast(), proactor, fence, value)
        }
    }

    /// Exports a platform fence (eventfd, sync file or Win32 event) that
    /// signals once this semaphore reaches `value`.
    ///
    /// # Safety
    /// Do not close the returned primitive until the semaphore reaches
    /// `value` or fails: a pending native callback may write to its handle.
    pub unsafe fn export_fence(
        &self,
        proactor: &crate::asynchronous::Proactor,
        value: u64,
    ) -> Result<crate::asynchronous::Primitive> {
        // SAFETY: the semaphore is live; the caller preserves the pending handle.
        unsafe { crate::asynchronous::timepoint::export_fence(self.0.cast(), proactor, value) }
    }

    /// Returns a wait source for the semaphore reaching `value`.
    pub fn await_value(&self, value: u64) -> crate::base::WaitSource<'_> {
        // SAFETY: the semaphore is live.
        let raw = unsafe { n::iree_hal_semaphore_await(self.0, value) };
        // SAFETY: the wait source borrows this semaphore.
        unsafe { crate::base::WaitSource::from_raw(raw, self) }
    }

    /// Returns the current payload value, or the failure the semaphore carries.
    pub fn query(&self) -> Result<u64> {
        let (status, value) = self.query_with_value();
        status.map(|()| value)
    }

    /// Returns both the native payload written to `out_value` and its status.
    ///
    /// Unlike [`query`](Self::query), this preserves the native output value
    /// when the semaphore has failed, matching the C API's out-parameter
    /// semantics for callers that need to inspect the failure payload.
    pub fn query_with_value(&self) -> (Result<()>, u64) {
        let mut value = 0;
        // SAFETY: the semaphore is live and `value` is writable.
        let status = check(unsafe { n::iree_hal_semaphore_query(self.0, &mut value) });
        (status, value)
    }

    /// Signals the semaphore with a monotonically increasing payload value.
    pub fn signal(&self, value: u64) -> Result<()> {
        // SAFETY: the semaphore is live; a null frontier is allowed.
        check(unsafe { n::iree_hal_semaphore_signal(self.0, value, ptr::null()) })
    }

    /// Signals `value`, merging `frontier` so waiters inherit its causal history.
    pub fn signal_with_frontier(
        &self,
        value: u64,
        frontier: &crate::asynchronous::frontier::Frontier,
    ) -> Result<()> {
        // SAFETY: the semaphore and frontier are live for the call.
        check(unsafe { n::iree_hal_semaphore_signal(self.0, value, frontier.as_raw()) })
    }

    /// Fails the semaphore with `code`; waiters and later queries observe the failure.
    pub fn fail(&self, code: Code) {
        // SAFETY: the semaphore is live; ownership of the status passes to IREE.
        unsafe { n::iree_hal_semaphore_fail(self.0, n::abi::status::from_code(code.to_raw())) }
    }

    /// Blocks until the payload reaches `value` or `timeout` (`None` waits forever) expires.
    pub fn wait(&self, value: u64, timeout: Option<Duration>, flags: WaitFlags) -> Result<()> {
        let timeout = n::timeout(timeout)?;
        // SAFETY: the semaphore is live.
        check(unsafe { n::iree_hal_semaphore_wait(self.0, value, timeout, flags.bits()) })
    }
}
// SAFETY: upstream semaphore operations and reference counting are thread-safe.
unsafe impl Send for Semaphore {}
// SAFETY: upstream semaphore operations and reference counting are thread-safe.
unsafe impl Sync for Semaphore {}
impl Clone for Semaphore {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_semaphore_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for Semaphore {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_semaphore_release(self.0) }
    }
}

/// A borrowed native semaphore list built from queue dependency points.
pub(crate) struct SemaphoreList<'a> {
    semaphores: Vec<*mut n::iree_hal_semaphore_t>,
    values: Vec<u64>,
    _points: PhantomData<&'a Semaphore>,
}
impl<'a> SemaphoreList<'a> {
    pub(crate) fn from_points(points: &[SemaphorePoint<'a>]) -> Self {
        Self {
            semaphores: points.iter().map(|p| p.semaphore.0).collect(),
            values: points.iter().map(|p| p.value).collect(),
            _points: PhantomData,
        }
    }

    /// Returns the native list view; IREE only reads through these pointers.
    pub(crate) fn as_raw(&self) -> n::iree_hal_semaphore_list_t {
        n::iree_hal_semaphore_list_t {
            count: self.semaphores.len(),
            semaphores: if self.semaphores.is_empty() {
                core::ptr::null_mut()
            } else {
                self.semaphores.as_ptr().cast_mut()
            },
            payload_values: if self.values.is_empty() {
                core::ptr::null_mut()
            } else {
                self.values.as_ptr().cast_mut()
            },
        }
    }
}
