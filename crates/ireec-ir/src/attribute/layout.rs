//! Strided memory layout attributes.
use super::Attribute;
use crate::{Context, Result, types::count};
use iree_native as n;

typed!(
    StridedLayoutAttr,
    mlirAttributeIsAStridedLayout,
    "A memory layout with offset and strides."
);

impl<'c> StridedLayoutAttr<'c> {
    /// Construct a layout, preserving signed strides and dynamic sentinels.
    pub fn new<O>(context: &'c Context<'_, O>, offset: i64, strides: &[i64]) -> Result<Self> {
        // SAFETY: `strides` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirStridedLayoutAttrGet(
                context.raw,
                offset,
                count(strides.len())?,
                strides.as_ptr(),
            )
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Layout offset.
    pub fn offset(&self) -> i64 {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirStridedLayoutAttrGetOffset`.
        unsafe { n::mlirStridedLayoutAttrGetOffset(self.raw) }
    }
    /// Strides in dimension order.
    pub fn strides(&self) -> Vec<i64> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirStridedLayoutAttrGetNumStrides`.
        let count = unsafe { n::mlirStridedLayoutAttrGetNumStrides(self.raw) };
        (0..count)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirStridedLayoutAttrGetStride`.
            .map(|i| unsafe { n::mlirStridedLayoutAttrGetStride(self.raw, i) })
            .collect()
    }
}
