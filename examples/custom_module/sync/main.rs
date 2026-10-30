//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireert::{
    hal::{BufferParams, BufferUsage, ElementType, EncodingType, MemoryAccess, MemoryType},
    vm::{BytecodeModule, Context, Instance, List, Module, NativeModule, Ref, TypeDef},
};
use std::error::Error;

const PROGRAM: &str =
    include_str!("../../../extern/iree/samples/custom_module/sync/test/example.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir("custom_module_sync.mlir", PROGRAM, &[])?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    let device = sample_device.device.clone();
    let custom = NativeModule::builder("custom")
        .state(move || Ok(device.clone()), |device| Ok(device.clone()))
        .export_typed_with_state(
            "call.sync",
            |device: &ireert::hal::GroupedDevice, reference: Ref| -> ireert::Result<Ref> {
                let input = ireert::hal::BufferView::from_ref(&reference)?;
                // SAFETY: this callback ABI receives the declared initialized i32 tensor.
                let values = unsafe { input.read_elements_unchecked::<i32>(ElementType::INT_32)? };
                let output: Vec<_> = values
                    .into_iter()
                    .map(|value| value.wrapping_mul(2))
                    .collect();
                let allocator = device.allocator();
                let params = BufferParams {
                    usage: BufferUsage::DEFAULT,
                    access: MemoryAccess::ALL,
                    memory_type: MemoryType::DEVICE_LOCAL,
                    ..Default::default()
                };
                let result = ireert::hal::BufferView::allocate_buffer_copy_typed(
                    device,
                    &allocator,
                    &input.shape()?,
                    ElementType::INT_32,
                    EncodingType::DENSE_ROW_MAJOR,
                    params,
                    &output,
                )?;
                Ok(result.to_ref())
            },
        )
        .build(&instance)?;
    // SAFETY: `archive` was just produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context =
        Context::with_modules(&instance, Default::default(), &[hal_module, custom, module])?;
    let function = context.resolve_function("example.main")?;
    let allocator = sample_device.allocator();
    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };
    let input = [1i32, 2, 3, 4, 5];
    let view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &[ireert::hal::Dim::try_from(input.len())?],
        ElementType::INT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &input,
    )?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.resize(1)?;
    inputs.set_buffer_view(0, &view)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let result = outputs
        .get_buffer_view(0)
        .ok_or_else(|| ireert::Error::new(ireert::Code::DataLoss, "sample returned no tensor"))?;
    // SAFETY: the compiled export returns an initialized i32 tensor.
    let values = unsafe { result.read_elements_unchecked::<i32>(ElementType::INT_32)? };
    assert_eq!(values, [4, 64, 324, 1024, 2500]);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
