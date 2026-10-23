//! CMake installation metadata, shared with build-script regression tests.
use super::config::Configuration;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Completed native artifacts and their configuration.
#[derive(Debug, Deserialize)]
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
    pub static_libraries: Vec<String>,
    pub system_libraries: Vec<String>,
    pub frameworks: Vec<String>,
    pub required_files: Vec<PathBuf>,
}

impl Installation {
    pub fn read(prefix: &Path) -> Result<Self, String> {
        let path = prefix.join("iree-native.json");
        let bytes = fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
        serde_json::from_slice(&bytes)
            .map_err(|error| format!("invalid {}: {error}", path.display()))
    }

    pub fn validate(&self, target: &str, features: &Configuration) -> Result<(), String> {
        if self.target != target || self.version != "3.13.0" {
            return Err(format!(
                "expected IREE 3.13.0 for {target}, found {} for {}",
                self.version, self.target
            ));
        }
        if !self.build.join("iree-bindgen-args.txt").is_file()
            || !self.build.join("iree-c-compiler.txt").is_file()
        {
            return Err("native installation lacks binding configuration; rebuild with this iree-sys version".into());
        }
        for feature in features.native_capabilities() {
            if !self.capabilities.iter().any(|value| value == feature) {
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
        Ok(())
    }

    pub fn link(&self, out: &Path) {
        println!(
            "cargo:rustc-env=WOODSHED_DEPLOYMENT_DIR={}",
            self.deployment.display()
        );
        println!("cargo:deployment_dir={}", self.deployment.display());
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
        if self.threading {
            println!("cargo:rustc-cfg=iree_native_threading");
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
        if self.capabilities.iter().any(|name| name == "compiler") {
            let bin = &self.deployment;
            woodshed::copy_shared_libs(&self.prefix, out).expect("stage compiler shared library");
            copy_directory(bin, out);
            println!("cargo:rustc-link-search=native={}", out.display());
            println!("cargo:rustc-link-lib=dylib=IREECompiler");
            println!("cargo:rustc-env=IREE_COMPILER_BIN_DIR={}", bin.display());
            println!("cargo:compiler_bin_dir={}", bin.display());
        }
    }
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create compiler deployment directory");
    for entry in fs::read_dir(source).expect("read compiler installation") {
        let entry = entry.expect("read compiler resource");
        if entry
            .file_type()
            .expect("inspect compiler resource")
            .is_dir()
        {
            copy_directory(&entry.path(), &destination.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))
                .expect("stage compiler resource");
        }
    }
}
