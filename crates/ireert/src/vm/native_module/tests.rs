//! Laws for native-module tables, callback ownership, and invocation.

use super::trampolines::boundary;
use super::*;
use proptest::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

proptest! {
    #[test]
    fn fragment_layout_matches_native(cconv in "[iIfFr]{0,20}") {
        let fragment = Fragment::parse(&cconv).unwrap();
        let mut native_size = 0;
        // SAFETY: the fragment view lives for the call and has no spans.
        crate::check(unsafe {
            n::iree_vm_function_call_compute_cconv_fragment_size(
                crate::view(&cconv), ptr::null(), &mut native_size,
            )
        }).unwrap();
        prop_assert_eq!(fragment.size, native_size);
        prop_assert_eq!(fragment.slots.len(), cconv.len());
        let alignment = std::mem::align_of::<n::iree_vm_ref_t>();
        prop_assert!(fragment.refs.iter().all(|offset| offset.is_multiple_of(alignment)));
        prop_assert!(fragment.slots.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn scalar_result_round_trips(value in any::<i32>()) {
        let mut definition = NativeModule::builder("test")
            .export("identity", "0i_i", |_| Ok(()));
        definition.exports[0].layout = Some(Layout::parse("0i_i").unwrap());
        let instance = Instance::new().unwrap();
        let data = ModuleData::new(definition, instance);
        let state = CallState { value: Arc::new(()), imports: Mutex::new(Vec::new()) };
        let input = value.to_ne_bytes();
        let mut output = [0; 4];
        let mut call = NativeCall {
            module: &data, state: &state,
            layout: data.definition.exports[0].layout.as_ref().unwrap(),
            arguments: &input,
            results: n::iree_byte_span_t { data: output.as_mut_ptr(), data_length: 4 },
            complete: false,
            phase: NativeCallPhase::Begin,
        };
        let argument: i32 = call.argument(0).unwrap();
        call.set_result(0, argument).unwrap();
        prop_assert_eq!(call.argument::<i64>(0).unwrap_err().code(), Code::OutOfRange);
        call.complete = true;
        drop(call);
        prop_assert_eq!(output, value.to_ne_bytes());
    }
}

#[test]
fn resumed_calls_reject_absent_scalar_and_reference_arguments() {
    let mut definition = NativeModule::builder("resume").export("read", "0ir_v", |_| Ok(()));
    definition.exports[0].layout = Some(Layout::parse("0ir_v").unwrap());
    let data = ModuleData::new(definition, Instance::new().unwrap());
    let state = CallState {
        value: Arc::new(()),
        imports: Mutex::new(Vec::new()),
    };
    let layout = data.definition.exports[0].layout.as_ref().unwrap();
    let call = NativeCall {
        module: &data,
        state: &state,
        layout,
        arguments: &[],
        results: n::iree_byte_span_t::default(),
        complete: true,
        phase: NativeCallPhase::Resume,
    };
    assert_eq!(
        call.argument::<i32>(0).unwrap_err().code(),
        Code::OutOfRange
    );
    assert_eq!(
        call.argument_ref(layout.arguments.refs[0])
            .err()
            .unwrap()
            .code(),
        Code::OutOfRange
    );
}

#[test]
fn duplicate_exports_are_rejected_before_create() {
    let instance = Instance::new().unwrap();
    let error = NativeModule::builder("test")
        .export("same", "0v_v", |_| Ok(()))
        .export("same", "0v_v", |_| Ok(()))
        .build(&instance)
        .err()
        .unwrap();
    assert_eq!(error.code(), Code::InvalidArgument);
}

#[derive(Clone, Copy)]
struct WrappedI32(i32);

impl NativeValue for WrappedI32 {
    const ABI: char = 'i';

    fn read(call: &NativeCall<'_>, index: usize) -> Result<Self> {
        let offset = call
            .argument_offset(index)
            .ok_or_else(|| Error::new(Code::OutOfRange, "argument index is out of range"))?;
        call.argument(offset).map(Self)
    }

    fn write(self, call: &mut NativeCall<'_>, index: usize) -> Result<()> {
        let offset = call
            .result_offset(index)
            .ok_or_else(|| Error::new(Code::OutOfRange, "result index is out of range"))?;
        call.set_result(offset, self.0)
    }
}

#[test]
fn typed_exports_derive_abi_and_round_trip_values() {
    use super::super::{Context, ContextFlags, InvocationFlags, List, TypeDef, Value};
    let instance = Instance::new().unwrap();
    let module = NativeModule::builder("typed")
        .export_typed("add", |(lhs, rhs): (i32, i32)| -> Result<i32> {
            Ok(lhs + rhs)
        })
        .export_typed(
            "round_trip",
            |values: (i32, i64, f32, f64)| -> Result<(i32, i64, f32, f64)> { Ok(values) },
        )
        .export_typed("wrapped", |value: WrappedI32| -> Result<WrappedI32> {
            Ok(WrappedI32(value.0 + 1))
        })
        .export_typed("bool_not", |value: bool| -> Result<bool> { Ok(!value) })
        .export_typed("unsigned", |values: (u32, u64)| -> Result<(u32, u64)> {
            Ok(values)
        })
        .build(&instance)
        .unwrap();
    let function = module
        .lookup_function(super::super::FunctionLinkage::Export, "add")
        .unwrap();
    assert_eq!(function.signature(), "0ii_i");
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let function = context.resolve_function("typed.add").unwrap();
    let mut inputs = List::new(TypeDef::Any, 2).unwrap();
    inputs.push(Value::I32(19)).unwrap();
    inputs.push(Value::I32(23)).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(42));

    assert_eq!(
        context
            .resolve_function("typed.round_trip")
            .unwrap()
            .signature(),
        "0iIfF_iIfF"
    );
    inputs.clear();
    inputs.push(Value::I32(-7)).unwrap();
    inputs.push(Value::I64(0x1020_3040_5060_7080)).unwrap();
    inputs.push(Value::F32(1.25)).unwrap();
    inputs.push(Value::F64(-9.5)).unwrap();
    outputs.clear();
    context
        .invoke(
            &context.resolve_function("typed.round_trip").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(-7));
    assert_eq!(
        outputs.get_value(1).unwrap(),
        Value::I64(0x1020_3040_5060_7080)
    );
    assert_eq!(outputs.get_value(2).unwrap(), Value::F32(1.25));
    assert_eq!(outputs.get_value(3).unwrap(), Value::F64(-9.5));

    assert_eq!(
        context
            .resolve_function("typed.unsigned")
            .unwrap()
            .signature(),
        "0iI_iI"
    );
    inputs.clear();
    inputs.push(Value::I32(-1)).unwrap();
    inputs.push(Value::I64(-1)).unwrap();
    outputs.clear();
    context
        .invoke(
            &context.resolve_function("typed.unsigned").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(-1));
    assert_eq!(outputs.get_value(1).unwrap(), Value::I64(-1));

    inputs.clear();
    inputs.push(Value::I32(1)).unwrap();
    outputs.clear();
    context
        .invoke(
            &context.resolve_function("typed.bool_not").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(0));
    inputs.clear();
    inputs.push(Value::I32(2)).unwrap();
    assert_eq!(
        context
            .invoke(
                &context.resolve_function("typed.bool_not").unwrap(),
                InvocationFlags::default(),
                &inputs,
                &mut outputs,
            )
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );

    inputs.clear();
    inputs.push(Value::I32(41)).unwrap();
    outputs.clear();
    context
        .invoke(
            &context.resolve_function("typed.wrapped").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(42));
}

#[test]
fn typed_exports_support_references_null_results_and_void_calls() {
    use super::super::{Context, ContextFlags, InvocationFlags, List, RefType, TypeDef};
    let instance = Instance::new().unwrap();
    let ref_type = RefType::register::<i32>(&instance, "typed.value").unwrap();
    let module = NativeModule::builder("typed")
        .export_typed("echo", |value: Ref| -> Result<Ref> { Ok(value) })
        .export_typed("null", |(): ()| -> Result<Option<Ref>> { Ok(None) })
        .export_typed("noop", |(): ()| -> Result<()> { Ok(()) })
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    assert_eq!(
        context.resolve_function("typed.null").unwrap().signature(),
        "0v_r"
    );
    assert_eq!(
        context.resolve_function("typed.noop").unwrap().signature(),
        "0v_v"
    );

    let reference = ref_type.create(42).unwrap();
    let mut inputs = List::new(TypeDef::Any, 1).unwrap();
    inputs.push(reference).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    context
        .invoke(
            &context.resolve_function("typed.echo").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    let returned = outputs.get_ref(0).unwrap();
    assert_eq!(ref_type.downcast::<i32>(&returned), Some(&42));

    inputs.clear();
    outputs.clear();
    context
        .invoke(
            &context.resolve_function("typed.null").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    assert!(outputs.get(0).unwrap().is_none());

    outputs.clear();
    context
        .invoke(
            &context.resolve_function("typed.noop").unwrap(),
            InvocationFlags::default(),
            &inputs,
            &mut outputs,
        )
        .unwrap();
    assert!(outputs.is_empty());
}

#[test]
fn native_destroy_owns_callback_state_after_success() {
    struct Counted(Arc<AtomicUsize>);
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let drops = Arc::new(AtomicUsize::new(0));
    let captured = Counted(drops.clone());
    let instance = Instance::new().unwrap();
    let module = NativeModule::builder("test")
        .export("run", "0v_v", move |_| {
            let _ = &captured;
            Ok(())
        })
        .build(&instance)
        .unwrap();
    let clone = module.clone();
    drop(module);
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(clone);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn panic_payload_destructor_is_contained() {
    struct PanickingPayload;
    impl Drop for PanickingPayload {
        fn drop(&mut self) {
            panic!("payload drop");
        }
    }
    let status = boundary(|| std::panic::panic_any(PanickingPayload));
    let error = crate::check(status).unwrap_err();
    assert_eq!(error.code(), Code::Internal);
}

#[test]
fn deferred_status_cannot_escape_non_call_callbacks() {
    let status = boundary(|| Err(Error::new(Code::Deferred, "yield")));
    assert_eq!(
        crate::check(status).unwrap_err().code(),
        Code::Unimplemented
    );
}

proptest! {
    #[test]
    fn vm_invoke_preserves_scalar_and_borrowed_ref(
        scalar in any::<i32>(), payload in any::<i64>(),
    ) {
        use super::super::{Context, ContextFlags, InvocationFlags, List, RefType, TypeDef, Value};
        let instance = Instance::new().unwrap();
        let ty = RefType::register::<i64>(&instance, "test.payload").unwrap();
        let reference = ty.create(payload).unwrap();
        let module = NativeModule::builder("test")
            .export("identity", "0ir_ir", |call| {
                let missing = || Error::new(Code::OutOfRange, "missing slot");
                let scalar: i32 = call.argument(call.argument_offset(0).ok_or_else(missing)?)?;
                let reference = call.argument_ref(call.argument_offset(1).ok_or_else(missing)?)?;
                call.set_result(call.result_offset(0).ok_or_else(missing)?, scalar)?;
                call.set_result_ref(call.result_offset(1).ok_or_else(missing)?, reference)
            }).build(&instance).unwrap();
        let context = Context::with_modules(
            &instance, ContextFlags::default(), &[module],
        ).unwrap();
        let function = context.resolve_function("test.identity").unwrap();
        let mut inputs = List::new( TypeDef::Any, 2).unwrap();
        inputs.push(Value::I32(scalar)).unwrap();
        inputs.push(reference.clone()).unwrap();
        let mut outputs = List::new( TypeDef::Any, 2).unwrap();
        context.invoke(&function, InvocationFlags::default(), &inputs, &mut outputs).unwrap();
        drop(inputs);
        drop(reference);
        prop_assert_eq!(outputs.get_value(0).unwrap(), Value::I32(scalar));
        let reference = outputs.get_ref(1).unwrap();
        prop_assert_eq!(ty.downcast::<i64>(&reference), Some(&payload));
    }
}

#[test]
fn context_fork_uses_explicit_state_fork() {
    use super::super::{Context, ContextFlags, InvocationFlags, List, TypeDef, Value};
    let instance = Instance::new().unwrap();
    let module = NativeModule::builder("test")
        .state(|| Ok(7_i32), |value| Ok(value + 1))
        .export_typed_with_state("state", |value: &i32, (): ()| -> Result<i32> { Ok(*value) })
        .build(&instance)
        .unwrap();
    let parent = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let child = parent.fork().unwrap();
    let inputs = List::new(TypeDef::Any, 0).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    let function = parent.resolve_function("test.state").unwrap();
    parent
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(7));
    outputs.clear();
    child
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(8));
}

#[test]
fn sorted_tables_keep_callbacks_paired_with_exports() {
    use super::super::FunctionLinkage;
    let instance = Instance::new().unwrap();
    let module = NativeModule::builder("test")
        .version(42)
        .attribute("key", "value")
        .export("z", "0v_v", |_| Ok(()))
        .export("a", "0v_v", |_| Ok(()))
        .export_attribute("a", "export.key", "export.value")
        .unwrap()
        .build(&instance)
        .unwrap();
    assert_eq!(module.signature().version, 42);
    assert_eq!(module.attribute("key").as_deref(), Some("value"));
    let first = module.function_at(FunctionLinkage::Export, 0).unwrap();
    assert_eq!(first.name(), "a");
    assert_eq!(
        first.attribute("export.key").as_deref(),
        Some("export.value")
    );
    assert_eq!(
        module
            .function_at(FunctionLinkage::Export, 1)
            .unwrap()
            .name(),
        "z"
    );
}

#[test]
fn import_resolution_retains_borrowed_function_handles() {
    use super::super::{Context, ContextFlags, InvocationFlags, List, TypeDef};
    let instance = Instance::new().unwrap();
    let producer = NativeModule::builder("producer")
        .export("run", "0v_v", |_| Ok(()))
        .build(&instance)
        .unwrap();
    let retained = Arc::new(Mutex::new(None));
    let captured = retained.clone();
    let consumer = NativeModule::builder("consumer")
        .import("producer.run", false)
        .import("missing.optional", true)
        .export("resolve", "0v_v", move |call| {
            assert!(call.import("missing.optional").is_none());
            *captured.lock().unwrap() = call.import("producer.run");
            Ok(())
        })
        .build(&instance)
        .unwrap();
    let context =
        Context::with_modules(&instance, ContextFlags::default(), &[producer, consumer]).unwrap();
    let function = context.resolve_function("consumer.resolve").unwrap();
    let inputs = List::new(TypeDef::Any, 0).unwrap();
    let mut outputs = List::new(TypeDef::Any, 0).unwrap();
    context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    drop(context);
    drop(function);
    let imported = retained.lock().unwrap().take().unwrap();
    assert_eq!(imported.name(), "run");
    assert_eq!(imported.signature(), "0v_v");
}

#[test]
fn failed_calls_release_partial_reference_results() {
    use super::super::{Context, ContextFlags, InvocationFlags, List, RefType, TypeDef};
    struct Counted(Arc<AtomicUsize>);
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let instance = Instance::new().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let ty = RefType::register::<Counted>(&instance, "test.failure_payload").unwrap();
    let reference = ty.create(Counted(drops.clone())).unwrap();
    let module = NativeModule::builder("test")
        .export("fail", "0r_r", |call| {
            let reference = call.argument_ref(0)?;
            call.set_result_ref(0, reference)?;
            Err(Error::new(Code::Aborted, "after result retain"))
        })
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let function = context.resolve_function("test.fail").unwrap();
    let mut inputs = List::new(TypeDef::Any, 1).unwrap();
    inputs.push(reference.clone()).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    let error = context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap_err();
    assert_eq!(error.code(), Code::Aborted);
    drop(inputs);
    drop(outputs);
    drop(reference);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}
