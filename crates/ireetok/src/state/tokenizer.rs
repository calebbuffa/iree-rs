use super::{OffsetRun, Progress, TokenOutput};
use crate::{
    DecodeOptions, EncodeOptions, Result, TokenId, Tokenizer, bytes_view, check, n, span, view,
    word_storage,
};
use std::ptr::{self, NonNull};

/// Streaming tokenizer encoder; the parent tokenizer must outlive the state.
pub struct EncodeState<'a> {
    _tokenizer: &'a Tokenizer,
    _storage: Vec<usize>,
    _transform: Vec<usize>,
    _offset_runs: Vec<OffsetRun>,
    raw: NonNull<n::iree_tokenizer_encode_state_t>,
}
impl Drop for EncodeState<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_encode_state_deinitialize(self.raw.as_ptr()) }
    }
}
impl<'a> EncodeState<'a> {
    pub(crate) fn new(
        tokenizer: &'a Tokenizer,
        max_chunk: usize,
        options: EncodeOptions,
    ) -> Result<Self> {
        let mut size = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_encode_state_calculate_size(tokenizer.0.as_ptr(), &mut size)
        })?;
        let mut storage = word_storage(size);
        let transform_size =
            // SAFETY: All pointer arguments refer to live storage for this call.
            unsafe { n::iree_tokenizer_transform_buffer_recommended_size(max_chunk) };
        let mut transform = word_storage(transform_size);
        let mut offset_runs = vec![OffsetRun::default(); max_chunk.saturating_add(16)];
        let runs = n::iree_tokenizer_offset_run_list_t {
            capacity: offset_runs.len(),
            values: offset_runs.as_mut_ptr().cast(),
        };
        let mut raw = ptr::null_mut();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        check(unsafe {
            n::iree_tokenizer_encode_state_initialize(
                tokenizer.0.as_ptr(),
                span(&mut storage),
                span(&mut transform),
                runs,
                options.flags(),
                &mut raw,
            )
        })?;
        Ok(Self {
            _tokenizer: tokenizer,
            _storage: storage,
            _transform: transform,
            _offset_runs: offset_runs,
            raw: crate::status::nonnull(raw, "encode state initialization succeeded with null")?,
        })
    }
    /// Feeds one chunk. Retry the unconsumed suffix when `consumed < chunk.len()`.
    pub fn feed(&mut self, chunk: &str, output: &mut TokenOutput<'_>) -> Result<Progress> {
        let mut consumed = 0;
        let mut written = 0;
        let raw = output.raw();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_encode_state_feed(
                self.raw.as_ptr(),
                view(chunk),
                raw,
                &mut consumed,
                &mut written,
            )
        })?;
        Ok(Progress { consumed, written })
    }
    /// Finalizes the stream. Size the output using [`Self::pending_token_bound`].
    pub fn finalize(&mut self, output: &mut TokenOutput<'_>) -> Result<usize> {
        let mut written = 0;
        let raw = output.raw();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_encode_state_finalize(self.raw.as_ptr(), raw, &mut written)
        })?;
        Ok(written)
    }
    /// Returns whether finalization can produce more tokens.
    pub fn has_pending(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_encode_state_has_pending(self.raw.as_ptr()) }
    }
    /// Returns an upper bound for finalization output capacity.
    pub fn pending_token_bound(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_encode_state_pending_token_bound(self.raw.as_ptr()) }
    }
    /// Resets the state to encode a new input using the existing buffers.
    pub fn reset(&mut self, options: EncodeOptions) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_encode_state_reset(self.raw.as_ptr(), options.flags()) }
    }
}

/// Streaming tokenizer decoder; the parent tokenizer must outlive the state.
pub struct DecodeState<'a> {
    _tokenizer: &'a Tokenizer,
    _storage: Vec<usize>,
    raw: NonNull<n::iree_tokenizer_decode_state_t>,
}
impl Drop for DecodeState<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_decode_state_deinitialize(self.raw.as_ptr()) }
    }
}
impl<'a> DecodeState<'a> {
    pub(crate) fn new(tokenizer: &'a Tokenizer, options: DecodeOptions) -> Result<Self> {
        let mut size = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decode_state_calculate_size(tokenizer.0.as_ptr(), &mut size)
        })?;
        let mut storage = word_storage(size);
        let mut raw = ptr::null_mut();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        check(unsafe {
            n::iree_tokenizer_decode_state_initialize(
                tokenizer.0.as_ptr(),
                options.flags(),
                span(&mut storage),
                &mut raw,
            )
        })?;
        Ok(Self {
            _tokenizer: tokenizer,
            _storage: storage,
            raw: crate::status::nonnull(raw, "decode state initialization succeeded with null")?,
        })
    }
    /// Feeds token IDs. Retry the unconsumed suffix when `consumed < tokens.len()`.
    pub fn feed(&mut self, tokens: &[TokenId], output: &mut [u8]) -> Result<Progress> {
        let input = n::iree_tokenizer_token_id_list_t {
            count: tokens.len(),
            values: tokens.as_ptr(),
        };
        let mut consumed = 0;
        let mut written = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decode_state_feed(
                self.raw.as_ptr(),
                input,
                bytes_view(output),
                &mut consumed,
                &mut written,
            )
        })?;
        Ok(Progress { consumed, written })
    }
    /// Finalizes decoding into the provided UTF-8 byte buffer.
    pub fn finalize(&mut self, output: &mut [u8]) -> Result<usize> {
        let mut written = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decode_state_finalize(
                self.raw.as_ptr(),
                bytes_view(output),
                &mut written,
            )
        })?;
        Ok(written)
    }
}
