//! Shared implementation of the externally allocated transient-buffer sample.

use iree_samples::vm_support::SampleDevice;
use ireert::{
    Code, Error,
    hal::{
        BufferParams, BufferUsage, BufferView, ElementType, EncodingType, MemoryAccess, MemoryType,
    },
    vm::{BytecodeModule, Context, Instance, List, Module, TypeDef, Value, ValueType},
};
use std::error::Error as StdError;

const ARCHIVE: &[u8] = include_bytes!("../assets/external_transients.vmfb");
const INPUT_SIZE: usize = 64;

pub fn run(uri: Option<&str>) -> Result<(), Box<dyn StdError>> {
    let instance = Instance::new()?;
    let sample_device = match uri {
        Some(uri) => SampleDevice::from_uri(uri)?,
        None => SampleDevice::from_environment()?,
    };
    let hal_module = sample_device.hal_module(&instance)?;
    // SAFETY: this archive is generated from the checked-in upstream sample MLIR.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), ARCHIVE)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let function = context.resolve_function("module.in_place_computation")?;
    let allocator = sample_device.allocator();
    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };

    let input = buffer_view(
        &sample_device.device,
        &allocator,
        &[1.0; INPUT_SIZE],
        params,
    )?;
    let output = buffer_view(
        &sample_device.device,
        &allocator,
        &[0.0; INPUT_SIZE],
        params,
    )?;

    let mut size_inputs = List::new(TypeDef::Any, 2)?;
    size_inputs.resize(2)?;
    size_inputs.set_buffer_view(0, &input)?;
    size_inputs.set_buffer_view(1, &output)?;
    let transient_size = query_transient_size(&context, &function, &size_inputs)?;

    let transient = allocator.allocate_buffer(params, transient_size)?;
    let mut inputs = List::new(TypeDef::Any, 3)?;
    inputs.resize(3)?;
    inputs.set_buffer_view(0, &input)?;
    inputs.set_buffer_view(1, &output)?;
    inputs.set_buffer(2, &transient)?;
    let mut results = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut results)?;

    let result = results
        .get_buffer_view(0)
        .ok_or_else(|| Error::new(Code::NotFound, "missing transient sample result"))?;
    let result_buffer = result.buffer();
    let mut values = [0.0f32; INPUT_SIZE];
    // SAFETY: in_place_computation initializes the full result buffer.
    unsafe {
        sample_device
            .device
            .transfer_d2h_typed(&result_buffer, 0, &mut values, None)?
    };
    assert_eq!(values, [5.0; INPUT_SIZE]);
    Ok(())
}

fn query_transient_size(
    context: &Context,
    function: &ireert::vm::Function,
    inputs: &List,
) -> Result<usize, Box<dyn StdError>> {
    if let Some(size) = function.attribute("iree.abi.transients.size.constant") {
        return Ok(size.parse()?);
    }

    let size_function_name = function
        .attribute("iree.abi.transients.size")
        .ok_or_else(|| Error::new(Code::NotFound, "function has no transient size metadata"))?;
    let size_function = context.resolve_function(&size_function_name)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&size_function, Default::default(), inputs, &mut outputs)?;
    let value = outputs.get_value_as(0, ValueType::I64)?;
    let Value::I64(size) = value else {
        return Err(Error::new(Code::DataLoss, "transient size query did not return i64").into());
    };
    Ok(usize::try_from(size)?)
}

fn buffer_view(
    device: &ireert::hal::Device,
    allocator: &ireert::hal::Allocator,
    values: &[f32],
    params: BufferParams,
) -> ireert::Result<BufferView> {
    BufferView::allocate_buffer_copy_typed(
        device,
        allocator,
        &[INPUT_SIZE],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        values,
    )
}
