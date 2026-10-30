#![no_std]
#![no_main]

mod board;

use core::sync::atomic::{AtomicU32, Ordering};
use iree::{
    Result, StatusCode,
    runtime::{
        ContextFlags, Value, VmBuffer, VmInstance, VmList, engine::RuntimeInstance,
        vm::custom_ref::VmRefType,
    },
};

static HOST_DROPS: AtomicU32 = AtomicU32::new(0);
struct HostObject(u32);
impl Drop for HostObject {
    fn drop(&mut self) {
        HOST_DROPS.fetch_add(1, Ordering::Relaxed);
    }
}

fn run() -> Result<()> {
    let deadline = unsafe { iree::native::iree_time_now() } + 2_000_000;
    assert!(unsafe { iree::native::iree_wait_until(deadline) });
    assert!(unsafe { iree::native::iree_time_now() } >= deadline);
    // Only this main context enters IREE; SysTick updates an atomic counter.
    unsafe { iree::runtime::initialize_single_context() };
    assert_eq!(
        iree::runtime::Instance::new().err().unwrap().code(),
        StatusCode::UNAVAILABLE
    );
    let runtime = RuntimeInstance::builder().no_drivers().build()?;
    assert_eq!(
        runtime.default_device("local-sync").err().unwrap().code(),
        StatusCode::UNAVAILABLE
    );
    let mut buffer = VmBuffer::new(runtime.instance(), 4)?;
    buffer.write(0, &[1, 2, 3, 4])?;
    let retained = buffer.clone();
    let ty = VmRefType::<HostObject>::new(runtime.instance(), "bare_metal.host")?;
    let object = ty.create(HostObject(42));
    let reference = object.to_native_ref();
    drop(object);
    assert_eq!(HOST_DROPS.load(Ordering::Relaxed), 0);
    assert_eq!(ty.downcast(&reference)?.get().0, 42);
    let runtime_module = unsafe {
        runtime.instance().module_from_bytes(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.vmfb")).as_slice(),
        )?
    };
    assert_eq!(runtime_module.info()?.name, "bare_metal");
    drop(runtime);
    drop(buffer);
    assert_eq!(retained.read()?, [1, 2, 3, 4]);
    drop(reference);
    assert_eq!(HOST_DROPS.load(Ordering::Relaxed), 1);
    drop(ty);
    drop(runtime_module);
    drop(retained);
    let instance = unsafe { VmInstance::new()? };
    let module = unsafe {
        instance.module_from_bytes(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.vmfb")).as_slice(),
        )?
    };
    let mut context = instance.context_with_modules(ContextFlags::NONE, &[module])?;
    assert!(context.resolve_function("bare_metal.missing").is_err());
    let mut inputs = VmList::for_vm(&instance, 2)?;
    let mut outputs = VmList::for_vm(&instance, 1)?;
    assert_eq!(
        inputs.push_scalar(&Value::Null).unwrap_err().code(),
        StatusCode::INVALID_ARGUMENT
    );
    for _ in 0..2 {
        inputs.clear();
        inputs.push_scalar(&Value::I32(11))?;
        inputs.push_scalar(&Value::I32(31))?;
        context.invoke("bare_metal.add", &inputs, &mut outputs)?;
        assert!(matches!(outputs.get_scalar(0)?, Value::I32(42)));
        assert_eq!(outputs.len(), 1);
    }
    inputs.clear();
    inputs.push_scalar(&Value::F32(6.0))?;
    inputs.push_scalar(&Value::F32(7.0))?;
    context.invoke("bare_metal.multiply", &inputs, &mut outputs)?;
    assert!(matches!(outputs.get_scalar(0)?, Value::F32(42.0)));
    // Drop all owned Rust/native resources before reporting success.
    Ok(())
}
