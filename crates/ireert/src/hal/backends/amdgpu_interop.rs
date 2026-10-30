//! HSA agent handles selected by an AMDGPU [`Topology`].
//!
//! Agent handles borrow the initialized HSA runtime that produced them.
use super::{LibHsa, Topology};
use core::marker::PhantomData;
use iree_native as n;

/// Opaque HSA agent handle borrowed from an initialized runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AmdgpuAgentHandle<'a> {
    handle: u64,
    _runtime: PhantomData<&'a LibHsa>,
}
impl<'a> AmdgpuAgentHandle<'a> {
    /// Adopts an HSA agent handle tied to a live runtime.
    ///
    /// # Safety
    /// `handle` must be a valid agent from `_libhsa`.
    pub unsafe fn from_raw(handle: u64, _libhsa: &'a LibHsa) -> Self {
        Self {
            handle,
            _runtime: PhantomData,
        }
    }

    pub(crate) fn raw(self) -> n::hsa_agent_t {
        n::hsa_agent_t {
            handle: self.handle,
        }
    }

    /// Returns the opaque HSA agent value.
    pub const fn as_raw(self) -> u64 {
        self.handle
    }
}

fn agents<'a>(agents: &[n::hsa_agent_t], count: usize) -> Vec<AmdgpuAgentHandle<'a>> {
    agents
        .iter()
        .take(count)
        .map(|agent| AmdgpuAgentHandle {
            handle: agent.handle,
            _runtime: PhantomData,
        })
        .collect()
}

/// Selected GPU agents in native topology order.
///
/// # Safety
/// `_libhsa` must be the runtime used to construct `topology`.
pub unsafe fn gpu_agents<'a>(
    topology: &Topology,
    _libhsa: &'a LibHsa,
) -> Vec<AmdgpuAgentHandle<'a>> {
    agents(&topology.0.gpu_agents, topology.0.gpu_agent_count)
}

/// Selected CPU agents in native topology order.
///
/// # Safety
/// `_libhsa` must be the runtime used to construct `topology`.
pub unsafe fn cpu_agents<'a>(
    topology: &Topology,
    _libhsa: &'a LibHsa,
) -> Vec<AmdgpuAgentHandle<'a>> {
    agents(&topology.0.cpu_agents, topology.0.cpu_agent_count)
}

/// Number of logical queues configured per GPU agent.
pub fn queues_per_gpu(topology: &Topology) -> usize {
    topology.0.gpu_agent_queue_count
}
