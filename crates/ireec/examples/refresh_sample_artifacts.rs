//! Refresh bytecode archives embedded by the upstream sample ports.

use ireec::{Compiler, Output, Pipeline};
use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let compiler = Compiler::new()?;
    compile(
        &compiler,
        &workspace.join("extern/iree/samples/simple_embedding/simple_embedding_test.mlir"),
        "simple_embedding_test.mlir",
        &workspace.join("examples/assets/simple_embedding.vmfb"),
        &[
            "--iree-hal-target-device=local",
            "--iree-hal-local-target-device-backends=vmvx",
        ],
    )?;
    compile(
        &compiler,
        &workspace.join("extern/iree/samples/simple_embedding/simple_embedding_test.mlir"),
        "simple_embedding_test.mlir",
        &workspace.join("examples/assets/simple_embedding_llvm_cpu.vmfb"),
        &[
            "--iree-hal-target-device=local",
            "--iree-hal-local-target-device-backends=llvm-cpu",
            "--iree-llvmcpu-target-triple=x86_64-pc-linux-elf",
            "--iree-llvmcpu-target-cpu=generic",
            "--iree-llvmcpu-debug-symbols=false",
            "--iree-vm-bytecode-module-strip-source-map=true",
            "--iree-vm-emit-polyglot-zip=false",
        ],
    )?;
    compile(
        &compiler,
        &workspace.join("extern/iree/samples/simple_embedding/simple_embedding_test.mlir"),
        "simple_embedding_test.mlir",
        &workspace.join("examples/assets/simple_embedding_vulkan.vmfb"),
        &["--iree-hal-target-device=vulkan"],
    )?;
    compile(
        &compiler,
        &workspace.join("extern/iree/samples/simple_embedding/simple_embedding_test.mlir"),
        "simple_embedding_test.mlir",
        &workspace.join("examples/assets/simple_embedding_cuda.vmfb"),
        &["--iree-hal-target-device=cuda"],
    )?;
    compile(
        &compiler,
        &workspace.join("extern/iree/samples/external_transients/in_place_computation.mlir"),
        "in_place_computation.mlir",
        &workspace.join("examples/assets/external_transients.vmfb"),
        &[
            "--iree-hal-target-device=local",
            "--iree-hal-local-target-device-backends=vmvx",
        ],
    )?;
    Ok(())
}

fn compile(
    compiler: &Compiler,
    source_path: &Path,
    source_name: &str,
    output_path: &Path,
    flags: &[&str],
) -> Result<(), Box<dyn Error>> {
    let source_text = fs::read(source_path)?;
    let mut session = compiler.session()?;
    session.set_flags(flags)?;
    let source = session.source_buffer(source_name, &source_text)?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    fs::create_dir_all(output_path.parent().ok_or("artifact path has no parent")?)?;
    fs::write(output_path, output.bytes()?)?;
    Ok(())
}
