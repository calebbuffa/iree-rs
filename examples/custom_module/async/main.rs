//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireert::{
    hal::{
        BufferParams, BufferUsage, ElementType, EncodingType, Fence, MemoryAccess, MemoryType,
        QueueAffinity, Semaphore,
    },
    vm::{BytecodeModule, Context, InlineLoop, Instance, List, Module, NativeModule, Ref, TypeDef},
};
use std::{error::Error, sync::mpsc, thread};

const PROGRAM: &str =
    include_str!("../../../extern/iree/samples/custom_module/async/test/example.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir(
        "custom_module_async.mlir",
        PROGRAM,
        &["--iree-execution-model=async-external"],
    )?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    let device = sample_device.device.clone();
    let custom = NativeModule::builder("custom")
        .state(move || Ok(device.clone()), |device| Ok(device.clone()))
        .export_typed_with_state(
            "call.async",
            |device: &ireert::hal::GroupedDevice,
             (source_ref, wait_ref, signal_ref): (Ref, Ref, Ref)|
             -> ireert::Result<Ref> {
                let source = ireert::hal::BufferView::from_ref(&source_ref)?;
                let wait_fence = Fence::from_ref(&wait_ref)?;
                let signal_fence = Fence::from_ref(&signal_ref)?;
                let allocator = device.allocator();
                let params = BufferParams {
                    usage: BufferUsage::DEFAULT,
                    access: MemoryAccess::ALL,
                    memory_type: MemoryType::DEVICE_LOCAL,
                    ..Default::default()
                };
                let zeros = vec![0i32; source.element_count()];
                let target = ireert::hal::BufferView::allocate_buffer_copy_typed(
                    device,
                    &allocator,
                    &source.shape()?,
                    ElementType::INT_32,
                    EncodingType::DENSE_ROW_MAJOR,
                    params,
                    &zeros,
                )?;
                let worker_source = source.clone();
                let worker_target = target.clone();
                thread::spawn(move || {
                    let result = wait_fence.wait(None, Default::default()).and_then(|()| {
                        // SAFETY: the VM-provided wait fence guarantees the tensor contents are ready.
                        let values = unsafe {
                            worker_source.read_elements_unchecked::<i32>(ElementType::INT_32)?
                        };
                        let doubled: Vec<_> = values
                            .into_iter()
                            .map(|value| value.wrapping_mul(2))
                            .collect();
                        worker_target.write_elements(ElementType::INT_32, &doubled)
                    });
                    match result {
                        Ok(()) => {
                            if let Err(error) = signal_fence.signal() {
                                signal_fence.fail(error.code());
                            }
                        }
                        Err(error) => signal_fence.fail(error.code()),
                    }
                });
                Ok(target.to_ref())
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
    let shape = [ireert::hal::Dim::try_from(input.len())?];
    let view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &shape,
        ElementType::INT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &input,
    )?;
    let semaphore = Semaphore::new(&sample_device.device, QueueAffinity::ANY, 0)?;
    let wait_fence = Fence::at(&semaphore, 1)?;
    let signal_fence = Fence::at(&semaphore, 2)?;
    wait_fence.signal()?;
    let mut inputs = List::new(TypeDef::Any, 3)?;
    inputs.resize(3)?;
    inputs.set_buffer_view(0, &view)?;
    inputs.set_fence(1, &wait_fence)?;
    inputs.set_fence(2, &signal_fence)?;
    let outputs = List::new(TypeDef::Any, 1)?;
    let (sender, receiver) = mpsc::channel();
    let inline_loop = InlineLoop::new();
    context.invoke_async(
        inline_loop.as_loop(),
        &function,
        Default::default(),
        &inputs,
        outputs,
        move |_, result| {
            let result = result.and_then(|outputs| {
                let result = outputs.get_buffer_view(0).ok_or_else(|| {
                    ireert::Error::new(ireert::Code::DataLoss, "async sample returned no tensor")
                })?;
                signal_fence.wait(None, Default::default())?;
                // SAFETY: the completion fence reaches T=2 only after the output bytes are ready.
                unsafe { result.read_elements_unchecked::<i32>(ElementType::INT_32) }
            });
            sender.send(result).map_err(|_| {
                ireert::Error::new(ireert::Code::Aborted, "async result receiver was dropped")
            })?;
            Ok(())
        },
    )?;
    inline_loop.as_loop().drain(None)?;
    let values = receiver.recv()??;
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
