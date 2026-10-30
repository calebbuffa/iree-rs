//! GPU lowering configuration, tile sizes, and subgroup distribution.
use super::mma::integer_array;
use crate::{Attribute, Context, Error, Result};
use iree_native as n;

/// GPU lowering configuration attribute.
pub struct LoweringConfigAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// Workgroup and reduction tile attributes.
pub struct TileSizes {
    /// Workgroup tile sizes, when present.
    pub workgroup: Option<Vec<i64>>,
    /// Reduction tile sizes, when present.
    pub reduction: Option<Vec<i64>>,
}
/// Subgroup distribution basis.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SubgroupBasis {
    /// Subgroup counts by dimension.
    pub counts: Vec<i64>,
    /// Iteration-dimension mapping.
    pub mapping: Vec<i64>,
}
impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Construct a lowering configuration from a dictionary attribute.
    pub fn gpu_lowering_config<'a>(
        &'a self,
        attributes: &Attribute<'_>,
    ) -> Result<LoweringConfigAttr<'a>> {
        // SAFETY: `attributes` is a live dictionary handle; the nested type and context queries are synchronous.
        if unsafe { n::mlirTypeGetContext(n::mlirAttributeGetType(attributes.raw)) }.ptr
            != self.raw.ptr
        {
            return Err(Error::Invalid(
                "lowering configuration belongs to another context".into(),
            ));
        }
        // SAFETY: `attributes` is a live attribute handle retained while its dictionary kind is checked.
        if !unsafe { n::mlirAttributeIsADictionary(attributes.raw) } {
            return Err(Error::Invalid(
                "lowering configuration requires a dictionary".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPULoweringConfigAttrGet`.
        let raw = unsafe { n::ireeGPULoweringConfigAttrGet(self.raw, attributes.raw) };
        Ok(LoweringConfigAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
}

impl<'ctx> LoweringConfigAttr<'ctx> {
    /// Validate and wrap a generic lowering configuration.
    pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
        // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeAttributeIsAGPULoweringConfigAttr`.
        if !unsafe { n::ireeAttributeIsAGPULoweringConfigAttr(attribute.raw) } {
            return Err(Error::Invalid(
                "attribute is not a GPU lowering configuration".into(),
            ));
        }
        Ok(Self { attribute })
    }
    /// Borrow the generic configuration attribute.
    pub fn attribute(&self) -> &Attribute<'ctx> {
        &self.attribute
    }
    /// Get its dictionary value.
    pub fn attributes(&self) -> Result<Attribute<'ctx>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPULoweringConfigAttrGetAttributes`.
        let raw = unsafe { n::ireeGPULoweringConfigAttrGetAttributes(self.attribute.raw) };
        related_attribute(&self.attribute, raw)
    }
    /// Retrieve workgroup and reduction tile attributes.
    pub fn tile_sizes(&self) -> Result<TileSizes> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPULoweringConfigAttrGetTileSizes`.
        let values = unsafe { n::ireeGPULoweringConfigAttrGetTileSizes(self.attribute.raw) };
        Ok(TileSizes {
            workgroup: if values.workgroupAttr.ptr.is_null() {
                None
            } else {
                Some(integer_array(values.workgroupAttr, "workgroup tile sizes")?)
            },
            reduction: if values.reductionAttr.ptr.is_null() {
                None
            } else {
                Some(integer_array(values.reductionAttr, "reduction tile sizes")?)
            },
        })
    }
    /// Retrieve subgroup basis attributes when present.
    pub fn subgroup_basis(&self) -> Result<Option<SubgroupBasis>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPULoweringConfigAttrGetSubgroupBasis`.
        let values = unsafe { n::ireeGPULoweringConfigAttrGetSubgroupBasis(self.attribute.raw) };
        if values.countsAttr.ptr.is_null() || values.mappingAttr.ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(SubgroupBasis {
            counts: integer_array(values.countsAttr, "subgroup basis counts")?,
            mapping: integer_array(values.mappingAttr, "subgroup basis mapping")?,
        }))
    }
    /// Get the optional MMA-kind attribute.
    pub fn mma_kind(&self) -> Result<Option<Attribute<'ctx>>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPULoweringConfigAttrGetMmaKind`.
        let raw = unsafe { n::ireeGPULoweringConfigAttrGetMmaKind(self.attribute.raw) };
        if raw.ptr.is_null() {
            Ok(None)
        } else {
            Ok(Some(related_attribute(&self.attribute, raw)?))
        }
    }
}

fn related_attribute<'ctx>(
    _source: &Attribute<'ctx>,
    raw: n::MlirAttribute,
) -> Result<Attribute<'ctx>> {
    // MLIR returns this attribute from a field of source, so both share its
    // context and lifetime.
    Attribute::checked(raw)
}
