//! CUDA driver configuration.
//!
//! Unset options keep the native defaults.
use crate::{Result, check};
use iree_native as n;

/// How CUDA command buffers are submitted.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CommandBufferMode {
    /// Record command buffers into CUDA graphs.
    #[default]
    Graph,
    /// Issue commands directly against a CUDA stream.
    Stream,
}

/// CUmemoryPool retention and release thresholds.
#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryPoolOptions {
    /// Minimum retained bytes when trimming.
    pub minimum_capacity: u64,
    /// Soft maximum retained bytes after synchronization.
    pub release_threshold: u64,
}
impl MemoryPoolOptions {
    fn raw(self) -> n::iree_hal_cuda_memory_pool_params_t {
        n::iree_hal_cuda_memory_pool_params_t {
            minimum_capacity: self.minimum_capacity,
            release_threshold: self.release_threshold,
        }
    }
}

/// CUDA device tuning parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceOptions {
    /// Number of exposed queues.
    pub queue_count: Option<usize>,
    /// Shared arena block size in bytes.
    pub arena_block_size: Option<usize>,
    /// Capacity of the host and device event pools.
    pub event_pool_capacity: Option<usize>,
    /// Command buffer recording and execution mode.
    pub command_buffer_mode: Option<CommandBufferMode>,
    /// Command-buffer stream tracing level when tracing is active.
    pub stream_tracing: Option<i32>,
    /// Enables asynchronous memory allocation.
    pub async_allocations: Option<bool>,
    /// Device-local allocation pool settings.
    pub device_local_pool: Option<MemoryPoolOptions>,
    /// Host-visible/local allocation pool settings.
    pub other_pool: Option<MemoryPoolOptions>,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_cuda_device_params_t {
        // SAFETY: plain-data parameters; the C initializer then fills every field.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_cuda_device_params_initialize(&mut raw) };
        if let Some(value) = self.queue_count {
            raw.queue_count = value;
        }
        if let Some(value) = self.arena_block_size {
            raw.arena_block_size = value;
        }
        if let Some(value) = self.event_pool_capacity {
            raw.event_pool_capacity = value;
        }
        if let Some(mode) = self.command_buffer_mode {
            raw.command_buffer_mode = match mode {
                CommandBufferMode::Graph => n::IREE_HAL_CUDA_COMMAND_BUFFER_MODE_GRAPH,
                CommandBufferMode::Stream => n::IREE_HAL_CUDA_COMMAND_BUFFER_MODE_STREAM,
            };
        }
        if let Some(value) = self.stream_tracing {
            raw.stream_tracing = value;
        }
        if let Some(value) = self.async_allocations {
            raw.async_allocations = value;
        }
        if let Some(value) = self.device_local_pool {
            raw.memory_pools.device_local = value.raw();
        }
        if let Some(value) = self.other_pool {
            raw.memory_pools.other = value.raw();
        }
        raw
    }
}

/// CUDA driver configuration.
#[derive(Clone, Copy, Debug, Default)]
pub struct DriverOptions {
    /// Index of the default CUDA device.
    pub default_device_index: Option<i32>,
    /// Device parameters used for devices created by this driver.
    pub device: DeviceOptions,
}

/// Creates a CUDA driver returning the generic HAL driver wrapper.
pub fn create_driver(identifier: &str, options: &DriverOptions) -> Result<super::super::Driver> {
    // SAFETY: plain-data options; the C initializer then fills every field.
    let mut raw_options = unsafe { core::mem::zeroed() };
    // SAFETY: `raw_options` is writable.
    unsafe { n::iree_hal_cuda_driver_options_initialize(&mut raw_options) };
    if let Some(index) = options.default_device_index {
        raw_options.default_device_index = index;
    }
    let params = options.device.raw();
    let mut raw = core::ptr::null_mut();
    // SAFETY: identifier and option structures are valid for the duration of the call.
    check(unsafe {
        n::iree_hal_cuda_driver_create(
            crate::view(identifier),
            &raw_options,
            &params,
            crate::system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns an owned generic HAL driver.
    Ok(unsafe { super::super::Driver::from_raw(raw) })
}
