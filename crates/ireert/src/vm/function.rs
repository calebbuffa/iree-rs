//! VM function handles and reflection.
use super::Module;
use crate::{Result, check};
use iree_native as n;
use n::view_string;

/// A function handle keeping its owning module alive.
#[derive(Clone)]
pub struct Function {
    raw: n::iree_vm_function_t,
    module: Module,
}

// SAFETY: modules support concurrent lookup and execution; a Function holds
// immutable metadata and an atomically counted module reference.
unsafe impl Send for Function {}
// SAFETY: every method reads immutable metadata.
unsafe impl Sync for Function {}

impl Function {
    /// Retains the module of a borrowed function descriptor.
    ///
    /// # Safety
    /// `raw` must describe a valid function of a live module.
    pub unsafe fn from_raw(raw: n::iree_vm_function_t) -> Self {
        // SAFETY: the caller guarantees the module is live; retaining gives
        // the handle its own reference.
        let module = unsafe {
            n::iree_vm_module_retain(raw.module);
            Module::from_raw(raw.module)
        };
        Self { raw, module }
    }

    pub(crate) unsafe fn from_module(raw: n::iree_vm_function_t, module: Module) -> Self {
        Self { raw, module }
    }

    /// Borrows the native function descriptor.
    pub fn as_raw(&self) -> *const n::iree_vm_function_t {
        &self.raw
    }

    /// Returns the module that defines this function.
    pub fn module(&self) -> &Module {
        &self.module
    }

    /// The module-local function ordinal.
    pub fn ordinal(&self) -> u16 {
        self.raw.ordinal
    }

    pub(crate) fn module_ptr(&self) -> *mut n::iree_vm_module_t {
        self.raw.module
    }

    /// The function name within its module, without the module prefix.
    pub fn name(&self) -> String {
        // SAFETY: the name lives as long as the retained module.
        unsafe { view_string(n::iree_vm_function_name(&self.raw)) }
    }

    /// The VM calling-convention signature string.
    pub fn signature(&self) -> String {
        // SAFETY: the signature lives as long as the retained module.
        unsafe { view_string(n::iree_vm_function_signature(&self.raw).calling_convention) }
    }

    /// Returns the number of arguments and results in the signature.
    pub fn argument_result_counts(&self) -> Result<(usize, usize)> {
        // SAFETY: the function is valid while its module is retained.
        let signature = unsafe { n::iree_vm_function_signature(&self.raw) };
        let (mut arguments, mut results) = (0, 0);
        // SAFETY: the signature and outputs are valid for the call.
        check(unsafe {
            n::iree_vm_function_call_count_arguments_and_results(
                &signature,
                &mut arguments,
                &mut results,
            )
        })?;
        Ok((arguments, results))
    }

    /// Looks up a function attribute by key.
    pub fn attribute(&self, key: &str) -> Option<String> {
        // SAFETY: the key view is valid; the result lives as long as the module.
        let value = unsafe { n::iree_vm_function_lookup_attr_by_name(&self.raw, crate::view(key)) };
        // SAFETY: as above.
        (!value.data.is_null()).then(|| unsafe { view_string(value) })
    }

    /// Returns the attribute key and value at `index`.
    pub fn attribute_at(&self, index: usize) -> Result<(String, String)> {
        let mut pair = n::iree_string_pair_t::default();
        // SAFETY: the function and output are valid for the call.
        check(unsafe { n::iree_vm_function_get_attr(self.raw, index, &mut pair) })?;
        // SAFETY: the pair views live as long as the module.
        Ok(unsafe {
            (
                view_string(pair.__bindgen_anon_1.key),
                view_string(pair.__bindgen_anon_2.value),
            )
        })
    }
}

/// VM function visibility for module lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FunctionLinkage {
    /// Module-private function.
    Internal,
    /// Imported function.
    Import,
    /// Exported function.
    Export,
    /// Optional imported function.
    ImportOptional,
    /// Optional exported function.
    ExportOptional,
}

impl FunctionLinkage {
    pub(crate) fn raw(self) -> n::iree_vm_function_linkage_t {
        match self {
            Self::Internal => n::IREE_VM_FUNCTION_LINKAGE_INTERNAL,
            Self::Import => n::IREE_VM_FUNCTION_LINKAGE_IMPORT,
            Self::Export => n::IREE_VM_FUNCTION_LINKAGE_EXPORT,
            Self::ImportOptional => n::IREE_VM_FUNCTION_LINKAGE_IMPORT_OPTIONAL,
            Self::ExportOptional => n::IREE_VM_FUNCTION_LINKAGE_EXPORT_OPTIONAL,
        }
    }
}
