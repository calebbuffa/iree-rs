use super::support::state_slot;
use crate::{
    Code, Error, Model, Progress, Result, Segment, TokenOutput, Vocabulary, VocabularyRef, n,
};
use core::{
    ffi::c_void,
    mem::{MaybeUninit, align_of, size_of},
    ptr::{self, NonNull},
};

const _: () = assert!(
    size_of::<Segment>() == size_of::<n::iree_tokenizer_segment_t>()
        && align_of::<Segment>() == align_of::<n::iree_tokenizer_segment_t>()
        && core::mem::offset_of!(Segment, start)
            == core::mem::offset_of!(n::iree_tokenizer_segment_t, start)
        && core::mem::offset_of!(Segment, end)
            == core::mem::offset_of!(n::iree_tokenizer_segment_t, end)
);

/// A safe, streaming custom tokenizer model.
///
/// The model owns its vocabulary, and its immutable implementation may be
/// shared between tokenizer pipelines. Each stream has an independent state.
pub trait ModelExtension: Send + Sync + 'static {
    /// A static native model type name used for introspection.
    const TYPE_NAME: &'static str;

    /// Mutable data owned by one encoding stream.
    type State: Default + Send + 'static;

    /// Creates initial state for one encoding stream.
    fn create_state(&self, _: VocabularyRef<'_>) -> Result<Self::State> {
        Ok(Self::State::default())
    }

    /// Encodes the next segment prefix into the caller's token buffers.
    fn encode(
        &self,
        state: &mut Self::State,
        vocab: VocabularyRef<'_>,
        transformed_text: &[u8],
        segments: &[Segment],
        last_is_partial: bool,
        output: &mut TokenOutput<'_>,
    ) -> Result<Progress>;

    /// Emits buffered tokens when the stream ends.
    fn finalize(
        &self,
        _: &mut Self::State,
        _: VocabularyRef<'_>,
        _: &mut TokenOutput<'_>,
    ) -> Result<usize> {
        Ok(0)
    }

    /// Reports whether finalization can produce more tokens.
    fn has_pending(&self, _: &Self::State) -> bool {
        false
    }
}

#[repr(C)]
struct CustomModel<E> {
    base: MaybeUninit<n::iree_tokenizer_model_t>,
    vocab: NonNull<n::iree_tokenizer_vocab_t>,
    extension: E,
    vtable: n::iree_tokenizer_model_vtable_t,
}

impl<E: ModelExtension> CustomModel<E> {
    const STATE_SIZE: usize =
        size_of::<n::iree_tokenizer_model_state_t>() + size_of::<*mut E::State>();
}

impl Model {
    /// Creates a model backed by a safe Rust implementation and an owned vocabulary.
    pub fn from_extension<E: ModelExtension>(vocab: Vocabulary, extension: E) -> Result<Self> {
        if E::TYPE_NAME.is_empty() {
            return Err(Error::new(
                Code::InvalidArgument,
                "custom model type name must not be empty",
            ));
        }
        let vtable = n::iree_tokenizer_model_vtable_t {
            destroy: Some(destroy::<E>),
            state_initialize: Some(initialize_state::<E>),
            state_deinitialize: Some(deinitialize_state::<E>),
            state_encode: Some(encode::<E>),
            state_finalize: Some(finalize::<E>),
            state_has_pending: Some(has_pending::<E>),
            state_reclaim: Some(reclaim_state),
            get_token_string: Some(get_token_string::<E>),
        };
        let vocab_raw = vocab.0;
        let component = Box::into_raw(Box::new(CustomModel {
            base: MaybeUninit::uninit(),
            vocab: vocab_raw,
            extension,
            vtable,
        }));
        // SAFETY: the base is the first field of a live boxed component and
        // its vtable and type-name bytes remain stable until native destruction.
        let base = unsafe { ptr::addr_of_mut!((*component).base).cast() };
        let type_name = n::iree_string_view_t {
            data: E::TYPE_NAME.as_ptr().cast(),
            size: E::TYPE_NAME.len(),
        };
        // SAFETY: boxed component storage outlives native ownership; state
        // layout and callback signatures match the native model contract.
        unsafe {
            n::iree_tokenizer_model_initialize(
                base,
                ptr::addr_of!((*component).vtable),
                CustomModel::<E>::STATE_SIZE,
                type_name,
            )
        };
        // SAFETY: `component` came from Box::into_raw and `base` is its
        // zero-offset first field, so it is non-null and correctly aligned.
        let raw = unsafe { NonNull::new_unchecked(base) };
        // SAFETY: `Model` retains ownership of the vocabulary and drops the
        // native model before releasing it.
        Ok(unsafe { Model::from_extension_parts(raw, vocab) })
    }
}

unsafe extern "C" fn destroy<E: ModelExtension>(model: *mut n::iree_tokenizer_model_t) {
    n::contain(|| {
        // SAFETY: IREE calls destroy once for the Box transferred by
        // `Model::from_extension`.
        unsafe { drop(Box::from_raw(model.cast::<CustomModel<E>>())) };
    });
}

unsafe extern "C" fn initialize_state<E: ModelExtension>(
    model: *const n::iree_tokenizer_model_t,
    storage: *mut c_void,
    out_state: *mut *mut n::iree_tokenizer_model_state_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable output storage for a state of the
    // size declared by the component.
    unsafe { *out_state = ptr::null_mut() };
    n::callback_status(|| {
        // SAFETY: model points to the enclosing component whose base is the
        // first field; the component outlives every state.
        let component = unsafe { &*model.cast::<CustomModel<E>>() };
        // SAFETY: the model retains ownership of this vocabulary for the
        // lifetime of every native model state.
        let vocab = unsafe { VocabularyRef::from_raw(component.vocab.as_ptr()) };
        let state = Box::into_raw(Box::new(component.extension.create_state(vocab)?));
        let base = storage.cast::<n::iree_tokenizer_model_state_t>();
        // SAFETY: state storage has room for its base and trailing pointer.
        unsafe {
            ptr::write(base, n::iree_tokenizer_model_state_t { model });
            state_slot::<_, E::State>(base).write(state);
            *out_state = base;
        }
        Ok(())
    })
}

unsafe extern "C" fn deinitialize_state<E: ModelExtension>(
    state: *mut n::iree_tokenizer_model_state_t,
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

unsafe extern "C" fn encode<E: ModelExtension>(
    state: *mut n::iree_tokenizer_model_state_t,
    transform_buffer: n::iree_const_byte_span_t,
    segments: n::iree_tokenizer_segment_list_t,
    output: n::iree_tokenizer_token_output_t,
    out_segments_consumed: *mut n::iree_host_size_t,
    out_token_count: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable progress outputs for the callback.
    unsafe {
        *out_segments_consumed = 0;
        *out_token_count = 0;
    }
    n::callback_status(|| {
        let transformed_text = if transform_buffer.data_length == 0 {
            &[]
        } else if transform_buffer.data.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "model transform buffer storage is null",
            ));
        } else {
            // SAFETY: IREE supplies a readable byte span for this callback.
            unsafe { n::slice(transform_buffer.data, transform_buffer.data_length) }
        };
        let last_is_partial = segments.last_is_partial;
        let segments = if segments.count == 0 {
            &[]
        } else if segments.values.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "model segment list storage is null",
            ));
        } else {
            // SAFETY: IREE supplies a readable segment array for this callback.
            unsafe { n::slice(segments.values.cast::<Segment>(), segments.count) }
        };
        if segments
            .iter()
            .any(|segment| segment.start > segment.end || segment.end > transformed_text.len())
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "model received segments outside the transform buffer",
            ));
        }
        // SAFETY: the native output buffers are writable for their declared capacity.
        let mut output = unsafe { TokenOutput::from_raw(output) }?;
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let progress = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data, vocab| {
                extension.encode(
                    state_data,
                    vocab,
                    transformed_text,
                    segments,
                    last_is_partial,
                    &mut output,
                )
            })
        }?;
        if progress.consumed > segments.len() || progress.written > output.capacity() {
            return Err(Error::new(
                Code::InvalidArgument,
                "model extension reported progress outside its buffers",
            ));
        }
        // SAFETY: validated counts fit the native output slots.
        unsafe {
            *out_segments_consumed = progress.consumed;
            *out_token_count = progress.written;
        }
        Ok(())
    })
}

unsafe extern "C" fn finalize<E: ModelExtension>(
    state: *mut n::iree_tokenizer_model_state_t,
    output: n::iree_tokenizer_token_output_t,
    out_token_count: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides a writable progress output for the callback.
    unsafe { *out_token_count = 0 };
    n::callback_status(|| {
        // SAFETY: the native output buffers are writable for their declared capacity.
        let mut output = unsafe { TokenOutput::from_raw(output) }?;
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let count = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data, vocab| {
                extension.finalize(state_data, vocab, &mut output)
            })
        }?;
        if count > output.capacity() {
            return Err(Error::new(
                Code::InvalidArgument,
                "model extension reported output beyond its buffer",
            ));
        }
        // SAFETY: validated count fits the native output slot.
        unsafe { *out_token_count = count };
        Ok(())
    })
}

unsafe extern "C" fn has_pending<E: ModelExtension>(
    state: *const n::iree_tokenizer_model_state_t,
) -> bool {
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

unsafe extern "C" fn reclaim_state(_: *mut n::iree_tokenizer_model_state_t) -> usize {
    0
}

unsafe extern "C" fn get_token_string<E: ModelExtension>(
    model: *const n::iree_tokenizer_model_t,
    token_id: n::iree_tokenizer_token_id_t,
    out_string: *mut n::iree_string_view_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides a writable output slot.
    unsafe {
        *out_string = n::iree_string_view_t {
            data: ptr::null(),
            size: 0,
        }
    };
    n::callback_status(|| {
        // SAFETY: model points to the enclosing component whose base is the
        // first field and remains live through the callback.
        let component = unsafe { &*model.cast::<CustomModel<E>>() };
        // SAFETY: the owning Model keeps this vocabulary live through model callbacks.
        let vocab = unsafe { VocabularyRef::from_raw(component.vocab.as_ptr()) };
        let Some(token) = vocab.token_text(token_id) else {
            return Err(Error::new(
                Code::OutOfRange,
                "model token id is not in vocabulary",
            ));
        };
        // SAFETY: the vocabulary owns these bytes and outlives the model.
        unsafe {
            *out_string = n::iree_string_view_t {
                data: token.as_ptr().cast(),
                size: token.len(),
            }
        };
        Ok(())
    })
}

unsafe fn with_state_mut<E: ModelExtension, R>(
    state: *mut n::iree_tokenizer_model_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a mut E::State, VocabularyRef<'a>) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let model = unsafe { (*state).model };
    // SAFETY: model is the first field in the enclosing component.
    let component = unsafe { &*model.cast::<CustomModel<E>>() };
    // SAFETY: state has a live trailing Box slot initialized here.
    let state_data = unsafe { &mut **state_slot::<_, E::State>(state) };
    // SAFETY: the owning Model keeps this vocabulary live for the callback.
    let vocab = unsafe { VocabularyRef::from_raw(component.vocab.as_ptr()) };
    callback(&component.extension, state_data, vocab)
}

unsafe fn with_state_ref<E: ModelExtension, R>(
    state: *const n::iree_tokenizer_model_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let model = unsafe { (*state).model };
    // SAFETY: model is the first field in the enclosing component.
    let component = unsafe { &*model.cast::<CustomModel<E>>() };
    // SAFETY: state has a live trailing Box slot initialized here.
    let state_data = unsafe { &**state_slot::<_, E::State>(state.cast_mut()) };
    callback(&component.extension, state_data)
}
