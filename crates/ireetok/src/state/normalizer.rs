use super::Progress;
use crate::{Normalizer, NormalizerFlags, Result, bytes_view, check, n, view, word_storage};
use std::ptr::{self, NonNull};

/// Owned mutable streaming state for a normalizer.
pub struct NormalizerState<'a> {
    _parent: &'a Normalizer,
    _storage: Vec<usize>,
    raw: NonNull<n::iree_tokenizer_normalizer_state_t>,
}
impl Drop for NormalizerState<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_normalizer_state_deinitialize(self.raw.as_ptr()) }
    }
}
impl Normalizer {
    /// Creates a streaming state borrowing this normalizer.
    pub fn state(&self) -> Result<NormalizerState<'_>> {
        let mut storage = word_storage(self.state_size());
        let mut raw = ptr::null_mut();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        check(unsafe {
            n::iree_tokenizer_normalizer_state_initialize(
                self.0.as_ptr(),
                storage.as_mut_ptr().cast(),
                &mut raw,
            )
        })?;
        Ok(NormalizerState {
            _parent: self,
            _storage: storage,
            raw: crate::status::nonnull(
                raw,
                "normalizer state initialization succeeded with null",
            )?,
        })
    }
}
impl NormalizerState<'_> {
    /// Processes a text chunk and reports consumed input bytes and written output bytes.
    pub fn process(
        &mut self,
        input: &str,
        output: &mut [u8],
        flags: NormalizerFlags,
    ) -> Result<Progress> {
        let mut consumed = 0;
        let mut written = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_normalizer_state_process(
                self.raw.as_ptr(),
                view(input),
                bytes_view(output),
                flags.bits(),
                &mut consumed,
                &mut written,
            )
        })?;
        Ok(Progress { consumed, written })
    }
    /// Flushes buffered normalized output.
    pub fn finalize(&mut self, output: &mut [u8]) -> Result<usize> {
        let mut written = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_normalizer_state_finalize(
                self.raw.as_ptr(),
                bytes_view(output),
                &mut written,
            )
        })?;
        Ok(written)
    }
    /// Whether finalization can produce additional bytes.
    pub fn has_pending(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_normalizer_state_has_pending(self.raw.as_ptr()) }
    }
}
