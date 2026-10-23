//! Owned parameter indexes, archive storage, and model-weight execution.

use iree::{ParameterIndex, ParameterSignal, StatusCode};
use std::{
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct Archive(PathBuf);

impl Drop for Archive {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_file(&self.0) {
            eprintln!(
                "could not remove test archive {}: {error}",
                self.0.display()
            );
        }
    }
}

fn safetensors() -> Vec<u8> {
    let header = br#"{"tensor":{"dtype":"I32","shape":[2],"data_offsets":[0,8]}}"#;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(header.len() as u64).to_le_bytes());
    bytes.extend_from_slice(header);
    bytes.extend_from_slice(&7_i32.to_le_bytes());
    bytes.extend_from_slice(&11_i32.to_le_bytes());
    bytes
}

#[test]
fn index_metadata_storage_and_provider_lifetimes() -> iree::Result<()> {
    let mut index = ParameterIndex::new()?;
    assert!(index.is_empty());
    index.reserve(10)?;
    index.add_bytes("weights", vec![1, 2, 3, 4], b"opaque metadata")?;
    index.add_splat("zero", &[0, 0, 0, 0], 4096)?;
    assert_eq!(index.len(), 2);
    let info = index.lookup("weights")?;
    assert_eq!(info.metadata, b"opaque metadata");
    assert_eq!(info.byte_length, 4);
    assert!(!info.is_splat);
    assert!(index.lookup("zero")?.is_splat);
    assert_eq!(index.entries()?.len(), 2);
    assert_eq!(
        index.lookup("absent").unwrap_err().code(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        index.add_bytes("weights", vec![1], &[]).unwrap_err().code(),
        StatusCode::ALREADY_EXISTS
    );
    assert!(index.add_splat("bad", &[], 1).is_err());
    assert!(index.add_splat("bad", &[1, 2, 3], 12).is_err());
    assert!(index.add_splat("bad", &[1, 2, 3, 4], 3).is_err());
    assert!(index.provider("model", 0).is_err());
    let provider = index.provider("model", 16)?;
    drop(index);
    assert!(provider.supports("model"));
    assert!(!provider.supports("absent"));
    provider.notify(ParameterSignal::Suspend)?;
    provider.notify(ParameterSignal::Resume)?;
    provider.notify(ParameterSignal::LowMemory)?;
    Ok(())
}

#[test]
fn safetensors_archive_is_retained_after_parsing() -> iree::Result<()> {
    let mut index = ParameterIndex::new()?;
    index.load_bytes("safetensors", safetensors())?;
    assert_eq!(index.lookup("tensor")?.byte_length, 8);
    let provider = index.provider("archive", 2)?;
    drop(index);
    assert!(provider.supports("archive"));
    let mut invalid = ParameterIndex::new()?;
    assert!(invalid.load_bytes("safetensors", vec![0; 3]).is_err());
    assert!(invalid.load_bytes("unsupported-format", vec![]).is_err());
    Ok(())
}

#[test]
fn file_archive_and_cloned_provider_retain_native_storage() -> Result<(), Box<dyn std::error::Error>>
{
    let path = std::env::temp_dir().join(format!(
        "iree-parameters-{}-{}.safetensors",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos(),
    ));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    let archive = Archive(path);
    file.write_all(&safetensors())?;
    drop(file);
    let mut index = ParameterIndex::new()?;
    assert_eq!(
        index.load_file("invalid\0.safetensors").unwrap_err().code(),
        StatusCode::INVALID_ARGUMENT
    );
    index.load_file(&archive.0)?;
    assert_eq!(index.lookup("tensor")?.byte_length, 8);
    let original = index.provider("archive", 2)?;
    let provider = original.clone();
    drop(original);
    drop(index);
    assert!(provider.supports("archive"));
    #[cfg(all(feature = "compiler", feature = "cpu"))]
    {
        use iree::{CompileOptions, Compiler, Input, Instance, Target};
        let model = r#"
module {
  util.global private @tensor = #flow.parameter.named<"archive"::"tensor"> : tensor<2xi32>
  func.func @main() -> tensor<2xi32> {
    %tensor = util.global.load @tensor : tensor<2xi32>
    return %tensor : tensor<2xi32>
  }
}
"#;
        let artifact = Compiler::new()?
            .compile_with_options(model, &CompileOptions::new(Target::Cpu).input(Input::None))?;
        let instance = Instance::new()?;
        let module = instance.module(&artifact)?;
        let builder = instance
            .device("local-task")?
            .session_builder()
            .module(&module)
            .parameters(std::slice::from_ref(&provider));
        drop(provider);
        let mut session = builder.build()?;
        assert_eq!(session.call("module.main", &[])?[0].read::<i32>()?, [7, 11]);
    }
    Ok(())
}

#[cfg(all(feature = "compiler", feature = "cpu"))]
#[test]
fn external_weights_execute_after_index_and_provider_drop() -> iree::Result<()> {
    use iree::{CompileOptions, Compiler, Input, Instance, Target};
    let model = r#"
module {
  util.global private @weights = #flow.parameter.named<"model"::"weights"> : tensor<4xf32>
  func.func @main() -> tensor<4xf32> {
    %weights = util.global.load @weights : tensor<4xf32>
    return %weights : tensor<4xf32>
  }
}
"#;
    let artifact = Compiler::new()?
        .compile_with_options(model, &CompileOptions::new(Target::Cpu).input(Input::None))?;
    let mut index = ParameterIndex::new()?;
    let values = [1_f32, 2., 3., 4.];
    index.add_bytes(
        "weights",
        values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect(),
        &[],
    )?;
    let providers = [index.provider("model", 4)?];
    let device = Instance::new()?.device("local-task")?;
    let mut session = device.load_with_parameters(&artifact, &providers)?;
    drop(index);
    drop(providers);
    drop(device);
    drop(artifact);
    assert_eq!(session.call("module.main", &[])?[0].read::<f32>()?, values);
    Ok(())
}
