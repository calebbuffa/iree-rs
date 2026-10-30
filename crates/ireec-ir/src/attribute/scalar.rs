//! Laws: scalar attributes retain their type and value; typed downcasts reject
//! unrelated attributes rather than invoking unchecked native accessors.
use super::Attribute;
use crate::{
    Context, Error, Result, Type,
    string_ref::{copy_string_ref, string_ref},
    types::IntegerType,
};
use iree_native as n;
typed!(
    IntegerAttr,
    mlirAttributeIsAInteger,
    "An integer or index attribute, including arbitrary-width integers."
);
typed!(
    FloatAttr,
    mlirAttributeIsAFloat,
    "A floating point attribute."
);
typed!(
    BoolAttr,
    mlirAttributeIsABool,
    "A boolean attribute (an i1 integer)."
);
typed!(StringAttr, mlirAttributeIsAString, "A string attribute.");
typed!(UnitAttr, mlirAttributeIsAUnit, "The unit attribute.");
typed!(
    TypeAttr,
    mlirAttributeIsAType,
    "An attribute containing a type."
);
impl<'c> IntegerAttr<'c> {
    /// Construct an integer or index attribute; excess value bits are truncated.
    pub fn new(ty: Type<'c>, value: i64) -> Result<Self> {
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAInteger`.
        if !unsafe { n::mlirTypeIsAInteger(ty.raw) || n::mlirTypeIsAIndex(ty.raw) } {
            return Err(Error::Invalid(
                "integer attribute needs an integer or index type".into(),
            ));
        }
        // SAFETY: `ty` and `value` are live native handles; their wrappers keep them valid throughout `mlirIntegerAttrGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirIntegerAttrGet(ty.raw, value)
        })?))
    }
    /// Construct an arbitrary-width integer from little-endian 64-bit words.
    pub fn from_words(ty: IntegerType<'c>, words: &[u64]) -> Result<Self> {
        let expected = (ty.width() as usize).div_ceil(64);
        if words.len() != expected {
            return Err(Error::Invalid("incorrect integer word count".into()));
        }
        let raw =
// SAFETY: `words` remain allocated for the synchronous C call and are read only during the call.
            unsafe { n::mlirIntegerAttrGetFromWords(ty.raw, words.len() as u32, words.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Value bit width.
    pub fn width(&self) -> u32 {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerAttrGetValueBitWidth`.
        unsafe { n::mlirIntegerAttrGetValueBitWidth(self.raw) }
    }
    /// Little-endian words, preserving arbitrary-width bit patterns.
    pub fn words(&self) -> Vec<u64> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerAttrGetValueNumWords`.
        let count = unsafe { n::mlirIntegerAttrGetValueNumWords(self.raw) };
        let mut words = vec![0; count as usize];
        // SAFETY: `words` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results.
        unsafe { n::mlirIntegerAttrGetValueWords(self.raw, words.as_mut_ptr()) };
        words
    }
    /// Sign-extend a value of at most 64 bits, independent of type signedness.
    pub fn value(&self) -> Result<i64> {
        if self.width() > 64 {
            return Err(Error::Invalid(
                "integer value exceeds 64 bits; use words".into(),
            ));
        }
        let width = self.width();
        let bits = self.words()[0];
        Ok(((bits << (64 - width)) as i64) >> (64 - width))
    }
    /// Zero-extend a value of at most 64 bits.
    pub fn unsigned_value(&self) -> Result<u64> {
        if self.width() > 64 {
            return Err(Error::Invalid(
                "integer value exceeds 64 bits; use words".into(),
            ));
        }
        Ok(self.words()[0])
    }
}
impl<'c> FloatAttr<'c> {
    /// Construct a floating attribute, rounding to the supplied floating type.
    pub fn new(ty: Type<'c>, value: f64) -> Result<Self> {
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAFloat`.
        if !unsafe { n::mlirTypeIsAFloat(ty.raw) } {
            return Err(Error::Invalid(
                "float attribute needs a floating type".into(),
            ));
        }
        // SAFETY: `ty` and `context_raw` are live native handles; their wrappers keep them valid throughout `mlirLocationUnknownGet`.
        let loc = unsafe { n::mlirLocationUnknownGet(ty.context_raw()) };
        // SAFETY: `ty` and `value` are live native handles; their wrappers keep them valid throughout `mlirFloatAttrDoubleGetChecked`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirFloatAttrDoubleGetChecked(loc, ty.raw, value)
        })?))
    }
    /// The floating value converted to double precision.
    pub fn value(&self) -> f64 {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirFloatAttrGetValueDouble`.
        unsafe { n::mlirFloatAttrGetValueDouble(self.raw) }
    }
}
impl<'c> BoolAttr<'c> {
    /// Construct a boolean value.
    pub fn new<O>(context: &'c Context<'_, O>, value: bool) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirBoolAttrGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirBoolAttrGet(context.raw, i32::from(value))
        })?))
    }
    /// The boolean value.
    pub fn value(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirBoolAttrGetValue`.
        unsafe { n::mlirBoolAttrGetValue(self.raw) }
    }
}
impl<'c> StringAttr<'c> {
    /// Construct an untyped string attribute.
    pub fn new<O>(context: &'c Context<'_, O>, value: &str) -> Result<Self> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirStringAttrGet(context.raw, string_ref(value))
        })?))
    }
    /// Construct a typed string attribute.
    pub fn with_type(ty: Type<'c>, value: &str) -> Result<Self> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirStringAttrTypedGet(ty.raw, string_ref(value))
        })?))
    }
    /// String contents; invalid UTF-8 is reported rather than replaced.
    pub fn value(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirStringAttrGetValue`.
        copy_string_ref(unsafe { n::mlirStringAttrGetValue(self.raw) })
    }
}
impl<'c> UnitAttr<'c> {
    /// Construct the unique unit attribute in a context.
    pub fn new<O>(context: &'c Context<'_, O>) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirUnitAttrGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirUnitAttrGet(context.raw)
        })?))
    }
}
impl<'c> TypeAttr<'c> {
    /// Wrap a type as an attribute.
    pub fn new(ty: Type<'c>) -> Result<Self> {
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeAttrGet`.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirTypeAttrGet(ty.raw)
        })?))
    }
    /// The contained type.
    pub fn value(&self) -> Type<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeAttrGetValue`.
        Type::borrowed(unsafe { n::mlirTypeAttrGetValue(self.raw) })
    }
}
