//! MLIR dialect handles and IREE dialect extensions.
use crate::{Context, DialectRegistry, Error, Result, string_ref::copy_string_ref};
use iree_native as n;
use std::marker::PhantomData;

/// A non-owning MLIR dialect handle tied to its context.
#[derive(Clone, Copy)]
pub struct Dialect<'ctx> {
    raw: n::MlirDialect,
    _context: PhantomData<&'ctx ()>,
}

impl Dialect<'_> {
    /// Copy the dialect namespace.
    pub fn namespace(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDialectGetNamespace`.
        let namespace = unsafe { n::mlirDialectGetNamespace(self.raw) };
        copy_string_ref(namespace)
    }
}

/// A value-type handle that identifies a dialect implementation.
#[derive(Clone, Copy)]
pub struct DialectHandle {
    raw: n::MlirDialectHandle,
}

impl DialectHandle {
    /// Wrap a valid dialect handle from the matched native MLIR library.
    ///
    /// # Safety
    ///
    /// `raw` must be a live handle from the exact MLIR library used by IREE.
    pub unsafe fn from_raw(raw: n::MlirDialectHandle) -> Result<Self> {
        if raw.ptr.is_null() {
            return Err(Error::Null("dialect handle"));
        }
        Ok(Self { raw })
    }

    /// Copy the dialect namespace associated with this handle.
    pub fn namespace(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDialectHandleGetNamespace`.
        let namespace = unsafe { n::mlirDialectHandleGetNamespace(self.raw) };
        copy_string_ref(namespace)
    }

    /// Insert the dialect into a registry.
    pub fn insert(&self, registry: &mut DialectRegistry<'_>) {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDialectHandleInsertDialect`.
        unsafe { n::mlirDialectHandleInsertDialect(self.raw, registry.raw) };
    }

    /// Register the dialect with a context.
    pub fn register(&self, context: &Context<'_, impl Sized>) {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDialectHandleRegisterDialect`.
        unsafe { n::mlirDialectHandleRegisterDialect(self.raw, context.raw) };
    }

    /// Load this dialect in a context.
    pub fn load<'ctx>(&self, context: &'ctx Context<'_, impl Sized>) -> Result<Dialect<'ctx>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDialectHandleLoadDialect`.
        let raw = unsafe { n::mlirDialectHandleLoadDialect(self.raw, context.raw) };
        if raw.ptr.is_null() {
            return Err(Error::Null("loaded dialect"));
        }
        Ok(Dialect {
            raw,
            _context: PhantomData,
        })
    }
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Load a registered dialect by namespace, returning `None` if unknown.
    pub fn load_dialect(&self, namespace: &str) -> Result<Option<Dialect<'_>>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let dialect = unsafe {
            n::mlirContextGetOrLoadDialect(self.raw, crate::string_ref::string_ref(namespace))
        };
        if dialect.ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(Dialect {
            raw: dialect,
            _context: PhantomData,
        }))
    }
}

/// Code-generation dialect attributes and parameters.
pub mod codegen;
/// GPU dialect attributes and target queries.
pub mod gpu;

impl DialectHandle {
    /// Return MLIR's Linalg dialect handle.
    pub fn linalg() -> Result<Self> {
        // SAFETY: this parameterless getter returns the static Linalg dialect handle by value.
        unsafe { Self::from_raw(n::mlirGetDialectHandle__linalg__()) }
    }
}
