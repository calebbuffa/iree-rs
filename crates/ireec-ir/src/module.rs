//! MLIR modules and ownership transfer operations.
use crate::{Borrowed, Context, Error, Operation, Result, string_ref::string_ref};
use iree_native as n;
use std::{fmt, marker::PhantomData};

/// An owned MLIR module tied to the context in which it was created.
pub struct Module<'ctx> {
    pub(crate) raw: n::MlirModule,
    pub(crate) _context: PhantomData<&'ctx ()>,
}

impl<'ctx> Module<'ctx> {
    /// Take ownership of a module created in `context`.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null, uniquely owned module created in `context`.
    pub unsafe fn from_raw<Ownership>(
        _context: &'ctx Context<'_, Ownership>,
        raw: n::MlirModule,
    ) -> Result<Module<'ctx>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR module"));
        }
        Ok(Module {
            raw,
            _context: PhantomData,
        })
    }

    /// Parse a module from MLIR text.
    pub fn parse<Ownership>(
        context: &'ctx Context<'_, Ownership>,
        text: &str,
    ) -> Result<Module<'ctx>> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        let raw = unsafe { n::mlirModuleCreateParse(context.raw, string_ref(text)) };
        if raw.ptr.is_null() {
            return Err(Error::Parse("module"));
        }
        Ok(Module {
            raw,
            _context: PhantomData,
        })
    }

    /// The borrowed native module handle.
    pub fn as_raw(&self) -> n::MlirModule {
        self.raw
    }

    /// View the module as a borrowed generic operation.
    pub fn operation(&self) -> Operation<'_, Borrowed> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirModuleGetOperation`.
        let raw = unsafe { n::mlirModuleGetOperation(self.raw) };
        Operation {
            raw,
            owns: false,
            _ownership: PhantomData,
            _context: PhantomData,
        }
    }

    /// Transfer the module handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy the module exactly once and keep its context
    /// alive until destruction.
    pub unsafe fn into_raw(self) -> n::MlirModule {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl fmt::Display for Module<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.operation(), f)
    }
}

impl Drop for Module<'_> {
    fn drop(&mut self) {
        // SAFETY: `self` uniquely owns the native handle released by `mlirModuleDestroy`.
        unsafe { n::mlirModuleDestroy(self.raw) };
    }
}
