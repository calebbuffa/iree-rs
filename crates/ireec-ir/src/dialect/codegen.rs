//! # Laws
//! Codegen parameter structs contain only safe Rust data; native parameter
//! records are copied at the FFI boundary and attributes remain context-bound.
//! Module walks return non-owning operations tied to the module borrow, and
//! operation-derived attributes are validated before exposing typed Rust data.

use crate::{
    Attribute, Borrowed, Context, Error, Module, Operation, Result,
    string_ref::{copy_string_ref, string_ref},
};
use iree_native as n;

/// The dispatch lowering pipeline selector represented by Codegen attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispatchLoweringPipeline {
    /// IREE's VMVX pipeline.
    Vmvx,
    /// The transform-dialect pipeline.
    TransformDialect,
    /// Do not select a pipeline.
    None,
    /// A named MLIR pass pipeline.
    Pass(String),
}

/// Rust-owned parameters for an IREE Codegen translation-info attribute.
pub struct TranslationInfoParameters<'ctx> {
    /// Pipeline selector attribute.
    pub pass_pipeline: Attribute<'ctx>,
    /// Optional Codegen specification symbol.
    pub codegen_spec: Option<Attribute<'ctx>>,
    /// Workgroup size dimensions.
    pub workgroup_size: Vec<i64>,
    /// Optional subgroup size; zero is represented as `None`.
    pub subgroup_size: Option<i64>,
    /// Optional configuration dictionary.
    pub configuration: Option<Attribute<'ctx>>,
}

/// Rust-owned parameters for an IREE Codegen compilation-info attribute.
pub struct CompilationInfoParameters<'ctx> {
    /// GPU or other dialect lowering configuration.
    pub lowering_config: Attribute<'ctx>,
    /// Translation pipeline information.
    pub translation_info: Attribute<'ctx>,
}

/// A typed dispatch-lowering pipeline attribute.
pub struct DispatchLoweringPipelineAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// A typed Codegen translation-info attribute.
pub struct TranslationInfoAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// A typed Codegen compilation-info attribute.
pub struct CompilationInfoAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// A typed Codegen root-operation attribute.
pub struct RootOpAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// A typed Codegen integer-knob attribute.
pub struct IntKnobAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// A typed Codegen one-of-knob attribute.
pub struct OneOfKnobAttr<'ctx> {
    attribute: Attribute<'ctx>,
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Construct a Codegen dispatch-lowering pipeline attribute.
    pub fn dispatch_lowering_pipeline<'a>(
        &'a self,
        pipeline: &DispatchLoweringPipeline,
    ) -> Result<DispatchLoweringPipelineAttr<'a>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe {
            match pipeline {
                DispatchLoweringPipeline::Vmvx => n::ireeCodegenVMVXPipelineAttrGet(self.raw),
                DispatchLoweringPipeline::TransformDialect => {
                    n::ireeCodegenTransformDialectCodegenPipelineAttrGet(self.raw)
                }
                DispatchLoweringPipeline::None => n::ireeCodegenNoPipelineAttrGet(self.raw),
                DispatchLoweringPipeline::Pass(text) => {
                    n::ireeCodegenPassPipelineAttrGet(self.raw, string_ref(text))
                }
            }
        };
        Ok(DispatchLoweringPipelineAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }

    /// Construct translation info from safe Rust parameter values.
    pub fn translation_info<'a>(
        &'a self,
        parameters: &TranslationInfoParameters<'_>,
    ) -> Result<TranslationInfoAttr<'a>> {
        ensure_context(self, &parameters.pass_pipeline)?;
        for attribute in parameters
            .codegen_spec
            .iter()
            .chain(parameters.configuration.iter())
        {
            ensure_context(self, attribute)?;
        }
        let workgroup_count = parameters.workgroup_size.len();
        let workgroup_size = if workgroup_count == 0 {
            std::ptr::null()
        } else {
            parameters.workgroup_size.as_ptr()
        };
        let native = n::ireeCodegenTranslationInfoParameters {
            passPipeline: parameters.pass_pipeline.raw,
            codegenSpec: parameters
                .codegen_spec
                .as_ref()
                .map_or_else(n::MlirAttribute::default, |value| value.raw),
            workgroupSize: workgroup_size,
            numWorkgroupSizeElements: workgroup_count,
            subgroupSize: parameters.subgroup_size.unwrap_or_default(),
            configuration: parameters
                .configuration
                .as_ref()
                .map_or_else(n::MlirAttribute::default, |value| value.raw),
        };
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenTranslationInfoAttrGet`.
        let raw = unsafe { n::ireeCodegenTranslationInfoAttrGet(self.raw, native) };
        Ok(TranslationInfoAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }

    /// Construct compilation info from lowering and translation attributes.
    pub fn compilation_info<'a>(
        &'a self,
        parameters: &CompilationInfoParameters<'_>,
    ) -> Result<CompilationInfoAttr<'a>> {
        ensure_context(self, &parameters.lowering_config)?;
        ensure_context(self, &parameters.translation_info)?;
        let native = n::ireeCodegenCompilationInfoParameters {
            loweringConfig: parameters.lowering_config.raw,
            translationInfo: parameters.translation_info.raw,
        };
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenCompilationInfoAttrGet`.
        let raw = unsafe { n::ireeCodegenCompilationInfoAttrGet(self.raw, native) };
        Ok(CompilationInfoAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }

    /// Construct a Codegen root-op marker.
    pub fn root_op<'a>(&'a self, set: i64) -> Result<RootOpAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenRootOpAttrGet`.
        let raw = unsafe { n::ireeCodegenRootOpAttrGet(self.raw, set) };
        Ok(RootOpAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
}

fn ensure_context<Ownership>(
    context: &Context<'_, Ownership>,
    attribute: &Attribute<'_>,
) -> Result<()> {
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirAttributeGetType`.
    let ty = unsafe { n::mlirAttributeGetType(attribute.raw) };
    // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeGetContext`.
    if unsafe { n::mlirTypeGetContext(ty) }.ptr != context.raw.ptr {
        return Err(Error::Invalid(
            "Codegen attribute belongs to another context".into(),
        ));
    }
    Ok(())
}

fn related_attribute<'ctx>(
    _source: &Attribute<'ctx>,
    raw: n::MlirAttribute,
) -> Result<Attribute<'ctx>> {
    // MLIR returns this attribute from a field of source, so both share its
    // context and lifetime.
    Attribute::checked(raw)
}

impl<'ctx> DispatchLoweringPipelineAttr<'ctx> {
    /// Validate and wrap a generic dispatch-pipeline attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        let raw = attribute.raw;
        // SAFETY: `raw` was returned by MLIR and remains live for `ireeAttributeIsACodegenVMVXPipelineAttr`.
        let matches = unsafe {
            n::ireeAttributeIsACodegenVMVXPipelineAttr(raw)
                || n::ireeAttributeIsACodegenTransformDialectCodegenPipelineAttr(raw)
                || n::ireeAttributeIsACodegenNoPipelineAttr(raw)
                || n::ireeAttributeIsACodegenPassPipelineAttr(raw)
        };
        if !matches {
            return Err(Error::Invalid(
                "attribute is not a Codegen dispatch pipeline".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Decode the native pipeline selector.
    pub fn pipeline(&self) -> Result<DispatchLoweringPipeline> {
        let raw = self.attribute.raw;
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        unsafe {
            if n::ireeAttributeIsACodegenVMVXPipelineAttr(raw) {
                Ok(DispatchLoweringPipeline::Vmvx)
            } else if n::ireeAttributeIsACodegenTransformDialectCodegenPipelineAttr(raw) {
                Ok(DispatchLoweringPipeline::TransformDialect)
            } else if n::ireeAttributeIsACodegenNoPipelineAttr(raw) {
                Ok(DispatchLoweringPipeline::None)
            } else if n::ireeAttributeIsACodegenPassPipelineAttr(raw) {
                Ok(DispatchLoweringPipeline::Pass(copy_string_ref(
                    n::ireeCodegenPassPipelineAttrGetPipeline(raw),
                )?))
            } else {
                Err(Error::Invalid("attribute is not a Codegen pipeline".into()))
            }
        }
    }
}

impl<'ctx> TranslationInfoAttr<'ctx> {
    /// Validate and wrap a translation-info attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsACodegenTranslationInfoAttr`.
        if !unsafe { n::ireeAttributeIsACodegenTranslationInfoAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not Codegen translation info".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Copy the native parameters into Rust-owned values.
    pub fn parameters(&self) -> Result<TranslationInfoParameters<'_>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenTranslationInfoAttrGetParameters`.
        let raw = unsafe { n::ireeCodegenTranslationInfoAttrGetParameters(self.attribute.raw) };
        if raw.numWorkgroupSizeElements > isize::MAX as usize / std::mem::size_of::<i64>() {
            return Err(Error::Invalid(
                "workgroup size exceeds the maximum Rust slice size".into(),
            ));
        }
        if raw.numWorkgroupSizeElements > 0 && raw.workgroupSize.is_null() {
            return Err(Error::Null("Codegen workgroup sizes"));
        }
        let workgroup_size = if raw.numWorkgroupSizeElements == 0 {
            Vec::new()
        } else {
            // SAFETY: the callback rejects null pointers and oversized lengths before forming this borrowed slice.
            unsafe { n::slice(raw.workgroupSize, raw.numWorkgroupSizeElements) }.to_vec()
        };
        let pass_pipeline = related_attribute(&self.attribute, raw.passPipeline)?;
        let codegen_spec = if raw.codegenSpec.ptr.is_null() {
            None
        } else {
            Some(related_attribute(&self.attribute, raw.codegenSpec)?)
        };
        let configuration = if raw.configuration.ptr.is_null() {
            None
        } else {
            Some(related_attribute(&self.attribute, raw.configuration)?)
        };
        Ok(TranslationInfoParameters {
            pass_pipeline,
            codegen_spec,
            workgroup_size,
            subgroup_size: (raw.subgroupSize != 0).then_some(raw.subgroupSize),
            configuration,
        })
    }
}

impl<'ctx> CompilationInfoAttr<'ctx> {
    /// Validate and wrap a compilation-info attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsACodegenCompilationInfoAttr`.
        if !unsafe { n::ireeAttributeIsACodegenCompilationInfoAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not Codegen compilation info".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Copy the native compilation-info parameters.
    pub fn parameters(&self) -> Result<CompilationInfoParameters<'_>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenCompilationInfoAttrGetParameters`.
        let raw = unsafe { n::ireeCodegenCompilationInfoAttrGetParameters(self.attribute.raw) };
        let lowering_config = related_attribute(&self.attribute, raw.loweringConfig)?;
        let translation_info = related_attribute(&self.attribute, raw.translationInfo)?;
        Ok(CompilationInfoParameters {
            lowering_config,
            translation_info,
        })
    }
}
impl<'ctx> RootOpAttr<'ctx> {
    /// Validate and wrap a root-operation marker.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsACodegenRootOpAttr`.
        if !unsafe { n::ireeAttributeIsACodegenRootOpAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not a Codegen root-op marker".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Read the root-op set identifier.
    pub fn set(&self) -> i64 {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenRootOpAttrGetSet`.
        unsafe { n::ireeCodegenRootOpAttrGetSet(self.attribute.raw) }
    }
}
impl<'ctx> IntKnobAttr<'ctx> {
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Validate and wrap an integer-knob attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsACodegenIntKnobAttr`.
        if !unsafe { n::ireeAttributeIsACodegenIntKnobAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not a Codegen integer knob".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the attribute containing this knob's name.
    pub fn name(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenIntKnobAttrGetName`.
        let raw = unsafe { n::ireeCodegenIntKnobAttrGetName(self.attribute.raw) };
        string_attribute(raw)
    }
}
impl<'ctx> OneOfKnobAttr<'ctx> {
    /// Borrow the generic attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Validate and wrap a one-of-knob attribute.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsACodegenOneOfKnobAttr`.
        if !unsafe { n::ireeAttributeIsACodegenOneOfKnobAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not a Codegen one-of knob".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the attribute containing this knob's name.
    pub fn name(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenOneOfKnobAttrGetName`.
        let raw = unsafe { n::ireeCodegenOneOfKnobAttrGetName(self.attribute.raw) };
        string_attribute(raw)
    }
    /// Copy the options accepted by this one-of knob.
    pub fn options(&self) -> Result<Vec<Attribute<'_>>> {
        let mut count = 0isize;
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenOneOfKnobAttrGetOptions`.
        unsafe {
            n::ireeCodegenOneOfKnobAttrGetOptions(
                self.attribute.raw,
                &mut count,
                std::ptr::null_mut(),
            )
        };
        if count < 0 {
            return Err(Error::Invalid(
                "Codegen knob returned a negative option count".into(),
            ));
        }
        let mut raw = vec![n::MlirAttribute::default(); count as usize];
        if count > 0 {
            // SAFETY: `raw` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results.
            unsafe {
                n::ireeCodegenOneOfKnobAttrGetOptions(
                    self.attribute.raw,
                    &mut count,
                    raw.as_mut_ptr(),
                )
            };
        }
        raw.into_iter()
            .map(|value| related_attribute(&self.attribute, value))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn pass_pipeline_preserves_text(text in "[a-zA-Z0-9._-]{0,80}") {
            let pipeline = DispatchLoweringPipeline::Pass(text.clone());
            prop_assert_eq!(pipeline, DispatchLoweringPipeline::Pass(text));
        }
    }
}

/// The dimensions inferred for a Codegen attention operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttentionOpDetail<'ctx> {
    /// Batch dimension expression.
    pub batch: Vec<i64>,
    /// M dimension expression.
    pub m: Vec<i64>,
    /// First K dimension expression.
    pub k1: Vec<i64>,
    /// Second K dimension expression.
    pub k2: Vec<i64>,
    /// N dimension expression.
    pub n: Vec<i64>,
    /// Rank of the inferred iteration domain.
    pub domain_rank: i64,
    _context: std::marker::PhantomData<&'ctx ()>,
}

/// Rust-owned details for an IREE Codegen IGEMM convolution operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IGEMMGenericConvDetails<'ctx> {
    /// Contraction map attribute entries.
    pub contraction_maps: Vec<Attribute<'ctx>>,
    /// Loop bounds.
    pub loop_bounds: Vec<i64>,
    /// Loop iterator names.
    pub loop_iterators: Vec<String>,
    /// Output permutation used by im2col.
    pub im2col_output_permutation: Vec<i64>,
    /// Filter reassociation indices.
    pub filter_reassociation_indices: Vec<Vec<i64>>,
    /// Whether the output channel is first.
    pub output_channel_first: bool,
    /// Convolution-to-IGEMM dimension pairs.
    pub convolution_to_igemm_dimension_map: Vec<(i64, i64)>,
}

/// Dimensions inferred for a scaled contraction operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaledContractionDimensions {
    /// Batch dimensions.
    pub batch: Vec<u32>,
    /// M dimensions.
    pub m: Vec<u32>,
    /// N dimensions.
    pub n: Vec<u32>,
    /// K dimensions.
    pub k: Vec<u32>,
    /// Scale-related K dimensions.
    pub k_b: Vec<u32>,
}

/// An executable variant operation borrowed from an MLIR module.
pub struct ExecutableVariant<'module> {
    operation: Operation<'module, Borrowed>,
}

impl<'module> ExecutableVariant<'module> {
    /// Borrow the underlying executable variant operation.
    pub fn operation(&self) -> &Operation<'module, Borrowed> {
        &self.operation
    }

    /// Read the GPU target information from this variant's `target` attribute.
    pub fn gpu_target_info(&self) -> Result<Option<super::gpu::TargetInfo>> {
        let raw = self.operation.raw;
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let target = unsafe { n::mlirOperationGetAttributeByName(raw, string_ref("target")) };
        if target.ptr.is_null() {
            return Err(Error::Invalid(
                "executable variant has no target attribute".into(),
            ));
        }
        // SAFETY: `target` is a validated attribute handle and remains live while its GPU target info is queried.
        let target_info = unsafe { n::ireeHALExecutableTargetAttrGetGPUTargetInfo(target) };
        super::gpu::decode_target_info(target_info)
    }
}

impl<'ctx> Module<'ctx> {
    /// Collect executable variant operations in this module.
    pub fn executable_variants(&self) -> Result<Vec<ExecutableVariant<'_>>> {
        collect_module_operations(self, true).map(|operations| {
            operations
                .into_iter()
                .map(|operation| ExecutableVariant { operation })
                .collect()
        })
    }

    /// Collect operations marked as tuner roots in this module.
    pub fn tuner_root_operations(&self) -> Result<Vec<Operation<'_, Borrowed>>> {
        collect_module_operations(self, false)
    }
}

fn collect_module_operations<'a>(
    module: &'a Module<'_>,
    executable_variants: bool,
) -> Result<Vec<Operation<'a, Borrowed>>> {
    // SAFETY: `module` is a live native handle; their wrappers keep them valid throughout `ireeCodegenGetExecutableVariantOps`.
    let collect = |count: &mut usize, output: *mut n::MlirOperation| unsafe {
        if executable_variants {
            n::ireeCodegenGetExecutableVariantOps(module.raw, count, output)
        } else {
            n::ireeCodegenGetTunerRootOps(module.raw, count, output)
        }
    };
    let mut count = 0usize;
    collect(&mut count, std::ptr::null_mut());
    let mut raw = vec![n::MlirOperation::default(); count];
    if count > 0 {
        collect(&mut count, raw.as_mut_ptr());
        raw.truncate(count);
    }
    raw.into_iter()
        .map(|operation| {
            if operation.ptr.is_null() {
                return Err(Error::Null("Codegen operation result"));
            }
            Ok(Operation {
                raw: operation,
                owns: false,
                _ownership: std::marker::PhantomData,
                _context: std::marker::PhantomData,
            })
        })
        .collect()
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Infer dimensions from four affine-map attributes for an attention op.
    pub fn codegen_attention_op_detail<'a>(
        &'a self,
        q_map: &Attribute<'_>,
        k_map: &Attribute<'_>,
        v_map: &Attribute<'_>,
        o_map: &Attribute<'_>,
    ) -> Result<Option<AttentionOpDetail<'a>>> {
        for attribute in [q_map, k_map, v_map, o_map] {
            ensure_context(self, attribute)?;
            // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsAAffineMap`.
            if !unsafe { n::mlirAttributeIsAAffineMap(attribute.raw) } {
                return Err(Error::Invalid(
                    "attention dimension inference requires affine-map attributes".into(),
                ));
            }
        }
        // SAFETY: the four affine maps are live handles from the same context validated above and remain borrowed through this call.
        let detail = unsafe {
            n::ireeCodegenGetAttentionOpDetail(
                n::mlirAffineMapAttrGetValue(q_map.raw),
                n::mlirAffineMapAttrGetValue(k_map.raw),
                n::mlirAffineMapAttrGetValue(v_map.raw),
                n::mlirAffineMapAttrGetValue(o_map.raw),
            )
        };
        if detail.domainRank < 0 {
            return Ok(None);
        }
        let batch = integer_array(detail.batch)?;
        let m = integer_array(detail.m)?;
        let k1 = integer_array(detail.k1)?;
        let k2 = integer_array(detail.k2)?;
        let n = integer_array(detail.n)?;
        Ok(Some(AttentionOpDetail {
            batch,
            m,
            k1,
            k2,
            n,
            domain_rank: detail.domainRank,
            _context: std::marker::PhantomData,
        }))
    }
}

impl<'ctx> Operation<'ctx, Borrowed> {
    /// Whether this operation is an IREE Codegen attention operation.
    pub fn is_codegen_attention_op(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenMlirOperationIsACodegenAttentionOp`.
        unsafe { n::ireeCodegenMlirOperationIsACodegenAttentionOp(self.raw) }
    }

    /// Read IGEMM convolution details, if present on this operation.
    pub fn igemm_generic_conv_details(&self) -> Result<Option<IGEMMGenericConvDetails<'_>>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenHasIGEMMGenericConvDetails`.
        if !unsafe { n::ireeCodegenHasIGEMMGenericConvDetails(self.raw) } {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenGetIGEMMGenericConvDetails`.
        let details = unsafe { n::ireeCodegenGetIGEMMGenericConvDetails(self.raw) };
        let contraction_maps = affine_map_attribute_array(details.igemmContractionMaps)?;
        let loop_bounds = integer_array(details.igemmLoopBounds)?;
        let loop_iterators = string_array(details.igemmLoopIterators)?;
        let im2col_output_permutation = integer_array(details.im2colOutputPerm)?;
        let filter_reassociation_indices = nested_integer_array(details.filterReassocIndices)?;
        let convolution_to_igemm_dimension_map = integer_pair_array(details.convToIgemmDimMap)?;
        Ok(Some(IGEMMGenericConvDetails {
            contraction_maps,
            loop_bounds,
            loop_iterators,
            im2col_output_permutation,
            filter_reassociation_indices,
            output_channel_first: details.isOutputChannelFirst,
            convolution_to_igemm_dimension_map,
        }))
    }

    /// Check whether this operation is a scaled contraction.
    ///
    /// # Safety
    /// The native predicate performs an unchecked cast to MLIR's Linalg
    /// structured-operation interface. The caller must guarantee that this is
    /// a live Linalg structured operation; passing another operation is
    /// undefined behavior.
    pub unsafe fn is_scaled_contraction_op_unchecked(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenMlirOperationIsAScaledContractionOp`.
        unsafe { n::ireeCodegenMlirOperationIsAScaledContractionOp(self.raw) }
    }

    /// Infer scaled-contraction dimensions, or return `None` for other ops.
    ///
    /// The native inference API represents unsupported operations with a null
    /// batch attribute and is safe to query for any live operation.
    pub fn scaled_contraction_dimensions(&self) -> Result<Option<ScaledContractionDimensions>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenInferScaledContractionDimensions`.
        let native = unsafe { n::ireeCodegenInferScaledContractionDimensions(self.raw) };
        if native.batch.ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(ScaledContractionDimensions {
            batch: unsigned_array(native.batch)?,
            m: unsigned_array(native.m)?,
            n: unsigned_array(native.n)?,
            k: unsigned_array(native.k)?,
            k_b: unsigned_array(native.kB)?,
        }))
    }

    /// Convert a Codegen constraints operation to SMT-LIB text.
    pub fn codegen_constraints_to_smtlib(&self, emit_reset: bool) -> Result<String> {
        self.ensure_constraints_operation()?;
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCodegenConvertConstraintsOpToSMTLIB`.
        let raw = unsafe { n::ireeCodegenConvertConstraintsOpToSMTLIB(self.raw, emit_reset) };
        string_attribute(raw)
    }

    /// Materialize a configuration attribute from a constraints operation.
    pub fn materialize_codegen_configuration(
        &self,
        attribute_name: &str,
        assignments: &[(&str, i64)],
    ) -> Result<Attribute<'_>> {
        self.ensure_constraints_operation()?;
        if attribute_name.as_bytes().contains(&0)
            || assignments
                .iter()
                .any(|(name, _)| name.as_bytes().contains(&0))
        {
            return Err(Error::Invalid(
                "configuration names must not contain NUL bytes".into(),
            ));
        }
        let assignment_names: Vec<_> = assignments
            .iter()
            .map(|(name, _)| string_ref(name))
            .collect();
        let assignment_values: Vec<_> = assignments.iter().map(|(_, value)| *value).collect();
        let mut diagnostic = n::MlirAttribute::default();
        // SAFETY: `assignment_names` and `assignment_values` remain allocated for the synchronous C call and are read only during the call.
        let raw = unsafe {
            n::ireeCodegenMaterializeConfigurationAttrFromConstraintsOp(
                self.raw,
                string_ref(attribute_name),
                assignment_names.len(),
                assignment_names.as_ptr(),
                assignment_values.as_ptr(),
                &mut diagnostic,
            )
        };
        if raw.ptr.is_null() {
            if !diagnostic.ptr.is_null() {
                let message = string_attribute(diagnostic)?;
                return Err(Error::Invalid(format!(
                    "configuration materialization failed: {message}"
                )));
            }
            return Err(Error::Invalid(
                "configuration materialization failed".into(),
            ));
        }
        Attribute::checked(raw)
    }

    fn ensure_constraints_operation(&self) -> Result<()> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIdentifierStr`.
        let name = unsafe { n::mlirIdentifierStr(n::mlirOperationGetName(self.raw)) };
        let name = copy_string_ref(name)?;
        if name != CONSTRAINTS_OP {
            return Err(Error::Invalid(format!(
                "expected {CONSTRAINTS_OP} operation, got {name}"
            )));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOperationGetContext`.
        let context = unsafe { n::mlirOperationGetContext(self.raw) };
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        if !unsafe { n::mlirContextIsRegisteredOperation(context, string_ref(CONSTRAINTS_OP)) } {
            return Err(Error::Invalid(
                "Codegen SMT constraints operation is not registered".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOperationVerify`.
        if !unsafe { n::mlirOperationVerify(self.raw) } {
            return Err(Error::Invalid(
                "Codegen SMT constraints operation failed verification".into(),
            ));
        }
        Ok(())
    }
}

fn array_elements(attribute: n::MlirAttribute) -> Result<Vec<n::MlirAttribute>> {
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsAArray`.
    if attribute.ptr.is_null() || !unsafe { n::mlirAttributeIsAArray(attribute) } {
        return Err(Error::Invalid("expected an MLIR array attribute".into()));
    }
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirArrayAttrGetNumElements`.
    let count = unsafe { n::mlirArrayAttrGetNumElements(attribute) };
    if count < 0 {
        return Err(Error::Invalid(
            "MLIR array has a negative element count".into(),
        ));
    }
    (0..count)
        .map(|index| {
            // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirArrayAttrGetElement`.
            let element = unsafe { n::mlirArrayAttrGetElement(attribute, index) };
            if element.ptr.is_null() {
                return Err(Error::Null("MLIR array element"));
            }
            Ok(element)
        })
        .collect()
}

fn affine_map_attribute_array<'ctx>(attribute: n::MlirAttribute) -> Result<Vec<Attribute<'ctx>>> {
    array_elements(attribute)?
        .into_iter()
        .map(|raw| {
            // SAFETY: `raw` was returned by MLIR and remains live for `mlirAttributeIsAAffineMap`.
            if !unsafe { n::mlirAttributeIsAAffineMap(raw) } {
                return Err(Error::Invalid(
                    "expected affine-map attributes in Codegen map array".into(),
                ));
            }
            Attribute::checked(raw)
        })
        .collect()
}

fn integer_array(attribute: n::MlirAttribute) -> Result<Vec<i64>> {
    array_elements(attribute)?
        .into_iter()
        .map(|element| {
            // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsAInteger`.
            if !unsafe { n::mlirAttributeIsAInteger(element) } {
                return Err(Error::Invalid(
                    "expected integer elements in MLIR array".into(),
                ));
            }
            // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirIntegerAttrGetValueInt`.
            Ok(unsafe { n::mlirIntegerAttrGetValueInt(element) })
        })
        .collect()
}

fn unsigned_array(attribute: n::MlirAttribute) -> Result<Vec<u32>> {
    integer_array(attribute)?
        .into_iter()
        .map(|value| {
            u32::try_from(value).map_err(|_| {
                Error::Invalid("scaled-contraction dimension is outside u32 range".into())
            })
        })
        .collect()
}

fn string_array(attribute: n::MlirAttribute) -> Result<Vec<String>> {
    array_elements(attribute)?
        .into_iter()
        .map(string_attribute)
        .collect()
}

fn string_attribute(raw: n::MlirAttribute) -> Result<String> {
    // SAFETY: `raw` was returned by MLIR and remains live for `mlirAttributeIsAString`.
    if raw.ptr.is_null() || !unsafe { n::mlirAttributeIsAString(raw) } {
        return Err(Error::Invalid("expected an MLIR string attribute".into()));
    }
    // SAFETY: `raw` was returned by MLIR and remains live for `mlirStringAttrGetValue`.
    copy_string_ref(unsafe { n::mlirStringAttrGetValue(raw) })
}

fn nested_integer_array(attribute: n::MlirAttribute) -> Result<Vec<Vec<i64>>> {
    array_elements(attribute)?
        .into_iter()
        .map(integer_array)
        .collect()
}

const CONSTRAINTS_OP: &str = "iree_codegen.smt.constraints";

fn integer_pair_array(attribute: n::MlirAttribute) -> Result<Vec<(i64, i64)>> {
    nested_integer_array(attribute)?
        .into_iter()
        .map(|pair| {
            if pair.len() != 2 {
                return Err(Error::Invalid(
                    "expected two dimensions in Codegen dimension map entry".into(),
                ));
            }
            Ok((pair[0], pair[1]))
        })
        .collect()
}
