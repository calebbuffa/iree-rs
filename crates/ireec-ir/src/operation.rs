//! MLIR operations, results, operands, and printing.
use crate::{Context, Error, Owned, Result, callback::print_with};
use iree_native as n;
use std::{fmt, marker::PhantomData};

/// A generic MLIR operation with an explicit ownership marker.
pub struct Operation<'ctx, Ownership = Owned> {
    pub(crate) raw: n::MlirOperation,
    pub(crate) owns: bool,
    pub(crate) _ownership: PhantomData<Ownership>,
    pub(crate) _context: PhantomData<&'ctx ()>,
}

impl<'ctx, Ownership> Operation<'ctx, Ownership> {
    pub(crate) unsafe fn owned(
        raw: n::MlirOperation,
        context: n::MlirContext,
    ) -> Result<Operation<'ctx, Owned>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR operation"));
        }
        // SAFETY: `raw` is valid under this unsafe function's caller contract for `mlirOperationGetContext`.
        if unsafe { n::mlirOperationGetContext(raw) }.ptr != context.ptr {
            return Err(Error::Invalid(
                "operation belongs to another MLIR context".into(),
            ));
        }
        Ok(Operation {
            raw,
            owns: true,
            _ownership: PhantomData,
            _context: PhantomData,
        })
    }

    /// The borrowed native operation handle.
    pub fn as_raw(&self) -> n::MlirOperation {
        self.raw
    }

    /// Print the operation in MLIR assembly form.
    pub(crate) fn print(&self) -> String {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOperationPrint`.
        print_with(|callback, user_data| unsafe {
            n::mlirOperationPrint(self.raw, callback, user_data)
        })
    }
}

impl<'ctx, Ownership> fmt::Display for Operation<'ctx, Ownership> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.print())
    }
}

impl<'ctx> Operation<'ctx, Owned> {
    /// Take ownership of an operation created in `context`.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null, uniquely owned operation created in `context`.
    pub unsafe fn from_raw<ContextOwnership>(
        _context: &'ctx Context<'_, ContextOwnership>,
        raw: n::MlirOperation,
    ) -> Result<Operation<'ctx, Owned>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR operation"));
        }
        Ok(Operation {
            raw,
            owns: true,
            _ownership: PhantomData,
            _context: PhantomData,
        })
    }

    /// Transfer the owned operation handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy the operation exactly once and keep its context
    /// alive until destruction.
    pub unsafe fn into_raw(self) -> n::MlirOperation {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl<Ownership> Drop for Operation<'_, Ownership> {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: `self` uniquely owns the native handle released by `mlirOperationDestroy`.
            unsafe { n::mlirOperationDestroy(self.raw) };
        }
    }
}
