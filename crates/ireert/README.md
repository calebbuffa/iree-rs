# ireert

`ireert` provides owned Rust interfaces for IREE's VM, HAL, and I/O APIs.
Instances, modules, devices, buffers, lists, and other native resources use
RAII; safe methods validate ranges and preserve native ownership.

```toml
[dependencies]
ireert = "0.1.0"
```

```rust
use ireert::vm::{Buffer, BufferAccess, Instance};

fn main() -> ireert::Result<()> {
    let instance = Instance::new()?;
    let mut buffer = Buffer::new(&instance, BufferAccess::MUTABLE, 4, 1)?;
    buffer.write(0, &[1, 2, 3, 4])?;
    let mut bytes = [0; 4];
    buffer.read(0, &mut bytes)?;
    assert_eq!(bytes, [1, 2, 3, 4]);
    Ok(())
}
```

The default `cpu` feature enables the local CPU drivers. Disable default
features and select `local-sync`, `local-task`, `vulkan`, `cuda`, `hip`,
`amdgpu`, `metal`, or other supported backends as needed. Runtime drivers select
execution; compiler targets are selected separately in `ireec`.
Parameter I/O and the IRPA, GGUF, and safetensors parsers are optional runtime
features.

The [API documentation](https://docs.rs/ireert) includes parameter I/O features.
GPU backends additionally require their platform's SDKs and drivers; enabling
every backend at once is not a portable build configuration.

`ireert` has no dependency on `ireec`, including for tests. Run
`cargo test -p ireert` for runtime-only coverage without building LLVM/MLIR.
Compiler/runtime end-to-end coverage lives in the separate, unpublished
`iree-tests` package (`cargo test -p iree-tests`).

VM bytecode and executable plugins can run native code. Load only trusted
artifacts. The [`iree-samples` index](https://github.com/calebbuffa/iree-rs/tree/main/examples) shows runtime
and HAL usage; native build requirements are in
[`iree-native`](https://docs.rs/iree-native).
