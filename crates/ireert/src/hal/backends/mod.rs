//! Feature-gated HAL backend option wrappers.
//!
//! # Laws
//! - Backend-specific capabilities stay behind their feature-gated module.
//! - Shared HAL values and ownership remain common across drivers.
#[cfg(feature = "amdgpu")]
pub mod amdgpu;
#[cfg(feature = "cuda")]
pub mod cuda;
#[cfg(feature = "hip")]
pub mod hip;
#[cfg(any(feature = "cpu", feature = "local-sync", feature = "local-task"))]
pub mod local_loader;
#[cfg(feature = "local-sync")]
pub mod local_sync;
#[cfg(feature = "local-task")]
pub mod local_task;
#[cfg(feature = "metal")]
pub mod metal;
#[cfg(feature = "mpi")]
pub mod mpi;
#[cfg(feature = "vulkan")]
pub mod vulkan;
#[cfg(feature = "webgpu")]
pub mod webgpu;
