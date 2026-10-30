//! Dynamically loaded Rust implementation of IREE's custom-module sample.

use ireert::{
    Code, Error,
    hal::{BufferView, ElementType},
    vm::{Instance, NativeModule, Ref, RefType},
};
use std::sync::Arc;

/// Name used to locate this library in Cargo's platform-specific output.
pub const ARTIFACT_NAME: &str = "iree_sample_dynamic_module";

struct TypeRegistration {
    instance: Instance,
    string_type: RefType,
}

impl Drop for TypeRegistration {
    fn drop(&mut self) {
        // SAFETY: the factory contract requires custom references to be released
        // before the module; its callbacks keep this registration alive until then.
        // This handle came from `register`, so it has a native registration.
        let _ = unsafe { self.string_type.unregister(&self.instance) };
    }
}

/// Creates the dynamically loaded `custom` VM module.
///
/// # Safety
/// The pointers must be valid according to IREE's dynamic-module factory ABI,
/// and the host runtime must keep its instance alive for the returned module.
/// All `custom.string` references must be released before the module's final
/// reference, because their destructor belongs to this dynamic library.
/// A non-null `out_module` must point to a writable output slot. It is cleared
/// before validation, so an error never leaves a stale module pointer behind.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_custom_module(
    max_version: u32,
    raw_instance: *mut iree_native::iree_vm_instance_t,
    param_count: usize,
    _params: *const iree_native::iree_string_pair_t,
    _allocator: iree_native::iree_allocator_t,
    out_module: *mut *mut iree_native::iree_vm_module_t,
) -> iree_native::iree_status_t {
    iree_native::callback_status(|| {
        if out_module.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "dynamic module factory received a null pointer",
            ));
        }
        // Initialize the output even when validation or construction fails.
        // SAFETY: the ABI requires `out_module` to point to a writable output slot.
        unsafe { out_module.write(std::ptr::null_mut()) };
        if raw_instance.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "dynamic module factory received a null instance",
            ));
        }
        if max_version != iree_native::IREE_VM_DYNAMIC_MODULE_VERSION_LATEST {
            return Err(Error::new(
                Code::Unimplemented,
                "unsupported dynamic module API version",
            ));
        }
        if param_count != 0 {
            return Err(Error::new(
                Code::InvalidArgument,
                "this sample does not accept module parameters",
            ));
        }

        // SAFETY: the runtime supplied a live borrowed instance. Retain one
        // reference before adopting it into the Rust owner.
        unsafe { iree_native::iree_vm_instance_retain(raw_instance) };
        // SAFETY: the retain above gives this wrapper one owned reference.
        let instance = unsafe { Instance::from_raw(raw_instance) };
        // The shared library has its own linked IREE image; resolve its HAL
        // adapters against the host instance before converting VM references.
        // SAFETY: the instance remains live because of the retain above.
        unsafe {
            iree_native::consume(iree_native::iree_hal_module_resolve_all_types(
                instance.as_raw(),
            ))?;
        }
        let string_type = RefType::register::<String>(&instance, "custom.string")?;
        let registration = Arc::new(TypeRegistration {
            instance: instance.clone(),
            string_type,
        });

        let create_registration = registration.clone();
        let from_tensor = move |reference: Ref| -> ireert::Result<Ref> {
            let view = BufferView::from_ref(&reference)?;
            // SAFETY: the imported signature requires an initialized i8 buffer view.
            let values = unsafe { view.read_elements_unchecked::<i8>(ElementType::INT_8)? };
            if values != [0, 1, 2, 3, 4] {
                return Err(Error::new(
                    Code::InvalidArgument,
                    "unexpected custom-module tensor contents",
                ));
            }
            let shape = view
                .shape()?
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("x");
            let formatted = format!(
                "{shape}xi8={}",
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            println!("CREATE {formatted}");
            create_registration.string_type.create(formatted)
        };

        let print_registration = registration.clone();
        let print = move |reference: Ref| -> ireert::Result<()> {
            let value = print_registration
                .string_type
                .downcast::<String>(&reference)
                .ok_or_else(|| Error::new(Code::InvalidArgument, "unexpected string reference"))?;
            if value != "5xi8=0 1 2 3 4" {
                return Err(Error::new(
                    Code::InvalidArgument,
                    "unexpected custom string value",
                ));
            }
            println!("PRINT {value}");
            Ok(())
        };

        let error =
            |(): ()| -> ireert::Result<()> { Err(Error::new(Code::Unknown, "Forced failure")) };
        let module = NativeModule::builder("custom")
            .export_typed("string.from_tensor", from_tensor)
            .export_typed("string.print", print)
            .export_typed("error", error)
            .build(&instance)?;
        // SAFETY: the runtime supplied a writable output slot and `into_raw`
        // transfers one owned module reference to it.
        unsafe { out_module.write(module.into_raw()) };
        drop(registration);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ireert::vm::Module;
    use std::ptr::{NonNull, null, null_mut};
    use std::sync::Mutex;

    static TYPE_REGISTRATION_TEST: Mutex<()> = Mutex::new(());

    fn create(
        instance: Option<&Instance>,
        version: u32,
        parameters: &[iree_native::iree_string_pair_t],
        output: &mut *mut iree_native::iree_vm_module_t,
    ) -> ireert::Result<()> {
        // SAFETY: callers supply either a live instance or null (which is rejected).
        // The parameter slice and output slot remain valid for the entire call.
        let status = unsafe {
            create_custom_module(
                version,
                instance.map_or(null_mut(), Instance::as_raw),
                parameters.len(),
                parameters.as_ptr(),
                iree_native::iree_allocator_system(),
                output,
            )
        };
        // SAFETY: the factory returned an owned status; consume it exactly once.
        unsafe { iree_native::consume(status) }
    }

    #[test]
    fn validation_clears_the_output_without_registering_types() -> ireert::Result<()> {
        let _guard = TYPE_REGISTRATION_TEST
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let instance = Instance::new()?;
        let latest = iree_native::IREE_VM_DYNAMIC_MODULE_VERSION_LATEST;
        let mut output = NonNull::dangling().as_ptr();
        let error = create(None, latest, &[], &mut output).err();
        assert_eq!(error.map(|error| error.code()), Some(Code::InvalidArgument));
        assert!(output.is_null());

        output = NonNull::dangling().as_ptr();
        let error = create(Some(&instance), latest + 1, &[], &mut output).err();
        assert_eq!(error.map(|error| error.code()), Some(Code::Unimplemented));
        assert!(output.is_null());

        let parameter = iree_native::iree_string_pair_t::default();
        output = NonNull::dangling().as_ptr();
        let error = create(Some(&instance), latest, &[parameter], &mut output).err();
        assert_eq!(error.map(|error| error.code()), Some(Code::InvalidArgument));
        assert!(output.is_null());
        assert!(RefType::lookup(&instance, "custom.string").is_none());
        Ok(())
    }

    #[test]
    fn null_output_is_rejected() -> ireert::Result<()> {
        let instance = Instance::new()?;
        // SAFETY: the instance is live and a null output pointer is explicitly
        // rejected before the factory writes to it.
        let status = unsafe {
            create_custom_module(
                iree_native::IREE_VM_DYNAMIC_MODULE_VERSION_LATEST,
                instance.as_raw(),
                0,
                null(),
                iree_native::iree_allocator_system(),
                null_mut(),
            )
        };
        // SAFETY: the factory returned an owned status; consume it exactly once.
        let error = unsafe { iree_native::consume(status) }.err();
        assert_eq!(error.map(|error| error.code()), Some(Code::InvalidArgument));
        Ok(())
    }

    #[test]
    fn module_owns_its_type_registration() -> ireert::Result<()> {
        let _guard = TYPE_REGISTRATION_TEST
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let instance = Instance::new()?;
        let mut output = null_mut();
        create(
            Some(&instance),
            iree_native::IREE_VM_DYNAMIC_MODULE_VERSION_LATEST,
            &[],
            &mut output,
        )?;
        assert!(!output.is_null());
        // SAFETY: the successful factory call transferred one owned module
        // reference, and `instance` outlives the module.
        let module = unsafe { Module::from_raw(output) };
        assert!(RefType::lookup(&instance, "custom.string").is_some());
        drop(module);
        assert!(RefType::lookup(&instance, "custom.string").is_none());
        Ok(())
    }
}
