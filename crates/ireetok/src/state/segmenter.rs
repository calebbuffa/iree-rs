use super::Progress;
use crate::{Result, Segment, Segmenter, check, n, view, word_storage};
use std::ptr::{self, NonNull};

/// Owned mutable streaming state for a segmenter.
pub struct SegmenterState<'a> {
    _parent: &'a Segmenter,
    _storage: Vec<usize>,
    raw: NonNull<n::iree_tokenizer_segmenter_state_t>,
}
impl Drop for SegmenterState<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_segmenter_state_deinitialize(self.raw.as_ptr()) }
    }
}
impl Segmenter {
    /// Creates a streaming state borrowing this segmenter.
    pub fn state(&self) -> Result<SegmenterState<'_>> {
        let mut storage = word_storage(self.state_size());
        let mut raw = ptr::null_mut();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        check(unsafe {
            n::iree_tokenizer_segmenter_state_initialize(
                self.0.as_ptr(),
                storage.as_mut_ptr().cast(),
                &mut raw,
            )
        })?;
        Ok(SegmenterState {
            _parent: self,
            _storage: storage,
            raw: crate::status::nonnull(raw, "segmenter state initialization succeeded with null")?,
        })
    }
}
impl SegmenterState<'_> {
    /// Processes normalized text into segment ranges.
    pub fn process(&mut self, input: &str, output: &mut [Segment]) -> Result<Progress> {
        let mut consumed = 0;
        let mut count = 0;
        let native_output = n::iree_tokenizer_segment_output_t {
            capacity: output.len(),
            values: output.as_mut_ptr().cast(),
        };
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_segmenter_state_process(
                self.raw.as_ptr(),
                view(input),
                native_output,
                &mut consumed,
                &mut count,
            )
        })?;
        Ok(Progress {
            consumed,
            written: count,
        })
    }
    /// Emits the pending final segment using the unconsumed input suffix.
    pub fn finalize(&mut self, remaining_input: &str, output: &mut [Segment]) -> Result<usize> {
        let mut count = 0;
        let native_output = n::iree_tokenizer_segment_output_t {
            capacity: output.len(),
            values: output.as_mut_ptr().cast(),
        };
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_segmenter_state_finalize(
                self.raw.as_ptr(),
                view(remaining_input),
                native_output,
                &mut count,
            )
        })?;
        Ok(count)
    }
    /// Whether a segment is buffered for later emission.
    pub fn has_pending(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_segmenter_state_has_pending(self.raw.as_ptr()) }
    }
    /// Forces emission of pending segments without ending the stream.
    pub fn flush(&mut self, output: &mut [Segment]) -> Result<Progress> {
        let mut count = 0;
        let mut committed = 0;
        let native_output = n::iree_tokenizer_segment_output_t {
            capacity: output.len(),
            values: output.as_mut_ptr().cast(),
        };
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_segmenter_state_flush(
                self.raw.as_ptr(),
                native_output,
                &mut count,
                &mut committed,
            )
        })?;
        Ok(Progress {
            consumed: committed,
            written: count,
        })
    }
}
