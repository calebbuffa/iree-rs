//! Programmatic native worker topology and task-executor configuration.

use crate::{Error, Result, runtime::check, sys};
use std::{
    ptr::{self, NonNull},
    time::Duration,
};

/// Preference when selecting physical cores on heterogeneous CPUs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorePreference {
    /// Select all eligible core classes.
    Any,
    /// Prefer energy-efficient cores.
    Efficiency,
    /// Prefer high-performance cores.
    Performance,
}

/// Distribution of workers over shared-cache domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreDistribution {
    /// Fill one cache domain before moving to the next.
    Compact,
    /// Spread workers over cache domains for memory bandwidth.
    Scatter,
}

/// An owned native worker topology, independent of global command-line flags.
pub struct Topology(Box<sys::iree_task_topology_t>);

impl Topology {
    /// Select a fixed number of OS-scheduled workers without CPU pinning.
    ///
    /// Zero workers are rejected because this interface does not expose native
    /// thread donation. Such an executor could otherwise accept work forever
    /// without making forward progress.
    pub fn workers(count: usize) -> Result<Self> {
        if count == 0 || count > sys::IREE_TASK_TOPOLOGY_MAX_GROUP_COUNT as usize {
            return Err(Error::invalid(
                "worker count is outside the native topology capacity",
            ));
        }
        let mut topology = Box::new(sys::iree_task_topology_t::default());
        unsafe { sys::iree_task_topology_initialize_from_group_count(count, &mut *topology) };
        Ok(Self(topology))
    }

    /// Select explicit platform logical CPU IDs.
    pub fn logical_cpus(ids: &[u32]) -> Result<Self> {
        if ids.is_empty() || ids.len() > sys::IREE_TASK_TOPOLOGY_MAX_GROUP_COUNT as usize {
            return Err(Error::invalid(
                "logical CPU list is empty or exceeds native capacity",
            ));
        }
        if ids
            .iter()
            .enumerate()
            .any(|(index, id)| ids[..index].contains(id))
        {
            return Err(Error::invalid("logical CPU IDs must be unique"));
        }
        let mut topology = Box::new(sys::iree_task_topology_t::default());
        check(unsafe {
            sys::iree_task_topology_initialize_from_logical_cpu_set(
                ids.len(),
                ids.as_ptr(),
                &mut *topology,
            )
        })?;
        Ok(Self(topology))
    }

    /// Discover physical cores with NUMA, core-class, and cache distribution.
    ///
    /// `None` allows any NUMA node. `max_workers` is bounded by the native
    /// executor capacity. Unsupported host discovery fails explicitly.
    pub fn physical_cores(
        node: Option<u32>,
        preference: CorePreference,
        distribution: CoreDistribution,
        max_workers: usize,
    ) -> Result<Self> {
        if max_workers == 0 || max_workers > sys::IREE_TASK_TOPOLOGY_MAX_GROUP_COUNT as usize {
            return Err(Error::invalid(
                "physical core limit is outside native capacity",
            ));
        }
        if let Some(node) = node
            && !Self::numa_nodes().contains(&node)
        {
            return Err(Error::invalid("NUMA node was not discovered on this host"));
        }
        let preference = match preference {
            CorePreference::Any => sys::IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_ANY,
            CorePreference::Efficiency => sys::IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_LOW,
            CorePreference::Performance => sys::IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_HIGH,
        };
        let distribution = match distribution {
            CoreDistribution::Compact => sys::IREE_TASK_TOPOLOGY_DISTRIBUTION_COMPACT,
            CoreDistribution::Scatter => sys::IREE_TASK_TOPOLOGY_DISTRIBUTION_SCATTER,
        };
        let mut topology = Box::new(sys::iree_task_topology_t::default());
        check(unsafe {
            sys::iree_task_topology_initialize_from_physical_cores(
                node.unwrap_or(u32::MAX),
                preference,
                distribution,
                max_workers,
                &mut *topology,
            )
        })?;
        if topology.group_count == 0 {
            return Err(Error::invalid("core selection produced no workers"));
        }
        Ok(Self(topology))
    }

    /// Discover actual NUMA node IDs, including sparse platform numbering.
    pub fn numa_nodes() -> Vec<u32> {
        let mut ids = [0; sys::IREE_TASK_TOPOLOGY_MAX_NODES as usize];
        let count = unsafe { sys::iree_task_topology_query_node_ids(ids.len(), ids.as_mut_ptr()) };
        ids[..count.min(ids.len())].to_vec()
    }

    /// Number of workers in this topology.
    pub fn worker_count(&self) -> usize {
        self.0.group_count
    }
    /// NUMA node assignment, or `None` for unrestricted allocation.
    pub fn numa_node(&self) -> Option<u32> {
        (self.0.numa_node_id != u32::MAX).then_some(self.0.numa_node_id)
    }
}

impl Drop for Topology {
    fn drop(&mut self) {
        unsafe { sys::iree_task_topology_deinitialize(&mut *self.0) };
    }
}

/// Native task-executor options with programmatic, session-independent values.
pub struct ExecutorOptions(sys::iree_task_executor_options_t);

impl Default for ExecutorOptions {
    fn default() -> Self {
        let mut options = sys::iree_task_executor_options_t::default();
        unsafe { sys::iree_task_executor_options_initialize(&mut options) };
        Self(options)
    }
}

impl ExecutorOptions {
    /// Set the executor-local worker index base for multi-executor setups.
    pub fn worker_base_index(mut self, index: usize) -> Self {
        self.0.worker_base_index = index;
        self
    }
    /// Set minimum native thread stack size in bytes.
    pub fn stack_size(mut self, bytes: usize) -> Self {
        self.0.worker_stack_size = bytes;
        self
    }
    /// Set per-worker local memory capacity in bytes.
    pub fn local_memory_size(mut self, bytes: usize) -> Self {
        self.0.worker_local_memory_size = bytes;
        self
    }
    /// Bound worker spinning before sleep.
    ///
    /// Defaults to zero. Nonzero spinning increases thermal/power cost and can
    /// harm other processes; use only after measuring a latency requirement.
    pub fn spin_duration(mut self, duration: Duration) -> Result<Self> {
        self.0.worker_spin_ns = i64::try_from(duration.as_nanos())
            .map_err(|_| Error::invalid("worker spin duration exceeds native range"))?;
        Ok(self)
    }
    /// Create an owned native executor for a discovered or explicit topology.
    ///
    /// Worker indices must fit the native 32-bit dispatch context. Stack sizes
    /// and the aggregate requested local memory must fit the host addressable
    /// span; invalid sizes are rejected before native allocation or thread
    /// creation.
    pub fn build(self, topology: &Topology) -> Result<Executor> {
        let last_worker = self
            .0
            .worker_base_index
            .checked_add(topology.worker_count() - 1)
            .ok_or_else(|| Error::invalid("worker base index overflows"))?;
        u32::try_from(last_worker)
            .map_err(|_| Error::invalid("worker index exceeds the native 32-bit range"))?;
        if self.0.worker_stack_size > isize::MAX as usize {
            return Err(Error::invalid("worker stack size exceeds address space"));
        }
        let mut local_bytes = 0usize;
        for group in &topology.0.groups[..topology.worker_count()] {
            let requested = if self.0.worker_local_memory_size != 0 {
                self.0.worker_local_memory_size
            } else if group.caches.l2_data != 0 {
                group.caches.l2_data as usize
            } else {
                group.caches.l1_data as usize
            };
            local_bytes = local_bytes
                .checked_add(requested)
                .filter(|&bytes| bytes <= isize::MAX as usize)
                .ok_or_else(|| Error::invalid("worker local memory exceeds address space"))?;
        }
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_task_executor_create(
                self.0,
                &*topology.0,
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Executor(NonNull::new(raw).ok_or_else(|| {
            Error::invalid("IREE returned a null executor")
        })?))
    }
}

/// An owned native worker pool, reusable across local-task device queues.
pub struct Executor(NonNull<sys::iree_task_executor_t>);
// Executor scheduling, maintenance and ownership are synchronized by IREE.
unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}

impl Executor {
    /// Number of live native workers.
    pub fn worker_count(&self) -> usize {
        unsafe { sys::iree_task_executor_worker_count(self.as_raw()) }
    }
    /// Configured NUMA node, or `None` for an unrestricted topology.
    pub fn numa_node(&self) -> Option<u32> {
        let node = unsafe { sys::iree_task_executor_node_id(self.as_raw()) };
        (node != u32::MAX).then_some(node)
    }
    /// Trim unused pools and worker caches.
    pub fn trim(&self) {
        unsafe { sys::iree_task_executor_trim(self.as_raw()) };
    }
    /// Borrow the native executor without transferring ownership.
    pub fn as_raw(&self) -> *mut sys::iree_task_executor_t {
        self.0.as_ptr()
    }
}
impl Clone for Executor {
    fn clone(&self) -> Self {
        unsafe { sys::iree_task_executor_retain(self.as_raw()) };
        Self(self.0)
    }
}
impl Drop for Executor {
    fn drop(&mut self) {
        unsafe { sys::iree_task_executor_release(self.as_raw()) };
    }
}

#[cfg(feature = "local-task")]
mod device {
    use super::*;
    use crate::{Device, Instance, runtime::string_view};

    /// Native local-task queue configuration and executable loader selection.
    pub struct TaskDeviceOptions {
        params: sys::iree_hal_task_device_params_t,
        embedded_elf: bool,
        vmvx: bool,
    }

    impl Default for TaskDeviceOptions {
        fn default() -> Self {
            let mut params = sys::iree_hal_task_device_params_t::default();
            unsafe { sys::iree_hal_task_device_params_initialize(&mut params) };
            Self {
                params,
                embedded_elf: true,
                vmvx: true,
            }
        }
    }

    impl TaskDeviceOptions {
        /// Configure native transient allocation block size.
        pub fn arena_block_size(mut self, bytes: usize) -> Self {
            self.params.arena_block_size = bytes;
            self
        }
        /// Route smaller queue fill/copy operations through inline CPU transfers.
        pub fn inline_transfer_threshold(mut self, bytes: usize) -> Self {
            self.params.inline_transfer_threshold = bytes;
            self
        }
        /// Enable loading compiler-generated embedded ELF executables.
        pub fn embedded_elf(mut self, enabled: bool) -> Self {
            self.embedded_elf = enabled;
            self
        }
        /// Enable the portable VMVX executable loader.
        pub fn vmvx(mut self, enabled: bool) -> Self {
            self.vmvx = enabled;
            self
        }
        /// Create one logical queue per supplied executor reference.
        ///
        /// Repeating an executor shares its worker pool across queues; supplying
        /// different executors supports distinct NUMA/scheduling domains.
        /// Native queues retain their executors and loaders.
        pub fn build(self, instance: &Instance, executors: &[Executor]) -> Result<Device> {
            if executors.is_empty() || executors.len() > 64 {
                return Err(Error::invalid("local-task requires 1..=64 queue executors"));
            }
            struct Loaders(Vec<*mut sys::iree_hal_executable_loader_t>);
            impl Drop for Loaders {
                fn drop(&mut self) {
                    for loader in &self.0 {
                        unsafe { sys::iree_hal_executable_loader_release(*loader) };
                    }
                }
            }
            let mut loaders = Loaders(Vec::new());
            if self.embedded_elf {
                let mut raw = ptr::null_mut();
                check(unsafe {
                    sys::iree_hal_embedded_elf_loader_create(
                        ptr::null_mut(),
                        sys::iree_allocator_system(),
                        &mut raw,
                    )
                })?;
                loaders.0.push(raw);
            }
            if self.vmvx {
                let mut raw = ptr::null_mut();
                check(unsafe {
                    sys::iree_hal_vmvx_module_loader_create(
                        sys::iree_runtime_instance_vm_instance(instance.as_raw()),
                        0,
                        ptr::null_mut(),
                        sys::iree_allocator_system(),
                        &mut raw,
                    )
                })?;
                loaders.0.push(raw);
            }
            struct Allocator(*mut sys::iree_hal_allocator_t);
            impl Drop for Allocator {
                fn drop(&mut self) {
                    unsafe { sys::iree_hal_allocator_release(self.0) };
                }
            }
            let mut allocator = ptr::null_mut();
            check(unsafe {
                sys::iree_hal_allocator_create_heap(
                    string_view("rust-task"),
                    sys::iree_allocator_system(),
                    sys::iree_allocator_system(),
                    &mut allocator,
                )
            })?;
            let allocator = Allocator(allocator);
            let pool = crate::driver::ProactorPool::new()?;
            let create_params = sys::iree_hal_device_create_params_t {
                next: ptr::null(),
                proactor_pool: pool.0,
            };
            let executors: Vec<_> = executors.iter().map(Executor::as_raw).collect();
            let mut raw = ptr::null_mut();
            check(unsafe {
                sys::iree_hal_task_device_create(
                    string_view("local-task"),
                    &self.params,
                    executors.len(),
                    executors.as_ptr(),
                    loaders.0.len(),
                    loaders.0.as_mut_ptr(),
                    allocator.0,
                    &create_params,
                    sys::iree_allocator_system(),
                    &mut raw,
                )
            })?;
            unsafe { Device::from_owned(instance.clone(), raw, None) }
        }
    }
}

#[cfg(feature = "local-task")]
pub use device::TaskDeviceOptions;
