//! The I/O parameters VM module.
use super::{Instance, Module};
use crate::io::ParameterProvider;
use crate::{Result, check};
use iree_native as n;

impl Module {
    /// Creates an I/O parameters module that resolves parameters from `providers`.
    ///
    /// The module retains every provider.
    pub fn io_parameters(instance: &Instance, providers: &[ParameterProvider]) -> Result<Self> {
        let pointers: Vec<_> = providers.iter().map(ParameterProvider::as_raw).collect();
        let mut raw = std::ptr::null_mut();
        // SAFETY: the providers are live during the call and native retains them.
        check(unsafe {
            n::iree_io_parameters_module_create(
                instance.as_raw(),
                pointers.len(),
                pointers.as_ptr(),
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned module reference.
        Ok(unsafe { Self::from_parts(raw, instance.clone()) })
    }
}
