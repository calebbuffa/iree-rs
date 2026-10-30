//! MLIR context ownership, borrowing, and dialect setup.
use crate::{Attribute, Error, Location, Result, Type, string_ref::string_ref};
use iree_native as n;
use std::marker::PhantomData;

/// Marks a wrapper that owns its native MLIR object.
pub struct Owned(());

/// Marks a wrapper that borrows its native MLIR object.
pub struct Borrowed(());

/// An MLIR context, either owned directly or borrowed from a compiler session.
pub struct Context<'a, Ownership = Owned> {
    pub(crate) raw: n::MlirContext,
    owns: bool,
    _borrow: PhantomData<(&'a (), Ownership)>,
}

impl Context<'static, Owned> {
    /// Take ownership of an existing MLIR context.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null context handle uniquely owned by the caller.
    pub unsafe fn from_raw(raw: n::MlirContext) -> Result<Context<'static, Owned>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR context"));
        }
        Ok(Context {
            raw,
            owns: true,
            _borrow: PhantomData,
        })
    }

    /// Create an IREE-initialized context with all compiler dialects registered.
    pub fn new() -> Result<Context<'static, Owned>> {
        ireec::Compiler::new()?;
        // SAFETY: this constructor takes no inputs; the newly allocated registry handle is null-checked before use.
        let registry = unsafe { n::mlirDialectRegistryCreate() };
        if registry.ptr.is_null() {
            return Err(Error::Null("dialect registry"));
        }
        // SAFETY: `registry` remains live from creation until after dialect registration and context creation.
        unsafe { n::ireeCompilerRegisterDialects(registry) };
        // SAFETY: `registry` remains live until `mlirContextCreateWithRegistry` returns and is destroyed afterward.
        let raw = unsafe { n::mlirContextCreateWithRegistry(registry, true) };
        // SAFETY: `registry` is uniquely owned here, and `mlirDialectRegistryDestroy` releases it exactly once.
        unsafe { n::mlirDialectRegistryDestroy(registry) };
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR context"));
        }
        // SAFETY: `raw` is the newly created, non-null context whose ownership is transferred into the wrapper below.
        unsafe { n::ireeCompilerInitializeContext(raw) };
        Ok(Context {
            raw,
            owns: true,
            _borrow: PhantomData,
        })
    }
}

impl<'a> Context<'a, Borrowed> {
    pub(crate) unsafe fn borrow_raw(raw: n::MlirContext) -> Result<Context<'a, Borrowed>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR context"));
        }
        Ok(Context {
            raw,
            owns: false,
            _borrow: PhantomData,
        })
    }

    /// Borrow the MLIR context owned by a compiler session.
    pub fn borrow_session(session: &'a ireec::Session) -> Result<Context<'a, Borrowed>> {
        // SAFETY: `session` is a live native handle; their wrappers keep them valid throughout `ireeCompilerSessionBorrowContext`.
        let raw = unsafe { n::ireeCompilerSessionBorrowContext(session.as_raw()) };
        if raw.ptr.is_null() {
            return Err(Error::Null("session MLIR context"));
        }
        Ok(Context {
            raw,
            owns: false,
            _borrow: PhantomData,
        })
    }
}

impl<'a> Context<'a, Owned> {
    /// Transfer the session context into an owned wrapper.
    ///
    /// The session remains borrowed for the returned context's lifetime so it
    /// cannot be destroyed before the context is released.
    pub fn steal_session(session: &'a ireec::Session) -> Result<Context<'a, Owned>> {
        // SAFETY: the live session owns its context; `ireeCompilerSessionStealContext` transfers the context handle exactly once.
        let raw = unsafe { n::ireeCompilerSessionStealContext(session.as_raw()) };
        if raw.ptr.is_null() {
            return Err(Error::Null("stolen session MLIR context"));
        }
        Ok(Context {
            raw,
            owns: true,
            _borrow: PhantomData,
        })
    }

    /// Transfer an owned context handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy the handle exactly once and keep it alive as
    /// long as any MLIR objects created from it. This method exists only for
    /// owned contexts.
    pub unsafe fn into_raw(self) -> n::MlirContext {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl<'a, Ownership> Context<'a, Ownership> {
    /// The borrowed native context handle.
    pub fn as_raw(&self) -> n::MlirContext {
        self.raw
    }

    /// Create the unknown source location in this context.
    pub fn unknown_location(&self) -> Location<'_> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationUnknownGet`.
        let raw = unsafe { n::mlirLocationUnknownGet(self.raw) };
        Location {
            raw,
            _context: PhantomData,
        }
    }

    /// Construct a signless integer type of `bit_width` bits.
    pub fn integer_type(&self, bit_width: u32) -> Result<Type<'_>> {
        if bit_width == 0 {
            return Err(Error::Invalid("integer type width must be nonzero".into()));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerTypeGet`.
        let raw = unsafe { n::mlirIntegerTypeGet(self.raw, bit_width) };
        if raw.ptr.is_null() {
            return Err(Error::Null("integer type"));
        }
        Ok(Type {
            raw,
            _context: PhantomData,
        })
    }

    /// Construct the MLIR index type.
    pub fn index_type(&self) -> Result<Type<'_>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIndexTypeGet`.
        let raw = unsafe { n::mlirIndexTypeGet(self.raw) };
        if raw.ptr.is_null() {
            return Err(Error::Null("index type"));
        }
        Ok(Type {
            raw,
            _context: PhantomData,
        })
    }

    /// Construct the IEEE 32-bit floating-point type.
    pub fn f32_type(&self) -> Result<Type<'_>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirF32TypeGet`.
        let raw = unsafe { n::mlirF32TypeGet(self.raw) };
        if raw.ptr.is_null() {
            return Err(Error::Null("f32 type"));
        }
        Ok(Type {
            raw,
            _context: PhantomData,
        })
    }

    /// Construct the IEEE 64-bit floating-point type.
    pub fn f64_type(&self) -> Result<Type<'_>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirF64TypeGet`.
        let raw = unsafe { n::mlirF64TypeGet(self.raw) };
        if raw.ptr.is_null() {
            return Err(Error::Null("f64 type"));
        }
        Ok(Type {
            raw,
            _context: PhantomData,
        })
    }

    /// Construct a string attribute from UTF-8 text.
    pub fn string_attribute(&self, value: &str) -> Result<Attribute<'_>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe { n::mlirStringAttrGet(self.raw, string_ref(value)) };
        if raw.ptr.is_null() {
            return Err(Error::Null("string attribute"));
        }
        Ok(Attribute {
            raw,
            _context: PhantomData,
        })
    }

    /// Construct the unit attribute.
    pub fn unit_attribute(&self) -> Result<Attribute<'_>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirUnitAttrGet`.
        let raw = unsafe { n::mlirUnitAttrGet(self.raw) };
        if raw.ptr.is_null() {
            return Err(Error::Null("unit attribute"));
        }
        Ok(Attribute {
            raw,
            _context: PhantomData,
        })
    }

    /// Construct an integer attribute using an integer type from this context.
    pub fn integer_attribute(&self, ty: &Type<'_>, value: i64) -> Result<Attribute<'_>> {
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeGetContext`.
        if unsafe { n::mlirTypeGetContext(ty.raw) }.ptr != self.raw.ptr {
            return Err(Error::Invalid(
                "integer attribute type belongs to another context".into(),
            ));
        }
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAInteger`.
        if !unsafe { n::mlirTypeIsAInteger(ty.raw) } {
            return Err(Error::Invalid(
                "integer attribute requires an integer type".into(),
            ));
        }
        // SAFETY: `ty` and `value` are live native handles; their wrappers keep them valid throughout `mlirIntegerAttrGet`.
        let raw = unsafe { n::mlirIntegerAttrGet(ty.raw, value) };
        if raw.ptr.is_null() {
            return Err(Error::Null("integer attribute"));
        }
        Ok(Attribute {
            raw,
            _context: PhantomData,
        })
    }

    /// Parse an MLIR attribute in this context.
    pub fn parse_attribute(&self, text: &str) -> Result<Attribute<'_>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe { n::mlirAttributeParseGet(self.raw, string_ref(text)) };
        if raw.ptr.is_null() {
            return Err(Error::Parse("attribute"));
        }
        Ok(Attribute {
            raw,
            _context: PhantomData,
        })
    }

    /// Parse an MLIR type in this context.
    pub fn parse_type(&self, text: &str) -> Result<Type<'_>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe { n::mlirTypeParseGet(self.raw, string_ref(text)) };
        if raw.ptr.is_null() {
            return Err(Error::Parse("type"));
        }
        Ok(Type {
            raw,
            _context: PhantomData,
        })
    }
}

impl<Ownership> Drop for Context<'_, Ownership> {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: `self` uniquely owns the native handle released by `mlirContextDestroy`.
            unsafe { n::mlirContextDestroy(self.raw) };
        }
    }
}
