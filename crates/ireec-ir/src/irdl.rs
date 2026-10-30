//! # Laws
//! Loading IRDL definitions affects only the module's context; any definitions
//! loaded before a native failure remain registered in that context.

use crate::{Borrowed, Context, Dialect, DialectHandle, Error, Module, Result};
use iree_native as n;

impl DialectHandle {
    /// Return MLIR's IRDL dialect handle.
    pub fn irdl() -> Result<Self> {
        // SAFETY: this parameterless getter returns the static IRDL dialect handle by value.
        unsafe { Self::from_raw(n::mlirGetDialectHandle__irdl__()) }
    }
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Register and load the IRDL dialect in this context.
    pub fn load_irdl_dialect(&self) -> Result<Dialect<'_>> {
        let handle = DialectHandle::irdl()?;
        handle.register(self);
        handle.load(self)
    }
}

impl Module<'_> {
    /// Load IRDL dialect definitions declared by this module.
    ///
    /// Loading is not transactional: if the native loader fails after registering
    /// some definitions, those definitions remain in the context.
    pub fn load_irdl_dialects(&self) -> Result<()> {
        let operation = self.operation();
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
        let context_raw = unsafe { n::mlirOperationGetContext(operation.as_raw()) };
        // SAFETY: the context handle comes from the live receiver and is borrowed only for the receiver-bound lifetime.
        let context: Context<'_, Borrowed> = unsafe { Context::borrow_raw(context_raw)? };
        let handle = DialectHandle::irdl()?;
        handle.register(&context);
        let _dialect = handle.load(&context)?;
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirModuleFromOperation`.
        let module = unsafe { n::mlirModuleFromOperation(operation.as_raw()) };
        if module.ptr.is_null() {
            return Err(Error::Invalid(
                "module operation is not a builtin module".into(),
            ));
        }
        // SAFETY: `module` is a live native handle; their wrappers keep them valid throughout `mlirLoadIRDLDialects`.
        let result = unsafe { n::mlirLoadIRDLDialects(module) };
        if n::compiler::logical_result::is_success(result) {
            Ok(())
        } else {
            Err(Error::Invalid(
                "MLIR failed to load IRDL dialect definitions".into(),
            ))
        }
    }
}
