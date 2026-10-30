//! Compiler-backed HAL executable cache and reflection CTS ports.

#[path = "../../ireert/tests/support/cts.rs"]
mod cts;

use cts::devices;
use ireert::Code;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn compiled_executable(
    device: &ireert::hal::Device,
    driver: &str,
    source: &[u8],
) -> TestResult<Option<(String, Vec<u8>)>> {
    use ireec::{Compiler, Device as TargetDevice, Options, Output, Pipeline, Target};
    use ireert::hal::{ExecutableCache, ExecutableCachingMode};
    let candidates = if driver.starts_with("local-") {
        vec![
            Target::Vmvx,
            Target::LlvmCpu {
                triple: None,
                cpu: Some("generic".into()),
                cpu_features: None,
            },
        ]
    } else if driver == "vulkan" {
        vec![Target::VulkanSpirv {
            target: None,
            features: None,
        }]
    } else {
        // Upstream does not instantiate executable CTS suites without registered executable test data.
        return Ok(None);
    };
    let alias = if driver.starts_with("local-") {
        "local"
    } else {
        driver
    };
    let cache = ExecutableCache::create(device, "cts")?;
    for target in candidates {
        let compiler = Compiler::new()?;
        let mut session = compiler.session()?;
        session.configure(
            &Options::default()
                .with_device(TargetDevice::Alias(alias.into()))
                .with_target(target),
        )?;
        let mut invocation = session.invocation()?;
        invocation.parse_source(session.source_buffer("hal_cts_executable.mlir", source)?)?;
        invocation.pipeline(Pipeline::HalExecutable)?;
        let mut output = Output::memory(&compiler)?;
        invocation.output_hal_executable(&mut output)?;
        let bytes = output.bytes()?.to_vec();
        let (format, _) = cache.infer_format(ExecutableCachingMode::ALIAS_PROVIDED_DATA, &bytes)?;
        if cache.can_prepare_format(ExecutableCachingMode::ALIAS_PROVIDED_DATA, &format) {
            return Ok(Some((format, bytes)));
        }
    }
    Ok(None)
}

fn prepare_executable(
    device: &ireert::hal::Device,
    driver: &str,
    source: &[u8],
) -> TestResult<Option<(ireert::hal::ExecutableCache, ireert::hal::Executable)>> {
    use ireert::hal::{ExecutableCache, ExecutableCachingMode, ExecutableParams, QueueAffinity};
    let Some((format, bytes)) = compiled_executable(device, driver, source)? else {
        return Ok(None);
    };
    let cache = ExecutableCache::create(device, "cts")?;
    let params = ExecutableParams::new(
        QueueAffinity::ANY,
        ExecutableCachingMode::ALIAS_PROVIDED_DATA,
        &format,
        &bytes,
        &[],
    );
    // SAFETY: the test compiled this executable with the trusted IREE compiler.
    let executable = unsafe { cache.prepare(&params) }?;
    Ok(Some((cache, executable)))
}

#[test]
fn executable_cache_prepare_executable() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_cache_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        if prepare_executable(&device, &name, source)?.is_none() {
            eprintln!("skipping {name}: upstream has no executable test-data format");
        }
    }
    Ok(())
}

#[test]
fn executable_export_count() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        assert_eq!(executable.function_count(), 1);
    }
    Ok(())
}

#[test]
fn executable_export_info_out_of_range() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        assert_eq!(
            executable
                .function_info(ireert::hal::ExecutableFunction::from_index(100))
                .unwrap_err()
                .code(),
            Code::OutOfRange
        );
    }
    Ok(())
}

#[test]
fn executable_export_info() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        let info = executable
            .function_info(ireert::hal::ExecutableFunction::from_index(0))
            .unwrap();
        assert_eq!(info.name.as_deref(), Some("export0"));
        assert_eq!(info.flags.bits(), 0);
        assert_eq!(info.constant_count, 2);
        assert_eq!(info.binding_count, 2);
    }
    Ok(())
}

#[test]
fn executable_export_parameters_out_of_range() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        if executable
            .function_info(ireert::hal::ExecutableFunction::from_index(0))
            .unwrap()
            .parameter_count
            == 0
        {
            // Upstream skips parameter-reflection cases when the backend provides no parameter metadata.
            eprintln!("skipping {name}: parameter reflection not available");
            continue;
        }
        assert_eq!(
            executable
                .function_parameters(ireert::hal::ExecutableFunction::from_index(100))
                .unwrap_err()
                .code(),
            Code::OutOfRange
        );
    }
    Ok(())
}

#[test]
fn executable_export_parameters_no_capacity() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        let function = ireert::hal::ExecutableFunction::from_index(0);
        if executable.function_info(function).unwrap().parameter_count == 0 {
            // Upstream skips parameter-reflection cases when the backend provides no parameter metadata.
            eprintln!("skipping {name}: parameter reflection not available");
            continue;
        }
        let parameters = executable.function_parameters(function).unwrap();
        assert_eq!(
            parameters[0].kind,
            ireert::hal::executable::ParameterType::BINDING
        );
    }
    Ok(())
}

#[test]
fn executable_export_parameters() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        let function = ireert::hal::ExecutableFunction::from_index(0);
        if executable.function_info(function).unwrap().parameter_count == 0 {
            // Upstream skips parameter-reflection cases when the backend provides no parameter metadata.
            eprintln!("skipping {name}: parameter reflection not available");
            continue;
        }
        let parameters = executable.function_parameters(function).unwrap();
        assert!(!parameters.is_empty());
    }
    Ok(())
}

#[test]
fn executable_lookup_export_by_name_not_found() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        assert_eq!(
            executable.lookup_function("NOT_FOUND").unwrap_err().code(),
            Code::NotFound
        );
    }
    Ok(())
}

#[test]
fn executable_lookup_export_by_name() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        assert_eq!(executable.lookup_function("export0").unwrap().value(), 0);
    }
    Ok(())
}

#[test]
fn executable_lookup_global_by_name_not_found_or_unsupported() -> TestResult {
    let source = include_bytes!(
        "..\\..\\..\\extern\\iree\\runtime\\src\\iree\\hal\\cts\\testdata\\executable_test.mlir"
    );
    for (name, _, device) in devices().unwrap() {
        let Some((_cache, executable)) = prepare_executable(&device, &name, source)? else {
            eprintln!("skipping {name}: upstream has no executable test-data format");
            continue;
        };
        let error = executable
            .lookup_global("NOT_FOUND", ireert::hal::QueueAffinity::ANY)
            .err()
            .unwrap();
        assert!(matches!(error.code(), Code::NotFound | Code::Unimplemented));
    }
    Ok(())
}
