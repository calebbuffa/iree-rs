//! Raw, version-locked IREE C API bindings.
//!
//! Generated from the same pinned headers used to build the runtime. All
//! ownership, synchronization, device compatibility, and pointer validity
//! requirements of the upstream C APIs apply.
//!
//! Includes the base, async, VM, runtime, HAL, and parameter I/O module
//! interfaces. `task` adds task execution/topology APIs. Driver-specific APIs
//! follow their Cargo features.
//! `compiler` adds embedding, MLIR interop, and tool entry points. C inline
//! functions have generated C forwarding wrappers, so their Rust declarations
//! are linkable, not just header-only declarations.
//!
//! Native comments are retained by bindgen, with untyped examples marked as
//! text rather than executable Rust. They describe C contracts;
//! they are not a claim that raw pointers or native callbacks are safe Rust.
//! Build-generated declarations are organized into [`base`] and optional
//! API modules selected by Cargo features. Their symbols are also re-exported
//! at the crate root for conventional C API access. Bindgen runs against the
//! selected native target and configuration; no target snapshots are maintained.
#![allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    improper_ctypes,
    unsafe_op_in_unsafe_fn,
    unused_imports,
    clippy::all
)]
#![allow(
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_codeblock_attributes
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Whether the linked native runtime uses thread-safe synchronization.
///
/// This is read from the native build configuration, not inferred from Rust
/// target capabilities.
pub const THREADING_ENABLED: bool = cfg!(iree_native_threading);
