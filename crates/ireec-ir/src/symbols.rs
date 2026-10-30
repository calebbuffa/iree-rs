//! A symbol-table wrapper owns its native table handle but borrows the IR
//! operation that defines the table; returned operations remain borrowed views.

use crate::{
    Attribute, Borrowed, Error, Operation, Result,
    string_ref::{copy_string_ref, string_ref},
};
use iree_native as n;
use std::{ffi::c_void, marker::PhantomData, ptr};

/// An owned native symbol table borrowing its defining operation.
pub struct SymbolTable<'ir> {
    raw: n::MlirSymbolTable,
    context: n::MlirContext,
    _operation: PhantomData<&'ir ()>,
}

impl<'ir> SymbolTable<'ir> {
    /// Create a table for an operation with the SymbolTable trait.
    pub fn new<Ownership>(operation: &'ir Operation<'_, Ownership>) -> Result<Self> {
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirSymbolTableCreate`.
        let raw = unsafe { n::mlirSymbolTableCreate(operation.raw) };
        if raw.ptr.is_null() {
            return Err(Error::Invalid(
                "operation does not define an MLIR symbol table".into(),
            ));
        }
        Ok(Self {
            raw,
            // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
            context: unsafe { n::mlirOperationGetContext(operation.raw) },
            _operation: PhantomData,
        })
    }

    /// Take ownership of a native table associated with `operation`.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null table created for `operation` and must not be
    /// owned elsewhere.
    pub unsafe fn from_raw<Ownership>(
        operation: &'ir Operation<'_, Ownership>,
        raw: n::MlirSymbolTable,
    ) -> Result<Self> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR symbol table"));
        }
        Ok(Self {
            raw,
            // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
            context: unsafe { n::mlirOperationGetContext(operation.raw) },
            _operation: PhantomData,
        })
    }

    /// Borrow the native symbol-table handle.
    pub fn as_raw(&self) -> n::MlirSymbolTable {
        self.raw
    }

    /// Transfer the table handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy the table exactly once and keep the defining
    /// operation alive until then.
    pub unsafe fn into_raw(mut self) -> n::MlirSymbolTable {
        let raw = self.raw;
        self.raw = n::MlirSymbolTable {
            ptr: ptr::null_mut(),
        };
        raw
    }

    /// Look up a flat symbol name and return its borrowed operation.
    pub fn lookup(&self, name: &str) -> Result<Option<Operation<'ir, Borrowed>>> {
        validate_name(name)?;
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe { n::mlirSymbolTableLookup(self.raw, string_ref(name)) };
        if raw.ptr.is_null() {
            Ok(None)
        } else {
            Ok(Some(Operation {
                raw,
                owns: false,
                _ownership: PhantomData,
                _context: PhantomData,
            }))
        }
    }

    /// Insert a symbol operation and return the resulting symbol-name attribute.
    ///
    /// The operation is not moved into the table's parent; its block placement
    /// remains the caller's responsibility. MLIR may rename duplicate symbols.
    pub fn insert<Ownership>(
        &self,
        operation: &Operation<'ir, Ownership>,
    ) -> Result<Attribute<'ir>> {
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
        if unsafe { n::mlirOperationGetContext(operation.raw) }.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "symbol operation belongs to another MLIR context".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSymbolTableInsert`.
        let attribute = unsafe { n::mlirSymbolTableInsert(self.raw, operation.raw) };
        if attribute.ptr.is_null() {
            return Err(Error::Null("symbol name attribute"));
        }
        Ok(Attribute {
            raw: attribute,
            _context: PhantomData,
        })
    }

    /// Erase the symbol operation from the table and destroy it.
    pub fn erase(&self, operation: Operation<'ir, Borrowed>) -> Result<()> {
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
        if unsafe { n::mlirOperationGetContext(operation.raw) }.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "symbol operation belongs to another MLIR context".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSymbolTableErase`.
        unsafe { n::mlirSymbolTableErase(self.raw, operation.raw) };
        Ok(())
    }

    /// Return the native symbol-name attribute name used in operation IR.
    pub fn symbol_attribute_name<Ownership>(
        _context: &crate::Context<'_, Ownership>,
    ) -> Result<String> {
        // SAFETY: this accessor returns MLIR's process-lifetime symbol-name string reference, copied before returning.
        copy_string_ref(unsafe { n::mlirSymbolTableGetSymbolAttributeName() })
    }

    /// Return the native default-visibility attribute name.
    pub fn default_visibility_attribute_name<Ownership>(
        _context: &crate::Context<'_, Ownership>,
    ) -> Result<String> {
        // SAFETY: this accessor returns MLIR's process-lifetime visibility-name string reference, copied before returning.
        copy_string_ref(unsafe { n::mlirSymbolTableGetDefaultVisibilityAttributeName() })
    }
}

impl Drop for SymbolTable<'_> {
    fn drop(&mut self) {
        if !self.raw.ptr.is_null() {
            // SAFETY: `self` uniquely owns the native handle released by `mlirSymbolTableDestroy`.
            unsafe { n::mlirSymbolTableDestroy(self.raw) };
        }
    }
}

/// Replace symbol uses below `operation`; nested symbol tables are not traversed.
pub fn replace_all_symbol_uses<Ownership>(
    operation: &Operation<'_, Ownership>,
    old_symbol: &str,
    new_symbol: &str,
) -> Result<()> {
    validate_name(old_symbol)?;
    validate_name(new_symbol)?;
    // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
    let result = unsafe {
        n::mlirSymbolTableReplaceAllSymbolUses(
            string_ref(old_symbol),
            string_ref(new_symbol),
            operation.raw,
        )
    };
    if n::compiler::logical_result::is_success(result) {
        Ok(())
    } else {
        Err(Error::Invalid(
            "failed to replace all MLIR symbol uses".into(),
        ))
    }
}

/// A symbol-table scope found by a native walk, paired with use visibility.
pub struct SymbolTableScope<'ir> {
    /// The borrowed operation defining the symbol table.
    pub operation: Operation<'ir, Borrowed>,
    /// Whether all symbol uses within this scope are visible.
    pub all_uses_visible: bool,
}

/// Walk symbol-table operations nested within and including `operation`.
pub fn walk_symbol_tables<'ir, Ownership>(
    operation: &'ir Operation<'_, Ownership>,
    all_uses_visible: bool,
) -> Result<Vec<SymbolTableScope<'ir>>> {
    struct State<'ir> {
        scopes: Vec<SymbolTableScope<'ir>>,
        marker: PhantomData<&'ir ()>,
        failed: bool,
    }
    unsafe extern "C" fn collect(raw: n::MlirOperation, visible: bool, data: *mut c_void) {
        // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
        let state = unsafe { &mut *data.cast::<State<'_>>() };
        if state.failed {
            return;
        }
        if raw.ptr.is_null() || state.scopes.try_reserve(1).is_err() {
            state.failed = true;
            return;
        }
        state.scopes.push(SymbolTableScope {
            operation: Operation {
                raw,
                owns: false,
                _ownership: PhantomData,
                _context: PhantomData,
            },
            all_uses_visible: visible,
        });
    }
    let mut state = State {
        scopes: Vec::new(),
        marker: PhantomData,
        failed: false,
    };
    // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirSymbolTableWalkSymbolTables`.
    unsafe {
        n::mlirSymbolTableWalkSymbolTables(
            operation.raw,
            all_uses_visible,
            Some(collect),
            ptr::from_mut(&mut state).cast(),
        )
    };
    if state.failed {
        Err(Error::Invalid(
            "failed to collect MLIR symbol-table scopes".into(),
        ))
    } else {
        Ok(state.scopes)
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() || name.contains('\0') || name.starts_with('@') {
        Err(Error::Invalid(
            "expected a nonempty flat MLIR symbol name".into(),
        ))
    } else {
        Ok(())
    }
}
