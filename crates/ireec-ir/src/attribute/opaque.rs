//! Opaque dialect attribute payloads.
use super::Attribute;
use crate::{
    Context, Error, Result, Type,
    string_ref::{copy_string_ref, string_ref},
    types::{count, same_context},
};
use iree_native as n;

typed!(
    OpaqueAttr,
    mlirAttributeIsAOpaque,
    "An uninterpreted dialect attribute payload."
);
impl<'c> OpaqueAttr<'c> {
    /// Construct an opaque payload in the supplied context.
    pub fn new<O>(
        context: &'c Context<'_, O>,
        dialect: &str,
        data: &[u8],
        ty: Type<'c>,
    ) -> Result<Self> {
        same_context(context.raw, ty.context_raw())?;
        crate::types::validate_dialect(context.raw, dialect)?;
        // SAFETY: `data` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirOpaqueAttrGet(
                context.raw,
                string_ref(dialect),
                count(data.len())?,
                data.as_ptr().cast(),
                ty.raw,
            )
        };
        Ok(Self(Attribute::checked(raw)?))
    }
    /// Dialect namespace.
    pub fn dialect_namespace(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOpaqueAttrGetDialectNamespace`.
        copy_string_ref(unsafe { n::mlirOpaqueAttrGetDialectNamespace(self.raw) })
    }
    /// Uninterpreted payload bytes (not necessarily UTF-8).
    pub fn data(&self) -> Result<Vec<u8>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOpaqueAttrGetData`.
        let value = unsafe { n::mlirOpaqueAttrGetData(self.raw) };
        if value.length == 0 {
            return Ok(Vec::new());
        }
        if value.length > isize::MAX as usize {
            return Err(Error::Invalid(
                "opaque attribute payload exceeds the maximum Rust slice size".into(),
            ));
        }
        if value.data.is_null() {
            return Err(Error::Null("opaque attribute payload"));
        }
        // SAFETY: the callback rejects null pointers and oversized lengths before forming this borrowed slice.
        Ok(unsafe { n::slice(value.data.cast::<u8>(), value.length) }.to_vec())
    }
}
