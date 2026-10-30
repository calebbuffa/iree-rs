# ireec

`ireec` provides owned Rust wrappers for IREE's compiler. Create a
`Compiler`, configure a `Session`, compile through an `Invocation`, and collect
or write an `Output`.

```toml
[dependencies]
ireec = "0.1.0"
```

```rust
use ireec::{Compiler, Device, Options, Output, Pipeline, Target};

fn main() -> ireec::Result<()> {
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.configure(
        &Options::default()
            .with_device(Device::Alias("local".into()))
            .with_target(Target::Vmvx),
    )?;
    let source = session.source_buffer(
        "example.mlir",
        b"func.func @double(%x: i32) -> i32 { %0 = arith.addi %x, %x : i32 return %0 : i32 }",
    )?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    assert!(!output.bytes()?.is_empty());
    Ok(())
}
```

Compiler targets and input dialects are selected with Cargo features such as
`target-llvm-cpu` and `input-stablehlo`; see the
[feature guide](https://github.com/calebbuffa/iree-rs/blob/main/docs/guide.md).
The portable VMVX backend is available without extra features. Enabling a
backend makes it available; select it for each session with `Options` or flags.
Compiler calls are serialized because
upstream compiler state is process-global. MLIR IR construction and Rust-authored
passes are provided by the internal `ireec-ir` crate.

The compiler emits executable artifacts: compile and run only trusted input.
Build and deployment prerequisites, including LLVM/MLIR and native shared
libraries, are documented by [`iree-native`](https://docs.rs/iree-native).
Building the compiler requires LLVM/MLIR and is substantially more expensive
than a runtime-only build. Compatible native builds are cached automatically;
`IREE_NATIVE_DIR` is optional and selects a specific prebuilt SDK instead.

On Windows, deploy the compiler DLL and its dependencies alongside the executable
under `iree-native/<content-id>`, or set `IREE_COMPILER_RUNTIME_DIR` to their
absolute directory. The build-local deployment is used as a development fallback.
`IREE_NATIVE_DIR` selects build inputs; it is not the runtime DLL search setting.
