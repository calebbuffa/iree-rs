use super::support::{input_str, state_slot};
use crate::{Code, Error, Progress, Result, Segment, Segmenter, n};
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

/// A safe, streaming custom pre-tokenizer.
///
/// The immutable extension may be shared between pipelines. Each stream has
/// an independent [`State`](Self::State), and progress counts are checked
/// against the input and output buffers before returning to IREE.
pub trait SegmenterExtension: Send + Sync + 'static {
    /// Mutable data owned by one segmentation stream.
    type State: Default + Send + 'static;

    /// Creates initial state for one stream.
    fn create_state(&self) -> Result<Self::State> {
        Ok(Self::State::default())
    }

    /// Emits segments from an input prefix.
    fn process(
        &self,
        state: &mut Self::State,
        input: &str,
        output: &mut [Segment],
    ) -> Result<Progress>;

    /// Emits the final segment from the remaining input.
    fn finalize(&self, _: &mut Self::State, _: &str, _: &mut [Segment]) -> Result<usize> {
        Ok(0)
    }

    /// Reports whether a segment is buffered for later emission.
    fn has_pending(&self, _: &Self::State) -> bool {
        false
    }

    /// Flushes buffered segments without ending the stream.
    fn flush(&self, _: &mut Self::State, _: &mut [Segment]) -> Result<Progress> {
        Ok(Progress::default())
    }
}

#[repr(C)]
struct CustomSegmenter<E> {
    base: MaybeUninit<n::iree_tokenizer_segmenter_t>,
    extension: E,
    vtable: n::iree_tokenizer_segmenter_vtable_t,
}

impl<E: SegmenterExtension> CustomSegmenter<E> {
    const STATE_SIZE: usize =
        size_of::<n::iree_tokenizer_segmenter_state_t>() + size_of::<*mut E::State>();
}

impl Segmenter {
    /// Creates a native-owned segmenter backed by a safe Rust implementation.
    ///
    /// The returned wrapper owns a boxed component, which its native destroy
    /// callback releases. Each stream has an independently created Rust state.
    pub fn from_extension<E: SegmenterExtension>(extension: E) -> Self {
        let vtable = n::iree_tokenizer_segmenter_vtable_t {
            destroy: Some(destroy::<E>),
            state_initialize: Some(initialize_state::<E>),
            state_deinitialize: Some(deinitialize_state::<E>),
            state_process: Some(process::<E>),
            state_finalize: Some(finalize::<E>),
            state_has_pending: Some(has_pending::<E>),
            state_flush: Some(flush::<E>),
        };
        let component = Box::into_raw(Box::new(CustomSegmenter {
            base: MaybeUninit::uninit(),
            extension,
            vtable,
        }));
        // SAFETY: the base is the first field of a live boxed component and the
        // vtable remains stable until IREE invokes `destroy`.
        let base = unsafe { ptr::addr_of_mut!((*component).base).cast() };
        // SAFETY: storage outlives native ownership, and callback layout and
        // state size match the allocation expected by the native state API.
        unsafe {
            n::iree_tokenizer_segmenter_initialize(
                base,
                ptr::addr_of!((*component).vtable),
                CustomSegmenter::<E>::STATE_SIZE,
            )
        };
        // SAFETY: `component` came from Box::into_raw and `base` is its
        // zero-offset first field, so it is non-null and correctly aligned.
        Self(unsafe { NonNull::new_unchecked(base) })
    }
}

unsafe extern "C" fn destroy<E: SegmenterExtension>(segmenter: *mut n::iree_tokenizer_segmenter_t) {
    n::contain(|| {
        // SAFETY: IREE calls destroy once for the Box transferred by
        // `Segmenter::from_extension`.
        unsafe { drop(Box::from_raw(segmenter.cast::<CustomSegmenter<E>>())) };
    });
}

unsafe extern "C" fn initialize_state<E: SegmenterExtension>(
    segmenter: *const n::iree_tokenizer_segmenter_t,
    storage: *mut c_void,
    out_state: *mut *mut n::iree_tokenizer_segmenter_state_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable output storage for a state of the
    // size declared by the component.
    unsafe { *out_state = ptr::null_mut() };
    n::callback_status(|| {
        // SAFETY: segmenter points to the enclosing component whose base is
        // its first field; the component outlives every state.
        let component = unsafe { &*segmenter.cast::<CustomSegmenter<E>>() };
        let state = Box::into_raw(Box::new(component.extension.create_state()?));
        let base = storage.cast::<n::iree_tokenizer_segmenter_state_t>();
        // SAFETY: state storage has room for its base and trailing pointer.
        unsafe {
            ptr::write(base, n::iree_tokenizer_segmenter_state_t { segmenter });
            state_slot::<_, E::State>(base).write(state);
            *out_state = base;
        }
        Ok(())
    })
}

unsafe extern "C" fn deinitialize_state<E: SegmenterExtension>(
    state: *mut n::iree_tokenizer_segmenter_state_t,
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

unsafe extern "C" fn process<E: SegmenterExtension>(
    state: *mut n::iree_tokenizer_segmenter_state_t,
    input: n::iree_string_view_t,
    output: n::iree_tokenizer_segment_output_t,
    out_consumed: *mut n::iree_host_size_t,
    out_segment_count: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable progress outputs for the callback.
    unsafe {
        *out_consumed = 0;
        *out_segment_count = 0;
    }
    n::callback_status(|| {
        // SAFETY: input view is valid for this callback invocation.
        let input = unsafe { input_str(input) }?;
        // SAFETY: output describes a writable segment array for this callback.
        let output = unsafe { segment_output(output)? };
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let progress = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.process(state_data, input, output)
            })
        }?;
        if progress.consumed > input.len()
            || !input.is_char_boundary(progress.consumed)
            || progress.written > output.len()
            || !valid_segments(&output[..progress.written], input)
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "segmenter extension reported progress outside its buffers",
            ));
        }
        // SAFETY: output pointers are writable and the validated counts fit.
        unsafe {
            *out_consumed = progress.consumed;
            *out_segment_count = progress.written;
        }
        Ok(())
    })
}

unsafe extern "C" fn finalize<E: SegmenterExtension>(
    state: *mut n::iree_tokenizer_segmenter_state_t,
    remaining_input: n::iree_string_view_t,
    output: n::iree_tokenizer_segment_output_t,
    out_segment_count: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides a writable progress output for the callback.
    unsafe { *out_segment_count = 0 };
    n::callback_status(|| {
        // SAFETY: input view is valid for this callback invocation.
        let remaining_input = unsafe { input_str(remaining_input) }?;
        // SAFETY: output describes a writable segment array for this callback.
        let output = unsafe { segment_output(output)? };
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let count = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.finalize(state_data, remaining_input, output)
            })
        }?;
        if count > output.len() || !valid_segments(&output[..count], remaining_input) {
            return Err(Error::new(
                Code::InvalidArgument,
                "segmenter extension reported output beyond its buffer",
            ));
        }
        // SAFETY: output pointer is writable and the validated count fits.
        unsafe { *out_segment_count = count };
        Ok(())
    })
}

unsafe extern "C" fn has_pending<E: SegmenterExtension>(
    state: *const n::iree_tokenizer_segmenter_state_t,
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

unsafe extern "C" fn flush<E: SegmenterExtension>(
    state: *mut n::iree_tokenizer_segmenter_state_t,
    output: n::iree_tokenizer_segment_output_t,
    out_segment_count: *mut n::iree_host_size_t,
    out_bytes_committed: *mut n::iree_host_size_t,
) -> n::iree_status_t {
    // SAFETY: IREE provides writable progress outputs for the callback.
    unsafe {
        *out_segment_count = 0;
        *out_bytes_committed = 0;
    }
    n::callback_status(|| {
        // SAFETY: output describes a writable segment array for this callback.
        let output = unsafe { segment_output(output)? };
        // SAFETY: state parent and trailing Box slot were set by initialization.
        let progress = unsafe {
            with_state_mut::<E, _>(state, |extension, state_data| {
                extension.flush(state_data, output)
            })
        }?;
        if progress.written > output.len() {
            return Err(Error::new(
                Code::InvalidArgument,
                "segmenter extension reported output beyond its buffer",
            ));
        }
        if output[..progress.written]
            .iter()
            .any(|segment| segment.start > segment.end)
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "segmenter extension reported an invalid segment range",
            ));
        }
        // SAFETY: output pointers are writable and the validated segment count fits.
        unsafe {
            *out_segment_count = progress.written;
            *out_bytes_committed = progress.consumed;
        }
        Ok(())
    })
}

fn valid_segments(segments: &[Segment], input: &str) -> bool {
    segments.iter().all(|segment| {
        segment.start <= segment.end
            && segment.end <= input.len()
            && input.is_char_boundary(segment.start)
            && input.is_char_boundary(segment.end)
    })
}

unsafe fn segment_output<'a>(
    output: n::iree_tokenizer_segment_output_t,
) -> Result<&'a mut [Segment]> {
    if output.capacity == 0 {
        Ok(&mut [])
    } else if output.values.is_null() {
        Err(Error::new(
            Code::InvalidArgument,
            "segmenter output storage is null",
        ))
    } else {
        // SAFETY: `Segment` has the same repr(C) layout as the native segment;
        // IREE supplies writable storage for `capacity` entries.
        Ok(unsafe { n::slice_mut(output.values.cast::<Segment>(), output.capacity) })
    }
}

unsafe fn with_state_mut<E: SegmenterExtension, R>(
    state: *mut n::iree_tokenizer_segmenter_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a mut E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let segmenter = unsafe { (*state).segmenter };
    // SAFETY: segmenter is the first field in the enclosing component.
    let component = unsafe { &*segmenter.cast::<CustomSegmenter<E>>() };
    // SAFETY: the state pointer has a live trailing Box slot initialized here.
    let state_data = unsafe { &mut **state_slot::<_, E::State>(state) };
    callback(&component.extension, state_data)
}

unsafe fn with_state_ref<E: SegmenterExtension, R>(
    state: *const n::iree_tokenizer_segmenter_state_t,
    callback: impl for<'a> FnOnce(&'a E, &'a E::State) -> R,
) -> R {
    // SAFETY: caller ensures this is a live state initialized by this module.
    let segmenter = unsafe { (*state).segmenter };
    // SAFETY: segmenter is the first field in the enclosing component.
    let component = unsafe { &*segmenter.cast::<CustomSegmenter<E>>() };
    // SAFETY: the state pointer has a live trailing Box slot initialized here.
    let state_data = unsafe { &**state_slot::<_, E::State>(state.cast_mut()) };
    callback(&component.extension, state_data)
}
