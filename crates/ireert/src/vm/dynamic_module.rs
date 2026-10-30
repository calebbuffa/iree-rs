//! Dynamic VM module loading.
use super::{Instance, Module};
use crate::{Code, Error, Result, check, view};
use iree_native as n;
use std::path::Path;

impl Module {
    /// Loads the VM module exported as `export_name` by the dynamic library at `path`.
    ///
    /// `params` are passed to the module's create function.
    ///
    /// # Safety
    /// Loading runs the library's initializers and module factory; both must be trusted.
    /// The library may unload when the final native module reference is released.
    /// References with library-defined destructors, registered type descriptors,
    /// callbacks, and any other values that depend on the library must no longer
    /// be used after unloading. Release those values before releasing the last
    /// module reference and ensure the library unregisters its descriptors during
    /// cleanup, unless it independently pins itself for their entire lifetime.
    pub unsafe fn load_dynamic(
        instance: &Instance,
        path: &Path,
        export_name: &str,
        params: &[(&str, &str)],
    ) -> Result<Self> {
        let path = path
            .to_str()
            .ok_or_else(|| Error::new(Code::InvalidArgument, "module path is not UTF-8"))?;
        let params: Vec<_> = params
            .iter()
            .map(|(key, value)| {
                let mut pair = n::iree_string_pair_t::default();
                pair.__bindgen_anon_1.key = view(key);
                pair.__bindgen_anon_2.value = view(value);
                pair
            })
            .collect();
        let mut raw = std::ptr::null_mut();
        // SAFETY: all views, the params array and the output outlive the call;
        // the caller trusts the library.
        check(unsafe {
            n::iree_vm_dynamic_module_load_from_file(
                instance.as_raw(),
                view(path),
                view(export_name),
                params.len(),
                params.as_ptr(),
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned module reference.
        Ok(unsafe { Self::from_parts(raw, instance.clone()) })
    }
}
