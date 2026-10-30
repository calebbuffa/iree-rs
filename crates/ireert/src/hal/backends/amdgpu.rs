//! AMDGPU driver configuration, HSA runtime loading and agent topology.
//!
//! Unset options keep the native defaults.
use crate::{Result, check};
use iree_native as n;

/// AMDGPU queue placement policy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum QueuePlacement {
    /// Allow IREE to select the supported placement.
    #[default]
    Any = n::IREE_HAL_AMDGPU_QUEUE_PLACEMENT_ANY,
    /// Execute queue operations on host workers.
    Host = n::IREE_HAL_AMDGPU_QUEUE_PLACEMENT_HOST,
    /// Request the currently unsupported device-side queue path.
    Device = n::IREE_HAL_AMDGPU_QUEUE_PLACEMENT_DEVICE,
}

/// AMDGPU command-buffer encoding and execution mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum CommandBufferMode {
    /// Use AQL command-buffer programs.
    #[default]
    Aql = n::IREE_HAL_AMDGPU_COMMAND_BUFFER_MODE_AQL,
    /// Use resident PM4 instruction buffers.
    Pm4 = n::IREE_HAL_AMDGPU_COMMAND_BUFFER_MODE_PM4,
    /// Select PM4 only on validated hardware, otherwise use AQL.
    Auto = n::IREE_HAL_AMDGPU_COMMAND_BUFFER_MODE_AUTO,
}

/// Publication strategy for resident PM4 command buffers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum Pm4PublicationMode {
    /// Reserved direct-resident mode, rejected by driver validation.
    #[default]
    Direct = n::IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_DIRECT,
    /// Publish staging segments with host copies.
    HostCopy = n::IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_HOST_COPY,
    /// Publish the resident image with an asynchronous copy.
    HostAsyncCopy = n::IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_HOST_ASYNC_COPY,
    /// Return after launching the asynchronous staging copy.
    HostAsyncCopyNonblocking =
        n::IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_HOST_ASYNC_COPY_NONBLOCKING,
}

/// Host block-pool sizing parameters for an AMDGPU logical device.
#[derive(Clone, Copy, Debug, Default)]
pub struct HostBlockPoolOptions {
    /// Small block size in bytes.
    pub small_block_size: Option<usize>,
    /// Large block size in bytes.
    pub large_block_size: Option<usize>,
    /// Command-buffer recording block capacity in bytes.
    pub command_buffer_usable_block_size: Option<usize>,
}

/// Device block-pool sizing parameters for an AMDGPU logical device.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceBlockPoolOptions {
    /// Small block size in bytes.
    pub small_block_size: Option<u64>,
    /// Initial small block-pool capacity.
    pub small_initial_capacity: Option<usize>,
    /// Large block size in bytes.
    pub large_block_size: Option<u64>,
    /// Initial large block-pool capacity.
    pub large_initial_capacity: Option<usize>,
}

/// Default queue-allocation TLSF pool parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultPoolOptions {
    /// Logical byte length of the pool range per physical device.
    pub range_length: Option<u64>,
    /// Minimum reservation alignment in bytes.
    pub alignment: Option<u64>,
    /// Maximum death-frontier entries per free TLSF block.
    pub frontier_capacity: Option<u8>,
}

/// Per-physical-device host queue ring capacities.
#[derive(Clone, Copy, Debug, Default)]
pub struct HostQueueOptions {
    /// AQL ring capacity in packets.
    pub aql_capacity: Option<u32>,
    /// Completion/reclaim notification ring capacity.
    pub notification_capacity: Option<u32>,
    /// Kernarg ring capacity in 64-byte blocks.
    pub kernarg_capacity: Option<u32>,
    /// Device-visible control upload ring capacity in bytes.
    pub upload_capacity: Option<u32>,
}

/// Typed options for an AMDGPU logical device.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceOptions {
    /// Host block-pool sizing.
    pub host_block_pools: HostBlockPoolOptions,
    /// Device block-pool sizing.
    pub device_block_pools: DeviceBlockPoolOptions,
    /// Default queue-allocation pool settings.
    pub default_pool: DefaultPoolOptions,
    /// Queue scheduling location.
    pub queue_placement: Option<QueuePlacement>,
    /// Command-buffer encoding and execution path.
    pub command_buffer_mode: Option<CommandBufferMode>,
    /// PM4 command-buffer storage publication strategy.
    pub pm4_publication_mode: Option<Pm4PublicationMode>,
    /// Per-physical-device host queue ring capacities.
    pub host_queues: HostQueueOptions,
    /// Preallocate a reasonable number of device resources.
    pub preallocate_pools: Option<bool>,
    /// Reserved exclusive queue scheduling mode.
    pub exclusive_execution: Option<bool>,
    /// Force cross-queue wait barriers through software deferral.
    pub force_wait_barrier_defer: Option<bool>,
    /// Allow experimental PM4 command-buffer hardware targets.
    pub enable_experimental_pm4_command_buffers: Option<bool>,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_amdgpu_logical_device_options_t {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_amdgpu_logical_device_options_initialize(&mut raw) };
        if let Some(value) = self.host_block_pools.small_block_size {
            raw.host_block_pools.small.block_size = value;
        }
        if let Some(value) = self.host_block_pools.large_block_size {
            raw.host_block_pools.large.block_size = value;
        }
        if let Some(value) = self.host_block_pools.command_buffer_usable_block_size {
            raw.host_block_pools.command_buffer.usable_block_size = value;
        }
        if let Some(value) = self.device_block_pools.small_block_size {
            raw.device_block_pools.small.block_size = value;
        }
        if let Some(value) = self.device_block_pools.small_initial_capacity {
            raw.device_block_pools.small.initial_capacity = value;
        }
        if let Some(value) = self.device_block_pools.large_block_size {
            raw.device_block_pools.large.block_size = value;
        }
        if let Some(value) = self.device_block_pools.large_initial_capacity {
            raw.device_block_pools.large.initial_capacity = value;
        }
        if let Some(value) = self.default_pool.range_length {
            raw.default_pool.range_length = value;
        }
        if let Some(value) = self.default_pool.alignment {
            raw.default_pool.alignment = value;
        }
        if let Some(value) = self.default_pool.frontier_capacity {
            raw.default_pool.frontier_capacity = value;
        }
        if let Some(value) = self.queue_placement {
            raw.queue_placement = value as _;
        }
        if let Some(value) = self.command_buffer_mode {
            raw.command_buffer_mode = value as _;
        }
        if let Some(value) = self.pm4_publication_mode {
            raw.pm4_command_buffer_publication_mode = value as _;
        }
        if let Some(value) = self.host_queues.aql_capacity {
            raw.host_queues.aql_capacity = value;
        }
        if let Some(value) = self.host_queues.notification_capacity {
            raw.host_queues.notification_capacity = value;
        }
        if let Some(value) = self.host_queues.kernarg_capacity {
            raw.host_queues.kernarg_capacity = value;
        }
        if let Some(value) = self.host_queues.upload_capacity {
            raw.host_queues.upload_capacity = value;
        }
        if let Some(value) = self.preallocate_pools {
            raw.set_preallocate_pools(u64::from(value));
        }
        if let Some(value) = self.exclusive_execution {
            raw.set_exclusive_execution(u64::from(value));
        }
        if let Some(value) = self.force_wait_barrier_defer {
            raw.set_force_wait_barrier_defer(u64::from(value));
        }
        if let Some(value) = self.enable_experimental_pm4_command_buffers {
            raw.set_enable_experimental_pm4_command_buffers(u64::from(value));
        }
        raw
    }
}

/// AMDGPU driver options matching `iree_hal_amdgpu_driver_options_t`.
#[derive(Clone, Debug, Default)]
pub struct DriverOptions {
    /// Directories or file paths searched for the HSA runtime shared library.
    pub libhsa_search_paths: Vec<String>,
    /// Default logical-device options used when a device is created without explicit options.
    pub default_device: DeviceOptions,
}

/// A loaded HSA runtime library with RAII deinitialization.
pub struct LibHsa(n::iree_hal_amdgpu_libhsa_t);
impl LibHsa {
    /// Loads and initializes the HSA runtime from configured search paths.
    pub fn new(search_paths: &[String]) -> Result<Self> {
        let views: Vec<_> = search_paths.iter().map(|path| crate::view(path)).collect();
        let paths = n::iree_string_view_list_t {
            count: views.len(),
            values: if views.is_empty() {
                core::ptr::null()
            } else {
                views.as_ptr()
            },
        };
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: output value is writable and the path backing storage lives through initialization.
        check(unsafe {
            n::iree_hal_amdgpu_libhsa_initialize(
                n::IREE_HAL_AMDGPU_LIBHSA_FLAG_NONE as _,
                paths,
                crate::system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }
    /// Adopts an initialized HSA library value.
    ///
    /// # Safety
    /// `raw` must be initialized and its deinitialization ownership transferred here.
    pub unsafe fn from_raw(raw: n::iree_hal_amdgpu_libhsa_t) -> Self {
        Self(raw)
    }
    /// Borrows the native HSA library value.
    pub fn as_raw(&self) -> *const n::iree_hal_amdgpu_libhsa_t {
        &self.0
    }
    /// Transfers the initialized HSA library value to the caller.
    pub fn into_raw(self) -> n::iree_hal_amdgpu_libhsa_t {
        let this = core::mem::ManuallyDrop::new(self);
        // SAFETY: ManuallyDrop prevents the transferred resource from being deinitialized.
        unsafe { core::ptr::read(&this.0) }
    }
}
impl LibHsa {
    /// Retains the loaded HSA runtime state.
    pub fn try_clone(&self) -> Result<Self> {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: the source remains live and the output is uninitialized storage.
        check(unsafe { n::iree_hal_amdgpu_libhsa_copy(&self.0, &mut raw) })?;
        Ok(Self(raw))
    }
}
impl Drop for LibHsa {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one initialized HSA library value.
        unsafe { n::iree_hal_amdgpu_libhsa_deinitialize(&mut self.0) };
    }
}

/// An initialized AMDGPU topology with deterministic deinitialization.
pub struct Topology(n::iree_hal_amdgpu_topology_t);
impl Topology {
    /// Initializes a topology from the HSA runtime's visible-device defaults.
    pub fn from_defaults(libhsa: &LibHsa) -> Result<Self> {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: output value is writable and the HSA library remains live.
        check(unsafe {
            n::iree_hal_amdgpu_topology_initialize_with_defaults(&libhsa.0, &mut raw)
        })?;
        Ok(Self(raw))
    }
    /// Initializes a topology from a comma-separated device path.
    pub fn from_path(libhsa: &LibHsa, path: &str) -> Result<Self> {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: path and HSA library remain live during initialization.
        check(unsafe {
            n::iree_hal_amdgpu_topology_initialize_from_path(&libhsa.0, crate::view(path), &mut raw)
        })?;
        Ok(Self(raw))
    }
    /// Initializes a topology from a bitmask of HSA GPU-agent ordinals.
    pub fn from_gpu_agent_mask(libhsa: &LibHsa, mask: u64) -> Result<Self> {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: output value is writable and the HSA library remains live.
        check(unsafe {
            n::iree_hal_amdgpu_topology_initialize_from_gpu_agent_mask(&libhsa.0, mask, &mut raw)
        })?;
        Ok(Self(raw))
    }
    /// Inserts a CPU agent into this topology.
    ///
    /// # Safety
    /// `agent` and `libhsa` must refer to the same live HSA runtime used by
    /// this topology; the agent must remain valid while the topology is used.
    pub unsafe fn insert_cpu_agent(
        &mut self,
        libhsa: &LibHsa,
        agent: interop::AmdgpuAgentHandle<'_>,
    ) -> Result<usize> {
        let mut index = 0;
        // SAFETY: the caller guarantees that the agent and runtime belong to this topology.
        check(unsafe {
            n::iree_hal_amdgpu_topology_insert_cpu_agent(
                &mut self.0,
                &libhsa.0,
                agent.raw(),
                &mut index,
            )
        })?;
        Ok(index)
    }

    /// Inserts a GPU agent and associates it with a CPU agent.
    ///
    /// # Safety
    /// Both agent handles and `libhsa` must refer to the same live runtime used
    /// by this topology and remain valid while the topology is used.
    pub unsafe fn insert_gpu_agent(
        &mut self,
        libhsa: &LibHsa,
        gpu_agent: interop::AmdgpuAgentHandle<'_>,
        cpu_agent: interop::AmdgpuAgentHandle<'_>,
    ) -> Result<()> {
        // SAFETY: the caller guarantees both agents and the runtime match this topology.
        check(unsafe {
            n::iree_hal_amdgpu_topology_insert_gpu_agent(
                &mut self.0,
                &libhsa.0,
                gpu_agent.raw(),
                cpu_agent.raw(),
            )
        })
    }

    /// Inserts a GPU agent and assigns its nearest CPU agent.
    ///
    /// # Safety
    /// `gpu_agent` and `libhsa` must refer to the same live runtime used by
    /// this topology and remain valid while the topology is used.
    pub unsafe fn insert_gpu_agent_with_nearest_cpu_agent(
        &mut self,
        libhsa: &LibHsa,
        gpu_agent: interop::AmdgpuAgentHandle<'_>,
    ) -> Result<()> {
        // SAFETY: the caller guarantees the agent and runtime match this topology.
        check(unsafe {
            n::iree_hal_amdgpu_topology_insert_gpu_agent_with_nearest_cpu_agent(
                &mut self.0,
                &libhsa.0,
                gpu_agent.raw(),
            )
        })
    }

    /// Tests whether an agent is compatible with the agents already selected.
    ///
    /// # Safety
    /// `agent` and `libhsa` must refer to the same live runtime used by this topology.
    pub unsafe fn query_agent_compatibility(
        &mut self,
        libhsa: &LibHsa,
        agent: interop::AmdgpuAgentHandle<'_>,
    ) -> Result<bool> {
        let mut compatible = false;
        // SAFETY: the caller guarantees the agent and runtime match this topology.
        check(unsafe {
            n::iree_hal_amdgpu_topology_query_agent_compatibility(
                &mut self.0,
                &libhsa.0,
                agent.raw(),
                &mut compatible,
            )
        })?;
        Ok(compatible)
    }

    /// Verifies that all agents in the topology are compatible.
    ///
    /// # Safety
    /// `libhsa` must be the same live runtime used to construct this topology.
    pub unsafe fn verify(&self, libhsa: &LibHsa) -> Result<()> {
        // SAFETY: the caller guarantees this topology was constructed with `libhsa`.
        check(unsafe { n::iree_hal_amdgpu_topology_verify(&self.0, &libhsa.0) })
    }

    /// Adopts an initialized topology value.
    ///
    /// # Safety
    /// `raw` must be initialized and its deinitialization ownership transferred here.
    pub unsafe fn from_raw(raw: n::iree_hal_amdgpu_topology_t) -> Self {
        Self(raw)
    }
    /// Borrows the native topology value.
    pub fn as_raw(&self) -> *const n::iree_hal_amdgpu_topology_t {
        &self.0
    }
    /// Transfers the initialized topology value to the caller.
    pub fn into_raw(self) -> n::iree_hal_amdgpu_topology_t {
        let this = core::mem::ManuallyDrop::new(self);
        // SAFETY: ManuallyDrop prevents the transferred resource from being deinitialized.
        unsafe { core::ptr::read(&this.0) }
    }
}
impl Drop for Topology {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one initialized topology value.
        unsafe { n::iree_hal_amdgpu_topology_deinitialize(&mut self.0) };
    }
}

/// Creates an AMDGPU driver returning the generic HAL driver wrapper.
pub fn create_driver(identifier: &str, options: &DriverOptions) -> Result<super::super::Driver> {
    // SAFETY: plain-data options; the C initializer then fills every field.
    let mut raw_options = unsafe { core::mem::zeroed() };
    // SAFETY: `raw_options` is writable.
    unsafe { n::iree_hal_amdgpu_driver_options_initialize(&mut raw_options) };
    let views: Vec<_> = options
        .libhsa_search_paths
        .iter()
        .map(|path| crate::view(path))
        .collect();
    raw_options.libhsa_search_paths = n::iree_string_view_list_t {
        count: views.len(),
        values: if views.is_empty() {
            core::ptr::null()
        } else {
            views.as_ptr()
        },
    };
    raw_options.default_device_options = options.default_device.raw();
    let mut raw = core::ptr::null_mut();
    // SAFETY: path strings and views stay alive throughout creation; driver clones them.
    check(unsafe {
        n::iree_hal_amdgpu_driver_create(
            crate::view(identifier),
            &raw_options,
            crate::system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns an owned generic HAL driver.
    Ok(unsafe { super::super::Driver::from_raw(raw) })
}

/// Creates an AMDGPU logical device using a loaded HSA runtime and topology.
pub fn create_device(
    identifier: &str,
    options: &DeviceOptions,
    libhsa: &LibHsa,
    topology: &Topology,
    create_params: &super::super::DeviceCreateParams<'_>,
) -> Result<super::super::Device> {
    let options = options.raw();
    let mut raw = core::ptr::null_mut();
    // SAFETY: IREE clones the options, HSA state, and topology during this call.
    check(unsafe {
        n::iree_hal_amdgpu_logical_device_create(
            crate::view(identifier),
            &options,
            &libhsa.0,
            &topology.0,
            create_params.as_raw(),
            crate::system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns an owned generic HAL device.
    Ok(unsafe { super::super::Device::from_raw(raw) })
}

#[path = "amdgpu_interop.rs"]
pub mod interop;
