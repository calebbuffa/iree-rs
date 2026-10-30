//! Rust port of the compiler-side upstream EmitC module samples.

use ireec::{Compiler, Output, Pipeline};
use std::error::Error;

const ADD_MODULE: &str = include_str!("../../extern/iree/samples/emitc_modules/add.mlir");
const IMPORT_MODULE_A: &str =
    include_str!("../../extern/iree/samples/emitc_modules/import_module_a.mlir");
const IMPORT_MODULE_B: &str =
    include_str!("../../extern/iree/samples/emitc_modules/import_module_b.mlir");

fn compile_vm_c(
    compiler: &Compiler,
    session: &ireec::Session,
    name: &str,
    module: &str,
) -> Result<String, Box<dyn Error>> {
    let source = session.source_buffer(name, module.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Vm)?;
    let mut output = Output::memory(compiler)?;
    invocation.output_vm_c_source(&mut output)?;
    Ok(std::str::from_utf8(output.bytes()?)?.to_owned())
}

fn main() -> Result<(), Box<dyn Error>> {
    let compiler = Compiler::new()?;
    compiler.setup_global_cl(&["iree-compile"], "IREE EmitC sample", false)?;
    let session = compiler.session()?;

    let add = compile_vm_c(&compiler, &session, "add.mlir", ADD_MODULE)?;
    assert!(add.contains("add_and_double"), "{add}");
    assert!(add.contains("test_call"), "{add}");

    let import_a = compile_vm_c(&compiler, &session, "import_module_a.mlir", IMPORT_MODULE_A)?;
    assert!(import_a.contains("test_call"), "{import_a}");
    assert!(import_a.contains("module_b.square"), "{import_a}");

    let import_b = compile_vm_c(&compiler, &session, "import_module_b.mlir", IMPORT_MODULE_B)?;
    assert!(import_b.contains("square"), "{import_b}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn emits_upstream_vm_modules_as_c() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
