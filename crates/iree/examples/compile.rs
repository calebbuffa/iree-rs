//! Compile a model ahead of time for a runtime-only application.
use iree::{Compiler, Target};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = args
        .next()
        .ok_or("usage: compile <input.mlir> <output.vmfb>")?;
    let output = args
        .next()
        .ok_or("usage: compile <input.mlir> <output.vmfb>")?;
    if args.next().is_some() {
        return Err("usage: compile <input.mlir> <output.vmfb>".into());
    }
    let source = std::fs::read_to_string(input)?;
    let artifact = Compiler::new()?.compile(&source, Target::Cpu)?;
    std::fs::write(output, artifact.as_bytes())?;
    Ok(())
}
