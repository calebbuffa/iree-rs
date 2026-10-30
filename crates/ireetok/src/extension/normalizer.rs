use super::support::{input_str, output_bytes, state_slot};
use crate::{Code, Error, Normalizer, NormalizerFlags, Progress, Result, n};
use core::{
    ffi::c_void,
    mem::{MaybeUninit, size_of},
    ptr::{self, NonNull},
};

/// A safe, streaming custom normalizer.
///
/// The immutable extension may be shared by native tokenizer pipelines;
/// each stream gets a separate [`State`](Self::State). Implementations report
/// byte counts through [`Progress`], which are checked against the supplied
/// input and output buffers before returning to IREE.
pub trait NormalizerExtension: Send + Sync + 'static {
    /// Mutable data owned by one normalization stream.
    type State: Default + Send + 'static;

    /// Creates initial state for one stream.
    fn create_state(&self) -> Result<Self::State> {
        Ok(Self::State::default())
    }

    /// Processes an input prefix into the caller's output buffer.
    fn process(
        &self,
        state: &mut Self::State,
        input: &str,
        output: &mut [u8],
        flags: NormalizerFlags,
    ) -> Result<Progress>;

    /// Flushes buffered output at the end of the stream.
    fn finalize(&self, _: &mut Self::State, _: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    /// Reports whether finalization can produce more output.
    ///
    /// If this callback panics, it is contained and conservatively reports
    /// `true` because the native callback cannot return an error status.
    fn has_pending(&self, _: &Self::State) -> bool {
        false
    }
}

#[repr(C)]
struct CustomNormalizer<E> {
    base: MaybeUninit<n::iree_tokenizer_normalizer_t>,
    extension: E,
    vtable: n::iree_tokenizer_normalizer_vtable_t,
}

impl<E: NormalizerExtension> CustomNormalizer<E> {
    const STATE_SIZE: usize =
        size_of::<n::iree_tokenizer_normalizer_state_t>() + size_of::<*mut E::State>();
}

impl Normalizer {
    /// Creates a native-owned component backed by a safe Rust implementation.
    ///
    /// The returned wrapper owns a boxed component, which its native destroy
    /// callback releases. Each stream has an independently created Rust state.
    pub fn from_extension<E: NormalizerExtension>(extension: E) -> Self {
        let vtable = n::iree_tokenizer_normalizer_vtable_t {
            destroy: Some(destroy::<E>),
            state_initialize: Some(initialize_state::<E>),
            state_deinitialize: Some(deinitialize_state::<E>),
            state_process: Some(process::<E>),
            state_finalize: Some(finalize::<E>),
            state_has_pending: Some(has_pending::<E>),
        };
        let component = Box::into_raw(Box::new(CustomNormalizer {
            base: MaybeUninit::uninit(),
            extension,
            vtable,
        }));
        // SAFETY: the base is the first field of a live boxed component and the
        // vtable remains at a stable address until IREE invokes `destroy`.
        let base = unsafe { ptr::addr_of_mut!((*component).base).cast() };
        // SAFETY: storage outlives the native ownership, and callback layout and
        // state size match the allocation expected by the native state API.
        unsafe {
            n::iree_tokenizer_normalizer_initialize(
                base,
                ptr::addr_of!((*component).vtable),
                CustomNormalizer::<E>::STATE_SIZE,
            )
        };
        // SAFETY: `component` came from Box::into_raw and `base` is its
        // zero-offset first field, so it is non-null and correctly aligned.
        Self(unsafe { NonNull::new_unchecked(base) })
    }
}

unsafe extern "C" fn destroy<E: NormalizerExtension>(
    normalizer: *mut n::iree_tokenizer_normalizer_t,
) {
    n::contain(|| {
        // SAFETY: IREE calls destroy once for the Box transferred by
        // `Normalizer::from_extension`.
        unsafe { drop(Box::from_raw(normalizer.cast::<CustomNormalizer<E>>())) };
    });
}

unsafe extern "C" fn initialize_state<E: NormalizerExtension>(
    normalizer: *const n::iree_tokenizer_normalizer_t,
    storage: *mut c_void,
    out_state: *mut *mut n::iree_tokenizer_normalizer_state_t,
) -> n::iree_status_t {
    // SAFETY: IREE supplies writable output storage for a state of the
    // size declared by the component.
    unsafe { *out_state = ptr::null_mut() };
    n::callback_status(|| {
        // SAFETY: normalizer points to the enclosing component whose base is
        // its first field; the component outlives every state.
        let component = unsafe { &*normalizer.cast::<CustomNormalizer<E>>() };
        let state = Box::into_raw(Box::new(component.extension.create_state()?));
        let base = storage.cast::<n::iree_tokenizer_normalizer_state_t>();
        // SAFETY: the state storage has room for its base and trailing pointer.
        unsafe {
            ptr::write(base, n::iree_tokenizer_normalizer_state_t { normalizer });
            state_slot::<_, E::State>(base).write(state);
            *out_state = base;
        }
        Ok(())
    })
}

unsafe extern "C" fn deinitialize_state<E: NormalizerExtension>(
    state: *mut n::iree_tokenizer_normalizer_state_t,
) {
    n::contain(|| {
        // SAFETY: IREE calls this once for a state initialized by
        // `initialize_state`.
        let state = unsafe { state_slot::<_, E::State>(state).replace(ptr::null_mut()) };
        if !state.is_null() {
            // SAFETY: the slot contains the unique Box allocated at state init.
            unsafe { drop(Box::from_raw(state)) };
        }
    });
}

unsafe extern "C" fn process<E: NormalizerExtension>(
    state: *mut n::iree_tokenizer_normalizer_state_t,
    input: n::iree_string_view_t,
    output: n::iree_mutable_string_view_t,
    flags: n::iree_tokenizer_normalizer_flags_t,
    out_consumed: *mut n::iree_host_size_t,
    out_written: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable progress outputs for the callback.
    unsafe {
        *out_consumed = 0;
        *out_written = 0;
    }
    n::callback_status(|| {
        // SAFETY: views and state are valid for this callback invocation.
        let input = unsafe { input_str(input) }?;
        // SAFETY: output describes writable callback storage.
        let output = unsafe { output_bytes(output) }?;
        // SAFETY: the state's parent pointer and trailing Box slot were set by
        // initialize_state and remain live through this call.
        let progress = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.process(
                    state_data,
                    input,
                    output,
                    NormalizerFlags::from_bits_retain(flags),
                )
            })
        }?;
        if progress.consumed > input.len()
            || !input.is_char_boundary(progress.consumed)
            || progress.written > output.len()
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "normalizer extension reported progress outside its buffers",
            ));
        }
        // SAFETY: output pointers are writable and the validated counts fit.
        unsafe {
            *out_consumed = progress.consumed;
            *out_written = progress.written;
        }
        Ok(())
    })
}

unsafe extern "C" fn finalize<E: NormalizerExtension>(
    state: *mut n::iree_tokenizer_normalizer_state_t,
    output: n::iree_mutable_string_view_t,
    out_written: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides a writable progress output for the callback.
    unsafe { *out_written = 0 };
    n::callback_status(|| {
        // SAFETY: output describes writable callback storage.
        let output = unsafe { output_bytes(output) }?;
        // SAFETY: the state's parent pointer and trailing Box slot were set by
        // initialize_state and remain live through this call.
        let written = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.finalize(state_data, output)
            })
        }?;
        if written > output.len() {
            return Err(Error::new(
                Code::InvalidArgument,
                "normalizer extension reported output beyond its buffer",
            ));
        }
        // SAFETY: output pointer is writable and the validated count fits.
        unsafe { *out_written = written };
        Ok(())
    })
}

unsafe extern "C" fn has_pending<E: NormalizerExtension>(
    state: *const n::iree_tokenizer_normalizer_state_t,
) -> bool {
    // This native callback cannot return an error status; a panic is contained
    // and conservatively reports pending output so callers do not discard it.
    n::contain(|| {
        // SAFETY: the state's parent pointer and trailing Box slot were set by
        // initialize_state and remain live through this call.
        unsafe {
            with_state_ref::<E, _>(state, |extension, state_data| {
                extension.has_pending(state_data)
            })
        }
    })
    .unwrap_or(true)
}

unsafe fn with_state_mut<E: NormalizerExtension, R>(
    state: *mut n::iree_tokenizer_normalizer_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a mut E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let normalizer = unsafe { (*state).normalizer };
    // SAFETY: the normalizer is the first field in the enclosing component.
    let component = unsafe { &*normalizer.cast::<CustomNormalizer<E>>() };
    // SAFETY: the state pointer has a live trailing Box slot initialized here.
    let state_data = unsafe { &mut **state_slot::<_, E::State>(state) };
    callback(&component.extension, state_data)
}

unsafe fn with_state_ref<E: NormalizerExtension, R>(
    state: *const n::iree_tokenizer_normalizer_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let normalizer = unsafe { (*state).normalizer };
    // SAFETY: the normalizer is the first field in the enclosing component.
    let component = unsafe { &*normalizer.cast::<CustomNormalizer<E>>() };
    // SAFETY: the state pointer has a live trailing Box slot initialized here.
    let state_data = unsafe { &**state_slot::<_, E::State>(state.cast_mut()) };
    callback(&component.extension, state_data)
}
