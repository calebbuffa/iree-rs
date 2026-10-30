//! Mutable registries for MLIR dialect implementations.
use crate::{Error, Result};
use iree_native as n;
use std::marker::PhantomData;

/// An MLIR dialect registry with explicit native ownership.
pub struct DialectRegistry<'scope> {
    pub(crate) raw: n::MlirDialectRegistry,
    owns: bool,
    _scope: PhantomData<&'scope mut ()>,
}

impl DialectRegistry<'static> {
    /// Create an owned empty dialect registry.
    pub fn new() -> Result<Self> {
        ireec::Compiler::new()?;
        // SAFETY: this constructor takes no inputs; the newly allocated registry handle is null-checked before wrapping.
        let raw = unsafe { n::mlirDialectRegistryCreate() };
        if raw.ptr.is_null() {
            return Err(Error::Null("dialect registry"));
        }
        Ok(Self {
            raw,
            owns: true,
            _scope: PhantomData,
        })
    }
}

impl<'scope> DialectRegistry<'scope> {
    /// Register the dialects provided by the IREE compiler.
    pub fn register_iree_dialects(&mut self) {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCompilerRegisterDialects`.
        unsafe { n::ireeCompilerRegisterDialects(self.raw) };
    }

    /// The borrowed native registry handle.
    pub fn as_raw(&self) -> n::MlirDialectRegistry {
        self.raw
    }

    /// Transfer the owned registry handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy this registry exactly once.
    pub unsafe fn into_raw(self) -> n::MlirDialectRegistry {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl Drop for DialectRegistry<'_> {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: `self` uniquely owns the native handle released by `mlirDialectRegistryDestroy`.
            unsafe { n::mlirDialectRegistryDestroy(self.raw) };
        }
    }
}
