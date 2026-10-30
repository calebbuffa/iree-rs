//! Compilation of upstream HAL executable fixtures.

pub fn compile_cts_executable(
    device: &ireert::hal::Device,
    backend: &str,
    filename: &str,
) -> ireec::Result<Option<(String, Vec<u8>)>> {
    use ireec::{Compiler, Device as CompilerDevice, Options, Output, Pipeline, Target};
    use ireert::hal::{ExecutableCache, ExecutableCachingMode};
    use std::{fs, path::PathBuf};

    let source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("extern")
        .join("iree")
        .join("runtime")
        .join("src")
        .join("iree")
        .join("hal")
        .join("cts")
        .join("testdata")
        .join(filename);
    let source = fs::read(&source_path).map_err(|error| {
        ireec::Error::InvalidOptions(format!("read {}: {error}", source_path.display()))
    })?;
    let targets = match backend {
        "local-sync" | "local-task" => vec![
            Target::Vmvx,
            Target::LlvmCpu {
                triple: None,
                cpu: Some("generic".into()),
                cpu_features: None,
            },
        ],
        "vulkan" => vec![Target::VulkanSpirv {
            target: None,
            features: None,
        }],
        _ => return Ok(None),
    };
    let alias = if backend.starts_with("local-") {
        "local"
    } else {
        backend
    };
    let cache = ExecutableCache::create(device, "cts")?;
    for target in targets {
        let compiler = Compiler::new()?;
        let mut session = compiler.session()?;
        session.configure(
            &Options::default()
                .with_device(CompilerDevice::Alias(alias.to_owned()))
                .with_target(target),
        )?;
        let mut invocation = session.invocation()?;
        invocation.parse_source(session.source_buffer(filename, &source)?)?;
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
