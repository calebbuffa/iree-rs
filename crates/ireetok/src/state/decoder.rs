use super::Progress;
use crate::{Decoder, Result, bytes_view, check, const_bytes_view, n, word_storage};
use std::ptr::{self, NonNull};

/// Owned mutable streaming state for a decoder.
pub struct DecoderState<'a> {
    _parent: &'a Decoder,
    _storage: Vec<usize>,
    raw: NonNull<n::iree_tokenizer_decoder_state_t>,
}
impl Drop for DecoderState<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_decoder_state_deinitialize(self.raw.as_ptr()) }
    }
}
impl Decoder {
    /// Creates a streaming state borrowing this decoder.
    pub fn state(&self) -> Result<DecoderState<'_>> {
        let mut storage = word_storage(self.state_size());
        let mut raw = ptr::null_mut();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        check(unsafe {
            n::iree_tokenizer_decoder_state_initialize(
                self.0.as_ptr(),
                storage.as_mut_ptr().cast(),
                &mut raw,
            )
        })?;
        Ok(DecoderState {
            _parent: self,
            _storage: storage,
            raw: crate::status::nonnull(raw, "decoder state initialization succeeded with null")?,
        })
    }
}
impl DecoderState<'_> {
    /// Processes token byte strings and returns consumed token count and written bytes.
    pub fn process(&mut self, tokens: &[&[u8]], output: &mut [u8]) -> Result<Progress> {
        let views = tokens
            .iter()
            .map(|token| const_bytes_view(token))
            .collect::<Vec<_>>();
        let list = n::iree_tokenizer_string_list_t {
            count: views.len(),
            values: views.as_ptr(),
        };
        let mut consumed = 0;
        let mut written = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decoder_state_process(
                self.raw.as_ptr(),
                list,
                bytes_view(output),
                &mut consumed,
                &mut written,
            )
        })?;
        Ok(Progress { consumed, written })
    }
    /// Flushes buffered decoded output.
    pub fn finalize(&mut self, output: &mut [u8]) -> Result<usize> {
        let mut written = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decoder_state_finalize(
                self.raw.as_ptr(),
                bytes_view(output),
                &mut written,
            )
        })?;
        Ok(written)
    }
    /// Whether buffered decoder data remains.
    pub fn has_pending(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_decoder_state_has_pending(self.raw.as_ptr()) }
    }
}
