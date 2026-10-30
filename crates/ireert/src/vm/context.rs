//! VM context lifecycle and function resolution.
use super::{Function, Instance, Module};
use crate::hal::bitfield::bitfield;
use crate::{Result, check};
use iree_native as n;
use std::cell::Cell;
use std::marker::PhantomData;

/// Registered module set and its instance state.
///
/// The native context retains its instance and every registered module, so
/// this wrapper owns exactly one native reference and nothing else.
pub struct Context {
    raw: *mut n::iree_vm_context_t,
    _not_sync: PhantomData<Cell<()>>,
}

/// Borrowed module state owned by a context.
pub struct ModuleState<'a> {
    raw: *mut n::iree_vm_module_state_t,
    _context: &'a Context,
    _module: &'a Module,
}

impl ModuleState<'_> {
    /// # Safety
    /// `raw` must be a live state belonging to `context` and `module`.
    pub(crate) unsafe fn from_raw<'a>(
        raw: *mut n::iree_vm_module_state_t,
        context: &'a Context,
        module: &'a Module,
    ) -> ModuleState<'a> {
        ModuleState {
            raw,
            _context: context,
            _module: module,
        }
    }

    /// Borrows the native module-state pointer.
    pub fn as_raw(&self) -> *mut n::iree_vm_module_state_t {
        self.raw
    }
}

impl Context {
    /// Creates an empty context for an instance.
    pub fn new(instance: &Instance, flags: ContextFlags) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the instance is live and the output is writable.
        check(unsafe {
            n::iree_vm_context_create(instance.as_raw(), flags.bits(), crate::system(), &mut raw)
        })?;
        Ok(Self {
            raw,
            _not_sync: PhantomData,
        })
    }

    /// Adopts an owned context pointer.
    ///
    /// # Safety
    /// `raw` must be a non-null live, owned IREE VM context pointer.
    pub unsafe fn from_raw(raw: *mut n::iree_vm_context_t) -> Self {
        Self {
            raw,
            _not_sync: PhantomData,
        }
    }

    /// Borrows the native context pointer.
    pub fn as_raw(&self) -> *mut n::iree_vm_context_t {
        self.raw
    }

    /// Transfers the owned pointer to the caller.
    pub fn into_raw(self) -> *mut n::iree_vm_context_t {
        std::mem::ManuallyDrop::new(self).raw
    }

    /// Creates a context and registers the supplied modules atomically.
    pub fn with_modules(
        instance: &Instance,
        flags: ContextFlags,
        modules: &[Module],
    ) -> Result<Self> {
        let mut pointers: Vec<_> = modules.iter().map(Module::as_raw).collect();
        let mut raw = std::ptr::null_mut();
        // SAFETY: the instance and modules are live; native retains the modules.
        check(unsafe {
            n::iree_vm_context_create_with_modules(
                instance.as_raw(),
                flags.bits(),
                pointers.len(),
                pointers.as_mut_ptr(),
                crate::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _not_sync: PhantomData,
        })
    }

    /// Registers modules while the context remains mutable.
    pub fn register_modules(&mut self, modules: &[Module]) -> Result<()> {
        let mut pointers: Vec<_> = modules.iter().map(Module::as_raw).collect();
        // SAFETY: the modules are live; native retains them.
        check(unsafe {
            n::iree_vm_context_register_modules(self.raw, pointers.len(), pointers.as_mut_ptr())
        })
    }

    /// Creates a forked context with the same registered modules.
    pub fn fork(&self) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the context is live and the output is writable.
        check(unsafe { n::iree_vm_context_fork(self.raw, crate::system(), &mut raw) })?;
        Ok(Self {
            raw,
            _not_sync: PhantomData,
        })
    }

    /// Returns a retained handle to the context's VM instance.
    pub fn instance(&self) -> Instance {
        // SAFETY: the context retains its instance; retaining gives the
        // returned handle its own reference.
        let raw = unsafe { n::iree_vm_context_instance(self.raw) };
        // SAFETY: context_instance returns a retained non-null instance reference for this wrapper.
        unsafe {
            n::iree_vm_instance_retain(raw);
            Instance::from_raw(raw)
        }
    }

    /// Resolves state for a module registered in this context.
    pub fn resolve_module_state<'a>(&'a self, module: &'a Module) -> Result<ModuleState<'a>> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the context and module are live.
        check(unsafe {
            n::iree_vm_context_resolve_module_state(self.raw, module.as_raw(), &mut raw)
        })?;
        // SAFETY: the state is owned by the borrowed context.
        Ok(unsafe { ModuleState::from_raw(raw, self, module) })
    }

    /// Notifies the context of a VM signal.
    pub fn notify(&mut self, signal: ContextSignal) -> Result<()> {
        // SAFETY: the context is live.
        check(unsafe { n::iree_vm_context_notify(self.raw, signal.raw()) })
    }

    /// Prevents any further module registration.
    pub fn freeze(&mut self) -> Result<()> {
        // SAFETY: the context is live.
        check(unsafe { n::iree_vm_context_freeze(self.raw) })
    }

    /// Returns this context's stable identifier.
    pub fn id(&self) -> isize {
        // SAFETY: the context is live.
        unsafe { n::iree_vm_context_id(self.raw) }
    }

    /// Returns the context flags.
    pub fn flags(&self) -> ContextFlags {
        // SAFETY: the context is live.
        ContextFlags::from_bits_retain(unsafe { n::iree_vm_context_flags(self.raw) })
    }

    /// Returns the number of registered modules.
    pub fn module_count(&self) -> usize {
        // SAFETY: the context is live.
        unsafe { n::iree_vm_context_module_count(self.raw) }
    }

    /// Returns a retained module at an index, or `None` when out of range.
    pub fn module_at(&self, index: usize) -> Option<Module> {
        // SAFETY: the context is live; out-of-range indices return null.
        let raw = unsafe { n::iree_vm_context_module_at(self.raw, index) };
        if raw.is_null() {
            return None;
        }
        // SAFETY: retaining gives the returned handle its own reference.
        unsafe {
            n::iree_vm_module_retain(raw);
            Some(Module::from_raw(raw))
        }
    }

    /// Resolves an exported function by full name.
    pub fn resolve_function(&self, name: &str) -> Result<Function> {
        let mut raw = n::iree_vm_function_t::default();
        // SAFETY: the context and name view are valid for the call.
        check(unsafe {
            n::iree_vm_context_resolve_function(self.raw, crate::view(name), &mut raw)
        })?;
        // SAFETY: a successful resolve names a function of a module this context retains.
        Ok(unsafe { Function::from_raw(raw) })
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        // SAFETY: `self.raw` is a live ref-counted context.
        unsafe { n::iree_vm_context_retain(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: PhantomData,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one context reference.
        unsafe { n::iree_vm_context_release(self.raw) }
    }
}

bitfield! {
    /// Context behavior flags.
    ContextFlags: n::iree_vm_context_flags_t {
        /// Enables execution tracing.
        TRACE_EXECUTION = n::IREE_VM_CONTEXT_FLAG_TRACE_EXECUTION,
        /// Allows concurrent invocations within the context.
        CONCURRENT = n::IREE_VM_CONTEXT_FLAG_CONCURRENT,
    }
}

/// Signal delivered to a VM context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextSignal {
    /// Request that suspended execution resume.
    Resume,
    /// Request that active execution suspend.
    Suspend,
    /// Notify the context of memory pressure.
    LowMemory,
    /// Notify the context that it has been forked.
    Fork,
}

impl ContextSignal {
    pub(crate) fn raw(self) -> n::iree_vm_signal_t {
        match self {
            Self::Resume => n::IREE_VM_SIGNAL_RESUME,
            Self::Suspend => n::IREE_VM_SIGNAL_SUSPEND,
            Self::LowMemory => n::IREE_VM_SIGNAL_LOW_MEMORY,
            Self::Fork => n::IREE_VM_SIGNAL_FORK,
        }
    }

    pub(crate) fn from_raw(raw: n::iree_vm_signal_t) -> Result<Self> {
        Ok(match raw {
            n::IREE_VM_SIGNAL_RESUME => Self::Resume,
            n::IREE_VM_SIGNAL_SUSPEND => Self::Suspend,
            n::IREE_VM_SIGNAL_LOW_MEMORY => Self::LowMemory,
            n::IREE_VM_SIGNAL_FORK => Self::Fork,
            _ => {
                return Err(crate::Error::new(
                    crate::Code::Unimplemented,
                    "unknown VM context signal",
                ));
            }
        })
    }
}
