//! Rust port of the compiler-side CPU executable-plugin samples.

use ireec::{Compiler, Output, Phase, Pipeline, Session};
use std::{error::Error, path::Path};

const SYSTEM_MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/cpu/plugin/system_example.mlir");
const STANDALONE_MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/cpu/plugin/standalone_example.mlir");
const MLP_MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/cpu/mlp_plugin/mlp.mlir");

fn compile_to_executable_sources(
    compiler: &Compiler,
    session: &Session,
    name: &str,
    module: &str,
) -> Result<String, Box<dyn Error>> {
    let source = session.source_buffer(name, module.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.set_compile_to_phase(Phase::ExecutableSources)?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(compiler)?;
    invocation.output_ir(&mut output)?;
    Ok(std::str::from_utf8(output.bytes()?)?.to_owned())
}

fn main() -> Result<(), Box<dyn Error>> {
    let compiler = Compiler::new()?;
    let mut system_session = compiler.session()?;
    system_session.set_flags(&[
        "--iree-hal-target-device=local",
        "--iree-hal-local-target-device-backends=llvm-cpu",
    ])?;
    let system = compile_to_executable_sources(
        &compiler,
        &system_session,
        "custom_dispatch_cpu_plugin.mlir",
        SYSTEM_MODULE,
    )?;
    assert!(system.contains("@simple_mul_workgroup"), "{system}");
    assert!(system.contains("@mixed_invocation"), "{system}");
    assert!(system.contains("processor_data"), "{system}");

    let standalone = compile_to_executable_sources(
        &compiler,
        &system_session,
        "custom_dispatch_cpu_standalone_plugin.mlir",
        STANDALONE_MODULE,
    )?;
    assert!(standalone.contains("@simple_mul_workgroup"), "{standalone}");
    assert!(standalone.contains("@mixed_invocation"), "{standalone}");

    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../extern/iree/samples/custom_dispatch/cpu/mlp_plugin/mlp_spec.mlir")
        .canonicalize()?;
    let spec_path = spec_path
        .to_str()
        .ok_or("transform spec path is not UTF-8")?;
    let transform_flag = format!("--iree-preprocessing-transform-spec-filename={spec_path}");
    let mut mlp_session = compiler.session()?;
    mlp_session.set_flags(&[
        "--iree-hal-target-device=local",
        "--iree-hal-local-target-device-backends=llvm-cpu",
        &transform_flag,
    ])?;
    let mlp = compile_to_executable_sources(
        &compiler,
        &mlp_session,
        "custom_dispatch_cpu_mlp_plugin.mlir",
        MLP_MODULE,
    )?;
    assert!(mlp.contains("@mlp_external"), "{mlp}");
    assert!(mlp.contains("@mlp_invocation"), "{mlp}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles_external_cpu_plugin_abis() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
