//! Task-system worker topologies and executors for task HAL devices.

#[cfg(any(feature = "cpu", feature = "local-task"))]
mod executor;
#[cfg(any(feature = "cpu", feature = "local-task"))]
mod topology;
#[cfg(any(feature = "cpu", feature = "local-task"))]
pub use executor::{Executor, ExecutorOptions};
#[cfg(any(feature = "cpu", feature = "local-task"))]
pub use topology::{CoreDistribution, CorePerformance, Topology};
