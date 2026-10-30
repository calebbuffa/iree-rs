//! Laws: context-owned handles are immutable and copyable; equality is native
//! identity, and parsing printed assembly recovers the same interned value.
use crate::{Borrowed, Context, Error, Result, callback::print_with, string_ref::string_ref};
use iree_native as n;
use std::{fmt, marker::PhantomData};

/// A context-owned MLIR type.
#[derive(Clone, Copy)]
pub struct Type<'c> {
    pub(crate) raw: n::MlirType,
    pub(crate) _context: PhantomData<&'c ()>,
}
impl<'c> Type<'c> {
    pub(crate) fn borrowed(raw: n::MlirType) -> Self {
        Self {
            raw,
            _context: PhantomData,
        }
    }
    pub(crate) fn checked(raw: n::MlirType) -> Result<Self> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR type"));
        }
        Ok(Self::borrowed(raw))
    }
    /// Borrow a native handle from its context.
    ///
    /// # Safety
    /// `raw` must be non-null and belong to `context`, which must outlive the handle.
    pub unsafe fn from_raw<O>(_context: &'c Context<'_, O>, raw: n::MlirType) -> Result<Self> {
        Self::checked(raw)
    }
    /// Return the native handle without transferring context ownership.
    pub fn as_raw(&self) -> n::MlirType {
        self.raw
    }
    /// Parse assembly in the supplied context.
    pub fn parse<O>(context: &'c Context<'_, O>, text: &str) -> Result<Self> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        let raw = unsafe { n::mlirTypeParseGet(context.raw, string_ref(text)) };
        if raw.ptr.is_null() {
            return Err(Error::Parse("type"));
        }
        Ok(Self::borrowed(raw))
    }
    /// Borrow the context that owns this handle.
    pub fn context(&self) -> Result<Context<'c, Borrowed>> {
        // SAFETY: the context handle comes from the live receiver and is borrowed only for the receiver-bound lifetime.
        unsafe { Context::borrow_raw(n::mlirTypeGetContext(self.raw)) }
    }
    /// Whether this handle is null. Safe constructors never produce null handles.
    pub fn is_null(&self) -> bool {
        self.raw.ptr.is_null()
    }
    /// Native kind identity, represented by the shared MLIR type-ID wrapper.
    pub fn type_id(&self) -> Result<crate::interfaces::TypeId> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeGetTypeID`.
        unsafe { crate::interfaces::TypeId::from_raw(n::mlirTypeGetTypeID(self.raw)) }
    }
    /// Print canonical MLIR assembly.
    pub(crate) fn print(&self) -> String {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypePrint`.
        print_with(|callback, data| unsafe { n::mlirTypePrint(self.raw, callback, data) })
    }
    pub(crate) fn context_raw(&self) -> n::MlirContext {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeGetContext`.
        unsafe { n::mlirTypeGetContext(self.raw) }
    }
}
impl PartialEq for Type<'_> {
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeEqual`.
        unsafe { n::mlirTypeEqual(self.raw, other.raw) }
    }
}
impl Eq for Type<'_> {}
impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.print())
    }
}
impl fmt::Debug for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
macro_rules! typed {
    ($name:ident, $predicate:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name<'c>(pub(crate) Type<'c>);
        impl<'c> TryFrom<Type<'c>> for $name<'c> {
            type Error = crate::Error;
            fn try_from(value: Type<'c>) -> crate::Result<Self> {
                // SAFETY: the generic Attribute wrapper keeps its checked raw handle and context live while the native predicate checks its kind.
                if unsafe { iree_native::$predicate(value.raw) } {
                    Ok(Self(value))
                } else {
                    Err(crate::Error::Invalid(
                        concat!("expected ", stringify!($name)).into(),
                    ))
                }
            }
        }
        impl<'c> From<$name<'c>> for Type<'c> {
            fn from(value: $name<'c>) -> Self {
                value.0
            }
        }
        impl<'c> std::ops::Deref for $name<'c> {
            type Target = Type<'c>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

mod compound;
mod scalar;
mod shaped;
pub use compound::*;
pub use scalar::*;
pub use shaped::*;

pub(crate) fn same_context(a: n::MlirContext, b: n::MlirContext) -> Result<()> {
    if a.ptr != b.ptr {
        return Err(Error::Invalid("values belong to different contexts".into()));
    }
    Ok(())
}
pub(crate) fn count(length: usize) -> Result<isize> {
    isize::try_from(length).map_err(|_| Error::Invalid("slice length exceeds native limits".into()))
}

#[cfg(test)]
mod tests;

pub(crate) fn validate_dialect(context: n::MlirContext, namespace: &str) -> Result<()> {
    let mut bytes = namespace.bytes();
    let valid_start = bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_');
    if !valid_start
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$')
    {
        return Err(Error::Invalid("invalid dialect namespace".into()));
    }
    // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirContextGetAllowUnregisteredDialects`.
    let allows_unknown = unsafe { n::mlirContextGetAllowUnregisteredDialects(context) };
    if !allows_unknown {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let dialect = unsafe { n::mlirContextGetOrLoadDialect(context, string_ref(namespace)) };
        if dialect.ptr.is_null() {
            return Err(Error::Invalid(
                "opaque value requires a registered dialect or an unregistered-dialect context"
                    .into(),
            ));
        }
    }
    Ok(())
}
