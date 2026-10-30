//! Safe, owned Rust interfaces for IREE's VM, HAL, and I/O runtime APIs.
//!
//! Runtime objects release their native references on drop; cloning a wrapper
//! retains the corresponding native object. The [`vm`], [`hal`], and [`io`]
//! modules expose the main runtime domains, while [`native`] is the raw C API
//! escape hatch.
//!
//! The runtime does not impose a tensor frontend or model abstraction.
//! Applications choose their own input/output types and calling conventions.
//! VM bytecode and executable plugins may run native code, so load only trusted
//! artifacts.
//!
//! # Example
//!
//! Create a VM instance and an owned byte buffer without a compiler or model:
//!
//! ```
//! use ireert::vm::{Buffer, BufferAccess, Instance};
//!
//! let instance = Instance::new()?;
//! let mut buffer = Buffer::new(&instance, BufferAccess::MUTABLE, 4, 1)?;
//! buffer.write(0, &[1, 2, 3, 4])?;
//! let mut bytes = [0; 4];
//! buffer.read(0, &mut bytes)?;
//! assert_eq!(bytes, [1, 2, 3, 4]);
//! # Ok::<(), ireert::Error>(())
//! ```
#![deny(missing_docs, clippy::all)]

pub mod asynchronous;
pub mod base;
pub mod task;

pub mod hal;
pub mod io;
pub mod vm;

pub use iree_native as native;

use iree_native as n;

pub use n::{Code, Error, Result};

/// Convert an owned native status into a [`Result`].
pub(crate) fn check(status: n::iree_status_t) -> Result<()> {
    // SAFETY: every status handed to `check` is freshly returned and owned.
    unsafe { n::consume(status) }
}

pub(crate) use n::string_view as view;

/// The system allocator.
pub(crate) use n::system_allocator as system;
