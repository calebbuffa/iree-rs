use super::support::{input_bytes, output_bytes, state_slot};
use crate::{Code, Decoder, DecoderCapabilities, DecoderFlags, Error, Progress, Result, n};
use core::{
    ffi::c_void,
    mem::{MaybeUninit, size_of},
    ptr::{self, NonNull},
};

/// A safe, streaming custom decoder.
///
/// Token strings are byte slices, not `str`, so byte-fallback and partial UTF-8
/// tokens remain representable. The immutable extension may be shared between
/// pipelines; each stream has an independent [`State`](Self::State).
pub trait DecoderExtension: Send + Sync + 'static {
    /// Mutable data owned by one decoding stream.
    type State: Default + Send + 'static;

    /// Creates initial state for one stream.
    fn create_state(&self) -> Result<Self::State> {
        Ok(Self::State::default())
    }

    /// Decoder capabilities declared to native tokenization pipelines.
    fn capabilities(&self) -> DecoderCapabilities {
        DecoderCapabilities::NONE
    }

    /// Decoder behavior flags declared to native tokenization pipelines.
    fn flags(&self) -> DecoderFlags {
        DecoderFlags::NONE
    }

    /// Processes a prefix of token strings into the caller's output buffer.
    fn process(
        &self,
        state: &mut Self::State,
        tokens: &[&[u8]],
        output: &mut [u8],
    ) -> Result<Progress>;

    /// Flushes buffered decoded output at the end of the stream.
    fn finalize(&self, _: &mut Self::State, _: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    /// Reports whether decoded output is buffered for finalization.
    ///
    /// If this callback panics, it is contained and conservatively reports
    /// `true` because the native callback cannot return an error status.
    fn has_pending(&self, _: &Self::State) -> bool {
        false
    }
}

#[repr(C)]
struct CustomDecoder<E> {
    base: MaybeUninit<n::iree_tokenizer_decoder_t>,
    extension: E,
    vtable: n::iree_tokenizer_decoder_vtable_t,
}

impl<E: DecoderExtension> CustomDecoder<E> {
    const STATE_SIZE: usize =
        size_of::<n::iree_tokenizer_decoder_state_t>() + size_of::<*mut E::State>();
}

impl Decoder {
    /// Creates a native-owned decoder backed by a safe Rust implementation.
    ///
    /// The returned wrapper owns a boxed component, which its native destroy
    /// callback releases. Each stream has an independently created Rust state.
    pub fn from_extension<E: DecoderExtension>(extension: E) -> Self {
        let vtable = n::iree_tokenizer_decoder_vtable_t {
            destroy: Some(destroy::<E>),
            state_initialize: Some(initialize_state::<E>),
            state_deinitialize: Some(deinitialize_state::<E>),
            state_process: Some(process::<E>),
            state_finalize: Some(finalize::<E>),
            state_has_pending: Some(has_pending::<E>),
            flags: extension.flags().bits(),
        };
        let capabilities = extension.capabilities().bits();
        let component = Box::into_raw(Box::new(CustomDecoder {
            base: MaybeUninit::uninit(),
            extension,
            vtable,
        }));
        // SAFETY: the base is the first field of a live boxed component and the
        // vtable remains stable until IREE invokes `destroy`.
        let base = unsafe { ptr::addr_of_mut!((*component).base).cast() };
        // SAFETY: storage outlives native ownership; callback layout and state
        // size match the allocation expected by the native state API.
        unsafe {
            n::iree_tokenizer_decoder_initialize(
                base,
                ptr::addr_of!((*component).vtable),
                CustomDecoder::<E>::STATE_SIZE,
                capabilities,
            )
        };
        // SAFETY: `component` came from Box::into_raw and `base` is its
        // zero-offset first field, so it is non-null and correctly aligned.
        Self(unsafe { NonNull::new_unchecked(base) })
    }
}

unsafe extern "C" fn destroy<E: DecoderExtension>(decoder: *mut n::iree_tokenizer_decoder_t) {
    n::contain(|| {
        // SAFETY: IREE calls destroy once for the Box transferred by
        // `Decoder::from_extension`.
        unsafe { drop(Box::from_raw(decoder.cast::<CustomDecoder<E>>())) };
    });
}

unsafe extern "C" fn initialize_state<E: DecoderExtension>(
    decoder: *const n::iree_tokenizer_decoder_t,
    storage: *mut c_void,
    out_state: *mut *mut n::iree_tokenizer_decoder_state_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable output storage for a state of the
    // size declared by the component.
    unsafe { *out_state = ptr::null_mut() };
    n::callback_status(|| {
        // SAFETY: decoder points to the enclosing component whose base is its
        // first field; the component outlives every state.
        let component = unsafe { &*decoder.cast::<CustomDecoder<E>>() };
        let state = Box::into_raw(Box::new(component.extension.create_state()?));
        let base = storage.cast::<n::iree_tokenizer_decoder_state_t>();
        // SAFETY: state storage has room for its base and trailing pointer.
        unsafe {
            ptr::write(base, n::iree_tokenizer_decoder_state_t { decoder });
            state_slot::<_, E::State>(base).write(state);
            *out_state = base;
        }
        Ok(())
    })
}

unsafe extern "C" fn deinitialize_state<E: DecoderExtension>(
    state: *mut n::iree_tokenizer_decoder_state_t,
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

unsafe extern "C" fn process<E: DecoderExtension>(
    state: *mut n::iree_tokenizer_decoder_state_t,
    token_strings: n::iree_tokenizer_string_list_t,
    output: n::iree_mutable_string_view_t,
    out_strings_consumed: *mut n::iree_host_size_t,
    out_bytes_written: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable progress outputs for the callback.
    unsafe {
        *out_strings_consumed = 0;
        *out_bytes_written = 0;
    }
    n::callback_status(|| {
        let tokens = if token_strings.count == 0 {
            &[]
        } else if token_strings.values.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "decoder token list storage is null",
            ));
        } else {
            // SAFETY: IREE supplies a readable list of `count` string views.
            unsafe { n::slice(token_strings.values, token_strings.count) }
        };
        let mut token_bytes = Vec::new();
        token_bytes.try_reserve_exact(tokens.len()).map_err(|_| {
            Error::new(
                Code::ResourceExhausted,
                "decoder token list allocation failed",
            )
        })?;
        for token in tokens {
            // SAFETY: each view is valid for this callback invocation.
            token_bytes.push(unsafe { input_bytes(*token) }?);
        }
        // SAFETY: output describes writable callback storage.
        let output = unsafe { output_bytes(output) }?;
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let progress = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.process(state_data, &token_bytes, output)
            })
        }?;
        if progress.consumed > token_bytes.len() || progress.written > output.len() {
            return Err(Error::new(
                Code::InvalidArgument,
                "decoder extension reported progress outside its buffers",
            ));
        }
        // SAFETY: output pointers are writable and the validated counts fit.
        unsafe {
            *out_strings_consumed = progress.consumed;
            *out_bytes_written = progress.written;
        }
        Ok(())
    })
}

unsafe extern "C" fn finalize<E: DecoderExtension>(
    state: *mut n::iree_tokenizer_decoder_state_t,
    output: n::iree_mutable_string_view_t,
    out_written: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides a writable progress output for the callback.
    unsafe { *out_written = 0 };
    n::callback_status(|| {
        // SAFETY: output describes writable callback storage.
        let output = unsafe { output_bytes(output) }?;
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let written = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.finalize(state_data, output)
            })
        }?;
        if written > output.len() {
            return Err(Error::new(
                Code::InvalidArgument,
                "decoder extension reported output beyond its buffer",
            ));
        }
        // SAFETY: output pointer is writable and the validated count fits.
        unsafe { *out_written = written };
        Ok(())
    })
}

unsafe extern "C" fn has_pending<E: DecoderExtension>(
    state: *const n::iree_tokenizer_decoder_state_t,
) -> bool {
    // This native callback cannot return an error status; a panic is contained
    // and conservatively reports pending output.
    n::contain(|| {
        // SAFETY: state parent and trailing Box slot were set by initialization.
        unsafe {
            with_state_ref::<E, _>(state, |extension, state_data| {
                extension.has_pending(state_data)
            })
        }
    })
    .unwrap_or(true)
}

unsafe fn with_state_mut<E: DecoderExtension, R>(
    state: *mut n::iree_tokenizer_decoder_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a mut E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let decoder = unsafe { (*state).decoder };
    // SAFETY: decoder is the first field in the enclosing component.
    let component = unsafe { &*decoder.cast::<CustomDecoder<E>>() };
    // SAFETY: the state pointer has a live trailing Box slot initialized here.
    let state_data = unsafe { &mut **state_slot::<_, E::State>(state) };
    callback(&component.extension, state_data)
}

unsafe fn with_state_ref<E: DecoderExtension, R>(
    state: *const n::iree_tokenizer_decoder_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let decoder = unsafe { (*state).decoder };
    // SAFETY: decoder is the first field in the enclosing component.
    let component = unsafe { &*decoder.cast::<CustomDecoder<E>>() };
    // SAFETY: the state pointer has a live trailing Box slot initialized here.
    let state_data = unsafe { &**state_slot::<_, E::State>(state.cast_mut()) };
    callback(&component.extension, state_data)
}
