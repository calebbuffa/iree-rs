//! Rust port of the compiler-side custom Vulkan dispatch sample.

use ireec::{Compiler, Output, Phase, Pipeline};
use std::error::Error;

const MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/vulkan/shaders/example_inline.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.set_flags(&["--iree-hal-target-device=vulkan"])?;
    let source = session.source_buffer("custom_dispatch.mlir", MODULE.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.set_compile_to_phase(Phase::ExecutableSources)?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_ir(&mut output)?;
    let output = std::str::from_utf8(output.bytes()?)?;

    assert!(output.contains("@extern_dispatch"), "{output}");
    assert!(output.contains("simple_mul.spv"), "{output}");
    assert!(output.contains("simple_mul"));
    assert!(output.contains("mixed_invocation"));
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles_external_vulkan_dispatch_abi() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
