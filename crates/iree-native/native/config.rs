//! Build-time feature translation shared by the build script and its tests.

use std::collections::BTreeSet;

/// Cargo features which affect the native build.
pub const FEATURES: &[&str] = &[
    "bare-metal",
    "compiler",
    "compiler-tools",
    "mpi",
    "cpu",
    "local-sync",
    "local-task",
    "task",
    "tokenizer",
    "vulkan",
    "cuda",
    "hip",
    "amdgpu",
    "metal",
    "null",
    "webgpu",
    "io-index",
    "io-parameters",
    "io-format-irpa",
    "io-format-gguf",
    "io-format-safetensors",
    "target-llvm-cpu",
    "target-vulkan",
    "target-cuda",
    "target-rocm",
    "target-metal",
    "target-webgpu",
    "target-wasm",
    "llvm-aarch64",
    "llvm-arm",
    "llvm-riscv",
    "llvm-x86",
    "llvm-webassembly",
    "input-stablehlo",
    "input-tosa",
    "input-torch",
];

/// Native options selected by Cargo.
pub struct Configuration {
    enabled: BTreeSet<String>,
}

impl Configuration {
    /// Construct from feature names without consulting the environment.
    pub fn new(features: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            enabled: features.into_iter().map(Into::into).collect(),
        }
    }

    /// Whether a Cargo feature was enabled.
    pub fn has(&self, feature: &str) -> bool {
        self.enabled.contains(feature)
    }

    /// Native capabilities required from either a bundled or supplied installation.
    pub fn native_capabilities(&self) -> Vec<&'static str> {
        FEATURES
            .iter()
            .copied()
            .filter(|name| {
                (self.has(name) && !["compiler-tools", "mpi"].contains(name))
                    || (self.has("cpu") && ["local-sync", "local-task", "task"].contains(name))
                    || (self.has("local-task") && *name == "task")
            })
            .collect()
    }

    pub fn uses_compiler(&self) -> bool {
        self.has("compiler")
    }

    /// Manifest features for the effective superbuild capability selection.
    pub fn source_features(&self) -> Vec<String> {
        let mut features = self
            .native_capabilities()
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        if self.uses_compiler() && self.has("input-stablehlo") {
            features.push("compiler-stablehlo".into());
        }
        features
    }

    /// Reject impossible runtime/OS combinations before provisioning sources.
    pub fn validate_target(&self, target: &str) -> Result<(), String> {
        if self.has("bare-metal") {
            for incompatible in [
                "cpu",
                "local-task",
                "task",
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
                if self.has(incompatible) {
                    return Err(format!(
                        "bare-metal does not support {incompatible}; disable default features"
                    ));
                }
            }
            if !target.contains("-none-") && !target.ends_with("-none") {
                return Err("bare-metal requires a Rust freestanding *-none target".into());
            }
        }
        if self.has("metal") && !target.contains("apple") {
            return Err("the Metal runtime requires an Apple target; target-metal is a separate compiler backend".into());
        }
        if self.has("amdgpu") && !target.contains("linux") {
            return Err("the AMDGPU runtime requires Linux".into());
        }
        if self.has("webgpu") && !target.starts_with("wasm32") {
            return Err("the WebGPU runtime requires a wasm32 JavaScript embedding".into());
        }
        Ok(())
    }

    /// Compiler/backend selection is independent of runtime driver selection.
    pub fn options(&self, arch: &str) -> Vec<(String, String)> {
        let mut options = Vec::new();
        let mut flag = |name: &str, value: bool| {
            options.push((name.into(), if value { "ON" } else { "OFF" }.into()));
        };
        flag(
            "IREE_NATIVE_LOCAL_SYNC",
            self.has("cpu") || self.has("local-sync"),
        );
        flag(
            "IREE_NATIVE_LOCAL_TASK",
            self.has("cpu") || self.has("local-task"),
        );
        flag(
            "IREE_NATIVE_TASK",
            self.has("cpu") || self.has("local-task") || self.has("task"),
        );
        flag("IREE_NATIVE_VULKAN", self.has("vulkan"));
        for driver in ["cuda", "hip", "amdgpu", "metal", "null"] {
            flag(
                &format!("IREE_HAL_DRIVER_{}", driver.to_uppercase()),
                self.has(driver),
            );
        }
        for (feature, name) in [
            ("target-cuda", "CUDA"),
            ("target-rocm", "ROCM"),
            ("target-metal", "METAL_SPIRV"),
            ("target-webgpu", "WEBGPU_SPIRV"),
            ("target-wasm", "LLVM_CPU_WASM"),
        ] {
            flag(&format!("IREE_TARGET_BACKEND_{name}"), self.has(feature));
        }
        flag(
            "IREE_TARGET_BACKEND_LLVM_CPU",
            self.has("cpu")
                || self.has("local-sync")
                || self.has("local-task")
                || self.has("target-llvm-cpu"),
        );
        flag(
            "IREE_TARGET_BACKEND_VULKAN_SPIRV",
            self.has("vulkan") || self.has("target-vulkan"),
        );
        flag("IREE_NATIVE_WEBGPU", self.has("webgpu"));
        flag("IREE_NATIVE_IO_INDEX", self.has("io-index"));
        flag("IREE_NATIVE_IO_PARAMETERS", self.has("io-parameters"));
        flag("IREE_NATIVE_IO_FORMAT_IRPA", self.has("io-format-irpa"));
        flag("IREE_NATIVE_IO_FORMAT_GGUF", self.has("io-format-gguf"));
        flag(
            "IREE_NATIVE_IO_FORMAT_SAFETENSORS",
            self.has("io-format-safetensors"),
        );
        flag("IREE_NATIVE_BARE_METAL", self.has("bare-metal"));
        flag("IREE_NATIVE_TOKENIZER", self.has("tokenizer"));
        for input in ["stablehlo", "tosa", "torch"] {
            flag(
                &format!("IREE_INPUT_{}", input.to_uppercase()),
                self.has(&format!("input-{input}")),
            );
        }
        let mut targets = BTreeSet::new();
        if let Some(host) = match arch {
            "x86" | "x86_64" => Some("X86"),
            "aarch64" => Some("AArch64"),
            "arm" => Some("ARM"),
            "riscv32" | "riscv64" => Some("RISCV"),
            "wasm32" | "wasm64" => Some("WebAssembly"),
            _ => None,
        } {
            targets.insert(host);
        }
        for (feature, name) in [
            ("llvm-x86", "X86"),
            ("llvm-aarch64", "AArch64"),
            ("llvm-arm", "ARM"),
            ("llvm-riscv", "RISCV"),
            ("llvm-webassembly", "WebAssembly"),
            ("target-wasm", "WebAssembly"),
        ] {
            if self.has(feature) {
                targets.insert(name);
            }
        }
        options.push((
            "IREE_DEFAULT_CPU_LLVM_TARGETS".into(),
            targets.into_iter().collect::<Vec<_>>().join(";"),
        ));
        options
    }
}
