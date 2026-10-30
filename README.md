# IREE for Rust

This workspace provides Rust bindings for IREE's compiler, runtime, tokenizer,
and native C APIs. The wrappers use Rust ownership and RAII; applications choose
their own model, tensor, and execution abstractions.

| Crate | Purpose |
| --- | --- |
| [`ireec`](crates/ireec/README.md) | Compile input and configure compiler pipelines |
| [`ireec-ir`](crates/ireec-ir/README.md) | Internal, unpublished MLIR bindings |
| [`ireert`](crates/ireert/README.md) | VM, HAL, devices, buffers, and I/O |
| [`ireetok`](crates/ireetok/README.md) | Tokenization and custom tokenizer components |
| [`iree-native`](crates/iree-native/README.md) | Version-matched raw C bindings and native build |
| [`iree-wgpu`](crates/iree-wgpu/README.md) | Internal, unpublished WGPU/Vulkan interop |

Start with the [usage guide](docs/guide.md) or run a
[sample](examples/README.md):

```powershell
cargo run -p iree-samples --example simple-abs
```

`iree-native` builds pinned IREE sources and requires Rust 1.95+, CMake 3.26+,
Ninja, Python 3, host libclang, and the destination C/C++ toolchain and SDK.
Compiler features also build LLVM/MLIR. Cargo features select runtime drivers,
compiler targets, and input dialects independently; see the
[feature guide](docs/guide.md).

VM bytecode and native executables are trusted code, not a sandbox. Hosted
docs.rs builds are not supported because bindings and documentation use the
selected native installation and require libclang.
