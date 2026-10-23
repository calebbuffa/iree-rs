//! Version-locked IREE C bindings and owned Rust compiler/runtime interfaces.
//!
//! The default build contains CPU runtime drivers, not LLVM/MLIR. Enable
//! `compiler` for in-process compilation and select accelerator drivers and
//! compiler backends independently. See the crate README for the feature matrix,
//! native prerequisites, cross-compilation, and deployment instructions.
//!
//! [`sys`] exposes the native APIs and their upstream documentation. The owned
//! layer manages object lifetimes, driver discovery, dense tensors, mixed VM
//! values, callback-driven timeline futures, coarse-fence asynchronous calls,
//! ordered transfer recording, indexed parameter providers, and configurable
//! task executors. Reusable modules, ordered session composition, and owned
//! reflection metadata expose native code sharing and import resolution.
//! Its intended scope is comprehensive; general VM event-loop
//! integration, custom modules/providers, and direct dispatch recording remain
//! in development.
//!
//! # Trust and concurrency
//!
//! IREE artifacts may execute native code: load only trusted artifacts. Runtime
//! devices and sessions are currently thread-local. [`Timeline`] is
//! thread-shareable with `task`, backed by native synchronization rather than
//! blocking worker futures. Compiler invocations are serialized
//! across wrapper handles. Raw bindings require the upstream ownership,
//! synchronization, callback, and pointer-validity contracts.

pub use iree_sys as sys;

/// IREE release used for both the native libraries and generated bindings.
pub const IREE_VERSION: &str = "3.13.0";

mod error;
pub use error::{Error, Result, StatusCode};

mod synchronization;
pub use synchronization::{Semaphore, Timeline, Timeout, Wait};
mod fence;
pub use fence::{Fence, FenceWait};
mod parameters;
pub use parameters::{ParameterIndex, ParameterInfo, ParameterProvider, ParameterSignal};
mod command;
pub use command::{Buffer, Submission, TransferEncoder};
mod driver;
pub use driver::{DeviceBuilder, DeviceInfo, Driver, DriverInfo};
#[cfg(feature = "task")]
mod concurrent;
#[cfg(feature = "task")]
pub use concurrent::SharedDevice;
#[cfg(feature = "task")]
pub mod task;

#[cfg(feature = "compiler")]
mod compiler;
#[cfg(feature = "compiler")]
pub use compiler::{Artifact, CompileOptions, Compiler, ExecutionModel, Input, Target};

mod runtime;
pub use runtime::{Device, Element, Instance, Invocation, Object, Session, Tensor, Value};
mod reflection;
pub use reflection::{Attribute, FunctionInfo, ModuleInfo};
mod module;
pub use module::{Module, SessionBuilder, SessionSignal};
