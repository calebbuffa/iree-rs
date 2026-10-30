//! Rust port of the compiler-side upstream CPU custom-dispatch samples.

use ireec::{Compiler, Output, Phase, Pipeline, Session};
use std::error::Error;

const HAL_MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/cpu/embedded/example_hal.mlir");
const STREAM_MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/cpu/embedded/example_stream.mlir");

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
    let session = compiler.session()?;
    let hal = compile_to_executable_sources(
        &compiler,
        &session,
        "custom_dispatch_cpu_hal.mlir",
        HAL_MODULE,
    )?;
    assert!(hal.contains("@simple_mul_workgroup"), "{hal}");
    assert!(hal.contains("functions_x86_64.o"), "{hal}");
    assert!(hal.contains("@mixed_invocation"), "{hal}");

    let stream = compile_to_executable_sources(
        &compiler,
        &session,
        "custom_dispatch_cpu_stream.mlir",
        STREAM_MODULE,
    )?;
    assert!(stream.contains("@simple_mul_workgroup"), "{stream}");
    assert!(stream.contains("functions_x86_64.o"), "{stream}");
    assert!(stream.contains("@mixed_invocation"), "{stream}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles_external_cpu_dispatch_abis() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
