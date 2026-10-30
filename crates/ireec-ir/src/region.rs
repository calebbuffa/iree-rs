//! MLIR regions and their owned block lists.
use crate::{Borrowed, Context, Error, Owned, Result, block::Block};
use iree_native as n;
use std::marker::PhantomData;

/// An MLIR region with explicit ownership and a context lifetime.
pub struct Region<'ctx, Ownership = Owned> {
    raw: n::MlirRegion,
    context: n::MlirContext,
    owns: bool,
    _context: PhantomData<&'ctx ()>,
    _ownership: PhantomData<Ownership>,
}

impl<'ctx> Region<'ctx, Owned> {
    /// Create an empty, owned region associated with `context`.
    pub fn new<ContextOwnership>(
        context: &'ctx Context<'_, ContextOwnership>,
    ) -> Result<Region<'ctx, Owned>> {
        // SAFETY: this constructor takes no inputs; the newly allocated region handle is null-checked before wrapping.
        let raw = unsafe { n::mlirRegionCreate() };
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR region"));
        }
        Ok(Region {
            raw,
            context: context.raw,
            owns: true,
            _context: PhantomData,
            _ownership: PhantomData,
        })
    }

    /// Take ownership of a region.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null region uniquely owned by the caller and
    /// associated with the supplied context.
    pub unsafe fn from_raw<ContextOwnership>(
        context: &'ctx Context<'_, ContextOwnership>,
        raw: n::MlirRegion,
    ) -> Result<Region<'ctx, Owned>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR region"));
        }
        Ok(Region {
            raw,
            context: context.raw,
            owns: true,
            _context: PhantomData,
            _ownership: PhantomData,
        })
    }

    /// Transfer the region handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy this region exactly once.
    pub unsafe fn into_raw(self) -> n::MlirRegion {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl<'ctx, Ownership> Region<'ctx, Ownership> {
    pub(crate) fn context_raw(&self) -> n::MlirContext {
        self.context
    }

    /// The borrowed native region handle.
    pub fn as_raw(&self) -> n::MlirRegion {
        self.raw
    }

    /// Append an owned block, transferring its ownership to this region.
    pub fn append_block(&mut self, block: Block<'ctx, Owned>) -> Result<()> {
        if block.context.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "block belongs to another MLIR context".into(),
            ));
        }
        let block = std::mem::ManuallyDrop::new(block);
        // SAFETY: `mlirRegionAppendOwnedBlock` performs the documented single ownership transfer of its live native handle.
        unsafe { n::mlirRegionAppendOwnedBlock(self.raw, block.raw) };
        Ok(())
    }

    /// Borrow the first block in this region, if one exists.
    pub fn first_block(&self) -> Option<Block<'_, Borrowed>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirRegionGetFirstBlock`.
        let raw = unsafe { n::mlirRegionGetFirstBlock(self.raw) };
        if raw.ptr.is_null() {
            return None;
        }
        Some(Block::<'_, Borrowed>::borrowed(raw, self.context))
    }
}

impl<Ownership> Drop for Region<'_, Ownership> {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: `self` uniquely owns the native handle released by `mlirRegionDestroy`.
            unsafe { n::mlirRegionDestroy(self.raw) };
        }
    }
}
