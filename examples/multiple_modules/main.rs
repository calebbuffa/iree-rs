//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireert::{
    hal::{
        BufferParams, BufferUsage, ElementType, EncodingType, Fence, MemoryAccess, MemoryType,
        QueueAffinity, Semaphore,
    },
    vm::{BytecodeModule, Context, Instance, List, TypeDef},
};
use std::error::Error;

const MODULE_A: &str = include_str!("../../extern/iree/samples/multiple_modules/module_a.mlir");
const MODULE_B: &str = include_str!("../../extern/iree/samples/multiple_modules/module_b.mlir");
const PIPELINE_SYNC: &str =
    include_str!("../../extern/iree/samples/multiple_modules/pipeline_sync.mlir");
const PIPELINE_ASYNC: &str =
    include_str!("../../extern/iree/samples/multiple_modules/pipeline_async.mlir");
const ELEMENTS: usize = 4096;

fn main() -> Result<(), Box<dyn Error>> {
    run(false)?;
    run(true)?;
    Ok(())
}

fn run(asynchronous: bool) -> Result<(), Box<dyn Error>> {
    let async_flags: &[&str] = if asynchronous {
        &["--iree-execution-model=async-external"]
    } else {
        &[]
    };
    let module_a = compile_mlir("module_a.mlir", MODULE_A, async_flags)?;
    let module_b = compile_mlir("module_b.mlir", MODULE_B, async_flags)?;
    let pipeline_source = if asynchronous {
        PIPELINE_ASYNC
    } else {
        PIPELINE_SYNC
    };
    let pipeline = compile_mlir("pipeline.mlir", pipeline_source, async_flags)?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    // SAFETY: each archive was compiled from the upstream MLIR source above.
    let module_a = unsafe { BytecodeModule::new(&instance, Default::default(), module_a)? };
    // SAFETY: each archive was compiled from the upstream MLIR source above.
    let module_b = unsafe { BytecodeModule::new(&instance, Default::default(), module_b)? };
    // SAFETY: each archive was compiled from the upstream MLIR source above.
    let pipeline = unsafe { BytecodeModule::new(&instance, Default::default(), pipeline)? };
    let context = Context::with_modules(
        &instance,
        Default::default(),
        &[
            hal_module,
            module_a.into_module(),
            module_b.into_module(),
            pipeline.into_module(),
        ],
    )?;
    let function = context.resolve_function("module.run")?;
    let allocator = sample_device.allocator();
    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };
    let values = vec![-2.0f32; ELEMENTS];
    let shape = [ireert::hal::Dim::try_from(ELEMENTS)?];
    let view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &shape,
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &values,
    )?;
    let mut inputs = List::new(TypeDef::Any, if asynchronous { 3 } else { 1 })?;
    inputs.resize(1)?;
    inputs.set_buffer_view(0, &view)?;
    let output_fence = if asynchronous {
        let semaphore = Semaphore::new(&sample_device.device, QueueAffinity::ANY, 0)?;
        let wait_fence = Fence::at(&semaphore, 1)?;
        let signal_fence = Fence::at(&semaphore, 2)?;
        wait_fence.signal()?;
        inputs.resize(3)?;
        inputs.set_fence(1, &wait_fence)?;
        inputs.set_fence(2, &signal_fence)?;
        Some(signal_fence)
    } else {
        None
    };
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    if let Some(fence) = output_fence {
        fence.wait(None, Default::default())?;
    }
    let result = outputs.get_buffer_view(0).ok_or_else(|| {
        ireert::Error::new(ireert::Code::DataLoss, "pipeline returned no buffer view")
    })?;
    // SAFETY: the pipeline result is the initialized f32 tensor produced by the VM.
    let values = unsafe { result.read_elements_unchecked::<f32>(ElementType::FLOAT_32)? };
    assert_eq!(values, vec![4.0; ELEMENTS]);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
