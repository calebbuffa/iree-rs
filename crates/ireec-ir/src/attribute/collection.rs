//! Laws: arrays preserve order, dictionaries preserve unique name/value pairs,
//! and symbol references preserve the root-to-leaf path.
use super::Attribute;
use crate::{
    Context, Error, Result,
    string_ref::{copy_string_ref, string_ref},
    types::{count, same_context},
};
use iree_native as n;
use std::marker::PhantomData;
typed!(
    ArrayAttr,
    mlirAttributeIsAArray,
    "An ordered array of attributes."
);
typed!(
    DictionaryAttr,
    mlirAttributeIsADictionary,
    "A dictionary of unique named attributes."
);
typed!(
    SymbolRefAttr,
    mlirAttributeIsASymbolRef,
    "A root symbol and optional nested flat references."
);
typed!(
    FlatSymbolRefAttr,
    mlirAttributeIsAFlatSymbolRef,
    "A flat symbol reference."
);
/// A context-owned identifier paired with an attribute.
#[derive(Clone, Copy)]
pub struct NamedAttribute<'c> {
    raw: n::MlirNamedAttribute,
    _context: PhantomData<&'c ()>,
}
impl<'c> NamedAttribute<'c> {
    /// Name an attribute in its owning context.
    pub fn new(name: &str, value: Attribute<'c>) -> Result<Self> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let id = unsafe { n::mlirIdentifierGet(value.context_raw(), string_ref(name)) };
        // SAFETY: `value` and `id` are live native handles; their wrappers keep them valid throughout `mlirNamedAttributeGet`.
        let raw = unsafe { n::mlirNamedAttributeGet(id, value.raw) };
        Ok(Self {
            raw,
            _context: PhantomData,
        })
    }
    /// Borrow a native name/value pair.
    ///
    /// # Safety
    /// Both the identifier and attribute must belong to `context` and be non-null.
    pub unsafe fn from_raw<O>(
        _context: &'c Context<'_, O>,
        raw: n::MlirNamedAttribute,
    ) -> Result<Self> {
        if raw.name.ptr.is_null() || raw.attribute.ptr.is_null() {
            return Err(Error::Null("named attribute"));
        }
        Ok(Self {
            raw,
            _context: PhantomData,
        })
    }
    /// Native name/value pair.
    pub fn as_raw(&self) -> n::MlirNamedAttribute {
        self.raw
    }
    /// Attribute name.
    pub fn name(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIdentifierStr`.
        copy_string_ref(unsafe { n::mlirIdentifierStr(self.raw.name) })
    }
    /// Attribute value.
    pub fn value(&self) -> Attribute<'c> {
        Attribute::borrowed(self.raw.attribute)
    }
}
impl<'c> ArrayAttr<'c> {
    /// Construct an ordered array in a single context.
    pub fn new<O>(context: &'c Context<'_, O>, values: &[Attribute<'c>]) -> Result<Self> {
        for attr in values {
            same_context(context.raw, attr.context_raw())?;
        }
        let raw: Vec<_> = values.iter().map(|attr| attr.raw).collect();
        // SAFETY: `raw` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirArrayAttrGet(context.raw, count(raw.len())?, raw.as_ptr())
        })?))
    }
    /// Elements in order.
    pub fn elements(&self) -> Vec<Attribute<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirArrayAttrGetNumElements`.
        let count = unsafe { n::mlirArrayAttrGetNumElements(self.raw) };
        (0..count)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirArrayAttrGetElement`.
            .map(|i| Attribute::borrowed(unsafe { n::mlirArrayAttrGetElement(self.raw, i) }))
            .collect()
    }
}
impl<'c> DictionaryAttr<'c> {
    /// Construct a dictionary; duplicate names are rejected.
    pub fn new<O>(context: &'c Context<'_, O>, entries: &[NamedAttribute<'c>]) -> Result<Self> {
        let mut names = std::collections::HashSet::new();
        for entry in entries {
            same_context(context.raw, entry.value().context_raw())?;
            if !names.insert(entry.name()?) {
                return Err(Error::Invalid("duplicate dictionary key".into()));
            }
        }
        let raw: Vec<_> = entries.iter().map(|entry| entry.raw).collect();
        // SAFETY: `raw` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirDictionaryAttrGet(context.raw, count(raw.len())?, raw.as_ptr())
        })?))
    }
    /// Entries in MLIR's canonical name order.
    pub fn entries(&self) -> Vec<NamedAttribute<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDictionaryAttrGetNumElements`.
        let count = unsafe { n::mlirDictionaryAttrGetNumElements(self.raw) };
        (0..count)
            .map(|i| NamedAttribute {
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirDictionaryAttrGetElement`.
                raw: unsafe { n::mlirDictionaryAttrGetElement(self.raw, i) },
                _context: PhantomData,
            })
            .collect()
    }
    /// Look up a value by name.
    pub fn get(&self, name: &str) -> Option<Attribute<'c>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe { n::mlirDictionaryAttrGetElementByName(self.raw, string_ref(name)) };
        (!raw.ptr.is_null()).then(|| Attribute::borrowed(raw))
    }
}
impl<'c> FlatSymbolRefAttr<'c> {
    /// Construct a flat reference to a symbol.
    pub fn new<O>(context: &'c Context<'_, O>, symbol: &str) -> Result<Self> {
        if symbol.is_empty() {
            return Err(Error::Invalid("empty symbol name".into()));
        }
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        Ok(Self(Attribute::checked(unsafe {
            n::mlirFlatSymbolRefAttrGet(context.raw, string_ref(symbol))
        })?))
    }
    /// Referenced symbol name.
    pub fn value(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirFlatSymbolRefAttrGetValue`.
        copy_string_ref(unsafe { n::mlirFlatSymbolRefAttrGetValue(self.raw) })
    }
}
impl<'c> SymbolRefAttr<'c> {
    /// Construct a root reference with an ordered nested path.
    pub fn new<O>(
        context: &'c Context<'_, O>,
        root: &str,
        nested: &[FlatSymbolRefAttr<'c>],
    ) -> Result<Self> {
        if root.is_empty() {
            return Err(Error::Invalid("empty root symbol".into()));
        }
        for attr in nested {
            same_context(context.raw, attr.context_raw())?;
        }
        let raw: Vec<_> = nested.iter().map(|attr| attr.raw).collect();
        // SAFETY: `raw` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Attribute::checked(unsafe {
            n::mlirSymbolRefAttrGet(
                context.raw,
                string_ref(root),
                count(raw.len())?,
                raw.as_ptr(),
            )
        })?))
    }
    /// Root symbol name.
    pub fn root(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSymbolRefAttrGetRootReference`.
        copy_string_ref(unsafe { n::mlirSymbolRefAttrGetRootReference(self.raw) })
    }
    /// Leaf symbol name.
    pub fn leaf(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSymbolRefAttrGetLeafReference`.
        copy_string_ref(unsafe { n::mlirSymbolRefAttrGetLeafReference(self.raw) })
    }
    /// Nested references from root toward leaf.
    pub fn nested(&self) -> Vec<FlatSymbolRefAttr<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSymbolRefAttrGetNumNestedReferences`.
        let count = unsafe { n::mlirSymbolRefAttrGetNumNestedReferences(self.raw) };
        (0..count)
            .map(|i| {
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirSymbolRefAttrGetNestedReference`.
                FlatSymbolRefAttr(Attribute::borrowed(unsafe {
                    n::mlirSymbolRefAttrGetNestedReference(self.raw, i)
                }))
            })
            .collect()
    }
}

/// A homogeneous dense primitive array (not a shaped elements attribute).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenseArrayAttr<'c>(Attribute<'c>);
impl<'c> TryFrom<Attribute<'c>> for DenseArrayAttr<'c> {
    type Error = Error;
    fn try_from(value: Attribute<'c>) -> Result<Self> {
        // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsADenseBoolArray`.
        let valid = unsafe {
            n::mlirAttributeIsADenseBoolArray(value.raw)
                || n::mlirAttributeIsADenseI8Array(value.raw)
                || n::mlirAttributeIsADenseI16Array(value.raw)
                || n::mlirAttributeIsADenseI32Array(value.raw)
                || n::mlirAttributeIsADenseI64Array(value.raw)
                || n::mlirAttributeIsADenseF32Array(value.raw)
                || n::mlirAttributeIsADenseF64Array(value.raw)
        };
        if valid {
            Ok(Self(value))
        } else {
            Err(Error::Invalid("expected dense primitive array".into()))
        }
    }
}
impl<'c> From<DenseArrayAttr<'c>> for Attribute<'c> {
    fn from(value: DenseArrayAttr<'c>) -> Self {
        value.0
    }
}
impl<'c> std::ops::Deref for DenseArrayAttr<'c> {
    type Target = Attribute<'c>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'c> DenseArrayAttr<'c> {
    /// Number of primitive elements.
    pub fn len(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseArrayGetNumElements`.
        (unsafe { n::mlirDenseArrayGetNumElements(self.raw) }) as usize
    }
    /// Whether there are no primitive elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Construct a dense bool array.
    pub fn from_bool<O>(context: &'c Context<'_, O>, values: &[bool]) -> Result<Self> {
        let values: Vec<i32> = values.iter().map(|&v| i32::from(v)).collect();
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseBoolArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a bool array element; reject other storage types.
    pub fn get_bool(&self, index: usize) -> Result<Option<bool>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseBoolArray`.
        if !unsafe { n::mlirAttributeIsADenseBoolArray(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseBoolArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseBoolArrayGetElement(self.raw, count(index)?)
        }))
    }
    /// Construct a dense i8 array.
    pub fn from_i8<O>(context: &'c Context<'_, O>, values: &[i8]) -> Result<Self> {
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseI8ArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a i8 array element; reject other storage types.
    pub fn get_i8(&self, index: usize) -> Result<Option<i8>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseI8Array`.
        if !unsafe { n::mlirAttributeIsADenseI8Array(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseI8ArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseI8ArrayGetElement(self.raw, count(index)?)
        }))
    }
    /// Construct a dense i16 array.
    pub fn from_i16<O>(context: &'c Context<'_, O>, values: &[i16]) -> Result<Self> {
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseI16ArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a i16 array element; reject other storage types.
    pub fn get_i16(&self, index: usize) -> Result<Option<i16>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseI16Array`.
        if !unsafe { n::mlirAttributeIsADenseI16Array(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseI16ArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseI16ArrayGetElement(self.raw, count(index)?)
        }))
    }
    /// Construct a dense i32 array.
    pub fn from_i32<O>(context: &'c Context<'_, O>, values: &[i32]) -> Result<Self> {
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseI32ArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a i32 array element; reject other storage types.
    pub fn get_i32(&self, index: usize) -> Result<Option<i32>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseI32Array`.
        if !unsafe { n::mlirAttributeIsADenseI32Array(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseI32ArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseI32ArrayGetElement(self.raw, count(index)?)
        }))
    }
    /// Construct a dense i64 array.
    pub fn from_i64<O>(context: &'c Context<'_, O>, values: &[i64]) -> Result<Self> {
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseI64ArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a i64 array element; reject other storage types.
    pub fn get_i64(&self, index: usize) -> Result<Option<i64>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseI64Array`.
        if !unsafe { n::mlirAttributeIsADenseI64Array(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseI64ArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseI64ArrayGetElement(self.raw, count(index)?)
        }))
    }
    /// Construct a dense f32 array.
    pub fn from_f32<O>(context: &'c Context<'_, O>, values: &[f32]) -> Result<Self> {
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseF32ArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a f32 array element; reject other storage types.
    pub fn get_f32(&self, index: usize) -> Result<Option<f32>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseF32Array`.
        if !unsafe { n::mlirAttributeIsADenseF32Array(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseF32ArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseF32ArrayGetElement(self.raw, count(index)?)
        }))
    }
    /// Construct a dense f64 array.
    pub fn from_f64<O>(context: &'c Context<'_, O>, values: &[f64]) -> Result<Self> {
        let raw =
// SAFETY: `values` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirDenseF64ArrayGet(context.raw, count(values.len())?, values.as_ptr()) };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Read a f64 array element; reject other storage types.
    pub fn get_f64(&self, index: usize) -> Result<Option<f64>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeIsADenseF64Array`.
        if !unsafe { n::mlirAttributeIsADenseF64Array(self.raw) } {
            return Err(Error::Invalid("dense array storage type mismatch".into()));
        }
        if index >= self.len() {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDenseF64ArrayGetElement`.
        Ok(Some(unsafe {
            n::mlirDenseF64ArrayGetElement(self.raw, count(index)?)
        }))
    }
}
