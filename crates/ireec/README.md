# ireec

`ireec` provides owned Rust wrappers for IREE's compiler. Create a
`Compiler`, configure a `Session`, compile through an `Invocation`, and collect
or write an `Output`.

Compiler targets and input dialects are selected with Cargo features such as
`target-llvm-cpu` and `input-stablehlo`; see the workspace
[feature guide](../../docs/guide.md). Compiler calls are serialized because
upstream compiler state is process-global. MLIR IR construction and Rust-authored
passes are provided by the internal `ireec-ir` crate.

The compiler emits executable artifacts: compile and run only trusted input.
Build and deployment prerequisites, including LLVM/MLIR and native shared
libraries, are documented by [`iree-native`](../iree-native/README.md).

Run the end-to-end compiler/runtime examples from the workspace:

```powershell
cargo run -p iree-samples --example simple-abs
```
