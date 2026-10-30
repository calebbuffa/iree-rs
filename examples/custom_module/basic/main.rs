//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::compile_mlir_host_only;
use ireert::vm::{
    BytecodeModule, Context, Instance, List, Module, NativeModule, Ref, RefType, TypeDef,
};
use std::error::Error;
use std::sync::{Arc, Mutex};

const PROGRAM: &str =
    include_str!("../../../extern/iree/samples/custom_module/basic/test/example.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir_host_only("custom_module_basic.mlir", PROGRAM, &[])?;
    let instance = Instance::new()?;
    let string_type = RefType::register::<String>(&instance, "custom.string")?;
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let create_type = string_type;
    let create_log = log.clone();
    let create = move |reference: Ref| -> ireert::Result<Ref> {
        let buffer = ireert::vm::Buffer::from_ref(&reference)?;
        // VM code cannot mutate this constant while the borrowed bytes are read.
        // SAFETY: the VM constant buffer is immutable for the duration of this callback.
        let bytes = unsafe { buffer.map(0, buffer.len(), 1)? };
        let value = String::from_utf8_lossy(bytes).into_owned();
        create_log
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(format!("CREATE {value}"));
        create_type.create(value)
    };
    let length_type = string_type;
    let length_log = log.clone();
    let length = move |reference: Ref| -> ireert::Result<i64> {
        let value = length_type.downcast::<String>(&reference).ok_or_else(|| {
            ireert::Error::new(ireert::Code::InvalidArgument, "unexpected string payload")
        })?;
        length_log
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(format!("LENGTH {value} = {}", value.len()));
        i64::try_from(value.len())
            .map_err(|_| ireert::Error::new(ireert::Code::OutOfRange, "string length exceeds i64"))
    };
    let print_log = log.clone();
    let print = move |reference: Option<Ref>| -> ireert::Result<()> {
        if let Some(reference) = reference {
            let value = string_type.downcast::<String>(&reference).ok_or_else(|| {
                ireert::Error::new(ireert::Code::InvalidArgument, "unexpected string payload")
            })?;
            print_log
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(format!("PRINT {value}"));
        }
        Ok(())
    };
    let custom = NativeModule::builder("custom")
        .export_typed("string.create", create)
        .export_typed("string.length", length)
        .export_typed("string.print", print)
        .build(&instance)?;
    // SAFETY: `archive` was just produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[custom, module])?;
    let function = context.resolve_function("example.main")?;
    let inputs = List::new(TypeDef::Any, 0)?;
    let mut outputs = List::new(TypeDef::Any, 0)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let actual = log
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    assert_eq!(
        &actual[..3],
        ["CREATE hello", "PRINT hello", "LENGTH hello = 5"]
    );
    assert!(actual.contains(&"CREATE debug".to_owned()));
    assert!(
        actual
            .iter()
            .any(|entry| entry == "PRINT debug" || entry == "PRINT optimized")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
