//! Metal driver configuration and interop entry points.
//!
//! # Laws
//! - Metal handles are opaque and borrowed handles are valid only through a retaining lease.
//! - Imported resources remain externally owned until IREE releases their final reference.
use crate::{Result, check};
use iree_native as n;

/// Metal command-dispatch ordering.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum CommandDispatchType {
    /// Dispatch commands concurrently within a command buffer.
    #[default]
    Concurrent = n::IREE_HAL_METAL_COMMAND_DISPATCH_TYPE_CONCURRENT,
    /// Dispatch commands sequentially.
    Serial = n::IREE_HAL_METAL_COMMAND_DISPATCH_TYPE_SERIAL,
}

/// Whether command buffers retain their referenced resources.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum ResourceReferenceMode {
    /// Do not hold Metal references to resources used by commands.
    #[default]
    Unretained = n::IREE_HAL_METAL_COMMAND_BUFFER_RESOURCE_REFERENCE_MODE_UNRETAINED,
    /// Hold strong references to command resources.
    Retained = n::IREE_HAL_METAL_COMMAND_BUFFER_RESOURCE_REFERENCE_MODE_RETAINED,
}

/// Metal resource hazard checking policy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum HazardTrackingMode {
    /// Leave resource synchronization to the caller and IREE.
    #[default]
    Untracked = n::IREE_HAL_METAL_RESOURCE_HAZARD_TRACKING_MODE_UNTRACKED,
    /// Use Metal's runtime hazard tracking.
    Tracked = n::IREE_HAL_METAL_RESOURCE_HAZARD_TRACKING_MODE_TRACKED,
}

/// Metal device tuning parameters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeviceOptions {
    /// Arena block size in bytes; `None` keeps the native default.
    pub arena_block_size: Option<usize>,
    /// Per-queue uniform buffer capacity in bytes; `None` keeps the native default.
    pub queue_uniform_buffer_size: Option<usize>,
    /// Dispatch ordering for commands in a command buffer.
    pub command_dispatch_type: CommandDispatchType,
    /// Resource retention policy for command buffers.
    pub resource_reference_mode: ResourceReferenceMode,
    /// Resource hazard tracking policy.
    pub hazard_tracking_mode: HazardTrackingMode,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_metal_device_params_t {
        // SAFETY: plain-data parameters; the C initializer then fills every field.
        let mut raw: n::iree_hal_metal_device_params_t = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_metal_device_params_initialize(&mut raw) };
        if let Some(size) = self.arena_block_size {
            raw.arena_block_size = size;
        }
        if let Some(size) = self.queue_uniform_buffer_size {
            raw.queue_uniform_buffer_size = size;
        }
        raw.command_dispatch_type = self.command_dispatch_type as _;
        raw.command_buffer_resource_reference_mode = self.resource_reference_mode as _;
        raw.resource_hazard_tracking_mode = self.hazard_tracking_mode as _;
        raw
    }
}

/// Creates a Metal driver returning the generic HAL driver wrapper.
pub fn create_driver(identifier: &str, options: &DeviceOptions) -> Result<super::super::Driver> {
    let params = options.raw();
    let mut raw = core::ptr::null_mut();
    // SAFETY: identifier and parameters remain valid for the duration of the call.
    check(unsafe {
        n::iree_hal_metal_driver_create(crate::view(identifier), &params, crate::system(), &mut raw)
    })?;
    // SAFETY: successful creation returns an owned generic HAL driver.
    Ok(unsafe { super::super::Driver::from_raw(raw) })
}
