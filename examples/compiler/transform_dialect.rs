//! Rust port of IREE's custom Vulkan transform-dialect compiler sample.

use ireec::{Compiler, Output, Phase, Pipeline, Session};
use std::{error::Error, path::Path};

const MODULE: &str =
    include_str!("../../extern/iree/samples/transform_dialect/example_module.mlir");

fn compile_to_phase(
    compiler: &Compiler,
    session: &Session,
    phase: Phase,
) -> Result<String, Box<dyn Error>> {
    let source = session.source_buffer("example_module.mlir", MODULE.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.set_compile_from_phase(Phase::ExecutableSources)?;
    invocation.set_compile_to_phase(phase)?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(compiler)?;
    invocation.output_ir(&mut output)?;
    Ok(std::str::from_utf8(output.bytes()?)?.to_owned())
}

fn main() -> Result<(), Box<dyn Error>> {
    let compiler = Compiler::new()?;
    let transform_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../extern/iree/samples/transform_dialect/transform_library.mlir");
    let transform_path = transform_path.canonicalize()?;
    let transform_path = transform_path
        .to_str()
        .ok_or("transform path is not UTF-8")?;
    let transform_flag =
        format!("--iree-codegen-transform-dialect-library={transform_path}@kernel_config");
    compiler.setup_global_cl(
        &["iree-compile", &transform_flag, "--mlir-disable-threading"],
        "IREE transform dialect sample",
        false,
    )?;
    let mut session = compiler.session()?;
    session.set_flags(&["--iree-hal-target-device=vulkan"])?;
    let configured = compile_to_phase(&compiler, &session, Phase::ExecutableConfigurations)?;
    assert!(configured.contains("codegen_spec = @custom_transform_strategy"));
    let output = compile_to_phase(&compiler, &session, Phase::ExecutableTargets)?;
    assert!(output.contains("spirv.func @example_module_dispatch_0"));
    assert!(output.contains("spirv.func @example_module_dispatch_1"));
    assert!(output.contains("spirv.func @example_module_dispatch_2"));
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn applies_custom_codegen_strategy_to_vulkan_executables()
    -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
