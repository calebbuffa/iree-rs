//! Behavioural laws of the compiler bindings.
// Test fixtures outside `#[test]` functions assert by panicking.

use ireec::{Compiler, Device, Error, Options, Output, Pipeline, Target};
use proptest::prelude::*;

const MODULE: &str = r#"
func.func @add(%a: tensor<4xf32>, %b: tensor<4xf32>) -> tensor<4xf32> {
  %0 = arith.addf %a, %b : tensor<4xf32>
  return %0 : tensor<4xf32>
}
"#;

fn session() -> Result<ireec::Session, Error> {
    let mut s = Compiler::new()?.session()?;
    s.configure(
        &Options::default()
            .with_device(Device::Alias("local".into()))
            .with_target(Target::Vmvx),
    )?;
    Ok(s)
}

proptest! {
    #[test]
    fn flags_with_interior_nul_are_rejected(prefix in "[a-z]{0,8}", suffix in "[a-z]{0,8}") {
        let flag = format!("--{prefix}\0{suffix}");
        let mut session = Compiler::new()
            .map_err(|error| TestCaseError::fail(error.to_string()))?
            .session()
            .map_err(|error| TestCaseError::fail(error.to_string()))?;
        let r = session.set_flags(&[flag]);
        prop_assert!(matches!(r, Err(Error::Nul)));
    }
}

/// Recognises a size-prefixed `IREE` FlatBuffer, optionally behind the
/// polyglot zip header upstream emits by default.
fn is_vmfb(bytes: &[u8]) -> bool {
    const ZIP_LOCAL_FILE: &[u8] = b"PK\x03\x04";
    if bytes.starts_with(ZIP_LOCAL_FILE) {
        return bytes.windows(9).any(|name| name == b"module.fb");
    }
    bytes.get(8..12) == Some(b"IREE")
}

fn compile_vmfb(session: &ireec::Session, input: &[u8]) -> Result<Vec<u8>, ireec::Error> {
    let mut invocation = session.invocation()?;
    invocation.parse_source(session.source_buffer("input.mlir", input)?)?;
    invocation.pipeline(Pipeline::Std)?;
    let compiler = Compiler::new()?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    Ok(output.bytes()?.to_vec())
}

#[test]
fn compiles_valid_module_to_vmfb() -> Result<(), Box<dyn std::error::Error>> {
    let output = compile_vmfb(&session()?, MODULE.as_bytes())?;
    assert!(is_vmfb(&output), "VM bytecode archive header");
    Ok(())
}

#[test]
fn invalid_input_reports_diagnostics_and_session_stays_usable()
-> Result<(), Box<dyn std::error::Error>> {
    let s = session()?;
    match compile_vmfb(&s, b"this is not mlir") {
        Err(Error::Compile(ds)) => assert!(!ds.is_empty()),
        other => panic!("expected diagnostics, got {other:?}"),
    }
    compile_vmfb(&s, MODULE.as_bytes())?;
    Ok(())
}

#[test]
fn unregistered_typed_backend_does_not_change_session_flags()
-> Result<(), Box<dyn std::error::Error>> {
    let mut session = session()?;
    let before = session.flags(false);
    let result = session.configure(&ireec::Options::default().with_target(ireec::Target::Other(
        "copilot-test-backend-not-registered".into(),
    )));
    assert!(matches!(result, Err(Error::UnregisteredBackend(_))));
    assert_eq!(session.flags(false), before);
    Ok(())
}

fn artifact_path(label: &str) -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
        .map(|path| path.join(format!(".ireec-{label}-{}.bin", std::process::id())))
}

#[test]
fn output_file_drop_discards_and_keep_commits() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, io::Write};

    let compiler = Compiler::new()?;
    let discarded = artifact_path("discarded")?;
    let committed = artifact_path("committed")?;
    let _ = fs::remove_file(&discarded);
    let _ = fs::remove_file(&committed);

    {
        let mut output = Output::file(&compiler, discarded.clone())?;
        output.write_all(b"partial")?;
    }
    assert!(!discarded.exists());

    {
        let mut output = Output::file(&compiler, &committed)?;
        output.write_all(b"complete")?;
        output.keep();
    }
    assert_eq!(fs::read(&committed)?, b"complete");
    fs::remove_file(committed)?;
    Ok(())
}

#[test]
fn output_file_open_failure_is_reported() -> Result<(), Box<dyn std::error::Error>> {
    let compiler = Compiler::new()?;
    let path = std::env::temp_dir()
        .join(format!("ireec-missing-parent-{}", std::process::id()))
        .join("output.bin");
    assert!(Output::file(&compiler, &path).is_err());
    Ok(())
}

#[test]
fn source_file_open_failure_is_reported() -> Result<(), Box<dyn std::error::Error>> {
    let session = Compiler::new()?.session()?;
    let path = std::env::temp_dir()
        .join(format!("ireec-missing-source-{}", std::process::id()))
        .join("input.mlir");
    assert!(session.source_file(path).is_err());
    Ok(())
}

#[cfg(unix)]
#[test]
fn output_owned_fd_writes_and_closes() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, io::Write};

    let compiler = Compiler::new()?;
    let path = artifact_path("fd")?;
    let _ = fs::remove_file(&path);
    {
        let file = fs::File::create(&path)?;
        let mut output = Output::from_fd(&compiler, file.into())?;
        output.write_all(b"owned descriptor")?;
    }
    assert_eq!(fs::read(&path)?, b"owned descriptor");
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn invocation_file_output_commits_only_successful_vmfb() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let session = session()?;
    let path = artifact_path("vmfb")?;
    let _ = fs::remove_file(&path);
    let mut invocation = session.invocation()?;
    invocation.parse_source(session.source_buffer("input.mlir", MODULE.as_bytes())?)?;
    invocation.pipeline(Pipeline::Std)?;
    invocation.output_vm_bytecode_to(path.clone())?;
    let bytes = fs::read(&path)?;
    assert!(is_vmfb(&bytes), "VM bytecode archive header");
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn compilation_is_deterministic() -> Result<(), Box<dyn std::error::Error>> {
    let a_session = session()?;
    let b_session = session()?;
    let a = compile_vmfb(&a_session, MODULE.as_bytes())?;
    let b = compile_vmfb(&b_session, MODULE.as_bytes())?;
    assert_eq!(a, b);
    Ok(())
}
