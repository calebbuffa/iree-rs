//! Rust port of the corresponding IREE upstream sample.

use ireert::vm::{Context, Element, Instance, List, NativeModule, TypeDef, Value};
use std::error::Error;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    let instance = Instance::new()?;
    let module = NativeModule::builder("counter")
        .state(
            || Ok(AtomicI32::new(0)),
            |parent| Ok(AtomicI32::new(parent.load(Ordering::Acquire))),
        )
        .export_typed_with_state(
            "get_value",
            |state: &AtomicI32, (): ()| -> ireert::Result<i64> {
                Ok(i64::from(state.load(Ordering::Acquire)))
            },
        )
        .export_typed_with_state(
            "set_value",
            |state: &AtomicI32, value: i32| -> ireert::Result<()> {
                state.store(value, Ordering::Release);
                Ok(())
            },
        )
        .export_typed_with_state(
            "add_to_value",
            |state: &AtomicI32, increment: i32| -> ireert::Result<()> {
                state.fetch_add(increment, Ordering::AcqRel);
                Ok(())
            },
        )
        .export_typed_with_state("reset_value", |state: &AtomicI32, (): ()| {
            state.store(0, Ordering::Release);
            Ok(())
        })
        .build(&instance)?;
    let context = Context::with_modules(&instance, Default::default(), &[module])?;
    assert_eq!(call_value(&context, "counter.get_value")?, 0);
    call_void(&context, "counter.set_value", Some(101))?;
    assert_eq!(call_value(&context, "counter.get_value")?, 101);
    call_void(&context, "counter.add_to_value", Some(20))?;
    assert_eq!(call_value(&context, "counter.get_value")?, 121);
    call_void(&context, "counter.add_to_value", Some(-50))?;
    assert_eq!(call_value(&context, "counter.get_value")?, 71);
    call_void(&context, "counter.reset_value", None)?;
    assert_eq!(call_value(&context, "counter.get_value")?, 0);
    let fork = context.fork()?;
    call_void(&fork, "counter.add_to_value", Some(2))?;
    assert_eq!(call_value(&fork, "counter.get_value")?, 2);
    assert_eq!(call_value(&context, "counter.get_value")?, 0);
    Ok(())
}

fn call_value(context: &Context, name: &str) -> Result<i64, Box<dyn Error>> {
    call(context, name, None)?.ok_or_else(|| {
        ireert::Error::new(ireert::Code::DataLoss, "counter getter returned no value").into()
    })
}

fn call_void(context: &Context, name: &str, input: Option<i32>) -> Result<(), Box<dyn Error>> {
    if call(context, name, input)?.is_some() {
        return Err(
            ireert::Error::new(ireert::Code::DataLoss, "counter mutator returned a value").into(),
        );
    }
    Ok(())
}

fn call(context: &Context, name: &str, input: Option<i32>) -> Result<Option<i64>, Box<dyn Error>> {
    let function = context.resolve_function(name)?;
    let mut inputs = List::new(TypeDef::Any, usize::from(input.is_some()))?;
    if let Some(value) = input {
        inputs.push(Value::I32(value))?;
    }
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    match outputs.len() {
        0 => Ok(None),
        1 => match outputs.get(0)? {
            Some(Element::Value(Value::I64(value))) => Ok(Some(value)),
            _ => {
                Err(ireert::Error::new(ireert::Code::DataLoss, "unexpected counter result").into())
            }
        },
        _ => Err(ireert::Error::new(ireert::Code::DataLoss, "unexpected counter result").into()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
