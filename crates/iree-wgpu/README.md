# iree-wgpu

`iree-wgpu` is an unpublished Vulkan interop crate. It lets WGPU and IREE use
the same GPU buffers and orders submissions from both APIs through one
`Context`. No user-managed fences or unsafe handle sharing are needed.

```rust,ignore
let context = Context::new(ContextOptions::default()).await?;
let shared = context.shared_buffer(&[1.0_f32, 2.0, 3.0, 4.0])?;
context.submit([encoder.finish()])?;
let output = program.call_views("module.main", &[&shared.view(&[4], ElementType::FLOAT_32)?])?;
```

Submit WGPU work with `Context::submit`, `write_buffer`, `write_texture`, or
`present`, not directly to WGPU's queue. A `SharedBuffer` can be bound in WGPU
or passed to IREE as an input or destination-passing output. Reading data to
the host is `unsafe`: the caller must establish that every byte was initialized.

The context shares one Vulkan 1.3 device. `QueueTopology::Separate` uses GPU
timeline synchronization when another compute queue is available;
`QueueTopology::Shared` serializes work on one queue and blocks during IREE
calls. This affects performance, not ordering guarantees. Only buffers are
shared; images need an explicit copy.

The `compiler` feature enables runtime MLIR compilation. Without it, load a
precompiled module with `Context::load`. The crate re-exports its exact
`wgpu`, `ireert`, and optional `ireec` dependencies.

```powershell
cargo run -p iree-wgpu --features compiler --example shared_buffer
cargo run -p iree-wgpu --features compiler --example render_analysis
```

The tests require a compatible Vulkan device and are ignored by default. Native
build prerequisites are documented by [`iree-native`](../iree-native/README.md).
