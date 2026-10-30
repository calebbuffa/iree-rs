//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireert::{
    hal::{BufferParams, BufferUsage, ElementType, EncodingType, MemoryAccess, MemoryType},
    vm::{BytecodeModule, Context, Instance, List, Module, TypeDef},
};
use std::error::Error;

const PROGRAM: &str = include_str!("../../extern/iree/samples/models/simple_abs.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir("simple_abs.mlir", PROGRAM, &[])?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    // SAFETY: `archive` was just produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let function = context.resolve_function("module.abs")?;
    let allocator = sample_device.allocator();
    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };
    let view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &[],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &[-2.0f32],
    )?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.resize(1)?;
    inputs.set_buffer_view(0, &view)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let output = outputs.get_buffer_view(0).ok_or_else(|| {
        ireert::Error::new(ireert::Code::DataLoss, "missing absolute-value result")
    })?;
    // SAFETY: the compiled model returns an initialized f32 tensor.
    let values = unsafe { output.read_elements_unchecked::<f32>(ElementType::FLOAT_32)? };
    assert_eq!(values, [2.0]);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
