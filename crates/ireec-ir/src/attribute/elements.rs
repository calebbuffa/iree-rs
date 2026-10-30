//! Laws: dense values cover a static tensor/vector shape (or one splat value).
//! Typed element access checks the element representation before calling MLIR.
//! Resource construction transfers owned, aligned storage to the context.
use super::Attribute;
use crate::{
    Error, Result, Type,
    string_ref::{copy_string_ref, string_ref},
    types::{IntegerType, ShapedType, Signedness, count, same_context},
};
use iree_native as n;
use std::ffi::c_void;
typed!(
    ElementsAttr,
    mlirAttributeIsAElements,
    "An attribute containing shaped elements."
);
typed!(
    DenseElementsAttr,
    mlirAttributeIsADenseElements,
    "A dense tensor or vector attribute."
);
typed!(
    DenseResourceElementsAttr,
    mlirAttributeIsADenseResourceElements,
    "Context-owned external dense element storage."
);
typed!(
    SparseElementsAttr,
    mlirAttributeIsASparseElements,
    "Sparse tensor elements with dense indices and values."
);
impl<'c> ElementsAttr<'c> {
    /// Total number of elements, including implicit sparse zeroes.
    pub fn len(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirElementsAttrGetNumElements`.
        (unsafe { n::mlirElementsAttrGetNumElements(self.raw) }) as usize
    }
    /// Whether this shaped attribute has zero elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// An element at a multidimensional index, absent when out of bounds.
    pub fn get(&self, indices: &[u64]) -> Result<Option<Attribute<'c>>> {
        let shape = ShapedType::try_from(self.ty())?
            .dimensions()
            .ok_or_else(|| Error::Invalid("unranked elements".into()))?;
        if indices.len() != shape.len() || indices.iter().zip(&shape).any(|(&i, &d)| i >= d as u64)
        {
            return Ok(None);
        }
        // Resource attributes do not implement generic element materialization.
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseResourceElements`.
        if unsafe { n::mlirAttributeIsADenseResourceElements(self.raw) } {
            return Err(Error::Invalid("use typed resource element access".into()));
        }
        let mut indices = indices.to_vec();
        // SAFETY: `indices` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirElementsAttrGetValue(self.raw, count(indices.len())?, indices.as_mut_ptr())
        };
        Ok((!raw.ptr.is_null()).then(|| Attribute::borrowed(raw)))
    }
}
fn shape_count(ty: Type<'_>) -> Result<usize> {
    // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsARankedTensor`.
    if !unsafe { n::mlirTypeIsARankedTensor(ty.raw) || n::mlirTypeIsAVector(ty.raw) } {
        return Err(Error::Invalid(
            "dense elements require a ranked tensor or vector".into(),
        ));
    }
    // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAVector`.
    if unsafe { n::mlirTypeIsAVector(ty.raw) && n::mlirVectorTypeIsScalable(ty.raw) } {
        return Err(Error::Invalid(
            "dense elements require a fixed-size vector".into(),
        ));
    }
    ShapedType::try_from(ty)?
        .num_elements()
        .ok_or_else(|| Error::Invalid("dense elements need a finite static shape".into()))
}
fn validate_count(ty: Type<'_>, length: usize, splat: bool) -> Result<()> {
    let total = shape_count(ty)?;
    if length != total && !(splat && length == 1 && total > 0) {
        return Err(Error::Invalid(
            "element count differs from static shape".into(),
        ));
    }
    count(length)?;
    Ok(())
}
fn validate_element(ty: Type<'_>, width: u32, kind: &str) -> Result<()> {
    let element = ShapedType::try_from(ty)?.element_type();
    // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAF16`.
    let valid = unsafe {
        match kind {
            "f16" => n::mlirTypeIsAF16(element.raw),
            "bf16" => n::mlirTypeIsABF16(element.raw),
            "f32" => n::mlirTypeIsAF32(element.raw),
            "f64" => n::mlirTypeIsAF64(element.raw),
            "index" => n::mlirTypeIsAIndex(element.raw),
            _ => {
                if let Ok(integer) = IntegerType::try_from(element) {
                    integer.width() == width
                        && match kind {
                            "signed" => integer.signedness() != Signedness::Unsigned,
                            "unsigned" => integer.signedness() != Signedness::Signed,
                            _ => integer.signedness() == Signedness::Signless,
                        }
                } else {
                    false
                }
            }
        }
    };
    if !valid {
        return Err(Error::Invalid(
            "dense element type does not match Rust storage".into(),
        ));
    }
    Ok(())
}
impl<'c> DenseElementsAttr<'c> {
    /// Whether this value stores one repeated element.
    pub fn is_splat(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrIsSplat`.
        unsafe { n::mlirDenseElementsAttrIsSplat(self.raw) }
    }
    /// The repeated element, absent for nonsplats.
    pub fn splat_value(&self) -> Option<Attribute<'c>> {
        self.is_splat().then(|| {
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetSplatValue`.
            Attribute::borrowed(unsafe { n::mlirDenseElementsAttrGetSplatValue(self.raw) })
        })
    }
    /// Construct dense elements from scalar attributes of exactly the element type.
    pub fn from_attributes(ty: Type<'c>, values: &[Attribute<'c>]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        let element = ShapedType::try_from(ty)?.element_type();
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAInteger`.
        let numeric = unsafe {
            n::mlirTypeIsAInteger(element.raw)
                || n::mlirTypeIsAIndex(element.raw)
                || n::mlirTypeIsAFloat(element.raw)
        };
        if !numeric {
            return Err(Error::Invalid(
                "scalar attribute construction requires numeric elements".into(),
            ));
        }
        for value in values {
            same_context(ty.context_raw(), value.context_raw())?;
            // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsAInteger`.
            let scalar = unsafe {
                n::mlirAttributeIsAInteger(value.raw) || n::mlirAttributeIsAFloat(value.raw)
            };
            if !scalar {
                return Err(Error::Invalid(
                    "dense elements require integer or floating scalar attributes".into(),
                ));
            }
            if value.ty() != element {
                return Err(Error::Invalid("dense scalar type mismatch".into()));
            }
        }
        let raw: Vec<_> = values.iter().map(|value| value.raw).collect();
        // SAFETY: `raw` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDenseElementsAttrGet(ty.raw, count(raw.len())?, raw.as_ptr())
        })?))
    }
    /// Construct dense elements from MLIR-native packed bytes; bytes are copied.
    ///
    /// # Safety
    /// Bytes must encode the exact static shape or one splat element using MLIR's
    /// native storage format, including byte order and byte-aligned sub-byte elements.
    pub unsafe fn from_raw_buffer(ty: Type<'c>, bytes: &[u8]) -> Result<Self> {
        shape_count(ty)?;
        // SAFETY: `bytes` remain allocated for the synchronous C call and are read only during the call.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDenseElementsAttrRawBufferGet(ty.raw, bytes.len(), bytes.as_ptr().cast())
        })?))
    }
    /// Reshape without changing the element type or number of elements.
    pub fn reshape(&self, ty: Type<'c>) -> Result<Self> {
        same_context(self.context_raw(), ty.context_raw())?;
        if shape_count(ty)? != shape_count(self.ty())?
            || ShapedType::try_from(ty)?.element_type()
                != ShapedType::try_from(self.ty())?.element_type()
        {
            return Err(Error::Invalid(
                "dense reshape changes element count or type".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrReshapeGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDenseElementsAttrReshapeGet(self.raw, ty.raw)
        })?))
    }
    /// Copy the native packed integer/floating point storage, including splat storage.
    pub fn raw_data(&self) -> Result<Vec<u8>> {
        let ty = self.ty();
        let element = ShapedType::try_from(ty)?.element_type();
        let width = if let Ok(integer) = IntegerType::try_from(element) {
            integer.width() as usize
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAIndex`.
        } else if unsafe { n::mlirTypeIsAIndex(element.raw) } {
            64
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAFloat`.
        } else if unsafe { n::mlirTypeIsAFloat(element.raw) } {
            // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirFloatTypeGetWidth`.
            (unsafe { n::mlirFloatTypeGetWidth(element.raw) }) as usize
        } else {
            return Err(Error::Invalid(
                "raw storage only supports integer and floating elements".into(),
            ));
        };
        let elements = if self.is_splat() { 1 } else { shape_count(ty)? };
        // This MLIR version rounds every element, including i1, to whole bytes.
        let bytes = elements
            .checked_mul(width.div_ceil(8))
            .ok_or_else(|| Error::Invalid("raw storage size overflow".into()))?;
        if bytes == 0 {
            return Ok(Vec::new());
        }
        if bytes > isize::MAX as usize {
            return Err(Error::Invalid(
                "dense raw storage exceeds the maximum Rust slice size".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetRawData`.
        let data = unsafe { n::mlirDenseElementsAttrGetRawData(self.raw) };
        if data.is_null() {
            return Err(Error::Null("dense raw storage"));
        }
        // SAFETY: the callback rejects null pointers and oversized lengths before forming this borrowed slice.
        Ok(unsafe { n::slice(data.cast::<u8>(), bytes) }.to_vec())
    }
    /// Construct dense elements from bool storage; one value may describe a splat.
    pub fn from_bool(ty: Type<'c>, values: &[bool]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 1, "bool")?;
        let values: Vec<i32> = values.iter().map(|&v| i32::from(v)).collect();
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrBoolGet(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one bool element, returning none for an out-of-range position.
    pub fn get_bool(&self, index: usize) -> Result<Option<bool>> {
        validate_element(self.ty(), 1, "bool")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetBoolValue`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetBoolValue(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from i8 storage; one value may describe a splat.
    pub fn from_i8(ty: Type<'c>, values: &[i8]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 8, "signed")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrInt8Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one i8 element, returning none for an out-of-range position.
    pub fn get_i8(&self, index: usize) -> Result<Option<i8>> {
        validate_element(self.ty(), 8, "signed")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetInt8Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetInt8Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from u8 storage; one value may describe a splat.
    pub fn from_u8(ty: Type<'c>, values: &[u8]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 8, "unsigned")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrUInt8Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one u8 element, returning none for an out-of-range position.
    pub fn get_u8(&self, index: usize) -> Result<Option<u8>> {
        validate_element(self.ty(), 8, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetUInt8Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetUInt8Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from i16 storage; one value may describe a splat.
    pub fn from_i16(ty: Type<'c>, values: &[i16]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 16, "signed")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrInt16Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one i16 element, returning none for an out-of-range position.
    pub fn get_i16(&self, index: usize) -> Result<Option<i16>> {
        validate_element(self.ty(), 16, "signed")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetInt16Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetInt16Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from u16 storage; one value may describe a splat.
    pub fn from_u16(ty: Type<'c>, values: &[u16]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 16, "unsigned")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrUInt16Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one u16 element, returning none for an out-of-range position.
    pub fn get_u16(&self, index: usize) -> Result<Option<u16>> {
        validate_element(self.ty(), 16, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetUInt16Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetUInt16Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from i32 storage; one value may describe a splat.
    pub fn from_i32(ty: Type<'c>, values: &[i32]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 32, "signed")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrInt32Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one i32 element, returning none for an out-of-range position.
    pub fn get_i32(&self, index: usize) -> Result<Option<i32>> {
        validate_element(self.ty(), 32, "signed")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetInt32Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetInt32Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from u32 storage; one value may describe a splat.
    pub fn from_u32(ty: Type<'c>, values: &[u32]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 32, "unsigned")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrUInt32Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one u32 element, returning none for an out-of-range position.
    pub fn get_u32(&self, index: usize) -> Result<Option<u32>> {
        validate_element(self.ty(), 32, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetUInt32Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetUInt32Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from i64 storage; one value may describe a splat.
    pub fn from_i64(ty: Type<'c>, values: &[i64]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 64, "signed")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrInt64Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one i64 element, returning none for an out-of-range position.
    pub fn get_i64(&self, index: usize) -> Result<Option<i64>> {
        validate_element(self.ty(), 64, "signed")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetInt64Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetInt64Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from u64 storage; one value may describe a splat.
    pub fn from_u64(ty: Type<'c>, values: &[u64]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 64, "unsigned")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrUInt64Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one u64 element, returning none for an out-of-range position.
    pub fn get_u64(&self, index: usize) -> Result<Option<u64>> {
        validate_element(self.ty(), 64, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetUInt64Value`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetUInt64Value(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from f32 storage; one value may describe a splat.
    pub fn from_f32(ty: Type<'c>, values: &[f32]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 32, "f32")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrFloatGet(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one f32 element, returning none for an out-of-range position.
    pub fn get_f32(&self, index: usize) -> Result<Option<f32>> {
        validate_element(self.ty(), 32, "f32")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetFloatValue`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetFloatValue(self.raw, count(index)?)
        }))
    }
    /// Construct dense elements from f64 storage; one value may describe a splat.
    pub fn from_f64(ty: Type<'c>, values: &[f64]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 64, "f64")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrDoubleGet(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read one f64 element, returning none for an out-of-range position.
    pub fn get_f64(&self, index: usize) -> Result<Option<f64>> {
        validate_element(self.ty(), 64, "f64")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetDoubleValue`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetDoubleValue(self.raw, count(index)?)
        }))
    }
    /// Construct dense f16 values from native 16-bit IEEE storage bits.
    pub fn from_f16_bits(ty: Type<'c>, values: &[u16]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 16, "f16")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDenseElementsAttrFloat16Get(ty.raw, count(values.len())?, values.as_ptr())
        })?))
    }
    /// Construct dense bf16 values from native 16-bit IEEE storage bits.
    pub fn from_bf16_bits(ty: Type<'c>, values: &[u16]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 16, "bf16")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDenseElementsAttrBFloat16Get(ty.raw, count(values.len())?, values.as_ptr())
        })?))
    }
    /// Construct dense string values over a static shaped type.
    ///
    /// # Safety
    /// The element type must use MLIR string storage, not implement the
    /// DenseElementType interface. That interface has no C predicate.
    pub unsafe fn from_strings(ty: Type<'c>, values: &[&str]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        let element = ShapedType::try_from(ty)?.element_type();
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAInteger`.
        if unsafe {
            n::mlirTypeIsAInteger(element.raw)
                || n::mlirTypeIsAIndex(element.raw)
                || n::mlirTypeIsAFloat(element.raw)
                || n::mlirTypeIsAComplex(element.raw)
        } {
            return Err(Error::Invalid(
                "dense strings require a non-numeric element type".into(),
            ));
        }
        let mut strings: Vec<_> = values.iter().map(|text| string_ref(text)).collect();
        // SAFETY: `strings` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDenseElementsAttrStringGet(ty.raw, count(strings.len())?, strings.as_mut_ptr())
        })?))
    }
    /// Read a dense string, absent out of bounds.
    ///
    /// # Safety
    /// This must contain MLIR string storage, not a custom numeric dense type.
    /// The C API exposes no DenseStringElementsAttr predicate.
    pub unsafe fn get_string(&self, index: usize) -> Result<Option<String>> {
        let element = ShapedType::try_from(self.ty())?.element_type();
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAInteger`.
        if unsafe {
            n::mlirTypeIsAInteger(element.raw)
                || n::mlirTypeIsAIndex(element.raw)
                || n::mlirTypeIsAFloat(element.raw)
                || n::mlirTypeIsAComplex(element.raw)
        } {
            return Err(Error::Invalid("not string element storage".into()));
        }
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetStringValue`.
        Ok(Some(copy_string_ref(unsafe {
            n::mlirDenseElementsAttrGetStringValue(self.raw, count(index)?)
        })?))
    }
    /// Construct dense index elements using MLIR's fixed 64-bit index storage.
    pub fn from_index(ty: Type<'c>, values: &[i64]) -> Result<Self> {
        validate_count(ty, values.len(), true)?;
        validate_element(ty, 64, "index")?;
        // SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirDenseElementsAttrInt64Get(ty.raw, count(values.len())?, values.as_ptr())
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read an index element without narrowing its platform-independent 64-bit storage.
    pub fn get_index(&self, index: usize) -> Result<Option<u64>> {
        validate_element(self.ty(), 64, "index")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseElementsAttrGetIndexValue`.
        Ok(Some(unsafe {
            n::mlirDenseElementsAttrGetIndexValue(self.raw, count(index)?)
        }))
    }
}
fn resource<'c, T: Copy + 'static>(
    ty: Type<'c>,
    name: &str,
    values: &[T],
) -> Result<DenseResourceElementsAttr<'c>> {
    validate_count(ty, values.len(), false)?;
    unsafe extern "C" fn release<T>(user: *mut c_void, _: *const c_void, _: usize, _: usize) {
        // SAFETY: this deleter reconstructs the unique Box pointer transferred at registration, and the native API invokes it once.
        n::contain(|| drop(unsafe { Box::from_raw(user.cast::<Vec<T>>()) }));
    }
    let mut storage = Box::new(values.to_vec());
    let data = storage.as_mut_ptr().cast();
    let size = std::mem::size_of_val(values);
    let owner = Box::into_raw(storage).cast();
    // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
    let raw = unsafe {
        n::mlirUnmanagedDenseResourceElementsAttrGet(
            ty.raw,
            string_ref(name),
            data,
            size,
            std::mem::align_of::<T>(),
            false,
            Some(release::<T>),
            owner,
        )
    };
    // MLIR assumes ownership of the blob, including on attribute construction.
    Ok(DenseResourceElementsAttr(Attribute::checked(raw)?))
}
impl<'c> DenseResourceElementsAttr<'c> {
    /// Copy aligned bool data into a resource owned by the MLIR context.
    pub fn from_bool(ty: Type<'c>, name: &str, values: &[bool]) -> Result<Self> {
        validate_element(ty, 1, "bool")?;
        resource(ty, name, values)
    }
    /// Read a bool resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_bool(&self, index: usize) -> Result<Option<bool>> {
        validate_element(self.ty(), 1, "bool")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseBoolResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseBoolResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned i8 data into a resource owned by the MLIR context.
    pub fn from_i8(ty: Type<'c>, name: &str, values: &[i8]) -> Result<Self> {
        validate_element(ty, 8, "signed")?;
        resource(ty, name, values)
    }
    /// Read a i8 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    /// The upstream C accessor accepts signless/unsigned storage only; explicitly
    /// signed MLIR element types are rejected even though construction supports them.
    pub unsafe fn get_i8(&self, index: usize) -> Result<Option<i8>> {
        validate_element(self.ty(), 8, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseInt8ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseInt8ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned u8 data into a resource owned by the MLIR context.
    pub fn from_u8(ty: Type<'c>, name: &str, values: &[u8]) -> Result<Self> {
        validate_element(ty, 8, "unsigned")?;
        resource(ty, name, values)
    }
    /// Read a u8 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_u8(&self, index: usize) -> Result<Option<u8>> {
        validate_element(self.ty(), 8, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseUInt8ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseUInt8ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned i16 data into a resource owned by the MLIR context.
    pub fn from_i16(ty: Type<'c>, name: &str, values: &[i16]) -> Result<Self> {
        validate_element(ty, 16, "signed")?;
        resource(ty, name, values)
    }
    /// Read a i16 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    /// The upstream C accessor accepts signless/unsigned storage only; explicitly
    /// signed MLIR element types are rejected even though construction supports them.
    pub unsafe fn get_i16(&self, index: usize) -> Result<Option<i16>> {
        validate_element(self.ty(), 16, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseInt16ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseInt16ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned u16 data into a resource owned by the MLIR context.
    pub fn from_u16(ty: Type<'c>, name: &str, values: &[u16]) -> Result<Self> {
        validate_element(ty, 16, "unsigned")?;
        resource(ty, name, values)
    }
    /// Read a u16 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_u16(&self, index: usize) -> Result<Option<u16>> {
        validate_element(self.ty(), 16, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseUInt16ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseUInt16ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned i32 data into a resource owned by the MLIR context.
    pub fn from_i32(ty: Type<'c>, name: &str, values: &[i32]) -> Result<Self> {
        validate_element(ty, 32, "signed")?;
        resource(ty, name, values)
    }
    /// Read a i32 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    /// The upstream C accessor accepts signless/unsigned storage only; explicitly
    /// signed MLIR element types are rejected even though construction supports them.
    pub unsafe fn get_i32(&self, index: usize) -> Result<Option<i32>> {
        validate_element(self.ty(), 32, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseInt32ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseInt32ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned u32 data into a resource owned by the MLIR context.
    pub fn from_u32(ty: Type<'c>, name: &str, values: &[u32]) -> Result<Self> {
        validate_element(ty, 32, "unsigned")?;
        resource(ty, name, values)
    }
    /// Read a u32 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_u32(&self, index: usize) -> Result<Option<u32>> {
        validate_element(self.ty(), 32, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseUInt32ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseUInt32ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned i64 data into a resource owned by the MLIR context.
    pub fn from_i64(ty: Type<'c>, name: &str, values: &[i64]) -> Result<Self> {
        validate_element(ty, 64, "signed")?;
        resource(ty, name, values)
    }
    /// Read a i64 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    /// The upstream C accessor accepts signless/unsigned storage only; explicitly
    /// signed MLIR element types are rejected even though construction supports them.
    pub unsafe fn get_i64(&self, index: usize) -> Result<Option<i64>> {
        validate_element(self.ty(), 64, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseInt64ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseInt64ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned u64 data into a resource owned by the MLIR context.
    pub fn from_u64(ty: Type<'c>, name: &str, values: &[u64]) -> Result<Self> {
        validate_element(ty, 64, "unsigned")?;
        resource(ty, name, values)
    }
    /// Read a u64 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_u64(&self, index: usize) -> Result<Option<u64>> {
        validate_element(self.ty(), 64, "unsigned")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseUInt64ResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseUInt64ResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned f32 data into a resource owned by the MLIR context.
    pub fn from_f32(ty: Type<'c>, name: &str, values: &[f32]) -> Result<Self> {
        validate_element(ty, 32, "f32")?;
        resource(ty, name, values)
    }
    /// Read a f32 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_f32(&self, index: usize) -> Result<Option<f32>> {
        validate_element(self.ty(), 32, "f32")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseFloatResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseFloatResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
    /// Copy aligned f64 data into a resource owned by the MLIR context.
    pub fn from_f64(ty: Type<'c>, name: &str, values: &[f64]) -> Result<Self> {
        validate_element(ty, 64, "f64")?;
        resource(ty, name, values)
    }
    /// Read a f64 resource element, absent out of bounds.
    ///
    /// # Safety
    /// The resource blob must be resolved, correctly aligned, and contain the
    /// complete static shape. MLIR provides no C API to check blob resolution.
    pub unsafe fn get_f64(&self, index: usize) -> Result<Option<f64>> {
        validate_element(self.ty(), 64, "f64")?;
        if index >= shape_count(self.ty())? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseDoubleResourceElementsAttrGetValue`.
        Ok(Some(unsafe {
            n::mlirDenseDoubleResourceElementsAttrGetValue(self.raw, count(index)?)
        }))
    }
}
impl<'c> SparseElementsAttr<'c> {
    /// Construct sparse elements from rank-two i64 indices and rank-one values.
    /// Indices must be unique, lexicographically sorted, and within the shape.
    pub fn new(
        ty: Type<'c>,
        indices: DenseElementsAttr<'c>,
        values: DenseElementsAttr<'c>,
    ) -> Result<Self> {
        shape_count(ty)?;
        same_context(ty.context_raw(), indices.context_raw())?;
        same_context(ty.context_raw(), values.context_raw())?;
        let shaped = ShapedType::try_from(ty)?;
        let index_element =
            IntegerType::try_from(ShapedType::try_from(indices.ty())?.element_type())?;
        if index_element.width() != 64 || index_element.signedness() != Signedness::Signless {
            return Err(Error::Invalid(
                "sparse coordinates require signless i64 elements".into(),
            ));
        }
        let index_shape = ShapedType::try_from(indices.ty())?
            .dimensions()
            .unwrap_or_default();
        let value_shape = ShapedType::try_from(values.ty())?
            .dimensions()
            .unwrap_or_default();
        if index_shape.len() != 2
            || value_shape.len() != 1
            || index_shape[1] != shaped.rank().unwrap_or(0) as i64
            || index_shape[0] != value_shape[0]
            || ShapedType::try_from(values.ty())?.element_type() != shaped.element_type()
        {
            return Err(Error::Invalid(
                "sparse index/value shape or type mismatch".into(),
            ));
        }
        let dims = shaped.dimensions().unwrap_or_default();
        let mut previous: Option<Vec<i64>> = None;
        for row in 0..index_shape[0] as usize {
            let mut position = Vec::new();
            for (dim, &bound) in dims.iter().enumerate() {
                let value = indices
                    .get_i64(row * dims.len() + dim)?
                    .ok_or_else(|| Error::Invalid("missing sparse coordinate".into()))?;
                if value < 0 || value >= bound {
                    return Err(Error::Invalid("sparse coordinate out of bounds".into()));
                }
                position.push(value);
            }
            if previous.as_ref().is_some_and(|p| p >= &position) {
                return Err(Error::Invalid(
                    "sparse coordinates must be sorted and unique".into(),
                ));
            }
            previous = Some(position);
        }
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirSparseElementsAttribute`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirSparseElementsAttribute(ty.raw, indices.raw, values.raw)
        })?))
    }
    /// Dense coordinate matrix.
    pub fn indices(&self) -> DenseElementsAttr<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSparseElementsAttrGetIndices`.
        DenseElementsAttr(Attribute::borrowed(unsafe {
            n::mlirSparseElementsAttrGetIndices(self.raw)
        }))
    }
    /// Dense nonzero values.
    pub fn values(&self) -> DenseElementsAttr<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSparseElementsAttrGetValues`.
        DenseElementsAttr(Attribute::borrowed(unsafe {
            n::mlirSparseElementsAttrGetValues(self.raw)
        }))
    }
}
