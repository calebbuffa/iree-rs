//! GPU lowering pipeline selectors, options, and attributes.
use crate::{Attribute, Context, Error, Result};
use iree_native as n;

/// Optional settings for the GPU pipeline.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PipelineOptions {
    /// Number of prefetch stages, when specified.
    pub prefetch_num_stages: Option<i64>,
    /// Disable shared-memory bank conflict reduction.
    pub no_reduce_shared_memory_bank_conflicts: Option<bool>,
    /// Use implicit-GEMM convolution lowering.
    pub use_igemm_convolution: Option<bool>,
    /// Workgroup reorder strategy discriminant.
    pub reorder_workgroups_strategy: Option<ReorderWorkgroupsStrategy>,
}
/// GPU pipeline selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LoweringPipeline {
    /// Target-selected lowering.
    Default = 0,
    /// Base lowering.
    BaseLowering = 1,
    /// Distribute across GPU threads.
    Distribute = 2,
    /// Vectorize operations.
    Vectorize = 3,
    /// Distribute vectorized work.
    VectorDistribute = 4,
    /// Winograd vectorization.
    WinogradVectorize = 5,
    /// Tile and fuse.
    TileAndFuse = 6,
}
/// GPU workgroup-ordering strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ReorderWorkgroupsStrategy {
    /// Preserve order.
    None = 0,
    /// Transpose workgroups.
    Transpose = 1,
}
/// Validated GPU pipeline-options attribute.
pub struct PipelineOptionsAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// Validated GPU pipeline attribute.
pub struct PipelineAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// Validated workgroup reorder strategy attribute.
pub struct ReorderWorkgroupsStrategyAttr<'ctx> {
    attribute: Attribute<'ctx>,
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Create a GPU pipeline-options attribute.
    pub fn gpu_pipeline_options<'a>(
        &'a self,
        options: &PipelineOptions,
    ) -> Result<PipelineOptionsAttr<'a>> {
        let mut prefetch = options.prefetch_num_stages.unwrap_or_default();
        let mut no_reduce = options
            .no_reduce_shared_memory_bank_conflicts
            .unwrap_or_default();
        let mut igemm = options.use_igemm_convolution.unwrap_or_default();
        let mut reorder = options
            .reorder_workgroups_strategy
            // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUReorderWorkgroupsStrategyAttrGet`.
            .map(|value| unsafe {
                n::ireeGPUReorderWorkgroupsStrategyAttrGet(self.raw, value as u32)
            })
            .unwrap_or_default();
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUPipelineOptionsAttrGet`.
        let raw = unsafe {
            n::ireeGPUPipelineOptionsAttrGet(
                self.raw,
                if options.prefetch_num_stages.is_some() {
                    &mut prefetch
                } else {
                    std::ptr::null_mut()
                },
                if options.no_reduce_shared_memory_bank_conflicts.is_some() {
                    &mut no_reduce
                } else {
                    std::ptr::null_mut()
                },
                if options.use_igemm_convolution.is_some() {
                    &mut igemm
                } else {
                    std::ptr::null_mut()
                },
                if options.reorder_workgroups_strategy.is_some() {
                    &mut reorder
                } else {
                    std::ptr::null_mut()
                },
            )
        };
        Ok(PipelineOptionsAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
    /// Construct a GPU pipeline attribute.
    pub fn gpu_pipeline<'a>(&'a self, pipeline: LoweringPipeline) -> Result<PipelineAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUPipelineAttrGet`.
        let raw = unsafe { n::ireeGPUPipelineAttrGet(self.raw, pipeline as u32) };
        Ok(PipelineAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
    /// Construct a workgroup reorder strategy attribute.
    pub fn gpu_reorder_workgroups<'a>(
        &'a self,
        strategy: ReorderWorkgroupsStrategy,
    ) -> Result<ReorderWorkgroupsStrategyAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUReorderWorkgroupsStrategyAttrGet`.
        let raw = unsafe { n::ireeGPUReorderWorkgroupsStrategyAttrGet(self.raw, strategy as u32) };
        Ok(ReorderWorkgroupsStrategyAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
}

impl<'ctx> PipelineOptionsAttr<'ctx> {
    /// Validate and wrap a generic GPU pipeline-options attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsAGPUPipelineOptionsAttr`.
        if !unsafe { n::ireeAttributeIsAGPUPipelineOptionsAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not GPU pipeline options".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Decode all present pipeline options.
    pub fn options(&self) -> Result<PipelineOptions> {
        let raw = self.attribute.raw;
        // SAFETY: `raw` was returned by MLIR and remains live for `ireeGPUPipelineOptionsAttrGetPrefetchNumStages`.
        let prefetch = unsafe { n::ireeGPUPipelineOptionsAttrGetPrefetchNumStages(raw) };
        let no_reduce =
// SAFETY: `raw` was returned by MLIR and remains live for `ireeGPUPipelineOptionsAttrGetNoReduceSharedMemoryBankConflicts`.
            unsafe { n::ireeGPUPipelineOptionsAttrGetNoReduceSharedMemoryBankConflicts(raw) };
        // SAFETY: `raw` was returned by MLIR and remains live for `ireeGPUPipelineOptionsAttrGetUseIgemmConvolution`.
        let igemm = unsafe { n::ireeGPUPipelineOptionsAttrGetUseIgemmConvolution(raw) };
        // SAFETY: `raw` was returned by MLIR and remains live for `ireeGPUPipelineOptionsAttrGetReorderWorkgroupsStrategy`.
        let reorder = unsafe { n::ireeGPUPipelineOptionsAttrGetReorderWorkgroupsStrategy(raw) };
        let bool_value = |value: n::MlirAttribute| -> Result<Option<bool>> {
            if value.ptr.is_null() {
                return Ok(None);
            }
            // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsABool`.
            if unsafe { n::mlirAttributeIsABool(value) } {
                // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirBoolAttrGetValue`.
                Ok(Some(unsafe { n::mlirBoolAttrGetValue(value) }))
            } else {
                Err(Error::Invalid(
                    "pipeline option is not a boolean attribute".into(),
                ))
            }
        };
        let prefetch_num_stages = if prefetch.ptr.is_null() {
            None
        } else {
            // SAFETY: `prefetch` is a live attribute handle returned by the configuration and is inspected synchronously.
            if !unsafe { n::mlirAttributeIsAInteger(prefetch) } {
                return Err(Error::Invalid(
                    "prefetch stages is not an integer attribute".into(),
                ));
            }
            // SAFETY: the preceding kind check establishes `prefetch` is an integer attribute, and its owner remains live.
            Some(unsafe { n::mlirIntegerAttrGetValueInt(prefetch) })
        };
        let reorder_workgroups_strategy = if reorder.ptr.is_null() {
            None
        } else {
            // SAFETY: `prefetch` is a live strategy attribute whose native value is read before the wrapper is dropped.
            Some(ReorderWorkgroupsStrategy::try_from(unsafe {
                n::ireeGPUReorderWorkgroupsStrategyAttrGetValue(reorder)
            })?)
        };
        Ok(PipelineOptions {
            prefetch_num_stages,
            no_reduce_shared_memory_bank_conflicts: bool_value(no_reduce)?,
            use_igemm_convolution: bool_value(igemm)?,
            reorder_workgroups_strategy,
        })
    }
}
impl<'ctx> PipelineAttr<'ctx> {
    /// Validate and wrap a generic GPU pipeline attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsAGPUPipelineAttr`.
        if !unsafe { n::ireeAttributeIsAGPUPipelineAttr(attribute.raw) } {
            return Err(Error::Invalid("attribute is not a GPU pipeline".into()));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Decode the lowering pipeline.
    pub fn value(&self) -> Result<LoweringPipeline> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUPipelineAttrGetValue`.
        LoweringPipeline::try_from(unsafe { n::ireeGPUPipelineAttrGetValue(self.attribute.raw) })
    }
}
impl<'ctx> ReorderWorkgroupsStrategyAttr<'ctx> {
    /// Validate and wrap a generic strategy attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsAGPUReorderWorkgroupsStrategyAttr`.
        if !unsafe { n::ireeAttributeIsAGPUReorderWorkgroupsStrategyAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not a workgroup reorder strategy".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Decode the reorder strategy.
    pub fn value(&self) -> Result<ReorderWorkgroupsStrategy> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUReorderWorkgroupsStrategyAttrGetValue`.
        ReorderWorkgroupsStrategy::try_from(unsafe {
            n::ireeGPUReorderWorkgroupsStrategyAttrGetValue(self.attribute.raw)
        })
    }
}
impl TryFrom<u32> for LoweringPipeline {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::BaseLowering),
            2 => Ok(Self::Distribute),
            3 => Ok(Self::Vectorize),
            4 => Ok(Self::VectorDistribute),
            5 => Ok(Self::WinogradVectorize),
            6 => Ok(Self::TileAndFuse),
            _ => Err(Error::Invalid(format!("unknown GPU pipeline {value}"))),
        }
    }
}
impl TryFrom<u32> for ReorderWorkgroupsStrategy {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Transpose),
            _ => Err(Error::Invalid(format!(
                "unknown workgroup reorder strategy {value}"
            ))),
        }
    }
}
