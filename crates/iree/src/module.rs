//! Reusable VM modules and explicitly ordered session composition.

use crate::{
    Device, Error, Instance, ModuleInfo, ParameterProvider, Result, Session, runtime::check, sys,
};
use std::{any::Any, ptr, ptr::NonNull, rc::Rc, sync::Arc};

struct ModuleInner {
    raw: NonNull<sys::iree_vm_module_t>,
    instance: Instance,
    _storage: Box<dyn Any>,
}

impl Drop for ModuleInner {
    fn drop(&mut self) {
        unsafe { sys::iree_vm_module_release(self.raw.as_ptr()) };
    }
}

/// An immutable VM module reusable across sessions on the same instance.
///
/// Clones share code and backing storage, not mutable session state. A session
/// retains each module and its storage independently of the caller's handles.
/// Like sessions, module handles are thread-local.
#[derive(Clone)]
pub struct Module(Rc<ModuleInner>);

impl Module {
    /// Adopt one native module reference and retain its external backing owner.
    ///
    /// # Safety
    ///
    /// `raw` must be a live module compatible with `instance`. Transfer exactly
    /// one reference; the wrapper releases it before dropping `owner`. The owner
    /// must retain every external allocation, library, and callback required by
    /// the module. Its callbacks and implementation must obey the native VM
    /// contracts, including safe execution of the trusted functions it exports.
    pub unsafe fn from_raw(
        instance: Instance,
        raw: *mut sys::iree_vm_module_t,
        owner: impl Any,
    ) -> Result<Self> {
        let raw = NonNull::new(raw).ok_or_else(|| Error::invalid("IREE returned a null module"))?;
        Ok(Self(Rc::new(ModuleInner {
            raw,
            instance,
            _storage: Box::new(owner),
        })))
    }

    /// Borrow the native module without transferring its reference.
    pub fn as_raw(&self) -> *mut sys::iree_vm_module_t {
        self.0.raw.as_ptr()
    }

    /// Copy module attributes, exported signatures, and function attributes.
    pub fn info(&self) -> Result<ModuleInfo> {
        ModuleInfo::copy(self.as_raw())
    }
}

impl Instance {
    /// Create a reusable module from compiler output.
    ///
    /// The module retains the artifact storage. Mutable globals and initialized
    /// resources belong to each session, not to this immutable module handle.
    #[cfg(feature = "compiler")]
    pub fn module(&self, artifact: &crate::Artifact) -> Result<Module> {
        unsafe { self.module_from_bytes(artifact.bytes.clone()) }
    }

    /// Create a reusable module from trusted VM bytecode.
    ///
    /// # Safety
    ///
    /// The bytes must be trusted compatible IREE output. Embedded native code
    /// is not sandboxed. Prefer `Instance::module` when `compiler` is enabled.
    pub unsafe fn module_from_bytes(&self, bytes: impl Into<Arc<[u8]>>) -> Result<Module> {
        let bytes = bytes.into();
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_vm_bytecode_module_create(
                sys::iree_runtime_instance_vm_instance(self.as_raw()),
                0,
                sys::iree_const_byte_span_t {
                    data: bytes.as_ptr(),
                    data_length: bytes.len(),
                },
                sys::iree_rs_allocator_null(),
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        unsafe { Module::from_raw(self.clone(), raw, bytes) }
    }
}

/// Builder for an isolated context containing multiple reusable modules.
///
/// The device's HAL module and optional parameter module are registered first.
/// Explicit modules follow in insertion order, so dependencies must precede
/// their users. Native import resolution and initialization errors propagate.
#[must_use = "call build to create the configured session"]
pub struct SessionBuilder {
    device: Device,
    modules: Vec<Module>,
    providers: Vec<ParameterProvider>,
    trace_execution: bool,
}

impl Device {
    /// Configure a session with ordered modules and optional scoped parameters.
    pub fn session_builder(&self) -> SessionBuilder {
        SessionBuilder {
            device: self.clone(),
            modules: Vec::new(),
            providers: Vec::new(),
            trace_execution: false,
        }
    }
}

impl SessionBuilder {
    /// Append a reusable module, retaining its code and external storage.
    ///
    /// Modules must belong to this device's runtime instance. That requirement
    /// is checked by [`Self::build`] before any native context is initialized.
    pub fn module(mut self, module: &Module) -> Self {
        self.modules.push(module.clone());
        self
    }

    /// Append scoped providers for a single native parameter I/O module.
    ///
    /// Providers retain their indexes and backing files; originals may be
    /// dropped before this builder is built.
    pub fn parameters(mut self, providers: &[ParameterProvider]) -> Self {
        self.providers.extend_from_slice(providers);
        self
    }

    /// Request native VM instruction tracing to stderr.
    ///
    /// Tracing is available only if the native libraries were configured with
    /// `IREE_VM_EXECUTION_TRACING_ENABLE`. This flag does not enable that build
    /// option or change the program's execution model.
    pub fn trace_execution(mut self, enabled: bool) -> Self {
        self.trace_execution = enabled;
        self
    }

    /// Resolve imports, initialize module state, and freeze the module set.
    ///
    /// All modules are validated before context creation. Native initialization
    /// may allocate device resources and execute trusted module initializers.
    pub fn build(mut self) -> Result<Session> {
        let instance = &self.device.0.instance;
        for module in &self.modules {
            if module.0.instance.as_raw() != instance.as_raw() {
                return Err(Error::invalid(
                    "session module belongs to a different runtime instance",
                ));
            }
        }
        let vm_instance = unsafe { sys::iree_runtime_instance_vm_instance(instance.as_raw()) };
        let mut pointers = vec![self.device.0.hal_module.as_ptr()];
        if !self.providers.is_empty() {
            let providers: Vec<_> = self
                .providers
                .iter()
                .map(ParameterProvider::as_raw)
                .collect();
            let mut raw = ptr::null_mut();
            check(unsafe {
                sys::iree_io_parameters_module_create(
                    vm_instance,
                    providers.len(),
                    providers.as_ptr(),
                    sys::iree_allocator_system(),
                    &mut raw,
                )
            })?;
            let module = unsafe { Module::from_raw(instance.clone(), raw, ()) }?;
            self.modules.insert(0, module);
        }
        pointers.extend(self.modules.iter().map(Module::as_raw));
        let flags = if self.trace_execution {
            sys::IREE_VM_CONTEXT_FLAG_TRACE_EXECUTION
        } else {
            sys::IREE_VM_CONTEXT_FLAG_NONE
        };
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_vm_context_create_with_modules(
                vm_instance,
                flags as sys::iree_vm_context_flags_t,
                pointers.len(),
                pointers.as_mut_ptr(),
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Session {
            raw: NonNull::new(raw).ok_or_else(|| Error::invalid("IREE returned a null session"))?,
            device: self.device,
            _modules: self.modules,
            pending: Vec::new(),
        })
    }
}

/// Host lifecycle notification delivered to every module in a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionSignal {
    /// Resume execution and permit reallocation of transient resources.
    Resume,
    /// Suspend and release transient pools and caches where possible.
    Suspend,
    /// Aggressively reclaim resources in response to host memory pressure.
    LowMemory,
}

impl Session {
    /// Drain pending device calls, then notify all modules of a lifecycle event.
    ///
    /// Resume walks modules in registration order; suspend and low-memory
    /// notifications walk in reverse order, following the native contract.
    pub fn notify(&mut self, signal: SessionSignal) -> Result<()> {
        self.drain()?;
        let signal = match signal {
            SessionSignal::Resume => sys::IREE_VM_SIGNAL_RESUME,
            SessionSignal::Suspend => sys::IREE_VM_SIGNAL_SUSPEND,
            SessionSignal::LowMemory => sys::IREE_VM_SIGNAL_LOW_MEMORY,
        };
        check(unsafe { sys::iree_vm_context_notify(self.as_raw(), signal) })
    }
}
