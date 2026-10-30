//! Rust port of the corresponding IREE upstream dynamic custom-module sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireert::{
    Code,
    vm::{BytecodeModule, Context, Instance, List, Module, RefType, TypeDef},
};
use std::{error::Error, path::PathBuf};

const PROGRAM: &str =
    include_str!("../../../extern/iree/samples/custom_module/dynamic/test/example.mlir");

fn dynamic_module_path() -> Result<PathBuf, Box<dyn Error>> {
    let filename = format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        iree_sample_dynamic_module::ARTIFACT_NAME,
        std::env::consts::DLL_SUFFIX
    );
    let executable = std::env::current_exe()?;
    let profile = executable
        .parent()
        .ok_or("example executable has no parent directory")?
        .parent()
        .ok_or("example executable has no profile directory")?;
    for directory in [profile.join("deps"), profile.to_path_buf()] {
        let candidate = directory.join(&filename);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(format!("Cargo-built dynamic module was not found: {filename}").into())
}

fn invoke(context: &Context, name: &str) -> ireert::Result<()> {
    let function = context.resolve_function(name)?;
    let inputs = List::new(TypeDef::Any, 0)?;
    let mut outputs = List::new(TypeDef::Any, 0)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir("custom_module_dynamic.mlir", PROGRAM, &[])?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    {
        // SAFETY: Cargo built this library from this workspace; loading it
        // executes trusted module initialization and the factory implemented above.
        let custom = unsafe {
            Module::load_dynamic(
                &instance,
                &dynamic_module_path()?,
                "create_custom_module",
                &[],
            )?
        };
        // SAFETY: the bytecode was compiled in this process immediately above.
        let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
        let module: Module = bytecode.into_module();
        let context =
            Context::with_modules(&instance, Default::default(), &[custom, hal_module, module])?;
        invoke(&context, "example.main")?;
        let error = match invoke(&context, "example.error") {
            Ok(()) => return Err("dynamic module error export unexpectedly succeeded".into()),
            Err(error) => error,
        };
        assert_eq!(error.code(), Code::Unknown);
        assert!(error.to_string().contains("Forced failure"));
    }
    assert!(RefType::lookup(&instance, "custom.string").is_none());
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn loads_and_invokes_the_dynamic_module() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
