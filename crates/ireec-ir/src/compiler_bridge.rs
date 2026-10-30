//! Compiler-invocation import/export bridging for MLIR modules.
use crate::{Error, Module, Result};
use iree_native as n;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// Extension methods that transfer MLIR modules through an invocation.
pub trait InvocationExt<'session> {
    /// Import a module by reference while retaining its borrow through the
    /// invocation's lifetime.
    fn import_borrow<'module, 'ctx>(
        self,
        module: &'module Module<'ctx>,
    ) -> Result<BorrowedInvocation<'module, 'ctx, 'session>>;

    /// Import a module by transferring its ownership to the invocation.
    fn import_steal(self, module: Module<'_>) -> Result<ireec::Invocation<'session>>;

    /// Export the invocation's owned module, if available.
    fn export_steal(&mut self) -> Result<Module<'session>>;
}

/// An invocation that retains a borrowed module until the invocation is dropped.
pub struct BorrowedInvocation<'module, 'ctx, 'session> {
    invocation: ireec::Invocation<'session>,
    _module: &'module Module<'ctx>,
}

impl<'module, 'ctx, 'session> Deref for BorrowedInvocation<'module, 'ctx, 'session> {
    type Target = ireec::Invocation<'session>;

    fn deref(&self) -> &Self::Target {
        &self.invocation
    }
}

impl DerefMut for BorrowedInvocation<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.invocation
    }
}

fn same_session_context(
    session: *mut n::iree_compiler_session_t,
    module: n::MlirOperation,
) -> bool {
    if session.is_null() || module.ptr.is_null() {
        return false;
    }
    // SAFETY: `session` is a live native handle; their wrappers keep them valid throughout `ireeCompilerSessionBorrowContext`.
    let session_context = unsafe { n::ireeCompilerSessionBorrowContext(session) };
    // SAFETY: `module` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
    let module_context = unsafe { n::mlirOperationGetContext(module) };
    !session_context.ptr.is_null() && session_context.ptr == module_context.ptr
}

impl<'session> InvocationExt<'session> for ireec::Invocation<'session> {
    fn import_borrow<'module, 'ctx>(
        self,
        module: &'module Module<'ctx>,
    ) -> Result<BorrowedInvocation<'module, 'ctx, 'session>> {
        // SAFETY: `module` is a live native handle; their wrappers keep them valid throughout `mlirModuleGetOperation`.
        let module_op = unsafe { n::mlirModuleGetOperation(module.raw) };
        if !same_session_context(self.session_as_raw(), module_op) {
            return Err(Error::Invalid(
                "module must belong to the invocation's compiler session context".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeCompilerInvocationImportBorrowModule`.
        if !unsafe { n::ireeCompilerInvocationImportBorrowModule(self.as_raw(), module_op) } {
            return Err(ireec::Error::Compile(self.diagnostics()).into());
        }
        Ok(BorrowedInvocation {
            invocation: self,
            _module: module,
        })
    }

    fn import_steal(self, module: Module<'_>) -> Result<ireec::Invocation<'session>> {
        // SAFETY: `module` is a live native handle; their wrappers keep them valid throughout `mlirModuleGetOperation`.
        let module_op = unsafe { n::mlirModuleGetOperation(module.raw) };
        if !same_session_context(self.session_as_raw(), module_op) {
            return Err(Error::Invalid(
                "module must belong to the invocation's compiler session context".into(),
            ));
        }
        // The invocation takes ownership whether or not the import succeeds.
        std::mem::forget(module);
        // SAFETY: the module owner was deliberately forgotten, so `ireeCompilerInvocationImportStealModule` can consume its still-live operation handle once.
        if !unsafe { n::ireeCompilerInvocationImportStealModule(self.as_raw(), module_op) } {
            return Err(ireec::Error::Compile(self.diagnostics()).into());
        }
        Ok(self)
    }

    fn export_steal(&mut self) -> Result<Module<'session>> {
        // SAFETY: the live invocation owns the module; `ireeCompilerInvocationExportStealModule` transfers its native handle once to the returned operation.
        let operation = unsafe { n::ireeCompilerInvocationExportStealModule(self.as_raw()) };
        if operation.ptr.is_null() {
            return Err(Error::Null("exported module operation"));
        }
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirModuleFromOperation`.
        let module = unsafe { n::mlirModuleFromOperation(operation) };
        if module.ptr.is_null() {
            // SAFETY: `operation` is uniquely owned here, and `mlirOperationDestroy` releases it exactly once.
            unsafe { n::mlirOperationDestroy(operation) };
            return Err(Error::Null("exported module"));
        }
        Ok(Module {
            raw: module,
            _context: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_transfer_rejects_an_independent_context() -> Result<()> {
        let compiler = ireec::Compiler::new()?;
        let session = compiler.session()?;
        let context = crate::Context::new()?;
        let module = Module::parse(&context, "module {}")?;

        let invocation = session.invocation()?;
        assert!(matches!(
            invocation.import_steal(module),
            Err(Error::Invalid(_))
        ));

        let module = Module::parse(&context, "module {}")?;
        let invocation = session.invocation()?;
        assert!(matches!(
            invocation.import_borrow(&module),
            Err(Error::Invalid(_))
        ));
        Ok(())
    }
}
