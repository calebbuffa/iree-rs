//! Public native-module descriptor, fork, reference, and ABI-layout coverage.
// C++ template-only borrowed-pointer unpacking cases are not expressible through the safe descriptor API.

use ireert::vm::{
    Context, ContextFlags, Instance, InvocationFlags, List, NativeModule, RefType, TypeDef, Value,
};

#[test]
fn native_module_export_and_fork() {
    let instance = Instance::new().unwrap();
    let module = NativeModule::builder("native_state_test")
        .state(|| Ok(5_i32), |parent| Ok(parent + 1))
        .export("value", "0_i", |call| {
            call.set_result(0, *call.state::<i32>().unwrap())
        })
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let function = context.resolve_function("native_state_test.value").unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    context
        .invoke(
            &function,
            InvocationFlags::default(),
            &List::new(TypeDef::Any, 0).unwrap(),
            &mut outputs,
        )
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(5));

    let fork = context.fork().unwrap();
    let function = fork.resolve_function("native_state_test.value").unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    fork.invoke(
        &function,
        InvocationFlags::default(),
        &List::new(TypeDef::Any, 0).unwrap(),
        &mut outputs,
    )
    .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(6));
}

#[test]
fn native_module_borrowed_reference_round_trip() {
    let instance = Instance::new().unwrap();
    let ty = RefType::register::<i64>(&instance, "test.native_module.borrowed_ref").unwrap();
    let module = NativeModule::builder("native_ref_test")
        .export("identity", "0r_r", |call| {
            call.set_result_ref(0, call.argument_ref(0)?)
        })
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let function = context
        .resolve_function("native_ref_test.identity")
        .unwrap();
    let reference = ty.create(123_i64).unwrap();
    let mut inputs = List::new(TypeDef::Any, 1).unwrap();
    inputs.push(reference.clone()).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    let result = outputs.get_ref(0).unwrap();
    assert!(reference.is_equal_to(&result));
    assert_eq!(ty.downcast::<i64>(&result), Some(&123));
}

#[test]
fn native_module_mixed_alignment_matches_vm_cconv() {
    let instance = Instance::new().unwrap();
    let module = NativeModule::builder("native_alignment_test")
        .export("sum", "0iI_I", |call| {
            assert_eq!(call.argument_offset(0), Some(0));
            assert_eq!(call.argument_offset(1), Some(8));
            let small: i32 = call.argument(0)?;
            let large: i64 = call.argument(8)?;
            call.set_result(0, large + i64::from(small))
        })
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let function = context
        .resolve_function("native_alignment_test.sum")
        .unwrap();
    let mut inputs = List::new(TypeDef::Any, 2).unwrap();
    inputs.push(Value::I32(30)).unwrap();
    inputs.push(Value::I64(12)).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I64(42));
}

#[test]
fn native_module_high_arity_reference_call() {
    use ireert::{Code, Error};

    let instance = Instance::new().unwrap();
    let cconv = format!("0{}_{}", "r".repeat(600), "r".repeat(600));
    let module = NativeModule::builder("native_arity_test")
        .export("identity", &cconv, |call| {
            for index in 0..600 {
                let offset = call
                    .argument_offset(index)
                    .ok_or_else(|| Error::new(Code::OutOfRange, "missing ref argument"))?;
                call.argument_ref(offset)?;
                let result = call
                    .result_offset(index)
                    .ok_or_else(|| Error::new(Code::OutOfRange, "missing ref result"))?;
                call.set_result_ref(result, None)?;
            }
            Ok(())
        })
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[module]).unwrap();
    let function = context
        .resolve_function("native_arity_test.identity")
        .unwrap();
    let mut inputs = List::new(TypeDef::Any, 600).unwrap();
    for _ in 0..600 {
        inputs.push(ireert::vm::Ref::null()).unwrap();
    }
    let mut outputs = List::new(TypeDef::Any, 600).unwrap();
    context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    assert_eq!(outputs.len(), 600);
    for index in 0..600 {
        assert!(outputs.get_ref(index).unwrap().is_null());
    }
}

#[test]
fn resolving_state_for_an_unregistered_module_fails() {
    let instance = Instance::new().unwrap();
    let registered = NativeModule::builder("state_registered")
        .export("noop", "0", |_call| Ok(()))
        .build(&instance)
        .unwrap();
    let unregistered = NativeModule::builder("state_unregistered")
        .export("noop", "0", |_call| Ok(()))
        .build(&instance)
        .unwrap();
    let context = Context::with_modules(&instance, ContextFlags::default(), &[registered]).unwrap();
    assert!(context.resolve_module_state(&unregistered).is_err());
}
