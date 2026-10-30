//! Persistent compilation workspaces and immutable, configuration-specific SDKs.
use super::{config::Configuration, manifest::Installation};
use serde::{Deserialize, Serialize};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
use woodshed::{BuildDirs, BuildKey, Cache, Dep};

pub fn family_key<'a>(
    pins: impl Iterator<Item = &'a Dep>,
    features: &Configuration,
    target: &str,
    host: &str,
    toolchain: Option<&Path>,
) -> Result<BuildKey, String> {
    let mut key = BuildKey::new("iree-rs-workspace-v2")
        .mix(target)
        .mix(host)
        .mix(format!("bare-metal={}", features.has("bare-metal")));
    for dep in pins {
        key = key.mix(&dep.name).dep(dep);
    }
    // Feature toggles are deliberately absent. SDK/toolchain changes are not
    // interchangeable, even if they happen to select the same Rust target.
    for name in [
        "CC",
        "CXX",
        "AR",
        "CFLAGS",
        "CXXFLAGS",
        "CPPFLAGS",
        "LDFLAGS",
        "CMAKE_GENERATOR",
        "CMAKE_GENERATOR_PLATFORM",
        "CMAKE_GENERATOR_TOOLSET",
        "IREE_NATIVE_HOST_BIN_DIR",
        "IREE_CUDA_DEPS_DIR",
        "CUDAToolkit_ROOT",
        "CUDA_PATH",
        "IREE_NATIVE_ROCM_BITCODE_DIR",
        "SDKROOT",
        "MACOSX_DEPLOYMENT_TARGET",
        "VCToolsInstallDir",
        "WindowsSdkDir",
        "WindowsSDKVersion",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
        key = key
            .mix(name)
            .mix(env::var_os(name).unwrap_or_default().as_encoded_bytes());
    }
    key = key.mix(format!("crt-static={}", woodshed::crt_static()));
    if target.contains("msvc") && woodshed::crt_static() {
        key = key.mix(format!("compiler-crt={}", features.uses_compiler()));
    }
    let ninja = woodshed::find_on_path("ninja").is_some()
        && (!target.contains("msvc")
            || (env::var_os("INCLUDE").is_some() && env::var_os("LIB").is_some()));
    key = key.mix(format!("ninja={ninja}"));
    for (variable, fallback) in [
        ("CC", if target.contains("msvc") { "cl" } else { "cc" }),
        ("CXX", if target.contains("msvc") { "cl" } else { "c++" }),
    ] {
        let compiler = env::var(variable).unwrap_or_else(|_| fallback.into());
        let path = PathBuf::from(&compiler);
        if let Some(path) = path
            .is_file()
            .then_some(path)
            .or_else(|| woodshed::find_on_path(&compiler))
        {
            println!("cargo:rerun-if-changed={}", path.display());
            let metadata = fs::metadata(&path)
                .map_err(|error| format!("inspect native compiler {}: {error}", path.display()))?;
            key = key
                .mix(variable)
                .mix(path.as_os_str().as_encoded_bytes())
                .mix(metadata.len().to_le_bytes())
                .mix(format!(
                    "{:?}",
                    metadata.modified().map_err(|error| format!(
                        "read native compiler timestamp {}: {error}",
                        path.display()
                    ))?
                ));
        }
    }
    if let Some(path) = toolchain {
        key = key.mix(path.as_os_str().as_encoded_bytes()).mix(
            woodshed::sha256_file(path)
                .map_err(|error| format!("hash native toolchain {}: {error}", path.display()))?,
        );
    }
    Ok(key)
}

#[derive(Deserialize, Serialize)]
pub struct Workspace {
    pub build: PathBuf,
    pub mirror: PathBuf,
    pub source: Option<PathBuf>,
    pub capabilities: Vec<String>,
}

impl Workspace {
    pub fn new(build: PathBuf, mirror: PathBuf) -> Self {
        Self {
            build,
            mirror,
            source: None,
            capabilities: Vec::new(),
        }
    }

    pub fn load(path: &Path) -> io::Result<Option<Self>> {
        match fs::read(path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map(Some)
                .map_err(io::Error::other),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let parent = path.parent().ok_or_else(|| {
            io::Error::other(format!(
                "workspace state path has no parent: {}",
                path.display()
            ))
        })?;
        fs::create_dir_all(parent)?;
        // The workspace lock protects readers. Do not truncate a valid receipt
        // until its replacement has been fully written.
        let temporary = path.with_extension("tmp");
        fs::write(&temporary, serde_json::to_vec_pretty(self)?)?;
        fs::rename(temporary, path)
    }

    pub fn selection(&self, requested: &Configuration) -> Configuration {
        Configuration::new(
            self.capabilities
                .iter()
                .map(String::as_str)
                .chain(requested.native_capabilities()),
        )
    }
}

pub fn directories(cache: &Cache, key: &BuildKey, short: bool) -> BuildDirs {
    let mut dirs = cache.build_dir(key);
    if short {
        let key = key.as_str();
        let hash = key.rsplit_once('-').map_or(key.as_str(), |(_, hash)| hash);
        // Windows MAX_PATH leaves IREE's deepest compiler objects little headroom.
        dirs.build = cache.root().join("b").join(&hash[..hash.len().min(8)]);
        dirs.prefix = dirs.build.join("prefix");
    }
    dirs
}

pub fn installation_status(
    prefix: &Path,
    fingerprint: &str,
    target: &str,
    features: &Configuration,
) -> Result<(), String> {
    let installed = Installation::read(prefix)?;
    if installed.build != prefix.join("build-metadata") {
        return Err("installation does not have an isolated build-metadata snapshot".into());
    }
    if installed.fingerprint != fingerprint {
        return Err("native source/configuration fingerprint differs".into());
    }
    installed.validate(target, features)
}

pub fn build_plan(
    state: &Workspace,
    recorded: bool,
    requested: &Configuration,
    effective: &Configuration,
    sdk: Result<(), String>,
) -> Vec<String> {
    let mut lines = vec![match &sdk {
        Ok(()) => "SDK: reuse; installation metadata and required-file checks passed (not a content-hash audit)".into(),
        Err(reason) => format!("SDK: build/install required; {reason}"),
    }];
    if sdk.is_ok() {
        lines.push("Workspace/sources: not needed; skip source acquisition and CMake".into());
    } else {
        lines.push(if recorded && state.build.join("CMakeCache.txt").is_file() {
            format!("Workspace: reuse and reconfigure {}; matching source/target/toolchain family", state.build.display())
        } else {
            format!("Workspace: cold configure {}; {}; prior source/target/toolchain changes cannot be distinguished without a previous-family record",
                state.build.display(),
                if recorded { "CMakeCache.txt is missing" } else { "no receipt for this exact family" })
        });
        let added = effective
            .native_capabilities()
            .into_iter()
            .filter(|name| !state.capabilities.iter().any(|old| old == name))
            .collect::<Vec<_>>();
        if !added.is_empty() {
            lines.push(format!(
                "Capabilities: add {}; previously built capabilities retained",
                added.join(", ")
            ));
        }
        lines.push("Sources: verify selected locked trees, then reuse/repair/fetch as needed; cache outcomes unresolved until acquisition".into());
        lines.push("Compilation: CMake/build tool decides dirty objects after configure; no object-level rebuild prediction".into());
    }
    lines.push(format!(
        "Requested capabilities: {}; effective native capabilities: {}",
        requested.native_capabilities().join(", "),
        effective.native_capabilities().join(", ")
    ));
    lines.push("Bindings: regenerate for this Cargo invocation; no binding-output cache; support C compilation may run even on an SDK hit".into());
    lines
}

/// Snapshot generated headers as well as libraries: a later reconfigure must
/// never change the ABI metadata consumed by an earlier Cargo configuration.
pub fn publish(installed: &Installation, prefix: &Path, mirror: &Path) -> io::Result<Installation> {
    let snapshot = prefix.join("build-metadata");
    let mut metadata_files = copy_headers(&installed.build, &snapshot)?;
    metadata_files.extend(copy_headers(mirror, &snapshot.join("native"))?);
    for name in [
        "CMakeCache.txt",
        "iree-bindgen-args.txt",
        "iree-bindgen-sysroot.txt",
        "iree-c-compiler.txt",
        "iree-python-executable.txt",
    ] {
        let path = installed.build.join(name);
        match fs::read_to_string(&path) {
            Ok(value) => {
                let value = replace_path(
                    &replace_path(&value, &installed.build, &snapshot),
                    mirror,
                    &snapshot.join("native"),
                );
                fs::write(snapshot.join(name), value)?;
                metadata_files.push(snapshot.join(name));
            }
            Err(error)
                if error.kind() == io::ErrorKind::NotFound
                    && matches!(
                        name,
                        "iree-bindgen-sysroot.txt" | "iree-python-executable.txt"
                    ) => {}
            Err(error) => return Err(error),
        }
    }
    let mut document = serde_json::to_value(installed)?;
    document["build"] = serde_json::to_value(snapshot)?;
    document["required_files"] = serde_json::to_value(
        installed
            .required_files
            .iter()
            .chain(&metadata_files)
            .collect::<Vec<_>>(),
    )?;
    // The marker is published last; partial installations are never selected.
    let marker = prefix.join("iree-native.json");
    let temporary = prefix.join("iree-native.json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(&document)?)?;
    fs::rename(temporary, marker)?;
    Installation::read(prefix).map_err(io::Error::other)
}

fn replace_path(value: &str, from: &Path, to: &Path) -> String {
    let from = from.to_string_lossy();
    let to = to.to_string_lossy();
    value
        .replace(from.as_ref(), to.as_ref())
        .replace(&from.replace('\\', "/"), &to.replace('\\', "/"))
}

fn copy_headers(source: &Path, destination: &Path) -> io::Result<Vec<PathBuf>> {
    fs::create_dir_all(destination)?;
    let mut files = Vec::new();
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            // Old build trees contain their old installation under prefix.
            if entry.file_name() != "prefix" {
                files.extend(copy_headers(&path, &destination.join(entry.file_name()))?);
            }
        } else if matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("h" | "hh" | "hpp" | "hxx" | "inc" | "def" | "td" | "modulemap")
        ) {
            let target = destination.join(entry.file_name());
            fs::copy(path, &target)?;
            files.push(target);
        }
    }
    Ok(files)
}
