//! Retained task executors.
use super::Topology;
use crate::{Result, check, system};
use core::time::Duration;
use iree_native as n;

/// Worker settings for an [`Executor`]; unset fields keep native defaults.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExecutorOptions {
    /// First worker index assigned to this executor.
    pub worker_base_index: Option<usize>,
    /// How long idle workers spin before sleeping.
    pub worker_spin: Option<Duration>,
    /// Worker thread stack size in bytes.
    pub worker_stack_size: Option<usize>,
    /// Per-worker scratch memory in bytes.
    pub worker_local_memory_size: Option<usize>,
}

/// A retained task executor that drives task HAL devices.
pub struct Executor(*mut n::iree_task_executor_t);
impl Executor {
    /// Creates an executor with one worker per topology group.
    pub fn new(topology: &Topology, options: ExecutorOptions) -> Result<Self> {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw_options: n::iree_task_executor_options_t = unsafe { core::mem::zeroed() };
        // SAFETY: `raw_options` is writable.
        unsafe { n::iree_task_executor_options_initialize(&mut raw_options) };
        if let Some(value) = options.worker_base_index {
            raw_options.worker_base_index = value;
        }
        if let Some(value) = options.worker_spin {
            raw_options.worker_spin_ns = i64::try_from(value.as_nanos()).unwrap_or(i64::MAX);
        }
        if let Some(value) = options.worker_stack_size {
            raw_options.worker_stack_size = value;
        }
        if let Some(value) = options.worker_local_memory_size {
            raw_options.worker_local_memory_size = value;
        }
        let mut raw = core::ptr::null_mut();
        // SAFETY: topology and options are live for the call.
        check(unsafe {
            n::iree_task_executor_create(raw_options, &topology.0, system(), &mut raw)
        })?;
        Ok(Self(raw))
    }

    /// Borrows the native executor pointer.
    pub fn as_raw(&self) -> *mut n::iree_task_executor_t {
        self.0
    }
}
impl Clone for Executor {
    fn clone(&self) -> Self {
        // SAFETY: the live executor is retained before creating another owner.
        unsafe { n::iree_task_executor_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for Executor {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one executor reference.
        unsafe { n::iree_task_executor_release(self.0) };
    }
}
