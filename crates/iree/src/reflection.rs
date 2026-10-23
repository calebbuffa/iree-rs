//! Owned VM reflection metadata without borrowed native storage.

use crate::{Result, Session, StatusCode, driver::owned_string, runtime::check, sys};

/// One copied module or function reflection attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    /// Native attribute name.
    pub key: String,
    /// Native attribute value, including an empty value if present.
    pub value: String,
}

impl Attribute {
    fn copy(pair: sys::iree_string_pair_t) -> Self {
        unsafe {
            Self {
                key: owned_string(pair.__bindgen_anon_1.key),
                value: owned_string(pair.__bindgen_anon_2.value),
            }
        }
    }
}

/// Owned signature and reflection metadata for an exported VM function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionInfo {
    /// Module-qualified name accepted by session invocation.
    pub name: String,
    /// Native VM calling convention, including its version prefix.
    ///
    /// This is the low-level ABI, not a reconstructed MLIR source signature.
    /// For example, coarse-fence exports include their two fence references.
    pub calling_convention: String,
    /// All available reflection attributes.
    pub attributes: Vec<Attribute>,
}

impl FunctionInfo {
    pub(crate) fn copy(function: sys::iree_vm_function_t) -> Result<Self> {
        let module = unsafe { owned_string(sys::iree_vm_module_name(function.module)) };
        let local_name = unsafe { owned_string(sys::iree_vm_function_name(&function)) };
        let signature = unsafe { sys::iree_vm_function_signature(&function) };
        let mut attributes = Vec::new();
        loop {
            let mut pair = sys::iree_string_pair_t::default();
            match check(unsafe {
                sys::iree_vm_function_get_attr(function, attributes.len(), &mut pair)
            }) {
                Ok(()) => attributes.push(Attribute::copy(pair)),
                Err(error) if error.code() == StatusCode::OUT_OF_RANGE => break,
                Err(error) => return Err(error),
            }
        }
        Ok(Self {
            name: format!("{module}.{local_name}"),
            calling_convention: unsafe { owned_string(signature.calling_convention) },
            attributes,
        })
    }
}

/// Owned reflection metadata for a module registered in a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInfo {
    /// Native module namespace.
    pub name: String,
    /// Native module ABI version.
    pub version: u32,
    /// Number of imported functions, including optional imports.
    pub import_count: usize,
    /// Number of internal functions visible through native debug metadata.
    pub internal_count: usize,
    /// All available module-level reflection attributes.
    pub attributes: Vec<Attribute>,
    /// Exported functions and their low-level signatures.
    pub exports: Vec<FunctionInfo>,
}

impl ModuleInfo {
    pub(crate) fn copy(module: *mut sys::iree_vm_module_t) -> Result<Self> {
        let signature = unsafe { sys::iree_vm_module_signature(module) };
        let attributes = (0..signature.attr_count)
            .map(|index| {
                let mut pair = sys::iree_string_pair_t::default();
                check(unsafe { sys::iree_vm_module_get_attr(module, index, &mut pair) })?;
                Ok(Attribute::copy(pair))
            })
            .collect::<Result<_>>()?;
        let exports = (0..signature.export_function_count)
            .map(|ordinal| {
                let mut function = sys::iree_vm_function_t::default();
                check(unsafe {
                    sys::iree_vm_module_lookup_function_by_ordinal(
                        module,
                        sys::IREE_VM_FUNCTION_LINKAGE_EXPORT,
                        ordinal,
                        &mut function,
                    )
                })?;
                FunctionInfo::copy(function)
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            name: unsafe { owned_string(sys::iree_vm_module_name(module)) },
            version: signature.version,
            import_count: signature.import_function_count,
            internal_count: signature.internal_function_count,
            attributes,
            exports,
        })
    }
}

impl Session {
    /// Copy module and exported-function metadata out of this VM context.
    ///
    /// Includes supporting modules such as HAL and parameter I/O, not only the
    /// loaded bytecode module. Returned values remain valid after session drop.
    pub fn modules(&self) -> Result<Vec<ModuleInfo>> {
        let context = self.as_raw();
        let count = unsafe { sys::iree_vm_context_module_count(context) };
        (0..count)
            .map(|index| {
                let module = unsafe { sys::iree_vm_context_module_at(context, index) };
                ModuleInfo::copy(module)
            })
            .collect()
    }

    /// Copy one exported function's low-level signature and all attributes.
    pub fn function(&self, name: &str) -> Result<FunctionInfo> {
        FunctionInfo::copy(self.resolve(name)?)
    }
}
