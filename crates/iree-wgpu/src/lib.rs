//! Safe, zero-copy interoperability between [WGPU] and [IREE] on one Vulkan
//! device.
//!
//! A [`Context`] creates a single Vulkan device with the union of WGPU's and
//! IREE's requirements, and is the only submission authority for both APIs:
//!
//! - **WGPU** work is recorded with [`Context::device`] as usual and submitted
//!   through [`Context::submit`], [`Context::write_buffer`],
//!   [`Context::write_texture`] and [`Context::present`].
//! - **IREE** programs are loaded with [`Context::load`] (or
//!   `Context::compile` with the `compiler` feature) and invoked with
//!   [`Program::call`] or [`Program::call_views`].
//! - **[`SharedBuffer`]s** are one allocation visible to both APIs without
//!   copies: [`SharedBuffer::wgpu`] for bind groups and copies,
//!   [`SharedBuffer::view`] for IREE arguments and outputs.
//!
//! Because every submission goes through the context, the two APIs behave
//! like one logical queue: work submitted later observes the results of work
//! submitted earlier, whichever API either was submitted to. Callers need no
//! fences, barriers, leases or `unsafe`.
//!
//! # Quick start
//!
//! ```no_run
//! # #[cfg(feature = "compiler")]
//! # fn main() -> iree_wgpu::runtime::Result<()> {
//! use iree_wgpu::{Context, ContextOptions, compiler::InputType, runtime::hal::ElementType};
//!
//! let context = pollster::block_on(Context::new(ContextOptions::default()))?;
//! let program = context.compile(
//!     r#"func.func @main(%x: tensor<4xf32>) -> tensor<4xf32> {
//!          %y = stablehlo.add %x, %x : tensor<4xf32>
//!          return %y : tensor<4xf32>
//!        }"#,
//!     InputType::StableHlo,
//! )?;
//!
//! // One allocation, visible to both APIs.
//! let input = context.shared_buffer(&[1.0f32, 2.0, 3.0, 4.0])?;
//!
//! // WGPU writes it...
//! let mut encoder = context.device().create_command_encoder(&Default::default());
//! encoder.clear_buffer(input.wgpu(), 0, Some(4));
//! context.submit([encoder.finish()])?;
//!
//! // ...IREE observes the write without a copy or a host wait...
//! let doubled = program.call_views("module.main", &[&input.view(&[4], ElementType::FLOAT_32)?])?;
//!
//! // ...and WGPU can consume the result in turn.
//! let output = context.copy_to_shared(&doubled[0])?;
//! let _binding = output.binding();
//!
//! // SAFETY: module.main writes every output element.
//! assert_eq!(unsafe { context.read::<f32>(&output)? }, [0.0, 4.0, 6.0, 8.0]);
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "compiler"))]
//! # fn main() {}
//! ```
//!
//! Rendering to a window uses [`Context::with_window`], which also returns a
//! [`Surface`]; see the `render_analysis` example.
//! Pass [`ContextOptions`] to [`Context::new`] or [`Context::with_window`] to
//! choose an adapter power preference or require a software adapter.
//!
//! # Queue topology
//!
//! When the device exposes a second compute-capable queue, either in WGPU's
//! graphics/compute family or in another (such as an asynchronous compute
//! family), WGPU and IREE each own one ([`QueueTopology::Separate`]) and are
//! ordered with timeline semaphores without blocking the host: an
//! asynchronous IREE call returns once its work is enqueued, and later WGPU
//! submissions wait for it on the GPU. Otherwise both share one queue
//! ([`QueueTopology::Shared`]) and the context completes IREE work before
//! releasing the queue to WGPU. The topology affects performance only, never
//! correctness or the API.
//!
//! The context routes IREE's Vulkan calls through its own function table, so
//! both APIs take the same lock for each `VkQueue`, buffers are shared across
//! queue families without ownership transfers, and the `VkDevice` is
//! destroyed once the last WGPU or IREE object using it is.
//!
//! # Re-exports
//!
//! The crate re-exports the versions it is built against: [`wgpu`], the IREE
//! runtime as [`runtime`], and (with the `compiler` feature) the IREE compiler
//! as `compiler`. Use them to avoid version mismatches.
//!
//! # Features
//!
//! - `compiler`: enables `Context::compile` and the `compiler` re-export.
//!
//! # Limitations
//!
//! Only buffers are shared, not textures. Requires Vulkan 1.3 with timeline
//! semaphores, synchronization2 and scalar block layout. See
//! the crate README for the design, its assumptions and its limits.
//!
//! [WGPU]: https://wgpu.rs
//! [IREE]: https://iree.dev
#![cfg_attr(docsrs, feature(doc_cfg))]

mod buffer;
mod context;
mod device;
mod element;
mod error;
mod hooks;
mod marker;
mod ordering;
mod program;
mod surface;

pub use buffer::SharedBuffer;
pub use context::{Context, ContextOptions, QueueTopology};
pub use element::Element;
pub use program::Program;
pub use surface::Surface;

/// The IREE compiler (`ireec`), as used by `Context::compile`.
#[cfg(feature = "compiler")]
#[cfg_attr(docsrs, doc(cfg(feature = "compiler")))]
pub use ireec as compiler;
/// The IREE runtime (`ireert`): HAL types, VM modules, errors and results.
pub use ireert as runtime;
/// The WGPU version this crate is built against.
pub use wgpu;

#[cfg(test)]
mod tests {
    use super::{Context, SharedBuffer, Surface};

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn handles_are_send_and_sync() {
        assert_send_sync::<Context>();
        assert_send_sync::<SharedBuffer>();
        assert_send_sync::<Surface>();
    }
}
