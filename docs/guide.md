# Use IREE from Rust

Choose the crate that matches the work:

| Crate                                            | Use                                               |
| ------------------------------------------------ | ------------------------------------------------- |
| [`ireec`](../crates/ireec/README.md)             | Compile input and control compiler pipelines      |
| [`ireert`](../crates/ireert/README.md)           | Load VM modules and use runtime, HAL, and I/O     |
| [`ireetok`](../crates/ireetok/README.md)         | Tokenize and extend tokenizer components          |
| [`iree-native`](../crates/iree-native/README.md) | Call the version-matched raw C API                |
| [`iree-wgpu`](../crates/iree-wgpu/README.md)     | Share buffers between IREE and WGPU (unpublished) |

`ireec-ir` provides internal MLIR bindings for the same compiler instance as
`ireec`.

## Run a sample

From the workspace root, this example compiles a small model and executes it
with the CPU runtime:

```powershell
cargo run -p iree-samples --example simple-abs
```

See the [example index](../examples/README.md) for HAL, VM, custom-module,
compiler, and GPU examples.

## Select features

Runtime drivers and compiler output targets are separate feature selections.
For example, `ireert/vulkan` enables the runtime driver, while
`ireec/target-vulkan` enables compiler output for Vulkan. `ireec` input
dialects such as `input-stablehlo`, `input-tosa`, and `input-torch` are separate
again. See each crate's Cargo feature list for exact combinations; avoid
`--all-features` when platform backends conflict.

## Build and safety

The native build requires Rust 1.95+, CMake 3.26+, Ninja, Python 3, host
libclang, and the destination C/C++ toolchain and SDK. Compiler features also
build LLVM/MLIR. `IREE_NATIVE_DIR` can point to a compatible prebuilt native
installation. See [`iree-native`](../crates/iree-native/README.md) for
deployment details.

Rust wrappers own native resources and release them on drop. Raw pointers,
foreign callbacks, custom allocators, and external handles retain their native
safety contracts. VM bytecode and executable plugins may run native code; load
only trusted artifacts.
