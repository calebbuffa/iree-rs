//! Owned task worker topologies.
use crate::{Result, check};
use iree_native as n;

/// Which cores of a heterogeneous cluster to use.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CorePerformance {
    /// All cores.
    #[default]
    Any,
    /// Efficiency cores.
    Low,
    /// Performance cores.
    High,
}

/// How selected cores are spread across cache domains.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CoreDistribution {
    /// Fill each cache domain before the next.
    #[default]
    Compact,
    /// Round-robin across cache domains.
    Scatter,
}

/// A worker topology used to construct an [`Executor`](super::Executor).
pub struct Topology(pub(super) n::iree_task_topology_t);
impl Topology {
    /// Creates `count` unpinned worker groups, clamped to the native maximum.
    pub fn from_group_count(count: usize) -> Self {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_task_topology_initialize_from_group_count(count, &mut raw) };
        Self(raw)
    }

    /// Creates one worker group pinned to each listed logical CPU.
    pub fn from_logical_cpus(cpu_ids: &[u32]) -> Result<Self> {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `cpu_ids` outlives the call and `raw` is writable.
        check(unsafe {
            n::iree_task_topology_initialize_from_logical_cpu_set(
                cpu_ids.len(),
                cpu_ids.as_ptr(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }

    /// Creates one worker group per physical core, up to `max_cores`.
    ///
    /// `numa_node` limits selection to one NUMA node; `None` uses any node.
    pub fn from_physical_cores(
        numa_node: Option<u32>,
        performance: CorePerformance,
        distribution: CoreDistribution,
        max_cores: usize,
    ) -> Result<Self> {
        // IREE_TASK_TOPOLOGY_NODE_ID_ANY is the all-ones node id.
        let node = numa_node.unwrap_or(n::iree_task_topology_node_id_t::MAX);
        let performance = match performance {
            CorePerformance::Any => n::IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_ANY,
            CorePerformance::Low => n::IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_LOW,
            CorePerformance::High => n::IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_HIGH,
        };
        let distribution = match distribution {
            CoreDistribution::Compact => n::IREE_TASK_TOPOLOGY_DISTRIBUTION_COMPACT,
            CoreDistribution::Scatter => n::IREE_TASK_TOPOLOGY_DISTRIBUTION_SCATTER,
        };
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        check(unsafe {
            n::iree_task_topology_initialize_from_physical_cores(
                node,
                performance,
                distribution,
                max_cores,
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }

    /// Returns the number of worker groups.
    pub fn group_count(&self) -> usize {
        // SAFETY: `self.0` is an initialized topology.
        unsafe { n::iree_task_topology_group_count(&self.0) }
    }
}
impl Drop for Topology {
    fn drop(&mut self) {
        // SAFETY: this value was initialized exactly once.
        unsafe { n::iree_task_topology_deinitialize(&mut self.0) };
    }
}
