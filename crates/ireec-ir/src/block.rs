//! MLIR blocks and their context-scoped values.
use crate::{Borrowed, Context, Error, Location, Operation, Owned, Result, Type, Value};
use iree_native as n;
use std::marker::PhantomData;

/// An MLIR basic block, owned or borrowed from a region.
pub struct Block<'ctx, Ownership = Owned> {
    pub(crate) raw: n::MlirBlock,
    pub(crate) context: n::MlirContext,
    owns: bool,
    _context: PhantomData<&'ctx ()>,
    _ownership: PhantomData<Ownership>,
}

impl<'ctx> Block<'ctx, Owned> {
    /// Create an empty block with the requested arguments.
    pub fn new<ContextOwnership>(
        context: &'ctx Context<'_, ContextOwnership>,
        argument_types: &[Type<'_>],
        locations: &[Location<'_>],
    ) -> Result<Block<'ctx, Owned>> {
        if argument_types.len() != locations.len() {
            return Err(Error::Invalid(
                "block argument and location counts differ".into(),
            ));
        }
        for ty in argument_types {
            // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeGetContext`.
            if unsafe { n::mlirTypeGetContext(ty.raw) }.ptr != context.raw.ptr {
                return Err(Error::Invalid(
                    "block argument type belongs to another context".into(),
                ));
            }
        }
        for location in locations {
            // SAFETY: `location` is a live native handle; their wrappers keep them valid throughout `mlirLocationGetContext`.
            if unsafe { n::mlirLocationGetContext(location.raw) }.ptr != context.raw.ptr {
                return Err(Error::Invalid(
                    "block argument location belongs to another context".into(),
                ));
            }
        }
        let argument_count = isize::try_from(argument_types.len())
            .map_err(|_| Error::Invalid("too many block arguments".into()))?;
        let types: Vec<n::MlirType> = argument_types.iter().map(|ty| ty.raw).collect();
        let raw_locations: Vec<n::MlirLocation> =
            locations.iter().map(|location| location.raw).collect();
        let raw =
// SAFETY: `types` and `raw_locations` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
            unsafe { n::mlirBlockCreate(argument_count, types.as_ptr(), raw_locations.as_ptr()) };
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR block"));
        }
        Ok(Block {
            raw,
            context: context.raw,
            owns: true,
            _context: PhantomData,
            _ownership: PhantomData,
        })
    }

    /// Take ownership of a block.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null, uniquely owned block in `context`.
    pub unsafe fn from_raw<ContextOwnership>(
        context: &'ctx Context<'_, ContextOwnership>,
        raw: n::MlirBlock,
    ) -> Result<Block<'ctx, Owned>> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR block"));
        }
        Ok(Block {
            raw,
            context: context.raw,
            owns: true,
            _context: PhantomData,
            _ownership: PhantomData,
        })
    }

    /// Transfer the block handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy this block exactly once.
    pub unsafe fn into_raw(self) -> n::MlirBlock {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl<'ctx, Ownership> Block<'ctx, Ownership> {
    pub(crate) fn borrowed(raw: n::MlirBlock, context: n::MlirContext) -> Block<'ctx, Borrowed> {
        Block {
            raw,
            context,
            owns: false,
            _context: PhantomData,
            _ownership: PhantomData,
        }
    }

    /// The borrowed native block handle.
    pub fn as_raw(&self) -> n::MlirBlock {
        self.raw
    }

    /// Number of block arguments.
    pub fn num_arguments(&self) -> Result<usize> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirBlockGetNumArguments`.
        usize::try_from(unsafe { n::mlirBlockGetNumArguments(self.raw) })
            .map_err(|_| Error::Invalid("MLIR returned a negative block argument count".into()))
    }

    /// Borrow a block argument by index.
    pub fn argument(&self, index: usize) -> Result<Option<Value<'_>>> {
        if index >= self.num_arguments()? {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirBlockGetArgument`.
        let raw = unsafe { n::mlirBlockGetArgument(self.raw, index as isize) };
        // SAFETY: `mlirValueIsNull` accepts a nullable value handle and only tests its null sentinel.
        if unsafe { n::mlirValueIsNull(raw) } {
            return Ok(None);
        }
        Ok(Some(Value::borrowed(raw, self.context)))
    }

    /// Append an owned operation, transferring its ownership to this block.
    pub fn append_operation(&mut self, operation: Operation<'ctx, Owned>) -> Result<()> {
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
        let op_context = unsafe { n::mlirOperationGetContext(operation.raw) };
        if op_context.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "operation belongs to another MLIR context".into(),
            ));
        }
        let operation = std::mem::ManuallyDrop::new(operation);
        // SAFETY: `operation` is uniquely owned here, and appending transfers ownership to this block exactly once.
        unsafe { n::mlirBlockAppendOwnedOperation(self.raw, operation.raw) };
        Ok(())
    }
}

impl<Ownership> Drop for Block<'_, Ownership> {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: `self` uniquely owns the native handle released by `mlirBlockDestroy`.
            unsafe { n::mlirBlockDestroy(self.raw) };
        }
    }
}
