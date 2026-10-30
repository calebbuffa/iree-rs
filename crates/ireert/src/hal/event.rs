//! HAL event handles recorded by command buffers.

use super::{QueueAffinity, device::Device};
use crate::{Result, check};
use core::marker::PhantomData;
use iree_native as n;

/// An owned `iree_hal_event_t` reference.
pub struct Event {
    raw: *mut n::iree_hal_event_t,
    _not_thread_safe: PhantomData<*mut ()>,
}
impl Event {
    /// Creates an event usable on the selected queues.
    pub fn create(device: &Device, queue_affinity: QueueAffinity) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the device is live and `raw` is writable.
        check(unsafe {
            n::iree_hal_event_create(
                device.as_raw(),
                queue_affinity.bits(),
                n::IREE_HAL_EVENT_FLAG_NONE as n::iree_hal_event_flags_t,
                &mut raw,
            )
        })?;
        // SAFETY: successful creation returns one owned event reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned event reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_event_t) -> Self {
        Self {
            raw,
            _not_thread_safe: PhantomData,
        }
    }

    /// Returns the borrowed native event handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_event_t {
        self.raw
    }

    /// Transfers ownership of the native event handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_event_t {
        core::mem::ManuallyDrop::new(self).raw
    }
}
impl Clone for Event {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_event_retain(self.raw) };
        Self {
            raw: self.raw,
            _not_thread_safe: PhantomData,
        }
    }
}
impl Drop for Event {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_event_release(self.raw) }
    }
}
