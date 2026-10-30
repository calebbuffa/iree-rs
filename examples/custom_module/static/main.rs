//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireert::hal::{BufferView, ElementType};
use ireert::vm::{
    BytecodeModule, Context, Instance, List, Module, NativeModule, Ref, RefType, TypeDef,
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

const PROGRAM: &str =
    include_str!("../../../extern/iree/samples/custom_module/static/test/example.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir("custom_module_static.mlir", PROGRAM, &[])?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    let string_type = RefType::register::<String>(&instance, "custom.string")?;
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let create_type = string_type;
    let create_log = log.clone();
    let from_tensor = move |reference: Ref| -> ireert::Result<Ref> {
        let view = BufferView::from_ref(&reference)?;
        // SAFETY: the import signature requires an initialized signless i8 buffer view.
        let values = unsafe { view.read_elements_unchecked::<i8>(ElementType::INT_8)? };
        let shape = view
            .shape()?
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("x");
        let formatted = format!(
            "{}xi8={}",
            shape,
            values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        );
        create_log
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(format!("CREATE {formatted}"));
        create_type.create(formatted)
    };
    let print_log = log.clone();
    let print = move |reference: Option<Ref>| -> ireert::Result<()> {
        if let Some(reference) = reference {
            let text = string_type
                .downcast::<String>(&reference)
                .ok_or_else(missing_argument)?;
            print_log
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(format!("PRINT {text}"));
        }
        Ok(())
    };
    let custom = NativeModule::builder("custom")
        .export_typed("string.from_tensor", from_tensor)
        .export_typed("string.print", print)
        .build(&instance)?;
    // SAFETY: `archive` was just produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context =
        Context::with_modules(&instance, Default::default(), &[hal_module, custom, module])?;
    let function = context.resolve_function("example.main")?;
    let inputs = List::new(TypeDef::Any, 0)?;
    let mut outputs = List::new(TypeDef::Any, 0)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let actual = log
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    assert_eq!(actual, ["CREATE 5xi8=0 1 2 3 4", "PRINT 5xi8=0 1 2 3 4"]);
    Ok(())
}

fn missing_argument() -> ireert::Error {
    ireert::Error::new(
        ireert::Code::InvalidArgument,
        "missing or invalid custom string argument",
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
