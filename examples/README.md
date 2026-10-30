# Examples

Run commands from the workspace root. The `iree-samples` package keeps
compiler/runtime examples out of Cargo build scripts; embedded VMFB artifacts
are checked-in outputs. Treat VMFB files and executable plugins as trusted code.

## Run and test

```powershell
cargo run -p iree-samples --example simple-abs
cargo run -p iree-samples --features tokenizer --example tokenizer
cargo test -p iree-samples --examples
```

The first example compiles and executes a small model on the default CPU
backend. The test command runs examples enabled by the selected Cargo features;
feature-gated GPU and compiler-target examples may need additional features,
SDKs, and hardware.

## Example groups

- **VM:** modules, values, dynamic shapes, state, and invocation.
- **Custom modules:** typed synchronous/asynchronous callbacks, static
  composition, and a Rust `cdylib`.
- **Tokenizer:** parse a Hugging Face WordPiece configuration and encode/decode
  text (enable the `tokenizer` feature).
- **HAL:** buffer transfers, embedded bytecode, external transient buffers,
  and backend-specific execution.
- **Compiler:** Transform Dialect, EmitC, static libraries, and external
  dispatch. Several demonstrate compilation or ABI generation only; their
  generated native code still needs the matching linker or plugin.
- **GPU interop:** the unpublished [`iree-wgpu`](../crates/iree-wgpu/README.md)
  examples share Vulkan buffers with WGPU.

Some upstream runtime demos are not yet represented. The
[upstream coverage status](../docs/upstream-coverage.md) lists the largest
remaining test and example gaps.

## Platform prototypes

The [bare-metal](bare-metal/README.md) and [WebAssembly](wasm/README.md)
directories are historical, experimental prototypes. Their current code uses
APIs and package names that have since changed; they are not validated examples
for this workspace.
