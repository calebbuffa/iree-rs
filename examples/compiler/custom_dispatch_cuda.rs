//! Rust port of the compiler-side upstream CUDA custom-dispatch sample.

use ireec::{Compiler, Output, Phase, Pipeline};
use std::error::Error;

const MODULE: &str =
    include_str!("../../extern/iree/samples/custom_dispatch/cuda/kernels/example.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let compiler = Compiler::new()?;
    let session = compiler.session()?;
    let source = session.source_buffer("custom_dispatch_cuda.mlir", MODULE.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.set_compile_to_phase(Phase::ExecutableSources)?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_ir(&mut output)?;
    let output = std::str::from_utf8(output.bytes()?)?;

    assert!(output.contains("@simple_mul"), "{output}");
    assert!(output.contains("@simple_mul_inplace"), "{output}");
    assert!(output.contains("kernels_sm_52.ptx"), "{output}");
    assert!(output.contains("kernels_sm_80.ptx"), "{output}");
    assert!(output.contains("@mixed_invocation"), "{output}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles_external_cuda_dispatch_abi() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
