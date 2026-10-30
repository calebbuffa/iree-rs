//! MPI collective channel-provider ownership.
use crate::hal::Device;
use crate::{Result, check};
use iree_native as n;

/// A retained MPI channel provider; dropping its final reference finalizes MPI.
pub struct MpiChannelProvider(*mut n::iree_hal_channel_provider_t);
impl MpiChannelProvider {
    /// Reports whether the process environment indicates an MPI launch.
    pub fn is_configured() -> bool {
        // SAFETY: this query has no pointers or ownership effects.
        unsafe { n::iree_hal_mpi_is_configured() }
    }
    /// Initializes an MPI channel provider.
    ///
    /// The provider must be shared across devices: final release finalizes MPI,
    /// and MPI cannot be initialized again in this process afterward.
    pub fn new() -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: output pointer is valid and the system allocator is process-safe.
        check(unsafe { n::iree_hal_mpi_channel_provider_create(crate::system(), &mut raw) })?;
        // SAFETY: successful creation returns one owned provider reference.
        Ok(unsafe { Self::from_raw(raw) })
    }
    /// Adopts one owned channel-provider reference.
    ///
    /// # Safety
    /// `raw` must be a live provider whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_channel_provider_t) -> Self {
        Self(raw)
    }
    /// Borrows the native channel-provider pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_channel_provider_t {
        self.0
    }
    /// Transfers the provider reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_channel_provider_t {
        core::mem::ManuallyDrop::new(self).0
    }
    /// Makes this provider the source of `device`'s collective channels.
    ///
    /// The device retains its own reference to the provider.
    pub fn install(&self, device: &Device) {
        // SAFETY: both objects are live; the device retains the provider.
        unsafe { n::iree_hal_device_replace_channel_provider(device.as_raw(), self.0) }
    }
}
impl Clone for MpiChannelProvider {
    fn clone(&self) -> Self {
        // SAFETY: retain the live provider before creating another owner.
        unsafe { n::iree_hal_channel_provider_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for MpiChannelProvider {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one provider reference.
        unsafe { n::iree_hal_channel_provider_release(self.0) };
    }
}
