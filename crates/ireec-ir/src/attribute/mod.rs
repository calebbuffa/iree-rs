//! Laws: context-owned handles are immutable and copyable; equality is native
//! identity, and parsing printed assembly recovers the same interned value.
use crate::{Borrowed, Context, Error, Result, callback::print_with, string_ref::string_ref};
use iree_native as n;
use std::{fmt, marker::PhantomData};

/// A context-owned MLIR attribute.
#[derive(Clone, Copy)]
pub struct Attribute<'c> {
    pub(crate) raw: n::MlirAttribute,
    pub(crate) _context: PhantomData<&'c ()>,
}
impl<'c> Attribute<'c> {
    pub(crate) fn borrowed(raw: n::MlirAttribute) -> Self {
        Self {
            raw,
            _context: PhantomData,
        }
    }
    pub(crate) fn checked(raw: n::MlirAttribute) -> Result<Self> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR attribute"));
        }
        Ok(Self::borrowed(raw))
    }
    /// Borrow a native handle from its context.
    ///
    /// # Safety
    /// `raw` must be non-null and belong to `context`, which must outlive the handle.
    pub unsafe fn from_raw<O>(_context: &'c Context<'_, O>, raw: n::MlirAttribute) -> Result<Self> {
        Self::checked(raw)
    }
    /// Return the native handle without transferring context ownership.
    pub fn as_raw(&self) -> n::MlirAttribute {
        self.raw
    }
    /// Parse assembly in the supplied context.
    pub fn parse<O>(context: &'c Context<'_, O>, text: &str) -> Result<Self> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        let raw = unsafe { n::mlirAttributeParseGet(context.raw, string_ref(text)) };
        if raw.ptr.is_null() {
            return Err(Error::Parse("attribute"));
        }
        Ok(Self::borrowed(raw))
    }
    /// Borrow the context that owns this handle.
    pub fn context(&self) -> Result<Context<'c, Borrowed>> {
        // SAFETY: the context handle comes from the live receiver and is borrowed only for the receiver-bound lifetime.
        unsafe { Context::borrow_raw(n::mlirAttributeGetContext(self.raw)) }
    }
    /// Whether this handle is null. Safe constructors never produce null handles.
    pub fn is_null(&self) -> bool {
        self.raw.ptr.is_null()
    }
    /// Native kind identity, represented by the shared MLIR type-ID wrapper.
    pub fn type_id(&self) -> Result<crate::interfaces::TypeId> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeGetTypeID`.
        unsafe { crate::interfaces::TypeId::from_raw(n::mlirAttributeGetTypeID(self.raw)) }
    }
    /// Create a distinct identity referencing this attribute's value.
    pub fn distinct(&self) -> Result<Self> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDistinctAttrCreate`.
        Self::checked(unsafe { n::mlirDistinctAttrCreate(self.raw) })
    }
    /// Print canonical MLIR assembly.
    pub(crate) fn print(&self) -> String {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributePrint`.
        print_with(|callback, data| unsafe { n::mlirAttributePrint(self.raw, callback, data) })
    }
    pub(crate) fn context_raw(&self) -> n::MlirContext {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeGetContext`.
        unsafe { n::mlirAttributeGetContext(self.raw) }
    }
}
impl PartialEq for Attribute<'_> {
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeEqual`.
        unsafe { n::mlirAttributeEqual(self.raw, other.raw) }
    }
}
impl Eq for Attribute<'_> {}
impl fmt::Display for Attribute<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.print())
    }
}
impl fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
macro_rules! typed {
    ($name:ident, $predicate:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name<'c>(pub(crate) Attribute<'c>);
        impl<'c> TryFrom<Attribute<'c>> for $name<'c> {
            type Error = crate::Error;
            fn try_from(value: Attribute<'c>) -> crate::Result<Self> {
                // SAFETY: `value` retains its checked attribute handle and context while the native predicate checks its kind.
                if unsafe { iree_native::$predicate(value.raw) } {
                    Ok(Self(value))
                } else {
                    Err(crate::Error::Invalid(
                        concat!("expected ", stringify!($name)).into(),
                    ))
                }
            }
        }
        impl<'c> From<$name<'c>> for Attribute<'c> {
            fn from(value: $name<'c>) -> Self {
                value.0
            }
        }
        impl<'c> std::ops::Deref for $name<'c> {
            type Target = Attribute<'c>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

mod affine;
mod collection;
mod elements;
mod layout;
mod opaque;
mod scalar;
pub use affine::*;
pub use collection::*;
pub use elements::*;
pub use layout::*;
pub use opaque::*;
pub use scalar::*;
impl<'c> Attribute<'c> {
    /// The attribute's MLIR type (none for untyped attributes).
    pub fn ty(&self) -> crate::Type<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAttributeGetType`.
        crate::Type::borrowed(unsafe { n::mlirAttributeGetType(self.raw) })
    }
}

#[cfg(test)]
mod tests;
