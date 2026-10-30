//! Packaged, integrity-checked declarations for documentation without native tools.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "aarch64-apple-darwin",
    "wasm32-unknown-emscripten",
];

pub fn features(target: &str) -> Result<Vec<&'static str>> {
    if !TARGETS.contains(&target) {
        return Err(format!(
            "no documentation snapshot target {target}; supported targets: {}",
            TARGETS.join(", ")
        )
        .into());
    }
    Ok(super::config::FEATURES
        .iter()
        .copied()
        .filter(|name| match *name {
            "bare-metal" => false,
            "metal" => target.contains("apple"),
            "amdgpu" => target.contains("linux"),
            "webgpu" => target.starts_with("wasm32"),
            "cuda" | "hip" => target.contains("linux") || target.contains("windows"),
            "cpu" | "local-task" | "task" | "compiler" | "compiler-tools" | "mpi" => {
                !target.starts_with("wasm32")
            }
            name if name.starts_with("target-")
                || name.starts_with("llvm-")
                || name.starts_with("input-") =>
            {
                !target.starts_with("wasm32")
            }
            _ => true,
        })
        .collect())
}

const INPUTS: &[&str] = &[
    "CMakeLists.txt",
    "binding_policy.rs",
    "binding_support.rs",
    "bindings.rs",
    "config.rs",
    "configuration.cmake",
    "cxx-runtime.cmake",
    "installation.cmake",
    "manifest.rs",
    "targets.cmake",
    "workspace.rs",
    "wrapper.h",
    "woodshed.toml",
    "woodshed.lock",
];

#[derive(Deserialize, Serialize)]
struct Snapshot {
    version: String,
    generation_target: String,
    inputs: BTreeMap<String, String>,
    files: BTreeMap<String, String>,
}

fn digest(path: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}

fn inputs(root: &Path) -> Result<BTreeMap<String, String>> {
    INPUTS
        .iter()
        .map(|name| Ok(((*name).into(), digest(&root.join("native").join(name))?)))
        .collect()
}

pub fn record(root: &Path, directory: &Path, version: &str, target: &str) -> Result<()> {
    let mut files = BTreeMap::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "rs")
            && path.file_name().is_some_and(|name| name != "link_tests.rs")
        {
            let name = path.file_name().ok_or("snapshot has no filename")?;
            files.insert(
                name.to_str().ok_or("non-UTF-8 snapshot filename")?.into(),
                digest(&path)?,
            );
        }
    }
    if !files.contains_key("bindings.rs") {
        return Err("documentation snapshot lacks bindings.rs".into());
    }
    let snapshot = Snapshot {
        version: version.into(),
        generation_target: target.into(),
        inputs: inputs(root)?,
        files,
    };
    fs::write(
        directory.join("snapshot.json"),
        serde_json::to_vec_pretty(&snapshot)?,
    )?;
    Ok(())
}

pub fn stage(root: &Path, out: &Path, version: &str, target: &str) -> Result<()> {
    features(target)?;
    let directory = root.join("docs").join("bindings").join(target);
    println!("cargo:rerun-if-changed={}", directory.display());
    let snapshot: Snapshot = serde_json::from_slice(
        &fs::read(directory.join("snapshot.json"))
            .map_err(|error| format!("read packaged documentation snapshot for {target}: {error}; regenerate with this target's headers/toolchain"))?,
    )?;
    if snapshot.generation_target != target {
        return Err(format!(
            "documentation snapshot target mismatch: expected {target}, found {}",
            snapshot.generation_target
        )
        .into());
    }
    if snapshot.version != version || snapshot.inputs != inputs(root)? {
        return Err(
            "documentation snapshot is stale; regenerate it against the pinned native inputs"
                .into(),
        );
    }
    if !snapshot.files.contains_key("bindings.rs") {
        return Err("documentation snapshot lacks bindings.rs".into());
    }
    for (name, expected) in &snapshot.files {
        if Path::new(name).components().count() != 1
            || !name.ends_with(".rs")
            || name.contains(['/', '\\', ':'])
        {
            return Err(format!("invalid documentation snapshot filename: {name}").into());
        }
        let source = directory.join(name);
        if digest(&source)? != *expected {
            return Err(format!("documentation snapshot checksum mismatch: {name}").into());
        }
        fs::copy(source, out.join(name))?;
    }
    Ok(())
}
