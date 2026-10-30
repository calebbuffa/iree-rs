//! Affine-map and integer-set attributes.
use super::Attribute;
use crate::{
    Result,
    affine::{AffineMap, IntegerSet},
};
use iree_native as n;

typed!(
    AffineMapAttr,
    mlirAttributeIsAAffineMap,
    "An attribute containing an affine map."
);
typed!(
    IntegerSetAttr,
    mlirAttributeIsAIntegerSet,
    "An attribute containing an integer set."
);
impl<'c> AffineMapAttr<'c> {
    /// Wrap an affine map as an attribute.
    pub fn new(map: AffineMap<'c>) -> Result<Self> {
        // SAFETY: `map` is a live native handle; their wrappers keep them valid throughout `mlirAffineMapAttrGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirAffineMapAttrGet(map.raw)
        })?))
    }
    /// Contained affine map.
    pub fn value(&self) -> Result<AffineMap<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapAttrGetValue`.
        AffineMap::checked(unsafe { n::mlirAffineMapAttrGetValue(self.raw) })
    }
}

impl<'c> IntegerSetAttr<'c> {
    /// Wrap an integer set as an attribute.
    pub fn new(set: IntegerSet<'c>) -> Result<Self> {
        // SAFETY: `set` is a live native handle; their wrappers keep them valid throughout `mlirIntegerSetAttrGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirIntegerSetAttrGet(set.raw)
        })?))
    }
    /// Contained integer set.
    pub fn value(&self) -> Result<IntegerSet<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetAttrGetValue`.
        IntegerSet::checked(unsafe { n::mlirIntegerSetAttrGetValue(self.raw) })
    }
}
