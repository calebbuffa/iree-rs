use super::{Progress, TokenOutput};
use crate::{Code, Error, Model, Result, Segment, check, n, word_storage};
use std::ptr::{self, NonNull};

/// Owned mutable streaming state for a model.
pub struct ModelState<'a> {
    _parent: &'a Model,
    _storage: Vec<usize>,
    raw: NonNull<n::iree_tokenizer_model_state_t>,
}
impl Drop for ModelState<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_model_state_deinitialize(self.raw.as_ptr()) }
    }
}
impl Model {
    /// Creates a streaming state borrowing this model.
    pub fn state(&self) -> Result<ModelState<'_>> {
        let mut storage = word_storage(self.state_size());
        let mut raw = ptr::null_mut();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        check(unsafe {
            n::iree_tokenizer_model_state_initialize(
                self.raw.as_ptr(),
                storage.as_mut_ptr().cast(),
                &mut raw,
            )
        })?;
        Ok(ModelState {
            _parent: self,
            _storage: storage,
            raw: crate::status::nonnull(raw, "model state initialization succeeded with null")?,
        })
    }
}
impl ModelState<'_> {
    /// Encodes normalized segments and reports consumed segments and produced tokens.
    pub fn encode(
        &mut self,
        transform: &[u8],
        segments: &[Segment],
        last_is_partial: bool,
        output: &mut TokenOutput<'_>,
    ) -> Result<Progress> {
        if segments
            .iter()
            .any(|segment| segment.start > segment.end || segment.end > transform.len())
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "model segments must be ranges within the transform buffer",
            ));
        }
        let list = n::iree_tokenizer_segment_list_t {
            count: segments.len(),
            values: segments.as_ptr().cast(),
            last_is_partial,
        };
        let buffer = n::iree_const_byte_span_t {
            data: transform.as_ptr(),
            data_length: transform.len(),
        };
        let mut consumed = 0;
        let mut written = 0;
        let native_output = output.raw();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_model_state_encode(
                self.raw.as_ptr(),
                buffer,
                list,
                native_output,
                &mut consumed,
                &mut written,
            )
        })?;
        Ok(Progress { consumed, written })
    }
    /// Finalizes pending token output.
    pub fn finalize(&mut self, output: &mut TokenOutput<'_>) -> Result<usize> {
        let mut written = 0;
        let native_output = output.raw();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_model_state_finalize(self.raw.as_ptr(), native_output, &mut written)
        })?;
        Ok(written)
    }
    /// Whether model state contains output pending finalization.
    pub fn has_pending(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_model_state_has_pending(self.raw.as_ptr()) }
    }
    /// Reclaims bytes committed by the model during partial-segment processing.
    pub fn reclaim(&mut self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_model_state_reclaim(self.raw.as_ptr()) }
    }
}
