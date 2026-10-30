//! HIP driver configuration and external interop.
//!
//! # Laws
//! - External allocations remain owned until the final imported-buffer reference is released.
//! - HIP event destruction is deferred until the corresponding semaphore timeline is reached.
use crate::{Result, check};
use iree_native as n;

/// How HIP command buffers are submitted.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CommandBufferMode {
    /// Record command buffers into HIP graphs.
    #[default]
    Graph,
    /// Issue commands directly against a HIP stream.
    Stream,
}

/// hipMemPool retention and release thresholds.
#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryPoolOptions {
    /// Minimum retained bytes when trimming.
    pub minimum_capacity: u64,
    /// Soft maximum retained bytes after synchronization.
    pub release_threshold: u64,
}
impl MemoryPoolOptions {
    fn raw(self) -> n::iree_hal_hip_memory_pool_params_t {
        n::iree_hal_hip_memory_pool_params_t {
            minimum_capacity: self.minimum_capacity,
            release_threshold: self.release_threshold,
        }
    }
}

/// An externally owned HIP stream supplied to the HIP driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HipStreamHandle(u64);
impl HipStreamHandle {
    /// Adopts an opaque HIP stream handle.
    ///
    /// # Safety
    /// The caller must keep the stream valid until all devices created by the
    /// associated driver have been released.
    pub unsafe fn from_raw(handle: u64) -> Self {
        Self(handle)
    }
    /// Returns the opaque HIP stream value.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

/// HIP device tuning parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceOptions {
    /// Externally owned HIP stream supplied to the device.
    pub external_stream: Option<HipStreamHandle>,
    /// Number of exposed queues.
    pub queue_count: Option<usize>,
    /// Shared arena block size in bytes.
    pub arena_block_size: Option<usize>,
    /// Capacity of host and device event pools.
    pub event_pool_capacity: Option<usize>,
    /// Command buffer recording and execution mode.
    pub command_buffer_mode: Option<CommandBufferMode>,
    /// Command-buffer stream tracing level when tracing is active.
    pub stream_tracing: Option<i32>,
    /// Enables asynchronous memory allocation.
    pub async_allocations: Option<bool>,
    /// Asynchronous file-transfer staging size in bytes.
    pub file_transfer_buffer_size: Option<u64>,
    /// Maximum chunk size of asynchronous file transfers in bytes.
    pub file_transfer_chunk_size: Option<u64>,
    /// Device-local allocation pool settings.
    pub device_local_pool: Option<MemoryPoolOptions>,
    /// Host-visible/local allocation pool settings.
    pub other_pool: Option<MemoryPoolOptions>,
    /// Enables inline command-buffer execution.
    pub allow_inline_execution: Option<bool>,
    /// Enables asynchronous caching.
    pub async_caching: Option<bool>,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_hip_device_params_t {
        // SAFETY: plain-data parameters; the C initializer then fills every field.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_hip_device_params_initialize(&mut raw) };
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
                CommandBufferMode::Graph => n::IREE_HAL_HIP_COMMAND_BUFFER_MODE_GRAPH,
                CommandBufferMode::Stream => n::IREE_HAL_HIP_COMMAND_BUFFER_MODE_STREAM,
            };
        }
        if let Some(value) = self.stream_tracing {
            raw.stream_tracing = value;
        }
        if let Some(value) = self.async_allocations {
            raw.async_allocations = value;
        }
        if let Some(value) = self.file_transfer_buffer_size {
            raw.file_transfer_buffer_size = value;
        }
        if let Some(value) = self.file_transfer_chunk_size {
            raw.file_transfer_chunk_size = value;
        }
        if let Some(value) = self.device_local_pool {
            raw.memory_pools.device_local = value.raw();
        }
        if let Some(value) = self.other_pool {
            raw.memory_pools.other = value.raw();
        }
        if let Some(value) = self.allow_inline_execution {
            raw.allow_inline_execution = value;
        }
        if let Some(value) = self.async_caching {
            raw.async_caching = value;
        }
        if let Some(stream) = self.external_stream {
            raw.external_stream = stream.0;
        }
        raw
    }
}

/// HIP driver configuration.
#[derive(Clone, Debug, Default)]
pub struct DriverOptions {
    /// Index of the default HIP device.
    pub default_device_index: Option<i32>,
    /// Search paths for the HIP runtime library.
    pub hip_library_search_paths: Vec<String>,
    /// Device parameters used for devices created by this driver.
    pub device: DeviceOptions,
}

/// Creates a HIP driver returning the generic HAL driver wrapper.
pub fn create_driver(identifier: &str, options: &DriverOptions) -> Result<super::super::Driver> {
    // SAFETY: plain-data options; the C initializer then fills every field.
    let mut raw_options = unsafe { core::mem::zeroed() };
    // SAFETY: `raw_options` is writable.
    unsafe { n::iree_hal_hip_driver_options_initialize(&mut raw_options) };
    if let Some(index) = options.default_device_index {
        raw_options.default_device_index = index;
    }
    let views: Vec<_> = options
        .hip_library_search_paths
        .iter()
        .map(|path| crate::view(path))
        .collect();
    raw_options.hip_lib_search_paths = if views.is_empty() {
        core::ptr::null_mut()
    } else {
        views.as_ptr() as *mut _
    };
    raw_options.hip_lib_search_path_count = views.len();
    let params = options.device.raw();
    let mut raw = core::ptr::null_mut();
    // SAFETY: views, strings, and option structures remain live for the call.
    check(unsafe {
        n::iree_hal_hip_driver_create(
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
