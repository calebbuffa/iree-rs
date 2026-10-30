//! C callbacks for native modules.
use super::*;
use n::slice;
use std::ffi::c_void;

/// Runs a non-call callback while containing panics and rejecting deferral.
pub(super) fn boundary(callback: impl FnOnce() -> Result<()>) -> n::iree_status_t {
    n::callback_status(|| {
        callback().map_err(|error| match error.code() {
            Code::Deferred => Error::new(Code::Unimplemented, error.message()),
            _ => error,
        })
    })
}

/// Hands `state` to native through `out`.
///
/// # Safety
/// `out` must be writable.
unsafe fn publish(state: CallState, out: *mut *mut n::iree_vm_module_state_t) {
    // SAFETY: the caller guarantees `out` is writable.
    unsafe { *out = Box::into_raw(Box::new(state)).cast() };
}

pub(super) unsafe extern "C" fn destroy(data: *mut c_void) {
    // SAFETY: native destroys the module once, releasing the Arc from `build`.
    n::contain(|| drop(unsafe { Arc::from_raw(data.cast::<ModuleData>()) }));
}

pub(super) unsafe extern "C" fn allocate_state(
    data: *mut c_void,
    _allocator: n::iree_allocator_t,
    out: *mut *mut n::iree_vm_module_state_t,
) -> n::iree_status_t {
    boundary(|| {
        // SAFETY: the module data outlives its states.
        let data = unsafe { &*data.cast::<ModuleData>() };
        let state = CallState {
            value: (data.definition.state_factory)()?,
            imports: Mutex::new(vec![None; data.definition.imports.len()]),
        };
        // SAFETY: native passes a writable output.
        unsafe { publish(state, out) };
        Ok(())
    })
}

pub(super) unsafe extern "C" fn free_state(
    _data: *mut c_void,
    state: *mut n::iree_vm_module_state_t,
) {
    if !state.is_null() {
        // SAFETY: the state came from `publish` and is freed once.
        n::contain(|| drop(unsafe { Box::from_raw(state.cast::<CallState>()) }));
    }
}

pub(super) unsafe extern "C" fn fork_state(
    data: *mut c_void,
    parent: *mut n::iree_vm_module_state_t,
    _allocator: n::iree_allocator_t,
    out: *mut *mut n::iree_vm_module_state_t,
) -> n::iree_status_t {
    boundary(|| {
        // SAFETY: the module data outlives its states, and `parent` came from `publish`.
        let (data, parent) = unsafe { (&*data.cast::<ModuleData>(), &*parent.cast::<CallState>()) };
        let state = CallState {
            value: (data.definition.state_fork)(parent.value.as_ref())?,
            imports: Mutex::new(
                parent
                    .imports
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .clone(),
            ),
        };
        // SAFETY: native passes a writable output.
        unsafe { publish(state, out) };
        Ok(())
    })
}

pub(super) unsafe extern "C" fn resolve_import(
    _data: *mut c_void,
    state: *mut n::iree_vm_module_state_t,
    ordinal: usize,
    function: *const n::iree_vm_function_t,
    _signature: *const n::iree_vm_function_signature_t,
) -> n::iree_status_t {
    boundary(|| {
        // SAFETY: `state` came from `publish`; `function` is null or borrowed for the call.
        let (state, function) = unsafe { (&*state.cast::<CallState>(), function.as_ref()) };
        let mut imports = state
            .imports
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = imports
            .get_mut(ordinal)
            .ok_or_else(|| Error::new(Code::OutOfRange, "import ordinal out of range"))?;
        // An unresolved optional import has no module.
        // SAFETY: `Function::from_raw` retains the borrowed module.
        *slot = function
            .filter(|f| !f.module.is_null())
            .map(|f| unsafe { Function::from_raw(*f) });
        Ok(())
    })
}

pub(super) unsafe extern "C" fn notify(
    data: *mut c_void,
    state: *mut n::iree_vm_module_state_t,
    signal: n::iree_vm_signal_t,
) -> n::iree_status_t {
    boundary(|| {
        // SAFETY: the module data outlives its states, and `state` came from `publish`.
        let (data, state) = unsafe { (&*data.cast::<ModuleData>(), &*state.cast::<CallState>()) };
        match &data.definition.notify {
            Some(callback) => callback(state.value.as_ref(), ContextSignal::from_raw(signal)?),
            None => Ok(()),
        }
    })
}

pub(super) unsafe extern "C" fn call(
    stack: *mut n::iree_vm_stack_t,
    flags: n::iree_vm_native_function_flags_t,
    arguments: n::iree_byte_span_t,
    results: n::iree_byte_span_t,
    _target: n::iree_vm_native_function_target_t,
    data: *mut c_void,
    state: *mut c_void,
) -> n::iree_status_t {
    n::callback_status(|| {
        let phase = match flags {
            value
                if value
                    == n::IREE_VM_NATIVE_FUNCTION_CALL_BEGIN
                        as n::iree_vm_native_function_flags_t =>
            {
                NativeCallPhase::Begin
            }
            value
                if value
                    == n::IREE_VM_NATIVE_FUNCTION_CALL_RESUME
                        as n::iree_vm_native_function_flags_t =>
            {
                NativeCallPhase::Resume
            }
            _ => {
                return Err(Error::new(
                    Code::InvalidArgument,
                    "invalid native call flags",
                ));
            }
        };
        // SAFETY: the module data outlives its states, `state` came from
        // `publish`, and the stack's top frame is this call.
        let (data, state, frame) = unsafe {
            (
                &*data.cast::<ModuleData>(),
                &*state.cast::<CallState>(),
                n::iree_vm_stack_top(stack).as_ref(),
            )
        };
        let frame = frame
            .ok_or_else(|| Error::new(Code::FailedPrecondition, "missing native stack frame"))?;
        let export = data
            .definition
            .exports
            .get(usize::from(frame.function.ordinal))
            .ok_or_else(|| Error::new(Code::OutOfRange, "export ordinal out of range"))?;
        let layout = export
            .layout
            .as_ref()
            .ok_or_else(|| Error::new(Code::Internal, "export layout was not parsed"))?;
        if (phase == NativeCallPhase::Begin && arguments.data_length != layout.arguments.size)
            || results.data_length != layout.results.size
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "native call storage size mismatch",
            ));
        }
        if results.data_length != 0 {
            // SAFETY: the result storage is writable for its length.
            unsafe { ptr::write_bytes(results.data, 0, results.data_length) };
        }
        let mut call = NativeCall {
            module: data,
            state,
            layout,
            // SAFETY: the argument storage is borrowed for the call.
            arguments: unsafe { slice(arguments.data, arguments.data_length) },
            results,
            complete: false,
            phase,
        };
        (export.callback)(&mut call)?;
        call.complete = true;
        Ok(())
    })
}
