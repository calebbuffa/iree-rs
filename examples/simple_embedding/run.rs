//! Shared implementation of the embedded-bytecode HAL sample.

use iree_samples::vm_support::SampleDevice;
use ireert::{
    hal::{
        BufferParams, BufferUsage, BufferView, ElementType, EncodingType, MemoryAccess, MemoryType,
    },
    vm::{BytecodeModule, Context, Instance, List, Module, TypeDef},
};
use std::error::Error;

pub fn run_with_device(
    sample_device: SampleDevice,
    archive: &'static [u8],
) -> Result<(), Box<dyn Error>> {
    let instance = Instance::new()?;
    let hal_module = sample_device.hal_module(&instance)?;
    // SAFETY: this archive is generated from the checked-in upstream sample MLIR.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let function = context.resolve_function("module.simple_mul")?;

    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };
    let allocator = sample_device.allocator();
    let lhs = tensor(
        &sample_device.device,
        &allocator,
        &[4.0, 4.0, 4.0, 4.0],
        params,
    )?;
    let rhs = tensor(
        &sample_device.device,
        &allocator,
        &[2.0, 2.0, 2.0, 2.0],
        params,
    )?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.resize(2)?;
    inputs.set_buffer_view(0, &lhs)?;
    inputs.set_buffer_view(1, &rhs)?;
    let mut outputs = List::new(TypeDef::Any, 2)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;

    let product = outputs
        .get_buffer_view(0)
        .ok_or("simple_mul returned no product tensor")?;
    let product_buffer = product.buffer();
    let mut product_values = [0.0f32; 4];
    // SAFETY: the module initializes every product element before returning.
    unsafe {
        sample_device
            .device
            .transfer_d2h_typed(&product_buffer, 0, &mut product_values, None)?
    };
    assert_eq!(product_values, [8.0; 4]);

    let passthrough = outputs
        .get_buffer_view(1)
        .ok_or("simple_mul returned no passthrough tensor")?;
    let passthrough_buffer = passthrough.buffer();
    let mut passthrough_values = [0.0f32; 4];
    // SAFETY: the module initializes every passthrough element before returning.
    unsafe {
        sample_device.device.transfer_d2h_typed(
            &passthrough_buffer,
            0,
            &mut passthrough_values,
            None,
        )?
    };
    assert_eq!(passthrough_values, [4.0; 4]);
    Ok(())
}

fn tensor(
    device: &ireert::hal::Device,
    allocator: &ireert::hal::Allocator,
    values: &[f32],
    params: BufferParams,
) -> ireert::Result<BufferView> {
    BufferView::allocate_buffer_copy_typed(
        device,
        allocator,
        &[4],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        values,
    )
}
