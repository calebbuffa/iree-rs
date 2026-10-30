//! Diagnostic handles are borrowed only during callbacks; handler guards detach
//! before their context can be destroyed, and callback panics never cross C.

use crate::{
    Context, Location,
    callback::{contain, print_with},
};
use iree_native as n;
use std::{
    ffi::c_void,
    marker::PhantomData,
    sync::{Mutex, PoisonError},
};

/// The severity reported by MLIR.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSeverity {
    /// A compilation error.
    Error,
    /// A warning.
    Warning,
    /// An informational note.
    Note,
    /// An optimization remark.
    Remark,
    /// A future severity unknown to this binding.
    Other,
}

/// A diagnostic view valid only while its context handler is invoked.
pub struct Diagnostic<'callback> {
    raw: n::MlirDiagnostic,
    _callback: PhantomData<&'callback mut ()>,
}

/// A context-bound diagnostic handler, detached when this guard is dropped.
pub struct DiagnosticHandler<'ctx> {
    context: n::MlirContext,
    id: n::MlirDiagnosticHandlerID,
    _context: PhantomData<&'ctx ()>,
}

type HandlerState = Mutex<Box<dyn for<'d> FnMut(&Diagnostic<'d>) -> bool + Send>>;

impl<'callback> Diagnostic<'callback> {
    /// Borrow the native diagnostic handle.
    pub fn as_raw(&self) -> n::MlirDiagnostic {
        self.raw
    }

    /// Return the diagnostic severity.
    pub fn severity(&self) -> DiagnosticSeverity {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDiagnosticGetSeverity`.
        match unsafe { n::mlirDiagnosticGetSeverity(self.raw) } {
            n::MlirDiagnosticError => DiagnosticSeverity::Error,
            n::MlirDiagnosticWarning => DiagnosticSeverity::Warning,
            n::MlirDiagnosticNote => DiagnosticSeverity::Note,
            n::MlirDiagnosticRemark => DiagnosticSeverity::Remark,
            _ => DiagnosticSeverity::Other,
        }
    }

    /// Borrow the source location for this callback's lifetime.
    pub fn location(&self) -> Location<'callback> {
        Location {
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirDiagnosticGetLocation`.
            raw: unsafe { n::mlirDiagnosticGetLocation(self.raw) },
            _context: PhantomData,
        }
    }

    /// Copy the native diagnostic's printed representation.
    pub fn message(&self) -> String {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDiagnosticPrint`.
        print_with(|callback, data| unsafe { n::mlirDiagnosticPrint(self.raw, callback, data) })
    }

    /// Return borrowed views of attached notes.
    pub fn notes(&self) -> Vec<Diagnostic<'callback>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirDiagnosticGetNumNotes`.
        let count = unsafe { n::mlirDiagnosticGetNumNotes(self.raw) };
        let Ok(count) = usize::try_from(count) else {
            return Vec::new();
        };
        (0..count)
            .map(|index| Diagnostic {
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirDiagnosticGetNote`.
                raw: unsafe { n::mlirDiagnosticGetNote(self.raw, index as isize) },
                _callback: PhantomData,
            })
            .collect()
    }
}

impl<'ctx> DiagnosticHandler<'ctx> {
    /// Attach a handler to `context`.
    ///
    /// Return `true` to mark a diagnostic handled; return `false` to allow
    /// later handlers to process it. Panics are contained and treated as false.
    pub fn attach<Ctx>(
        context: &'ctx Context<'_, Ctx>,
        callback: impl for<'d> FnMut(&Diagnostic<'d>) -> bool + Send + 'static,
    ) -> Self {
        let state: Box<HandlerState> = Box::new(Mutex::new(Box::new(callback)));
        let data = Box::into_raw(state).cast::<c_void>();
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirContextAttachDiagnosticHandler`.
        let id = unsafe {
            n::mlirContextAttachDiagnosticHandler(
                context.raw,
                Some(dispatch),
                data,
                Some(delete_state),
            )
        };
        Self {
            context: context.raw,
            id,
            _context: PhantomData,
        }
    }
}

impl Drop for DiagnosticHandler<'_> {
    fn drop(&mut self) {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirContextDetachDiagnosticHandler`.
        unsafe { n::mlirContextDetachDiagnosticHandler(self.context, self.id) };
    }
}

unsafe extern "C" fn dispatch(raw: n::MlirDiagnostic, data: *mut c_void) -> n::MlirLogicalResult {
    // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
    let state = unsafe { &*data.cast::<HandlerState>() };
    let handled = contain(|| {
        let mut callback = state.lock().unwrap_or_else(PoisonError::into_inner);
        callback(&Diagnostic {
            raw,
            _callback: PhantomData,
        })
    });
    if handled == Some(true) {
        n::compiler::logical_result::success()
    } else {
        n::compiler::logical_result::failure()
    }
}

unsafe extern "C" fn delete_state(data: *mut c_void) {
    // SAFETY: this deleter reconstructs the unique Box pointer transferred at registration, and the native API invokes it once.
    contain(|| drop(unsafe { Box::from_raw(data.cast::<HandlerState>()) }));
}
