//! WebGPU driver creation and externally supplied device wrapping.
use crate::{Result, check};
use iree_native as n;

/// An opaque WebGPU handle-table identifier supplied by the embedding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WebGpuDeviceHandle(u32);
impl WebGpuDeviceHandle {
    /// Constructs a handle from the embedding's 32-bit handle-table identifier.
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    /// Returns the opaque handle-table identifier.
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// Creates a WebGPU driver returning the generic HAL driver wrapper.
pub fn create_driver(identifier: &str) -> Result<super::super::Driver> {
    // SAFETY: plain-data options; the C initializer then fills every field.
    let mut options = unsafe { core::mem::zeroed::<n::iree_hal_webgpu_driver_options_t>() };
    // SAFETY: `options` is writable.
    unsafe { n::iree_hal_webgpu_driver_options_initialize(&mut options) };
    let mut raw = core::ptr::null_mut();
    // SAFETY: identifier and options are live for the call.
    check(unsafe {
        n::iree_hal_webgpu_driver_create(
            crate::view(identifier),
            &options,
            crate::system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns an owned generic HAL driver.
    Ok(unsafe { super::super::Driver::from_raw(raw) })
}

/// Wraps an externally owned WebGPU device handle as a generic HAL device.
///
/// # Safety
/// The embedding must keep the WebGPU `GPUDevice` corresponding to `handle`
/// alive for at least as long as the returned HAL device.
pub unsafe fn wrap_device(
    identifier: &str,
    handle: WebGpuDeviceHandle,
) -> Result<super::super::Device> {
    let mut raw = core::ptr::null_mut();
    // SAFETY: the caller guarantees the external GPUDevice outlives the HAL device.
    check(unsafe {
        n::iree_hal_webgpu_device_wrap(crate::view(identifier), handle.0, crate::system(), &mut raw)
    })?;
    // SAFETY: successful wrapping returns an owned generic HAL device.
    Ok(unsafe { super::super::Device::from_raw(raw) })
}
