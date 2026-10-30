//! Tokenization models and model-specific configuration.
use crate::{
    AddedTokens, Code, Error, PreTokenizerFlags, Result, TokenId, Vocabulary, VocabularyRef, check,
    n, view,
};

/// Configuration flags for BPE model construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BpeFlags(u32);
impl BpeFlags {
    /// No BPE options.
    pub const NONE: Self = Self(0);
    /// Disable byte fallback for unknown characters.
    pub const NO_BYTE_FALLBACK: Self = Self(1);
    /// Perform greedy vocabulary matching without merge rules.
    pub const IGNORE_MERGES: Self = Self(2);
    /// Fuse consecutive unknown tokens.
    pub const FUSE_UNK: Self = Self(4);
    /// Transform bytes using the GPT-2 byte-level alphabet.
    pub const BYTE_LEVEL_INPUT: Self = Self(8);
    /// Enable repeated-segment caching.
    pub const ENABLE_WORD_CACHE: Self = Self(16);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Configuration flags for WordPiece model construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WordPieceFlags(u32);
impl WordPieceFlags {
    /// No WordPiece options.
    pub const NONE: Self = Self(0);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Configuration flags for Unigram model construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct UnigramFlags(u32);
impl UnigramFlags {
    /// No Unigram options.
    pub const NONE: Self = Self(0);
    /// Disable byte fallback for unknown characters.
    pub const NO_BYTE_FALLBACK: Self = Self(1);
    /// Disable fusion of consecutive unknown tokens.
    pub const NO_FUSE_UNK: Self = Self(2);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}
use std::{
    mem::MaybeUninit,
    ptr::{self, NonNull},
};

/// An owned model and the vocabulary it references.
pub struct Model {
    pub(crate) raw: NonNull<n::iree_tokenizer_model_t>,
    pub(crate) vocab: Vocabulary,
    is_bpe: bool,
}
fn model_is_bpe(raw: NonNull<n::iree_tokenizer_model_t>) -> bool {
    // SAFETY: IREE stores a static type-name view in the live model base object.
    let type_name = unsafe { (*raw.as_ptr()).type_name };
    type_name.size == 3
        // SAFETY: the live model supplies a valid static type-name byte view of size three.
        && unsafe { n::slice(type_name.data.cast::<u8>(), type_name.size) == b"BPE" }
}

impl Drop for Model {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the model; the vocabulary outlives this call.
        unsafe { n::iree_tokenizer_model_free(self.raw.as_ptr()) }
    }
}
impl Model {
    fn parsed(
        parse: impl FnOnce(
            *mut *mut n::iree_tokenizer_model_t,
            *mut *mut n::iree_tokenizer_vocab_t,
        ) -> n::iree_status_t,
    ) -> Result<Self> {
        let mut model = ptr::null_mut();
        let mut vocab = ptr::null_mut();
        check(parse(&mut model, &mut vocab))?;
        let vocab = Vocabulary(crate::status::nonnull(
            vocab,
            "model parse succeeded with a null vocabulary",
        )?);
        let raw = crate::status::nonnull(model, "model parse succeeded with a null model")?;
        Ok(Self {
            raw,
            vocab,
            is_bpe: model_is_bpe(raw),
        })
    }

    /// Constructs the Rust view around a custom model with its owned vocabulary.
    ///
    /// # Safety
    /// `raw` must be a live model whose callbacks use only the supplied
    /// vocabulary, which remains owned by this wrapper.
    pub(crate) unsafe fn from_extension_parts(
        raw: NonNull<n::iree_tokenizer_model_t>,
        vocab: Vocabulary,
    ) -> Self {
        Self {
            raw,
            vocab,
            is_bpe: false,
        }
    }

    /// Parses any HuggingFace model value, with optional added tokens and
    /// pre-tokenizer properties, and owns its vocabulary.
    pub fn from_huggingface_json(
        json: &str,
        added_tokens: Option<&AddedTokens>,
        pre_tokenizer: PreTokenizerFlags,
    ) -> Result<Self> {
        let added_tokens = added_tokens.map_or(ptr::null(), |value| &value.0 as *const _);
        // SAFETY: Input storage and both output slots outlive the native call.
        Self::parsed(|model, vocab| unsafe {
            n::iree_tokenizer_huggingface_parse_model(
                view(json),
                added_tokens,
                pre_tokenizer.bits(),
                crate::status::system(),
                model,
                vocab,
            )
        })
    }

    /// Parses a HuggingFace BPE model value and owns its vocabulary.
    pub fn from_huggingface_bpe_json(
        json: &str,
        added_tokens: Option<&AddedTokens>,
        flags: BpeFlags,
    ) -> Result<Self> {
        let added_tokens = added_tokens.map_or(ptr::null(), |value| &value.0 as *const _);
        // SAFETY: Input storage and both output slots outlive the native call.
        Self::parsed(|model, vocab| unsafe {
            n::iree_tokenizer_huggingface_parse_bpe_model(
                view(json),
                added_tokens,
                flags.bits(),
                crate::status::system(),
                model,
                vocab,
            )
        })
    }

    /// Parses a HuggingFace WordPiece model value and owns its vocabulary.
    pub fn from_huggingface_wordpiece_json(
        json: &str,
        added_tokens: Option<&AddedTokens>,
    ) -> Result<Self> {
        let added_tokens = added_tokens.map_or(ptr::null(), |value| &value.0 as *const _);
        // SAFETY: Input storage and both output slots outlive the native call.
        Self::parsed(|model, vocab| unsafe {
            n::iree_tokenizer_huggingface_parse_wordpiece_model(
                view(json),
                added_tokens,
                crate::status::system(),
                model,
                vocab,
            )
        })
    }

    /// Parses a HuggingFace Unigram model value and owns its vocabulary.
    pub fn from_huggingface_unigram_json(
        json: &str,
        added_tokens: Option<&AddedTokens>,
    ) -> Result<Self> {
        let added_tokens = added_tokens.map_or(ptr::null(), |value| &value.0 as *const _);
        // SAFETY: Input storage and both output slots outlive the native call.
        Self::parsed(|model, vocab| unsafe {
            n::iree_tokenizer_huggingface_parse_unigram_model(
                view(json),
                added_tokens,
                crate::status::system(),
                model,
                vocab,
            )
        })
    }

    /// Creates a BPE model that owns `vocab`.
    pub fn bpe(vocab: Vocabulary, flags: BpeFlags) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_bpe_model_allocate(
                vocab.0.as_ptr(),
                flags.bits(),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw: crate::status::nonnull(raw, "BPE model allocation succeeded with a null output")?,
            vocab,
            is_bpe: true,
        })
    }

    /// Creates a WordPiece model that owns `vocab`.
    pub fn wordpiece(
        vocab: Vocabulary,
        continuing_subword_prefix: &str,
        max_input_chars_per_word: usize,
        flags: WordPieceFlags,
    ) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_wordpiece_model_allocate(
                vocab.0.as_ptr(),
                view(continuing_subword_prefix),
                max_input_chars_per_word,
                flags.bits(),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw: crate::status::nonnull(
                raw,
                "WordPiece model allocation succeeded with a null output",
            )?,
            vocab,
            is_bpe: false,
        })
    }

    /// Creates a Unigram model that owns `vocab`.
    pub fn unigram(
        vocab: Vocabulary,
        unk_token_id: TokenId,
        unk_score: f32,
        flags: UnigramFlags,
    ) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_unigram_model_allocate(
                vocab.0.as_ptr(),
                unk_token_id,
                unk_score,
                flags.bits(),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw: crate::status::nonnull(
                raw,
                "Unigram model allocation succeeded with a null output",
            )?,
            vocab,
            is_bpe: false,
        })
    }

    /// Sets the end-of-word suffix for a BPE model.
    pub fn set_end_of_word_suffix(&mut self, suffix: &str) -> Result<()> {
        if !self.is_bpe {
            return Err(Error::new(
                Code::InvalidArgument,
                "end-of-word suffix is only supported by BPE models",
            ));
        }
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_bpe_model_set_end_of_word_suffix(self.raw.as_ptr(), view(suffix))
        })
    }

    /// Returns an immutable borrowed view of the model vocabulary.
    pub fn vocab(&self) -> VocabularyRef<'_> {
        self.vocab.view()
    }

    /// Returns the vocabulary spelling for a model token, if valid.
    pub fn token_text(&self, id: TokenId) -> Result<&[u8]> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: The successful native call initialized this output value.
        check(unsafe {
            n::iree_tokenizer_model_get_token_string(self.raw.as_ptr(), id, raw.as_mut_ptr())
        })?;
        // SAFETY: The successful native call initialized this output value.
        let value = unsafe { raw.assume_init() };
        if value.size == 0 {
            return Ok(&[]);
        }
        // SAFETY: the view points into vocabulary storage owned by this model.
        Ok(unsafe { n::slice(value.data.cast::<u8>(), value.size) })
    }

    /// Returns the required streaming-state storage size.
    pub(crate) fn state_size(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_model_state_size(self.raw.as_ptr()) }
    }
}
