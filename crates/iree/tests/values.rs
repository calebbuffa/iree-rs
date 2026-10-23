//! Mixed primitive VM invocation through the owned Rust interface.
#![cfg(all(feature = "compiler", feature = "cpu"))]

use iree::{CompileOptions, Compiler, Input, Instance, Target, Value};

#[test]
fn scalars_roundtrip_without_tensor_wrapping() -> iree::Result<()> {
    let options = CompileOptions::new(Target::Vmvx).input(Input::None);
    let artifact = Compiler::new()?.compile_with_options(
        r#"
module {
  func.func @mixed(%a: i32, %b: i64, %c: f32) -> (i32, i64, f32) {
    return %a, %b, %c : i32, i64, f32
  }
}
"#,
        &options,
    )?;
    let device = Instance::new()?.device("local-sync")?;
    let mut session = device.load(&artifact)?;
    let outputs = session.call_values(
        "module.mixed",
        &[7_i32.into(), i64::MAX.into(), 1.25_f32.into()],
    )?;
    assert!(
        matches!(outputs.as_slice(), [Value::I32(7), Value::I64(i64::MAX), Value::F32(value)] if *value == 1.25)
    );
    assert!(session.call_values("module.mixed", &[Value::Null]).is_err());
    let name = "x".repeat(6000);
    let missing = format!("module.{name}");
    let error = match session.call_values(&missing, &[]) {
        Ok(_) => panic!("missing function must fail"),
        Err(error) => error,
    };
    assert_eq!(error.code(), iree::StatusCode::NOT_FOUND);
    assert!(
        error.message().contains(&name),
        "native diagnostic was truncated to {} bytes: {error}",
        error.message().len()
    );
    Ok(())
}

#[test]
fn typed_options_reject_disabled_or_inapplicable_configuration() -> iree::Result<()> {
    let mut compiler = Compiler::new()?;
    let options = CompileOptions::new(Target::Vmvx).cpu("host");
    assert!(
        compiler
            .compile_with_options("module {}", &options)
            .is_err()
    );
    let options = CompileOptions::new(Target::Cpu).flag("--iree-not-a-real-option");
    assert!(
        compiler
            .compile_with_options("module {}", &options)
            .is_err()
    );
    #[cfg(not(feature = "input-torch"))]
    assert!(
        compiler
            .compile_with_options(
                "module {}",
                &CompileOptions::new(Target::Cpu).input(Input::Torch)
            )
            .is_err()
    );
    Ok(())
}
