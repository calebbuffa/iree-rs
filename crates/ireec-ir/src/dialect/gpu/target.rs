//! GPU target capabilities and normalized target information.
use super::mma::{MmaIntrinsic, VirtualMmaIntrinsic, integer_array};
use crate::{Context, Error, Result, string_ref::copy_string_ref};
use iree_native as n;
use std::ffi::CString;

/// Hardware or virtual intrinsic supported by a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetMmaIntrinsic {
    /// Hardware MMA intrinsic.
    Mma(MmaIntrinsic),
    /// Virtual MMA intrinsic.
    Virtual(VirtualMmaIntrinsic),
}
/// GPU target properties normalized through IREE's target-info constructor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TargetInfo {
    /// Architecture identifier.
    pub arch: String,
    /// Supported subgroup sizes.
    pub subgroup_size_choices: Vec<i32>,
    /// Maximum workgroup sizes for each dimension.
    pub max_workgroup_sizes: Vec<i32>,
    /// Maximum threads in a workgroup.
    pub max_thread_count_per_workgroup: i32,
    /// Maximum workgroup memory bytes.
    pub max_workgroup_memory_bytes: i32,
    /// Target-supported MMA intrinsics.
    pub mma_intrinsics: Vec<TargetMmaIntrinsic>,
    /// Number of workgroup processors.
    pub wgp_count: u32,
    /// SIMD units per workgroup processor.
    pub simds_per_wgp: i32,
}

fn decode_i32_array(attribute: n::MlirAttribute, label: &str) -> Result<Vec<i32>> {
    integer_array(attribute, label)?
        .into_iter()
        .map(|value| {
            i32::try_from(value).map_err(|_| Error::Invalid(format!("{label} element exceeds i32")))
        })
        .collect()
}

pub(crate) fn decode_target_info(raw: n::ireeGPUTargetInfo) -> Result<Option<TargetInfo>> {
    if raw.arch.ptr.is_null() {
        return Ok(None);
    }
    // SAFETY: `raw` was returned by MLIR and remains live for `mlirIdentifierStr`.
    let arch = copy_string_ref(unsafe { n::mlirIdentifierStr(raw.arch) })?;
    let count = if raw.mmaIntrinsics.ptr.is_null() {
        0
    } else {
        // SAFETY: `raw` was returned by MLIR and remains live for `mlirArrayAttrGetNumElements`.
        unsafe { n::mlirArrayAttrGetNumElements(raw.mmaIntrinsics) }
    };
    if count < 0 {
        return Err(Error::Invalid(
            "GPU target MMA array has negative length".into(),
        ));
    }
    let count = usize::try_from(count)
        .map_err(|_| Error::Invalid("GPU target MMA array is too large".into()))?;
    let mut values = vec![0u32; count];
    let mut tags = vec![0u8; count];
    if count > 0 {
        // SAFETY: `values` and `tags` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results.
        unsafe {
            n::ireeGPUTargetInfoGetMMAIntrinsics(
                raw.mmaIntrinsics,
                values.as_mut_ptr(),
                tags.as_mut_ptr(),
            )
        };
    }
    let mma_intrinsics = values
        .into_iter()
        .zip(tags)
        .map(|(value, tag)| match tag {
            0 => MmaIntrinsic::from_value(value).map(TargetMmaIntrinsic::Mma),
            1 => VirtualMmaIntrinsic::from_value(value).map(TargetMmaIntrinsic::Virtual),
            _ => Err(Error::Invalid(format!("unknown target MMA tag {tag}"))),
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Some(TargetInfo {
        arch,
        subgroup_size_choices: decode_i32_array(raw.subgroupSizeChoices, "subgroup size choices")?,
        max_workgroup_sizes: decode_i32_array(raw.maxWorkgroupSizes, "maximum workgroup sizes")?,
        max_thread_count_per_workgroup: raw.maxThreadCountPerWorkgroup,
        max_workgroup_memory_bytes: raw.maxWorkgroupMemoryBytes,
        mma_intrinsics,
        wgp_count: raw.wgpCount,
        simds_per_wgp: raw.simdsPerWgp,
    }))
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Construct and normalize GPU target information.
    pub fn target_info(&self, target: &TargetInfo) -> Result<TargetInfo> {
        let arch = CString::new(target.arch.as_str())
            .map_err(|_| Error::Invalid("GPU architecture contains NUL".into()))?;
        let intrinsics = target
            .mma_intrinsics
            .iter()
            .map(|value| match value {
                TargetMmaIntrinsic::Mma(value) => *value as u32,
                TargetMmaIntrinsic::Virtual(value) => *value as u32,
            })
            .collect::<Vec<_>>();
        // SAFETY: `arch`, `subgroup_size_choices`, `max_workgroup_sizes` and `intrinsics` remain allocated for the synchronous C call and are read only during the call.
        let native = unsafe {
            n::ireeGPUTargetInfoGet(
                self.raw,
                arch.as_ptr(),
                target.subgroup_size_choices.as_ptr(),
                target.subgroup_size_choices.len(),
                target.max_workgroup_sizes.as_ptr(),
                target.max_workgroup_sizes.len(),
                target.max_thread_count_per_workgroup,
                target.max_workgroup_memory_bytes,
                target.wgp_count,
                target.simds_per_wgp,
                intrinsics.as_ptr(),
                intrinsics.len(),
            )
        };
        decode_target_info(native)?
            .ok_or_else(|| Error::Invalid("target info has no GPU architecture".into()))
    }
}
