//! CMake installation metadata, shared with build-script regression tests.
use super::config::Configuration;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

/// Completed native artifacts and their configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct Installation {
    pub fingerprint: String,
    pub target: String,
    pub version: String,
    pub threading: bool,
    pub capabilities: Vec<String>,
    pub source: PathBuf,
    pub build: PathBuf,
    pub prefix: PathBuf,
    pub deployment: PathBuf,
    pub runtime_deployment_files: Vec<PathBuf>,
    pub static_libraries: Vec<String>,
    pub system_libraries: Vec<String>,
    pub frameworks: Vec<String>,
    pub required_files: Vec<PathBuf>,
}

impl Installation {
    /// Require actual archive definitions, not merely installed declarations.
    pub fn has_static_symbols(&self, names: &[&str]) -> Result<bool, String> {
        Ok(self.missing_static_symbols(names)?.is_empty())
    }

    /// Admit an unmodified support source only when its whole symbol group is absent.
    pub fn needs_static_support(&self, names: &[&str]) -> Result<bool, String> {
        let missing = self.missing_static_symbols(names)?;
        if missing.is_empty() {
            return Ok(false);
        }
        let required = names
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if missing == required {
            return Ok(true);
        }
        Err(format!(
            "native archive contains only part of support symbol group; missing: {}",
            missing.into_iter().collect::<Vec<_>>().join(", ")
        ))
    }

    fn missing_static_symbols<'a>(
        &self,
        names: &[&'a str],
    ) -> Result<std::collections::BTreeSet<&'a str>, String> {
        let mut missing = names
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let leading_underscore = self.target.contains("apple")
            || self.target.starts_with("i686-pc-windows")
            || self.target.starts_with("i586-pc-windows");
        for library in &self.static_libraries {
            if missing.is_empty() {
                break;
            }
            let file_name = if self.target.contains("msvc") {
                format!("{library}.lib")
            } else {
                format!("lib{library}.a")
            };
            let path = self.prefix.join("lib").join(file_name);
            println!("cargo:rerun-if-changed={}", path.display());
            let bytes = fs::read(&path)
                .map_err(|error| format!("read native archive {}: {error}", path.display()))?;
            let archive = object::read::archive::ArchiveFile::parse(bytes.as_slice())
                .map_err(|error| format!("read native archive {}: {error}", path.display()))?;
            let symbols = archive
                .symbols()
                .map_err(|error| format!("read archive symbols {}: {error}", path.display()))?
                .ok_or_else(|| format!("native archive has no symbol index: {}", path.display()))?;
            for symbol in symbols {
                let symbol = symbol
                    .map_err(|error| format!("read archive symbol {}: {error}", path.display()))?;
                if let Ok(name) = std::str::from_utf8(symbol.name()) {
                    missing.remove(name);
                    if leading_underscore && let Some(name) = name.strip_prefix('_') {
                        missing.remove(name);
                    }
                }
            }
        }
        Ok(missing)
    }

    pub fn read(prefix: &Path) -> Result<Self, String> {
        let path = prefix.join("iree-native.json");
        let bytes = fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
        serde_json::from_slice(&bytes)
            .map_err(|error| format!("invalid {}: {error}", path.display()))
    }

    pub fn validate(&self, target: &str, features: &Configuration) -> Result<(), String> {
        if features.has("bare-metal") && self.threading {
            return Err("bare-metal requires a native installation with threading disabled".into());
        }
        let project =
            woodshed::project::Project::parse("woodshed.toml", include_str!("woodshed.toml"))
                .map_err(|error| format!("invalid pinned native versions: {error}"))?;
        let locked = project
            .parse_lock(include_str!("woodshed.lock"))
            .map_err(|error| format!("invalid pinned native versions: {error}"))?;
        let expected = &locked
            .sources
            .get("iree")
            .ok_or("missing IREE version pin")?
            .version;
        if self.target != target || self.version != *expected {
            return Err(format!(
                "expected IREE {expected} for {target}, found {} for {}",
                self.version, self.target
            ));
        }
        if !self.build.join("iree-bindgen-args.txt").is_file()
            || !self.build.join("iree-c-compiler.txt").is_file()
        {
            return Err("native installation lacks binding configuration; rebuild with this iree-native version".into());
        }
        let has_capability = |name| self.capabilities.iter().any(|value| value == name);
        for feature in features.native_capabilities() {
            let implied_backend = has_capability("compiler")
                && match feature {
                    "target-llvm-cpu" => ["cpu", "local-sync", "local-task"]
                        .iter()
                        .any(|name| has_capability(name)),
                    "target-vulkan" => has_capability("vulkan"),
                    _ => false,
                };
            if !has_capability(feature) && !implied_backend {
                return Err(format!(
                    "native installation lacks requested capability {feature}"
                ));
            }
        }
        if self.required_files.is_empty() {
            return Err("native installation has no required artifacts".into());
        }
        if !self.deployment.is_dir() {
            return Err(format!(
                "missing native deployment directory {}",
                self.deployment.display()
            ));
        }
        for file in &self.required_files {
            if !file.is_file() {
                return Err(format!("native installation is missing {}", file.display()));
            }
        }
        for file in &self.runtime_deployment_files {
            let relative = file.strip_prefix(&self.deployment).map_err(|_| {
                format!(
                    "runtime deployment resource is outside deployment: {}",
                    file.display()
                )
            })?;
            if relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
                || !file.is_file()
            {
                return Err(format!(
                    "invalid runtime deployment resource {}",
                    file.display()
                ));
            }
        }
        Ok(())
    }

    pub fn stage_deployment(
        &self,
        out: &Path,
        features: &Configuration,
    ) -> Result<PathBuf, String> {
        if features.uses_compiler() {
            return Ok(self.deployment.clone());
        }
        let destination = out.join("runtime-deploy");
        if destination.exists() {
            fs::remove_dir_all(&destination)
                .map_err(|error| format!("clear {}: {error}", destination.display()))?;
        }
        fs::create_dir_all(&destination)
            .map_err(|error| format!("create {}: {error}", destination.display()))?;
        for file in &self.runtime_deployment_files {
            let relative = file
                .strip_prefix(&self.deployment)
                .map_err(|_| format!("resource is outside deployment: {}", file.display()))?;
            if relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
            {
                return Err(format!(
                    "invalid runtime deployment resource {}",
                    file.display()
                ));
            }
            let target = destination.join(relative);
            let parent = target.parent().ok_or_else(|| {
                format!("deployment resource has no parent: {}", target.display())
            })?;
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "create runtime resource directory {}: {error}",
                    parent.display()
                )
            })?;
            fs::copy(file, &target)
                .map_err(|error| format!("stage {}: {error}", file.display()))?;
        }
        Ok(destination)
    }

    pub fn stage_compiler_deployment(&self, out: &Path) -> Result<(PathBuf, String), String> {
        let fingerprint = self.compiler_deployment_fingerprint()?;
        let profile = cargo_profile(out)?;
        let destination = profile.join("iree-native").join(&fingerprint);
        if destination.exists() {
            fs::remove_dir_all(&destination)
                .map_err(|error| format!("clear {}: {error}", destination.display()))?;
        }
        copy_directory(&self.deployment, &destination)?;
        Ok((destination, fingerprint))
    }

    pub fn stage_compiler_loader_libraries(&self, out: &Path) -> Result<(), String> {
        let profile = cargo_profile(out)?.to_path_buf();
        let mut files = Vec::new();
        collect_files(&self.deployment, &self.deployment, &mut files)?;
        for directory in [profile.clone(), profile.join("deps")] {
            fs::create_dir_all(&directory).map_err(|error| {
                format!(
                    "create compiler loader directory {}: {error}",
                    directory.display()
                )
            })?;
            for (_, source) in files.iter().filter(|(_, path)| {
                path.extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("dll"))
            }) {
                let destination = directory.join(source.file_name().ok_or_else(|| {
                    format!("compiler DLL has no file name: {}", source.display())
                })?);
                fs::copy(source, &destination).map_err(|error| {
                    format!(
                        "stage compiler loader library {} to {}: {error}",
                        source.display(),
                        destination.display()
                    )
                })?;
            }
        }
        Ok(())
    }

    pub fn compiler_deployment_fingerprint(&self) -> Result<String, String> {
        let mut files = Vec::new();
        collect_files(&self.deployment, &self.deployment, &mut files)?;
        files.sort();
        if files.is_empty() {
            return Err("compiler deployment contains no files".into());
        }
        let mut hasher = Sha256::new();
        hasher.update(self.fingerprint.as_bytes());
        hasher.update(self.target.as_bytes());
        hasher.update(self.version.as_bytes());
        let mut capabilities = self.capabilities.clone();
        capabilities.sort();
        for capability in capabilities {
            hasher.update(capability.as_bytes());
            hasher.update([0]);
        }
        for (relative, path) in files {
            hasher.update(relative.as_os_str().as_encoded_bytes());
            hasher.update([0]);
            hasher.update(
                fs::read(&path).map_err(|error| {
                    format!("read compiler resource {}: {error}", path.display())
                })?,
            );
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn link(&self, out: &Path, features: &Configuration) -> Result<(), String> {
        let deployment = self
            .stage_deployment(out, features)
            .map_err(|error| format!("stage selected native deployment: {error}"))?;
        println!(
            "cargo:rustc-env=WOODSHED_DEPLOYMENT_DIR={}",
            deployment.display()
        );
        println!("cargo:deployment_dir={}", deployment.display());
        println!(
            "cargo:rustc-link-search=native={}",
            self.prefix.join("lib").display()
        );
        for name in &self.static_libraries {
            println!("cargo:rustc-link-lib=static={name}");
        }
        for name in &self.system_libraries {
            println!("cargo:rustc-link-lib={name}");
        }
        for name in &self.frameworks {
            println!("cargo:rustc-link-lib=framework={name}");
        }
        println!(
            "cargo:rustc-env=IREE_NATIVE_SOURCE_DIR={}",
            self.source.display()
        );
        println!(
            "cargo:rustc-env=IREE_NATIVE_PREFIX={}",
            self.prefix.display()
        );
        println!("cargo:source_dir={}", self.source.display());
        println!("cargo:build_dir={}", self.build.display());
        println!(
            "cargo:include={}",
            self.source.join("runtime/src").display()
        );
        if features.uses_compiler() {
            let bin = &self.deployment;
            woodshed::copy_shared_libs(&self.prefix, out).map_err(|error| {
                format!(
                    "stage compiler shared libraries from {} to {}: {error}",
                    self.prefix.display(),
                    out.display()
                )
            })?;
            copy_directory(bin, out)?;
            println!("cargo:rustc-link-search=native={}", out.display());
            if features.has("compiler") {
                let (runtime_bin, deployment_id) = self
                    .stage_compiler_deployment(out)
                    .map_err(|error| format!("stage compiler runtime deployment: {error}"))?;
                if self.target.contains("windows") {
                    self.stage_compiler_loader_libraries(out).map_err(|error| {
                        format!("stage compiler DLLs beside Cargo executables: {error}")
                    })?;
                }
                println!("cargo:rustc-link-lib=dylib=IREECompiler");
                println!("cargo:rustc-link-search=native={}", runtime_bin.display());
                println!(
                    "cargo:rustc-env=IREE_COMPILER_BIN_DIR={}",
                    runtime_bin.display()
                );
                println!(
                    "cargo:rustc-env=IREE_COMPILER_DEPLOYMENT_ID={}",
                    deployment_id
                );
                println!("cargo:compiler_bin_dir={}", runtime_bin.display());
            } else {
                println!("cargo:rustc-env=IREE_COMPILER_BIN_DIR={}", bin.display());
                println!("cargo:compiler_bin_dir={}", bin.display());
            }
        }
        Ok(())
    }
}

fn cargo_profile(out: &Path) -> Result<&Path, String> {
    out.ancestors()
        .nth(3)
        .ok_or_else(|| "Cargo OUT_DIR is not nested under a profile directory".into())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("create compiler deployment directory: {error}"))?;
    for entry in
        fs::read_dir(source).map_err(|error| format!("read compiler installation: {error}"))?
    {
        let entry = entry.map_err(|error| format!("read compiler resource: {error}"))?;
        if entry
            .file_type()
            .map_err(|error| format!("inspect compiler resource: {error}"))?
            .is_dir()
        {
            copy_directory(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            let source = entry.path();
            fs::copy(&source, destination.join(entry.file_name()))
                .map_err(|error| format!("stage {}: {error}", source.display()))?;
        }
    }
    Ok(())
}

fn collect_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<(PathBuf, PathBuf)>,
) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("read compiler deployment {}: {error}", directory.display()))?
    {
        let entry = entry.map_err(|error| format!("read compiler resource: {error}"))?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|error| format!("inspect compiler resource {}: {error}", path.display()))?
            .is_dir()
        {
            collect_files(root, &path, files)?;
        } else {
            let relative = path
                .strip_prefix(root)
                .map_err(|error| format!("resolve compiler resource path: {error}"))?
                .to_path_buf();
            files.push((relative, path));
        }
    }
    Ok(())
}
