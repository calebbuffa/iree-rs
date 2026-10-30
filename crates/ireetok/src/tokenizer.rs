//! Immutable tokenizer composition, encoding, decoding, and state creation.
use crate::{
    Code, DecodeState, Decoder, EncodeState, Error, Model, Normalizer, Postprocessor, Result,
    Segmenter, TokenId, TokenOutput, VocabularyRef, bytes_view, capacity_for, check,
    copy_string_view, grow_capacity, n, view,
};

/// A half-open byte range in the original input.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Offset {
    /// Inclusive starting byte offset.
    pub start: usize,
    /// Exclusive ending byte offset.
    pub end: usize,
}

/// Per-token output written by an encoding operation.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Encoded {
    /// Token identifiers.
    pub ids: Vec<TokenId>,
    /// Byte offsets, present when offset tracking was requested.
    pub offsets: Option<Vec<Offset>>,
    /// Sequence/type identifiers, present when requested by a postprocessor.
    pub type_ids: Option<Vec<u8>>,
}

/// Options for [`Tokenizer::encode_with`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EncodeOptions {
    /// Insert the model's special tokens (e.g. BOS/EOS).
    pub add_special_tokens: bool,
    /// Track token byte offsets in the original input.
    pub track_offsets: bool,
    /// Tokenize special-token spellings as ordinary text.
    pub no_special_token_matching: bool,
}

impl EncodeOptions {
    pub(crate) fn flags(self) -> n::iree_tokenizer_encode_flags_t {
        let mut flags = n::IREE_TOKENIZER_ENCODE_FLAG_AT_INPUT_START as u32;
        if self.add_special_tokens {
            flags |= n::IREE_TOKENIZER_ENCODE_FLAG_ADD_SPECIAL_TOKENS as u32;
        }
        if self.track_offsets {
            flags |= n::IREE_TOKENIZER_ENCODE_FLAG_TRACK_OFFSETS as u32;
        }
        if self.no_special_token_matching {
            flags |= n::IREE_TOKENIZER_ENCODE_FLAG_NO_SPECIAL_TOKEN_MATCHING as u32;
        }
        flags
    }
}

/// Options for decoding token identifiers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DecodeOptions {
    /// Omit tokens marked special in the vocabulary.
    pub skip_special_tokens: bool,
}

impl DecodeOptions {
    pub(crate) fn flags(self) -> n::iree_tokenizer_decode_flags_t {
        if self.skip_special_tokens {
            n::IREE_TOKENIZER_DECODE_FLAG_SKIP_SPECIAL_TOKENS as u32
        } else {
            n::IREE_TOKENIZER_DECODE_FLAG_NONE as u32
        }
    }
}

use std::{
    mem::MaybeUninit,
    num::NonZeroUsize,
    ptr::{self, NonNull},
};

/// An immutable tokenizer owning its native components.
pub struct Tokenizer(pub(crate) NonNull<n::iree_tokenizer_t>);

// SAFETY: IREE's tokenizer.h explicitly documents `iree_tokenizer_t` as
// thread-safe and immutable. This wrapper exposes only shared operations after
// construction; every safe constructor uses IREE's system allocator, so the
// owned native object has no thread-affine allocation or other resources.
unsafe impl Send for Tokenizer {}
// SAFETY: The native thread-safety contract permits concurrent shared access;
// each operation receives its own output buffers and streaming state.
unsafe impl Sync for Tokenizer {}

impl Drop for Tokenizer {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native tokenizer and drops it once.
        unsafe { n::iree_tokenizer_free(self.0.as_ptr()) }
    }
}

impl Tokenizer {
    /// Returns IREE's recommended bounded transform-buffer size for a chunk.
    pub fn recommended_transform_buffer_size(chunk_bytes: usize) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_transform_buffer_recommended_size(chunk_bytes) }
    }

    /// Returns IREE's transform-buffer size recommendation for one-shot input.
    pub fn oneshot_transform_buffer_size(text_bytes: usize) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_transform_buffer_oneshot_size(text_bytes) }
    }

    /// Estimates vocabulary capacity needed to parse a HuggingFace model document.
    pub fn estimate_huggingface_vocab_capacity(json_bytes: usize) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_huggingface_estimate_vocab_capacity(json_bytes) }
    }

    /// Builds from a HuggingFace `tokenizer.json` document.
    pub fn from_huggingface_json(json: &str) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: the string view and output pointer are valid for the call.
        check(unsafe {
            n::iree_tokenizer_from_huggingface_json(view(json), crate::status::system(), &mut raw)
        })?;
        Ok(Self(crate::status::nonnull(
            raw,
            "tokenizer creation succeeded with a null output",
        )?))
    }

    /// Encodes text with default options.
    pub fn encode(&self, text: &str) -> Result<Vec<TokenId>> {
        Ok(self.encode_with(text, EncodeOptions::default())?.ids)
    }

    /// Encodes text and returns requested offsets or type IDs.
    pub fn encode_with(&self, text: &str, options: EncodeOptions) -> Result<Encoded> {
        let mut cap = capacity_for(text.len())?;
        let mut growths = 0;
        loop {
            let mut ids = vec![0; cap];
            let mut offsets = options.track_offsets.then(|| vec![Offset::default(); cap]);
            let include_type_ids = options.add_special_tokens && self.max_special_token_count() > 0;
            let mut type_ids = include_type_ids.then(|| vec![0; cap]);
            let mut output =
                TokenOutput::new(&mut ids, offsets.as_deref_mut(), type_ids.as_deref_mut())?;
            let mut count = 0;
            // SAFETY: all output buffers have the declared capacity and live for the call.
            let status = unsafe {
                n::iree_tokenizer_encode(
                    self.0.as_ptr(),
                    view(text),
                    options.flags(),
                    output.raw(),
                    crate::status::system(),
                    &mut count,
                )
            };
            match check(status) {
                Ok(()) => {
                    ids.truncate(count);
                    if let Some(ref mut values) = offsets {
                        values.truncate(count);
                    }
                    if let Some(values) = &mut type_ids {
                        values.truncate(count);
                    }
                    return Ok(Encoded {
                        ids,
                        offsets,
                        type_ids,
                    });
                }
                Err(e) if e.code() == Code::ResourceExhausted => {
                    cap = grow_capacity(cap, &mut growths, e)?;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Decodes tokens to raw bytes with default options.
    pub fn decode_bytes(&self, tokens: &[TokenId]) -> Result<Vec<u8>> {
        self.decode_bytes_with(tokens, DecodeOptions::default())
    }

    /// Decodes tokens to raw bytes with the requested special-token behavior.
    pub fn decode_bytes_with(&self, tokens: &[TokenId], options: DecodeOptions) -> Result<Vec<u8>> {
        let mut cap = capacity_for(tokens.len().saturating_mul(self.vocab().max_token_length()))?;
        let mut growths = 0;
        loop {
            let mut text = vec![0u8; cap];
            let input = n::iree_tokenizer_token_id_list_t {
                count: tokens.len(),
                values: tokens.as_ptr(),
            };
            let mut length = 0;
            // SAFETY: token and output slices remain valid for the call.
            let status = unsafe {
                n::iree_tokenizer_decode(
                    self.0.as_ptr(),
                    input,
                    options.flags(),
                    bytes_view(&mut text),
                    crate::status::system(),
                    &mut length,
                )
            };
            match check(status) {
                Ok(()) => {
                    text.truncate(length);
                    return Ok(text);
                }
                Err(e) if e.code() == Code::ResourceExhausted => {
                    cap = grow_capacity(cap, &mut growths, e)?;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Decodes tokens with default options.
    pub fn decode(&self, tokens: &[TokenId]) -> Result<String> {
        self.decode_with(tokens, DecodeOptions::default())
    }

    /// Decodes tokens and validates the output as UTF-8.
    pub fn decode_with(&self, tokens: &[TokenId], options: DecodeOptions) -> Result<String> {
        String::from_utf8(self.decode_bytes_with(tokens, options)?)
            .map_err(|e| Error::new(Code::DataLoss, e.to_string()))
    }

    /// Returns a borrowed view of the tokenizer's immutable vocabulary.
    pub fn vocab(&self) -> VocabularyRef<'_> {
        // SAFETY: the vocabulary is owned by and lives as long as this tokenizer.
        unsafe { VocabularyRef::from_raw(n::iree_tokenizer_vocab(self.0.as_ptr())) }
    }

    /// Returns the model's descriptive type name.
    pub fn model_type_name(&self) -> String {
        // SAFETY: returned view is valid for the tokenizer lifetime.
        let value = unsafe { n::iree_tokenizer_model_type_name(self.0.as_ptr()) };
        copy_string_view(value)
    }

    /// Returns the postprocessor's maximum additional-token count.
    pub fn max_special_token_count(&self) -> usize {
        // SAFETY: tokenizer is valid and immutable.
        unsafe { n::iree_tokenizer_max_special_token_count(self.0.as_ptr()) }
    }

    /// Creates a streaming encoder with a buffer sized for the largest input chunk.
    pub fn encode_state(
        &self,
        max_chunk_size: usize,
        options: EncodeOptions,
    ) -> Result<EncodeState<'_>> {
        EncodeState::new(self, max_chunk_size, options)
    }

    /// Creates a streaming decoder.
    pub fn decode_state(&self, options: DecodeOptions) -> Result<DecodeState<'_>> {
        DecodeState::new(self, options)
    }
}

/// Builder for composing tokenizer components into one immutable tokenizer.
pub struct TokenizerBuilder {
    raw: MaybeUninit<n::iree_tokenizer_builder_t>,
    initialized: bool,
}
impl TokenizerBuilder {
    /// Creates an empty builder using the system allocator.
    pub fn new() -> Self {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_builder_initialize(crate::status::system(), raw.as_mut_ptr()) };
        Self {
            raw,
            initialized: true,
        }
    }
    pub(crate) fn raw(&mut self) -> *mut n::iree_tokenizer_builder_t {
        self.raw.as_mut_ptr()
    }
    /// Installs and consumes the normalizer.
    pub fn set_normalizer(&mut self, component: Normalizer) {
        // SAFETY: Live handles are passed; this C API transfers component ownership.
        unsafe { n::iree_tokenizer_builder_set_normalizer(self.raw(), component.0.as_ptr()) };
        std::mem::forget(component);
    }
    /// Installs and consumes the segmenter.
    pub fn set_segmenter(&mut self, component: Segmenter) {
        // SAFETY: Live handles are passed; this C API transfers component ownership.
        unsafe { n::iree_tokenizer_builder_set_segmenter(self.raw(), component.0.as_ptr()) };
        std::mem::forget(component);
    }
    /// Installs and consumes the model together with its owned vocabulary.
    pub fn set_model(&mut self, component: Model) {
        let component = std::mem::ManuallyDrop::new(component);
        // SAFETY: Live handles are passed; this C API transfers component ownership.
        unsafe {
            n::iree_tokenizer_builder_set_vocab(self.raw(), component.vocab.0.as_ptr());
            n::iree_tokenizer_builder_set_model(self.raw(), component.raw.as_ptr());
        }
    }
    /// Installs and consumes the decoder.
    pub fn set_decoder(&mut self, component: Decoder) {
        // SAFETY: Live handles are passed; this C API transfers component ownership.
        unsafe { n::iree_tokenizer_builder_set_decoder(self.raw(), component.0.as_ptr()) };
        std::mem::forget(component);
    }
    /// Configures the postprocessor.
    pub fn set_postprocessor(&mut self, value: &Postprocessor) {
        // SAFETY: The builder and all referenced values remain live for this call.
        unsafe { n::iree_tokenizer_builder_set_postprocessor(self.raw(), value.0) }
    }
    /// Sets the decoder's vocabulary-string batch size.
    pub fn set_string_batch_size(&mut self, size: NonZeroUsize) {
        // SAFETY: The builder and all referenced values remain live for this call.
        unsafe { n::iree_tokenizer_builder_set_string_batch_size(self.raw(), size.get()) }
    }
    /// Builds the tokenizer, consuming the builder.
    pub fn build(mut self) -> Result<Tokenizer> {
        let mut raw = ptr::null_mut();
        // SAFETY: The builder and output slot are valid; success transfers ownership.
        check(unsafe { n::iree_tokenizer_builder_build(self.raw(), &mut raw) })?;
        self.initialized = false;
        Ok(Tokenizer(crate::status::nonnull(
            raw,
            "tokenizer build succeeded with a null output",
        )?))
    }
}
impl Default for TokenizerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for TokenizerBuilder {
    fn drop(&mut self) {
        if self.initialized {
            // SAFETY: This wrapper uniquely owns the native value and its backing storage.
            unsafe { n::iree_tokenizer_builder_deinitialize(self.raw.as_mut_ptr()) }
        }
    }
}
