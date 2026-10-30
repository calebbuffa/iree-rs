//! Rust port of the upstream bare-metal RISC-V VMVX-inline runner.

use ireec::{Compiler, Output, Pipeline};
use ireert::{
    hal::{
        Allocator, BufferParams, BufferUsage, BufferView, ElementType, EncodingType, MemoryAccess,
        MemoryType,
    },
    vm::{BytecodeModule, Context, DebugSink, Instance, List, Module, TypeDef},
};
use std::error::Error;

const MODULE: &str = include_str!("../../extern/iree/samples/baremetal_riscv64/simple_mul.mlir");

fn compile_module() -> Result<Vec<u8>, Box<dyn Error>> {
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.set_flags(&[
        "--iree-execution-model=inline-static",
        "--iree-hal-target-device=local",
        "--iree-hal-local-target-device-backends=vmvx-inline",
        "--iree-vm-bytecode-module-strip-source-map=true",
        "--iree-vm-emit-polyglot-zip=false",
    ])?;
    let source = session.source_buffer("simple_mul.mlir", MODULE.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    Ok(output.bytes()?.to_vec())
}

fn input_view(allocator: &Allocator, values: &[f32]) -> Result<BufferView, Box<dyn Error>> {
    let params = BufferParams {
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
        usage: BufferUsage::DEFAULT,
        ..Default::default()
    };
    let buffer = allocator.allocate_buffer(params, 16)?;
    buffer.write_typed(0, values)?;
    Ok(BufferView::create(
        &buffer,
        &[4],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
    )?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_module()?;
    let instance = Instance::new()?;
    let allocator = Allocator::heap("bare-metal")?;
    let hal_module = Module::hal_inline(&instance, &allocator, DebugSink::default())?;
    // SAFETY: the module was compiled to VM bytecode in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let function = context.resolve_function("module.simple_mul")?;
    let lhs = input_view(&allocator, &[4.0; 4])?;
    let rhs = input_view(&allocator, &[2.0; 4])?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.resize(2)?;
    inputs.set_buffer_view(0, &lhs)?;
    inputs.set_buffer_view(1, &rhs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let result = outputs
        .get_buffer_view(0)
        .ok_or("missing simple_mul result")?;
    // SAFETY: the compiled module initializes a four-element f32 result tensor.
    let values = unsafe { result.read_elements_unchecked::<f32>(ElementType::FLOAT_32)? };
    assert_eq!(values, [8.0; 4]);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_the_bare_metal_vmvx_inline_runner() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
