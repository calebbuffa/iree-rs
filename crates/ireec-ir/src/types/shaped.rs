//! Laws: ranked shapes preserve dimension order; dynamic dimensions use MLIR's
//! sentinel, never an arbitrary negative size. Unranked shapes have no rank.
use super::{Type, count, same_context};
use crate::{Attribute, Error, Result, affine::AffineMap};
use iree_native as n;
typed!(
    ShapedType,
    mlirTypeIsAShaped,
    "A type with an element type and optional ranked shape."
);
typed!(
    RankedTensorType,
    mlirTypeIsARankedTensor,
    "A ranked tensor with optional encoding."
);
typed!(
    UnrankedTensorType,
    mlirTypeIsAUnrankedTensor,
    "An unranked tensor."
);
typed!(
    MemRefType,
    mlirTypeIsAMemRef,
    "A ranked memory reference with layout and memory space."
);
typed!(
    UnrankedMemRefType,
    mlirTypeIsAUnrankedMemRef,
    "An unranked memory reference."
);
typed!(
    VectorType,
    mlirTypeIsAVector,
    "A vector with fixed or scalable dimensions."
);
pub(crate) fn optional_attribute(
    ty: Type<'_>,
    attr: Option<Attribute<'_>>,
) -> Result<n::MlirAttribute> {
    if let Some(attr) = attr {
        same_context(ty.context_raw(), attr.context_raw())?;
        Ok(attr.raw)
    } else {
        Ok(n::MlirAttribute {
            ptr: std::ptr::null(),
        })
    }
}
fn validate_shape(_ty: Type<'_>, shape: &[i64]) -> Result<()> {
    // SAFETY: this parameterless getter returns MLIR's scalar dynamic-dimension sentinel by value.
    let dynamic = unsafe { n::mlirShapedTypeGetDynamicSize() };
    if shape.iter().any(|&d| d < 0 && d != dynamic) {
        return Err(Error::Invalid("invalid negative dimension".into()));
    }
    count(shape.len())?;
    Ok(())
}
impl<'c> ShapedType<'c> {
    /// Rank, absent for unranked types.
    pub fn rank(&self) -> Option<usize> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirShapedTypeHasRank`.
        if unsafe { n::mlirShapedTypeHasRank(self.raw) } {
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirShapedTypeGetRank`.
            Some(unsafe { n::mlirShapedTypeGetRank(self.raw) } as usize)
        } else {
            None
        }
    }
    /// Dimensions in order, absent for unranked types.
    pub fn dimensions(&self) -> Option<Vec<i64>> {
        self.rank().map(|rank| {
            (0..rank)
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirShapedTypeGetDimSize`.
                .map(|i| unsafe { n::mlirShapedTypeGetDimSize(self.raw, i as isize) })
                .collect()
        })
    }
    /// A dimension size, or none if unranked or out of bounds.
    pub fn dimension(&self, index: usize) -> Option<i64> {
        if index >= self.rank()? {
            return None;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirShapedTypeGetDimSize`.
        Some(unsafe { n::mlirShapedTypeGetDimSize(self.raw, index as isize) })
    }
    /// Whether a valid dimension is dynamic.
    pub fn is_dynamic_dimension(&self, index: usize) -> Option<bool> {
        self.dimension(index)
            // SAFETY: this query compares a by-value dimension against MLIR's scalar dynamic-size sentinel.
            .map(|size| unsafe { n::mlirShapedTypeIsDynamicSize(size) })
    }
    /// The sentinel used for a dynamic dimension.
    pub fn dynamic_size() -> Result<i64> {
        // SAFETY: this parameterless getter returns MLIR's scalar dynamic-dimension sentinel by value.
        Ok(unsafe { n::mlirShapedTypeGetDynamicSize() })
    }
    /// The sentinel used for a dynamic stride or offset.
    pub fn dynamic_stride_or_offset() -> Result<i64> {
        // SAFETY: this parameterless getter returns MLIR's scalar dynamic-stride sentinel by value.
        Ok(unsafe { n::mlirShapedTypeGetDynamicStrideOrOffset() })
    }
    /// Whether every dimension is statically known.
    pub fn has_static_shape(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirShapedTypeHasStaticShape`.
        unsafe { n::mlirShapedTypeHasStaticShape(self.raw) }
    }
    /// Element type.
    pub fn element_type(&self) -> Type<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirShapedTypeGetElementType`.
        Type::borrowed(unsafe { n::mlirShapedTypeGetElementType(self.raw) })
    }
    /// Static element count, absent for dynamic shapes or arithmetic overflow.
    pub fn num_elements(&self) -> Option<usize> {
        self.dimensions()?
            .into_iter()
            .try_fold(1usize, |n, d| n.checked_mul(usize::try_from(d).ok()?))
    }
}
impl<'c> RankedTensorType<'c> {
    /// Construct a ranked tensor, validating dimensions and encoding context.
    pub fn new(shape: &[i64], element: Type<'c>, encoding: Option<Attribute<'c>>) -> Result<Self> {
        validate_shape(element, shape)?;
        let encoding = optional_attribute(element, encoding)?;
        // SAFETY: `element` and `context_raw` are live native handles; their wrappers keep them valid throughout `mlirLocationUnknownGet`.
        let location = unsafe { n::mlirLocationUnknownGet(element.context_raw()) };
        // SAFETY: `shape` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirRankedTensorTypeGetChecked(
                location,
                count(shape.len())?,
                shape.as_ptr(),
                element.raw,
                encoding,
            )
        };
        Ok(Self(Type::checked(raw)?))
    }
    /// Tensor encoding, absent for the default encoding.
    pub fn encoding(&self) -> Option<Attribute<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirRankedTensorTypeGetEncoding`.
        let raw = unsafe { n::mlirRankedTensorTypeGetEncoding(self.raw) };
        (!raw.ptr.is_null()).then(|| Attribute::borrowed(raw))
    }
}
impl<'c> UnrankedTensorType<'c> {
    /// Construct an unranked tensor with the given element type.
    pub fn new(element: Type<'c>) -> Result<Self> {
        // SAFETY: `element` and `context_raw` are live native handles; their wrappers keep them valid throughout `mlirLocationUnknownGet`.
        let loc = unsafe { n::mlirLocationUnknownGet(element.context_raw()) };
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirUnrankedTensorTypeGetChecked`.
        Ok(Self(Type::checked(unsafe {
            n::mlirUnrankedTensorTypeGetChecked(loc, element.raw)
        })?))
    }
}
impl<'c> MemRefType<'c> {
    /// Construct a ranked memory reference. None selects default layout/space.
    pub fn new(
        shape: &[i64],
        element: Type<'c>,
        layout: Option<Attribute<'c>>,
        memory_space: Option<Attribute<'c>>,
    ) -> Result<Self> {
        validate_shape(element, shape)?;
        if let Some(layout) = layout {
            // SAFETY: `layout` is a live attribute handle retained through both native layout-kind checks.
            if !unsafe {
                n::mlirAttributeIsAAffineMap(layout.raw)
                    || n::mlirAttributeIsAStridedLayout(layout.raw)
            } {
                return Err(Error::Invalid(
                    "memref layout must be an affine map or strided layout".into(),
                ));
            }
        }
        let layout = optional_attribute(element, layout)?;
        let memory = optional_attribute(element, memory_space)?;
        // SAFETY: `element` and `context_raw` are live native handles; their wrappers keep them valid throughout `mlirLocationUnknownGet`.
        let loc = unsafe { n::mlirLocationUnknownGet(element.context_raw()) };
        // SAFETY: `shape` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirMemRefTypeGetChecked(
                loc,
                element.raw,
                count(shape.len())?,
                shape.as_ptr(),
                layout,
                memory,
            )
        };
        Ok(Self(Type::checked(raw)?))
    }
    /// Layout attribute.
    pub fn layout(&self) -> Attribute<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirMemRefTypeGetLayout`.
        Attribute::borrowed(unsafe { n::mlirMemRefTypeGetLayout(self.raw) })
    }
    /// Layout represented as an affine map.
    pub fn affine_map(&self) -> Result<AffineMap<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirMemRefTypeGetAffineMap`.
        AffineMap::checked(unsafe { n::mlirMemRefTypeGetAffineMap(self.raw) })
    }
    /// Memory space attribute.
    pub fn memory_space(&self) -> Option<Attribute<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirMemRefTypeGetMemorySpace`.
        let raw = unsafe { n::mlirMemRefTypeGetMemorySpace(self.raw) };
        (!raw.ptr.is_null()).then(|| Attribute::borrowed(raw))
    }
    /// Static/dynamic strides and offset, absent for a non-strided layout.
    pub fn strides_and_offset(&self) -> Option<(Vec<i64>, i64)> {
        let rank = ShapedType(self.0).rank()?;
        let mut strides = vec![0; rank];
        let mut offset = 0;
        // SAFETY: `strides` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results.
        let result = unsafe {
            n::mlirMemRefTypeGetStridesAndOffset(self.raw, strides.as_mut_ptr(), &mut offset)
        };
        (result.value != 0).then_some((strides, offset))
    }
}
impl<'c> UnrankedMemRefType<'c> {
    /// Construct an unranked memory reference.
    pub fn new(element: Type<'c>, memory_space: Option<Attribute<'c>>) -> Result<Self> {
        let memory = optional_attribute(element, memory_space)?;
        // SAFETY: `element` and `context_raw` are live native handles; their wrappers keep them valid throughout `mlirLocationUnknownGet`.
        let loc = unsafe { n::mlirLocationUnknownGet(element.context_raw()) };
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirUnrankedMemRefTypeGetChecked`.
        let raw = unsafe { n::mlirUnrankedMemRefTypeGetChecked(loc, element.raw, memory) };
        Ok(Self(Type::checked(raw)?))
    }
    /// Memory space attribute.
    pub fn memory_space(&self) -> Option<Attribute<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirUnrankedMemrefGetMemorySpace`.
        let raw = unsafe { n::mlirUnrankedMemrefGetMemorySpace(self.raw) };
        (!raw.ptr.is_null()).then(|| Attribute::borrowed(raw))
    }
}
impl<'c> VectorType<'c> {
    /// Construct a vector; each scalable flag corresponds to one dimension.
    pub fn new(shape: &[i64], scalable: &[bool], element: Type<'c>) -> Result<Self> {
        if shape.len() != scalable.len() || shape.iter().any(|&d| d <= 0) {
            return Err(Error::Invalid(
                "vector dimensions must be positive with one scalable flag each".into(),
            ));
        }
        // SAFETY: `element` and `context_raw` are live native handles; their wrappers keep them valid throughout `mlirLocationUnknownGet`.
        let loc = unsafe { n::mlirLocationUnknownGet(element.context_raw()) };
        // SAFETY: `shape` and `scalable` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirVectorTypeGetScalableChecked(
                loc,
                count(shape.len())?,
                shape.as_ptr(),
                scalable.as_ptr(),
                element.raw,
            )
        };
        Ok(Self(Type::checked(raw)?))
    }
    /// Whether any dimension is scalable.
    pub fn is_scalable(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirVectorTypeIsScalable`.
        unsafe { n::mlirVectorTypeIsScalable(self.raw) }
    }
    /// Scalable flags in dimension order.
    pub fn scalable_dimensions(&self) -> Vec<bool> {
        let rank = ShapedType(self.0).rank().unwrap_or(0);
        (0..rank)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirVectorTypeIsDimScalable`.
            .map(|i| unsafe { n::mlirVectorTypeIsDimScalable(self.raw, i as isize) })
            .collect()
    }
}
