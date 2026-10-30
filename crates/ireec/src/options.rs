//! Typed options lowered directly to the compiler's session flag API.
//!
//! Flag mappings are verified against upstream IREE `main`:
//! - `TargetOptions.cpp`: `iree-hal-target-backends` and repeatable
//!   `iree-hal-target-device`.
//! - [`compiler/src/iree/compiler/Pipelines/Options.cpp`](https://raw.githubusercontent.com/iree-org/iree/main/compiler/src/iree/compiler/Pipelines/Options.cpp):
//!   `iree-opt-level`, `iree-input-type`, `iree-opt-data-tiling`,
//!   `iree-opt-const-expr-hoisting`, and all five `iree-input-demote-*` /
//!   `iree-input-promote-*` flags named by [`InputConversion`].
//!   Parameter mappings are `iree-parameter-import` (repeated paths),
//!   `iree-parameter-export`, `iree-parameter-splat`, and
//!   `iree-parameter-export-minimum-size` (bytes). The old
//!   `iree-opt-export-parameters` and `iree-opt-splat-parameters` names are
//!   deprecated aliases; these bindings emit the current names.
//!   All boolean overrides emit explicit `=true` or `=false`; omitted settings
//!   leave compiler defaults unchanged. Paths must be valid UTF-8.
//! - `LLVMCPU/LLVMTargetOptions.cpp`: `iree-llvmcpu-target-triple`,
//!   `iree-llvmcpu-target-cpu`, and `iree-llvmcpu-target-cpu-features`.
//! - `ROCM/ROCMTarget.cpp`: `iree-rocm-target` and
//!   `iree-rocm-target-features`.
//! - `CUDA/CUDATarget.cpp`: `iree-cuda-target` and
//!   `iree-cuda-target-features`.
//! - `VulkanSPIRV/VulkanSPIRVTarget.cpp`: `iree-vulkan-target` and
//!   `iree-vulkan-target-features`.
//! - `Dialect/VM/Target/Bytecode/BytecodeModuleTarget.cpp` and
//!   `Dialect/VM/Target/C/TranslationFlags.cpp`: the VM output-format flags.
//!
//! The exact flags are applied only through `ireeCompilerSessionSetFlags`.
//! No target is inferred from the host and no backend is selected by default.
//!
//! Single target:
//! ```no_run
//! use ireec::{Compiler, Options, Target};
//! fn main() -> ireec::Result<()> {
//!     let options = Options::default().with_target(Target::Vmvx);
//!     let compiler = Compiler::new()?;
//!     let mut session = compiler.session()?;
//!     session.configure(&options)
//! }
//! ```
//!
//! Multi-backend compilation puts all requested backends into one module; the
//! runtime can choose an executable appropriate to its device:
//! ```no_run
//! use ireec::{Compiler, Options, Target};
//! fn main() -> ireec::Result<()> {
//!     let options = Options::default()
//!         .with_target(Target::LlvmCpu { triple: None, cpu: None, cpu_features: None })
//!         .with_target(Target::VulkanSpirv { target: Some("rdna3".into()), features: None });
//!     let compiler = Compiler::new()?;
//!     let mut session = compiler.session()?;
//!     session.configure(&options)
//! }
//! ```
//!
//! Heterogeneous device declarations become repeated target-device flags:
//! ```no_run
//! use ireec::{Compiler, Device, Options, Target};
//! fn main() -> ireec::Result<()> {
//!     let options = Options::default()
//!         .with_target(Target::Vmvx)
//!         .with_device(Device::Alias("local-task".into()))
//!         .with_device(Device::Named {
//!             name: "gpu".into(),
//!             alias: "vulkan".into(),
//!             ordinal: Some(0),
//!         });
//!     let compiler = Compiler::new()?;
//!     let mut session = compiler.session()?;
//!     session.configure(&options)
//! }
//! ```

use crate::{Error, Result};
use std::path::{Path, PathBuf};

/// One IREE HAL executable backend and its typed backend-specific target flags.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// `llvm-cpu`; maps its optional fields to LLVM CPU flags.
    LlvmCpu {
        /// LLVM target machine triple (`iree-llvmcpu-target-triple`).
        triple: Option<String>,
        /// LLVM target CPU (`iree-llvmcpu-target-cpu`).
        cpu: Option<String>,
        /// LLVM CPU features (`iree-llvmcpu-target-cpu-features`).
        cpu_features: Option<String>,
    },
    /// `vmvx` portable backend.
    Vmvx,
    /// `vulkan-spirv`; target can be a Vulkan environment/profile, architecture, or product.
    VulkanSpirv {
        /// Vulkan target (`iree-vulkan-target`), for example `vp_android_baseline_2022` or `rdna3`.
        target: Option<String>,
        /// Vulkan SPIR-V features (`iree-vulkan-target-features`).
        features: Option<String>,
    },
    /// `rocm` (HIP/AMDGPU); `chip` is passed to the compiler's ROCm target flag.
    Rocm {
        /// ROCm architecture such as `gfx942` (`iree-rocm-target`).
        chip: Option<String>,
        /// ROCm target feature string (`iree-rocm-target-features`).
        features: Option<String>,
    },
    /// `cuda`; `sm_target` accepts compiler-supported target spellings such as `sm_90`.
    Cuda {
        /// CUDA target (`iree-cuda-target`).
        sm_target: Option<String>,
        /// CUDA target features (`iree-cuda-target-features`).
        features: Option<String>,
    },
    /// `metal-spirv` backend.
    MetalSpirv,
    /// `webgpu-spirv` backend.
    WebGpuSpirv,
    /// An out-of-tree backend name. Backend-specific flags remain available through `Session::set_flags`.
    Other(String),
}

impl Target {
    fn backend(&self) -> &str {
        match self {
            Self::LlvmCpu { .. } => "llvm-cpu",
            Self::Vmvx => "vmvx",
            Self::VulkanSpirv { .. } => "vulkan-spirv",
            Self::Rocm { .. } => "rocm",
            Self::Cuda { .. } => "cuda",
            Self::MetalSpirv => "metal-spirv",
            Self::WebGpuSpirv => "webgpu-spirv",
            Self::Other(name) => name,
        }
    }

    fn append_flags(&self, flags: &mut Vec<String>) {
        match self {
            Self::LlvmCpu {
                triple,
                cpu,
                cpu_features,
            } => {
                append_optional(flags, "iree-llvmcpu-target-triple", triple);
                append_optional(flags, "iree-llvmcpu-target-cpu", cpu);
                append_optional(flags, "iree-llvmcpu-target-cpu-features", cpu_features);
            }
            Self::VulkanSpirv { target, features } => {
                append_optional(flags, "iree-vulkan-target", target);
                append_optional(flags, "iree-vulkan-target-features", features);
            }
            Self::Rocm { chip, features } => {
                append_optional(flags, "iree-rocm-target", chip);
                append_optional(flags, "iree-rocm-target-features", features);
            }
            Self::Cuda {
                sm_target,
                features,
            } => {
                append_optional(flags, "iree-cuda-target", sm_target);
                append_optional(flags, "iree-cuda-target-features", features);
            }
            Self::Vmvx | Self::MetalSpirv | Self::WebGpuSpirv | Self::Other(_) => {}
        }
    }
}

/// A repeatable `iree-hal-target-device` specification.
///
/// Alias ordinals use the upstream `alias[ordinal]` syntax. Named devices use
/// the upstream `name=alias[ordinal]` syntax; names are kept distinct from
/// executable backend targets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Device {
    /// An alias, such as `local-task` or `vulkan`.
    Alias(String),
    /// An alias and numeric ordinal, such as `vulkan[0]`.
    AliasOrdinal {
        /// Device alias (for example `vulkan`).
        alias: String,
        /// Ordinal within the alias.
        ordinal: u32,
    },
    /// A named target device, such as `gpu=vulkan[0]`.
    Named {
        /// Device name used in the module's HAL device-target mapping.
        name: String,
        /// Device alias (for example `vulkan`).
        alias: String,
        /// Optional ordinal within the alias.
        ordinal: Option<u32>,
    },
}

impl Device {
    fn flag_value(&self) -> Result<String> {
        let (name, alias, ordinal) = match self {
            Self::Alias(alias) => (None, alias.as_str(), None),
            Self::AliasOrdinal { alias, ordinal } => (None, alias.as_str(), Some(*ordinal)),
            Self::Named {
                name,
                alias,
                ordinal,
            } => (Some(name.as_str()), alias.as_str(), *ordinal),
        };
        validate_alias(alias)?;
        if let Some(name) = name
            && (name.is_empty() || !name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_'))
        {
            return Err(Error::InvalidOptions(
                "device names must use ASCII letters, digits, or underscores".into(),
            ));
        }
        let mut value = String::new();
        if let Some(name) = name {
            value.push_str(name);
            value.push('=');
        }
        value.push_str(alias);
        if let Some(ordinal) = ordinal {
            value.push('[');
            value.push_str(&ordinal.to_string());
            value.push(']');
        }
        Ok(value)
    }
}

/// IREE's global optimization level (`iree-opt-level`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptLevel {
    /// No optimization.
    O0,
    /// Basic optimization.
    O1,
    /// Standard optimization.
    O2,
    /// Aggressive optimization.
    O3,
}

impl OptLevel {
    fn flag_value(self) -> &'static str {
        match self {
            Self::O0 => "O0",
            Self::O1 => "O1",
            Self::O2 => "O2",
            Self::O3 => "O3",
        }
    }
}

/// Built-in IREE input dialect, or an input mnemonic registered by a compiler plugin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    /// Let the compiler analyze input.
    Auto,
    /// Do not apply input dialect conversion.
    None,
    /// StableHLO input.
    StableHlo,
    /// StableHLO input with XLA de-tupling.
    StableHloXla,
    /// TOSA input.
    Tosa,
    /// Torch dialect input.
    Torch,
    /// Tensor-based Torch input.
    TmTensor,
    /// An input mnemonic registered by an out-of-tree plugin.
    Other(String),
}

impl InputType {
    fn flag_value(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::StableHlo => "stablehlo",
            Self::StableHloXla => "stablehlo_xla",
            Self::Tosa => "tosa",
            Self::Torch => "torch",
            Self::TmTensor => "tm_tensor",
            Self::Other(value) => value,
        }
    }
}

/// Unconditional input type conversions performed before global optimization.
///
/// Configure each conversion with [`Options::with_input_conversion`]; explicit
/// `false` disables a conversion without relying on compiler defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputConversion {
    /// `iree-input-demote-i64-to-i32`.
    DemoteI64ToI32,
    /// `iree-input-demote-f32-to-f16`.
    DemoteF32ToF16,
    /// `iree-input-demote-f64-to-f32`.
    DemoteF64ToF32,
    /// `iree-input-promote-f16-to-f32`.
    PromoteF16ToF32,
    /// `iree-input-promote-bf16-to-f32`.
    PromoteBf16ToF32,
}

impl InputConversion {
    fn flag_name(self) -> &'static str {
        match self {
            Self::DemoteI64ToI32 => "iree-input-demote-i64-to-i32",
            Self::DemoteF32ToF16 => "iree-input-demote-f32-to-f16",
            Self::DemoteF64ToF32 => "iree-input-demote-f64-to-f32",
            Self::PromoteF16ToF32 => "iree-input-promote-f16-to-f32",
            Self::PromoteBf16ToF32 => "iree-input-promote-bf16-to-f32",
        }
    }
}

/// Output format flags accepted by IREE's VM bytecode and VM C targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// VM bytecode output format.
    VmBytecode(VmBytecodeFormat),
    /// VM C output format.
    VmC(VmCFormat),
}

/// Format accepted by `iree-vm-bytecode-module-output-format`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmBytecodeFormat {
    /// Binary FlatBuffer VMFB.
    FlatbufferBinary,
    /// Debug-only text FlatBuffer.
    FlatbufferText,
    /// VM dialect MLIR text.
    MlirText,
    /// VM dialect MLIR text with annotations.
    AnnotatedMlirText,
}

impl VmBytecodeFormat {
    fn flag_value(self) -> &'static str {
        match self {
            Self::FlatbufferBinary => "flatbuffer-binary",
            Self::FlatbufferText => "flatbuffer-text",
            Self::MlirText => "mlir-text",
            Self::AnnotatedMlirText => "annotated-mlir-text",
        }
    }
}

/// Format accepted by `iree-vm-c-module-output-format`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmCFormat {
    /// C source code.
    Code,
    /// VM and EmitC dialect MLIR text.
    MlirText,
}

impl VmCFormat {
    fn flag_value(self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::MlirText => "mlir-text",
        }
    }
}

/// Typed compiler settings applied by `Session::configure`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Options {
    targets: Vec<Target>,
    devices: Vec<Device>,
    opt_level: Option<OptLevel>,
    input_type: Option<InputType>,
    output_format: Option<OutputFormat>,
    data_tiling: Option<bool>,
    const_expr_hoisting: Option<bool>,
    input_conversions: Vec<(InputConversion, bool)>,
    parameter_imports: Vec<PathBuf>,
    parameter_export: Option<PathBuf>,
    parameter_splat: Option<PathBuf>,
    parameter_export_minimum_size: Option<u64>,
}

impl Options {
    /// Add one executable backend. Multiple targets create a multi-backend module.
    pub fn with_target(mut self, target: Target) -> Self {
        self.targets.push(target);
        self
    }

    /// Add one HAL device specification. Each device becomes a repeated flag.
    pub fn with_device(mut self, device: Device) -> Self {
        self.devices.push(device);
        self
    }

    /// Set global optimization level.
    pub fn with_opt_level(mut self, level: OptLevel) -> Self {
        self.opt_level = Some(level);
        self
    }

    /// Set the input dialect mnemonic.
    pub fn with_input_type(mut self, input_type: InputType) -> Self {
        self.input_type = Some(input_type);
        self
    }

    /// Set the target's output format.
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    /// Override data tiling (`iree-opt-data-tiling`).
    pub fn with_data_tiling(mut self, enabled: bool) -> Self {
        self.data_tiling = Some(enabled);
        self
    }

    /// Override constant-expression hoisting (`iree-opt-const-expr-hoisting`).
    pub fn with_const_expr_hoisting(mut self, enabled: bool) -> Self {
        self.const_expr_hoisting = Some(enabled);
        self
    }

    /// Override an input conversion. The latest setting for that conversion wins.
    pub fn with_input_conversion(mut self, conversion: InputConversion, enabled: bool) -> Self {
        if let Some((_, current)) = self
            .input_conversions
            .iter_mut()
            .find(|(kind, _)| *kind == conversion)
        {
            *current = enabled;
        } else {
            self.input_conversions.push((conversion, enabled));
        }
        self
    }

    /// Add an archive to import (`iree-parameter-import`), preserving insertion order.
    /// Non-UTF-8 paths are rejected by `Session::configure` with [`Error::Path`].
    pub fn with_parameter_import(mut self, path: impl AsRef<Path>) -> Self {
        self.parameter_imports.push(path.as_ref().to_path_buf());
        self
    }

    /// Set the exported parameter archive (`iree-parameter-export`).
    /// Non-UTF-8 paths are rejected by `Session::configure` with [`Error::Path`].
    pub fn with_parameter_export(mut self, path: impl AsRef<Path>) -> Self {
        self.parameter_export = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the splat-value archive (`iree-parameter-splat`), not a boolean flag.
    /// Non-UTF-8 paths are rejected by `Session::configure` with [`Error::Path`].
    pub fn with_parameter_splat(mut self, path: impl AsRef<Path>) -> Self {
        self.parameter_splat = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the minimum constant size in bytes for parameter export.
    /// Values above the upstream signed 64-bit range are rejected during configuration.
    pub fn with_parameter_export_minimum_size(mut self, bytes: u64) -> Self {
        self.parameter_export_minimum_size = Some(bytes);
        self
    }

    pub(crate) fn flags(&self) -> Result<(Vec<String>, Vec<String>)> {
        let mut backends = Vec::new();
        for target in &self.targets {
            let backend = target.backend();
            if backend.is_empty() || backend.contains(',') || backend.contains('=') {
                return Err(Error::InvalidOptions(format!(
                    "invalid backend name {backend:?}"
                )));
            }
            if backends.iter().any(|existing| existing == backend) {
                return Err(Error::InvalidOptions(format!(
                    "backend {backend:?} was configured more than once"
                )));
            }
            backends.push(backend.to_owned());
        }
        let mut flags = Vec::new();
        if !backends.is_empty() {
            flags.push(format!("--iree-hal-target-backends={}", backends.join(",")));
        }
        for target in &self.targets {
            target.append_flags(&mut flags);
        }
        for device in &self.devices {
            flags.push(format!("--iree-hal-target-device={}", device.flag_value()?));
        }
        if let Some(level) = self.opt_level {
            flags.push(format!("--iree-opt-level={}", level.flag_value()));
        }
        if let Some(input_type) = &self.input_type {
            let mnemonic = input_type.flag_value();
            if mnemonic.is_empty() {
                return Err(Error::InvalidOptions(
                    "input type mnemonic cannot be empty".into(),
                ));
            }
            flags.push(format!("--iree-input-type={mnemonic}"));
        }
        if let Some(format) = self.output_format {
            let (flag, value) = match format {
                OutputFormat::VmBytecode(value) => {
                    ("iree-vm-bytecode-module-output-format", value.flag_value())
                }
                OutputFormat::VmC(value) => ("iree-vm-c-module-output-format", value.flag_value()),
            };
            flags.push(format!("--{flag}={value}"));
        }
        if let Some(enabled) = self.data_tiling {
            flags.push(format!("--iree-opt-data-tiling={enabled}"));
        }
        if let Some(enabled) = self.const_expr_hoisting {
            flags.push(format!("--iree-opt-const-expr-hoisting={enabled}"));
        }
        for (conversion, enabled) in &self.input_conversions {
            flags.push(format!("--{}={enabled}", conversion.flag_name()));
        }
        for path in &self.parameter_imports {
            append_path(&mut flags, "iree-parameter-import", path)?;
        }
        if let Some(path) = &self.parameter_export {
            append_path(&mut flags, "iree-parameter-export", path)?;
        }
        if let Some(path) = &self.parameter_splat {
            append_path(&mut flags, "iree-parameter-splat", path)?;
        }
        if let Some(bytes) = self.parameter_export_minimum_size {
            let bytes = i64::try_from(bytes).map_err(|_| {
                Error::InvalidOptions("parameter export minimum size exceeds i64::MAX".into())
            })?;
            flags.push(format!("--iree-parameter-export-minimum-size={bytes}"));
        }
        Ok((flags, backends))
    }
}

fn append_path(flags: &mut Vec<String>, name: &str, path: &Path) -> Result<()> {
    let path = path.to_str().ok_or(Error::Path)?;
    if path.contains('\0') {
        return Err(Error::Nul);
    }
    flags.push(format!("--{name}={path}"));
    Ok(())
}

fn append_optional(flags: &mut Vec<String>, name: &str, value: &Option<String>) {
    if let Some(value) = value {
        flags.push(format!("--{name}={value}"));
    }
}

fn validate_alias(alias: &str) -> Result<()> {
    if alias.is_empty()
        || alias
            .bytes()
            .any(|byte| matches!(byte, b'\0' | b',' | b'[' | b']' | b'=' | b'"'))
    {
        return Err(Error::InvalidOptions(
            "device alias is empty or contains target-spec syntax".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_and_device_flags_keep_distinct_selection_models() {
        let (flags, backends) = Options::default()
            .with_target(Target::Vmvx)
            .with_target(Target::VulkanSpirv {
                target: Some("rdna3".into()),
                features: None,
            })
            .with_device(Device::Named {
                name: "gpu".into(),
                alias: "vulkan".into(),
                ordinal: Some(0),
            })
            .flags()
            .unwrap();
        assert_eq!(backends, ["vmvx", "vulkan-spirv"]);
        assert!(flags.contains(&"--iree-hal-target-backends=vmvx,vulkan-spirv".into()));
        assert!(flags.contains(&"--iree-hal-target-device=gpu=vulkan[0]".into()));
        assert!(flags.contains(&"--iree-vulkan-target=rdna3".into()));
    }

    #[test]
    fn duplicate_backends_are_rejected_before_application() {
        let result = Options::default()
            .with_target(Target::Vmvx)
            .with_target(Target::Vmvx)
            .flags();
        assert!(matches!(result, Err(Error::InvalidOptions(_))));
    }

    #[test]
    fn optimization_overrides_emit_explicit_booleans() {
        for enabled in [false, true] {
            let (flags, _) = Options::default()
                .with_data_tiling(enabled)
                .with_const_expr_hoisting(enabled)
                .flags()
                .unwrap();
            assert_eq!(
                flags,
                [
                    format!("--iree-opt-data-tiling={enabled}"),
                    format!("--iree-opt-const-expr-hoisting={enabled}"),
                ]
            );
        }
        assert!(Options::default().flags().unwrap().0.is_empty());
    }

    #[test]
    fn every_input_conversion_emits_explicit_booleans() {
        let conversions = [
            (
                InputConversion::DemoteI64ToI32,
                "iree-input-demote-i64-to-i32",
            ),
            (
                InputConversion::DemoteF32ToF16,
                "iree-input-demote-f32-to-f16",
            ),
            (
                InputConversion::DemoteF64ToF32,
                "iree-input-demote-f64-to-f32",
            ),
            (
                InputConversion::PromoteF16ToF32,
                "iree-input-promote-f16-to-f32",
            ),
            (
                InputConversion::PromoteBf16ToF32,
                "iree-input-promote-bf16-to-f32",
            ),
        ];
        for enabled in [false, true] {
            let mut options = Options::default();
            for (conversion, _) in conversions {
                options = options
                    .with_input_conversion(conversion, !enabled)
                    .with_input_conversion(conversion, enabled);
            }
            let (flags, _) = options.flags().unwrap();
            let expected: Vec<_> = conversions
                .iter()
                .map(|(_, name)| format!("--{name}={enabled}"))
                .collect();
            assert_eq!(flags, expected);
        }
    }

    #[test]
    fn parameter_flags_preserve_paths_and_repeated_imports() {
        let (flags, _) = Options::default()
            .with_parameter_import("first archive.irpa")
            .with_parameter_import(PathBuf::from("second.irpa"))
            .with_parameter_export("output archive.irpa")
            .with_parameter_splat("splat.irpa")
            .with_parameter_export_minimum_size(4096)
            .flags()
            .unwrap();
        assert_eq!(
            flags,
            [
                "--iree-parameter-import=first archive.irpa",
                "--iree-parameter-import=second.irpa",
                "--iree-parameter-export=output archive.irpa",
                "--iree-parameter-splat=splat.irpa",
                "--iree-parameter-export-minimum-size=4096",
            ]
        );
    }

    #[test]
    fn parameter_size_is_checked_against_native_range() {
        for bytes in [0, i64::MAX as u64] {
            assert_eq!(
                Options::default()
                    .with_parameter_export_minimum_size(bytes)
                    .flags()
                    .unwrap()
                    .0,
                [format!("--iree-parameter-export-minimum-size={bytes}")]
            );
        }
        assert!(matches!(
            Options::default()
                .with_parameter_export_minimum_size(i64::MAX as u64 + 1)
                .flags(),
            Err(Error::InvalidOptions(_))
        ));
    }

    #[test]
    fn parameter_paths_reject_nul() {
        assert!(matches!(
            Options::default()
                .with_parameter_import(PathBuf::from("bad\0path"))
                .flags(),
            Err(Error::Nul)
        ));
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn parameter_paths_reject_non_utf8() {
        #[cfg(unix)]
        let path = {
            use std::os::unix::ffi::OsStringExt;
            PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]))
        };
        #[cfg(windows)]
        let path = {
            use std::os::windows::ffi::OsStringExt;
            PathBuf::from(std::ffi::OsString::from_wide(&[0xd800]))
        };
        for options in [
            Options::default().with_parameter_import(path.clone()),
            Options::default().with_parameter_export(path.clone()),
            Options::default().with_parameter_splat(path),
        ] {
            assert!(matches!(options.flags(), Err(Error::Path)));
        }
    }
}
