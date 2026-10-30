//! Packaged documentation must be complete and detect stale or edited snapshots.
#[path = "../native/config.rs"]
mod config;
#[path = "../native/documentation.rs"]
mod documentation;

use std::{fs, path::PathBuf, time::SystemTime};

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "iree-native-docs-test-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&directory)?;
        Ok(Self(directory))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!(
                "remove documentation test directory {}: {error}",
                self.0.display()
            );
        }
    }
}

#[test]
fn snapshot_stages_every_driver_without_native_tools() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut families = std::collections::BTreeSet::new();
    let mut complete_index = String::new();
    for target in documentation::TARGETS {
        let out = Scratch::new()?;
        documentation::stage(&root, &out.0, iree_native::IREE_VERSION, target)?;
        let index = fs::read_to_string(out.0.join("bindings.rs"))?;
        assert!(!index.contains("mod link_tests"));
        for entry in fs::read_dir(&out.0)? {
            families.insert(entry?.file_name().to_string_lossy().into_owned());
        }
        complete_index.push_str(&index);
    }
    for family in [
        "base",
        "local",
        "task",
        "local-sync",
        "local-task",
        "compiler",
        "tokenizer",
        "vulkan",
        "cuda",
        "hip",
        "amdgpu",
        "metal",
        "null",
        "webgpu",
    ] {
        assert!(families.contains(&format!("{family}.rs")), "{family}");
        assert!(
            complete_index.contains(&format!("#[path = \"{family}.rs\"]")),
            "{family}"
        );
    }
    for feature in [
        "compiler-tools",
        "mpi",
        "io-index",
        "io-parameters",
        "io-format-irpa",
        "io-format-gguf",
        "io-format-safetensors",
    ] {
        assert!(
            complete_index.contains(&format!("feature = \"{feature}\"")),
            "{feature}"
        );
    }
    Ok(())
}

#[test]
fn snapshot_rejects_stale_version_and_modified_declarations()
-> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out = Scratch::new()?;
    let target = "x86_64-pc-windows-msvc";
    assert!(documentation::stage(&root, &out.0, "not-the-pinned-version", target).is_err());
    assert!(
        documentation::stage(
            &root,
            &out.0,
            iree_native::IREE_VERSION,
            "unsupported-target"
        )
        .is_err()
    );
    let copy = Scratch::new()?;
    fs::create_dir(copy.0.join("native"))?;
    let metadata: serde_json::Value = serde_json::from_slice(&fs::read(
        root.join("docs")
            .join("bindings")
            .join(target)
            .join("snapshot.json"),
    )?)?;
    for name in metadata["inputs"]
        .as_object()
        .ok_or("missing inputs")?
        .keys()
    {
        fs::copy(
            root.join("native").join(name),
            copy.0.join("native").join(name),
        )?;
    }
    let snapshot = copy.0.join("docs").join("bindings").join(target);
    fs::create_dir_all(&snapshot)?;
    for entry in fs::read_dir(root.join("docs").join("bindings").join(target))? {
        let entry = entry?;
        fs::copy(entry.path(), snapshot.join(entry.file_name()))?;
    }
    fs::write(snapshot.join("base.rs"), "// edited declarations")?;
    let error = documentation::stage(&copy.0, &out.0, iree_native::IREE_VERSION, target)
        .err()
        .ok_or("edited snapshot was accepted")?;
    assert!(error.to_string().contains("checksum mismatch: base.rs"));
    Ok(())
}

#[test]
fn documentation_features_match_target_driver_constraints() -> Result<(), Box<dyn std::error::Error>>
{
    for target in documentation::TARGETS {
        let features = documentation::features(target)?;
        assert!(!features.contains(&"bare-metal"));
        assert_eq!(features.contains(&"metal"), target.contains("apple"));
        assert_eq!(features.contains(&"amdgpu"), target.contains("linux"));
        assert_eq!(features.contains(&"webgpu"), target.starts_with("wasm32"));
        assert_eq!(
            features.contains(&"compiler"),
            !target.starts_with("wasm32")
        );
        config::Configuration::new(features).validate_target(target)?;
    }
    Ok(())
}
