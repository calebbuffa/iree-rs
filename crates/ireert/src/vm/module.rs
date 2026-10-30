//! VM module ownership and reflection.
use super::{Function, FunctionLinkage, Instance};
use crate::hal::DeviceGroup;
use crate::{Code, Error, Result, check};
use core::ffi::c_void;
use iree_native as n;
use n::view_string;

/// A retained VM module handle.
pub struct Module {
    raw: *mut n::iree_vm_module_t,
    // Modules resolve types from their instance without retaining it.
    _instance: Option<Instance>,
    _device_group: Option<DeviceGroup>,
}

/// Public metadata from a module signature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleSignature {
    /// Version used during module dependency resolution.
    pub version: u32,
    /// Number of module reflection attributes.
    pub attribute_count: usize,
    /// Number of imported functions.
    pub import_function_count: usize,
    /// Number of exported functions.
    pub export_function_count: usize,
    /// Number of internal functions.
    pub internal_function_count: usize,
}

/// One dependency declared by a module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleDependency {
    /// Fully qualified dependency module name.
    pub name: String,
    /// Minimum compatible dependency version.
    pub minimum_version: u32,
    /// Whether the module can run without this dependency.
    pub optional: bool,
}

impl Module {
    /// Adopts an owned module pointer.
    ///
    /// # Safety
    /// `raw` must be a live, owned module reference. Its instance and any
    /// storage it references must outlive its final native reference.
    pub unsafe fn from_raw(raw: *mut n::iree_vm_module_t) -> Self {
        Self {
            raw,
            _instance: None,
            _device_group: None,
        }
    }

    /// Adopts an owned module reference whose types belong to `instance`.
    pub(crate) unsafe fn from_parts(raw: *mut n::iree_vm_module_t, instance: Instance) -> Self {
        Self {
            raw,
            _instance: Some(instance),
            _device_group: None,
        }
    }

    pub(crate) fn retain_device_group(&mut self, group: DeviceGroup) {
        self._device_group = Some(group);
    }

    pub(crate) fn device_group(&self) -> Option<&DeviceGroup> {
        self._device_group.as_ref()
    }

    /// Borrows the native module pointer.
    pub fn as_raw(&self) -> *mut n::iree_vm_module_t {
        self.raw
    }

    /// Transfers the module reference to the caller, who must keep its
    /// instance alive while the reference is used.
    pub fn into_raw(self) -> *mut n::iree_vm_module_t {
        let mut this = core::mem::ManuallyDrop::new(self);
        this._instance.take();
        this._device_group.take();
        this.raw
    }

    /// Returns the module name.
    pub fn name(&self) -> String {
        // SAFETY: the name view lives as long as the module.
        unsafe { view_string(n::iree_vm_module_name(self.raw)) }
    }

    /// Returns module version and function/attribute counts.
    pub fn signature(&self) -> ModuleSignature {
        // SAFETY: the module is live.
        let raw = unsafe { n::iree_vm_module_signature(self.raw) };
        ModuleSignature {
            version: raw.version,
            attribute_count: raw.attr_count,
            import_function_count: raw.import_function_count,
            export_function_count: raw.export_function_count,
            internal_function_count: raw.internal_function_count,
        }
    }

    /// Looks up a function by linkage and name.
    pub fn lookup_function(&self, linkage: FunctionLinkage, name: &str) -> Result<Function> {
        let mut raw = n::iree_vm_function_t::default();
        // SAFETY: the module, name view and output are valid for the call.
        check(unsafe {
            n::iree_vm_module_lookup_function_by_name(
                self.raw,
                linkage.raw(),
                crate::view(name),
                &mut raw,
            )
        })?;
        // SAFETY: the descriptor belongs to this module, which the function retains.
        Ok(unsafe { Function::from_module(raw, self.clone()) })
    }

    /// Looks up a function by linkage and ordinal.
    pub fn function_at(&self, linkage: FunctionLinkage, ordinal: usize) -> Result<Function> {
        let mut raw = n::iree_vm_function_t::default();
        // SAFETY: the module and output are valid for the call.
        check(unsafe {
            n::iree_vm_module_lookup_function_by_ordinal(self.raw, linkage.raw(), ordinal, &mut raw)
        })?;
        // SAFETY: the descriptor belongs to this module, which the function retains.
        Ok(unsafe { Function::from_module(raw, self.clone()) })
    }

    /// Looks up a module attribute by name.
    pub fn attribute(&self, key: &str) -> Option<String> {
        // SAFETY: the module and key view are valid; the result lives as long
        // as the module.
        let value = unsafe { n::iree_vm_module_lookup_attr_by_name(self.raw, crate::view(key)) };
        // SAFETY: as above.
        (!value.data.is_null()).then(|| unsafe { view_string(value) })
    }

    /// Returns the attribute key and value at `index`.
    pub fn attribute_at(&self, index: usize) -> Result<(String, String)> {
        let mut pair = n::iree_string_pair_t::default();
        // SAFETY: the module and output are valid for the call.
        check(unsafe { n::iree_vm_module_get_attr(self.raw, index, &mut pair) })?;
        // SAFETY: the pair views live as long as the module.
        Ok(unsafe {
            (
                view_string(pair.__bindgen_anon_1.key),
                view_string(pair.__bindgen_anon_2.value),
            )
        })
    }

    /// Returns the dependencies declared by this module.
    pub fn dependencies(&self) -> Result<Vec<ModuleDependency>> {
        unsafe extern "C" fn collect(
            user_data: *mut c_void,
            dependency: *const n::iree_vm_module_dependency_t,
        ) -> n::iree_status_t {
            n::callback_status(|| {
                // SAFETY: native passes the live vector and dependency for this call.
                let (values, dependency) = unsafe {
                    (
                        &mut *user_data.cast::<Vec<ModuleDependency>>(),
                        &*dependency,
                    )
                };
                values.push(ModuleDependency {
                    // SAFETY: the name view is valid during the callback.
                    name: unsafe { view_string(dependency.name) },
                    minimum_version: dependency.minimum_version,
                    optional: dependency.flags & n::IREE_VM_MODULE_DEPENDENCY_FLAG_OPTIONAL as u32
                        != 0,
                });
                Ok(())
            })
        }
        let mut values = Vec::<ModuleDependency>::new();
        // SAFETY: `values` outlives the synchronous enumeration.
        check(unsafe {
            n::iree_vm_module_enumerate_dependencies(
                self.raw,
                Some(collect),
                (&mut values as *mut Vec<ModuleDependency>).cast(),
            )
        })?;
        Ok(values)
    }

    /// Formats the source location of `pc` within `function`, or `None` when
    /// the module has no location information.
    pub fn source_location(
        &self,
        function: &Function,
        pc: i64,
        single_line: bool,
    ) -> Result<Option<String>> {
        if function.module_ptr() != self.raw {
            return Err(Error::new(
                Code::InvalidArgument,
                "function belongs to another module",
            ));
        }
        let mut location = n::iree_vm_source_location_t::default();
        // SAFETY: the function belongs to this module and the output is writable.
        check(unsafe {
            n::iree_vm_module_resolve_source_location(
                self.raw,
                *function.as_raw(),
                pc,
                &mut location,
            )
        })?;
        if location.format.is_none() {
            return Ok(None);
        }
        let flags = if single_line {
            n::IREE_VM_SOURCE_LOCATION_FORMAT_FLAG_SINGLE_LINE
        } else {
            n::IREE_VM_SOURCE_LOCATION_FORMAT_FLAG_NONE
        } as n::iree_vm_source_location_format_flags_t;
        // SAFETY: the resolved location refers to this live module.
        crate::base::string_builder::format(|builder| unsafe {
            n::iree_vm_source_location_format(&mut location, flags, builder)
        })
        .map(Some)
    }
}

impl Clone for Module {
    fn clone(&self) -> Self {
        // SAFETY: `self.raw` is a live ref-counted module.
        unsafe { n::iree_vm_module_retain(self.raw) };
        Self {
            raw: self.raw,
            _instance: self._instance.clone(),
            _device_group: self._device_group.clone(),
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one module reference; the instance field
        // drops afterwards.
        unsafe { n::iree_vm_module_release(self.raw) }
    }
}
