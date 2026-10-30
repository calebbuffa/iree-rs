//! Literal special tokens and HuggingFace added tokens.
use crate::{Result, TokenId, check, const_bytes_view, copy_string_view, n, view};

/// Special vocabulary slot configured by a tokenizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SpecialTokenKind {
    /// Unknown token.
    Unknown = 0,
    /// Beginning-of-sequence token.
    Bos = 1,
    /// End-of-sequence token.
    Eos = 2,
    /// Padding token.
    Pad = 3,
    /// Separator token.
    Sep = 4,
    /// Classification token.
    Cls = 5,
    /// Mask token.
    Mask = 6,
}

/// Flags for literal special-token recognition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SpecialTokenFlags(u8);
impl SpecialTokenFlags {
    /// No match restrictions.
    pub const NONE: Self = Self(0);
    /// Require a preceding whitespace or input-start boundary.
    pub const LSTRIP: Self = Self(1);
    /// Strip whitespace following the matched token.
    pub const RSTRIP: Self = Self(2);
    /// Require a whole-word match.
    pub const SINGLE_WORD: Self = Self(4);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u8) -> Self {
        Self(bits)
    }
    /// Returns the underlying flag bits.
    pub const fn bits(self) -> u8 {
        self.0
    }
    /// Tests whether all flags in `other` are present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
use std::mem::MaybeUninit;

/// An owned special-token collection with literal matching semantics.
pub struct SpecialTokens(pub(crate) n::iree_tokenizer_special_tokens_t);
impl SpecialTokens {
    /// Creates an empty special-token collection.
    pub fn new() -> Self {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: The successful native call initialized this output value.
        unsafe { n::iree_tokenizer_special_tokens_initialize(raw.as_mut_ptr()) };
        // SAFETY: The successful native call initialized this output value.
        Self(unsafe { raw.assume_init() })
    }
    /// Whether this collection contains no entries.
    pub fn is_empty(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_special_tokens_is_empty(&self.0) }
    }
}
impl Default for SpecialTokens {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for SpecialTokens {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_special_tokens_deinitialize(&mut self.0) }
    }
}

/// Builder for a literal special-token collection.
pub struct SpecialTokensBuilder {
    raw: MaybeUninit<n::iree_tokenizer_special_tokens_builder_t>,
}
impl SpecialTokensBuilder {
    /// Creates an empty builder using the system allocator.
    pub fn new() -> Self {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe {
            n::iree_tokenizer_special_tokens_builder_initialize(
                crate::status::system(),
                raw.as_mut_ptr(),
            )
        };
        Self { raw }
    }
    /// Adds a spelling, token ID, and match flags.
    pub fn add(
        &mut self,
        content: &str,
        token_id: TokenId,
        flags: SpecialTokenFlags,
    ) -> Result<()> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_special_tokens_builder_add(
                self.raw.as_mut_ptr(),
                view(content),
                token_id,
                flags.bits(),
            )
        })
    }
    /// Builds an owned collection. The builder remains usable until dropped.
    pub fn build(&mut self) -> Result<SpecialTokens> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: The builder and output slot are valid; success transfers ownership.
        check(unsafe {
            n::iree_tokenizer_special_tokens_builder_build(
                self.raw.as_mut_ptr(),
                crate::status::system(),
                raw.as_mut_ptr(),
            )
        })?;
        // SAFETY: The successful native call initialized this output value.
        Ok(SpecialTokens(unsafe { raw.assume_init() }))
    }
}
impl Default for SpecialTokensBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for SpecialTokensBuilder {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_special_tokens_builder_deinitialize(self.raw.as_mut_ptr()) }
    }
}

/// HuggingFace added-token behavior flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct AddedTokenFlags(u32);
impl AddedTokenFlags {
    /// No additional behavior.
    pub const NONE: Self = Self(0);
    /// Require a whole-word match.
    pub const SINGLE_WORD: Self = Self(1);
    /// Require a leading word boundary.
    pub const LSTRIP: Self = Self(2);
    /// Strip following whitespace.
    pub const RSTRIP: Self = Self(4);
    /// Match after normalization.
    pub const NORMALIZED: Self = Self(8);
    /// Marks the entry as special.
    pub const SPECIAL: Self = Self(16);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the underlying flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
    /// Tests whether all flags in `other` are present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

/// Parsed HuggingFace added-token data with owned native storage.
pub struct AddedTokens(pub(crate) n::iree_tokenizer_huggingface_added_tokens_t);
impl Drop for AddedTokens {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns and drops the native value once.
        unsafe { n::iree_tokenizer_huggingface_added_tokens_free(&mut self.0) }
    }
}

/// One owned added-token entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddedToken {
    /// Vocabulary identifier.
    pub id: TokenId,
    /// Match and normalization behavior.
    pub flags: AddedTokenFlags,
    /// Literal token spelling.
    pub content: String,
}

impl AddedTokens {
    /// Parses an added-token JSON array/object.
    pub fn from_huggingface_json(json: &str) -> Result<Self> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: Input/configuration storage and every output slot outlive the native call.
        check(unsafe {
            n::iree_tokenizer_huggingface_parse_added_tokens_json(
                view(json),
                crate::status::system(),
                raw.as_mut_ptr(),
            )
        })?;
        // SAFETY: The successful native call initialized this output value.
        Ok(Self(unsafe { raw.assume_init() }))
    }

    /// Number of parsed entries.
    pub fn len(&self) -> usize {
        self.0.count
    }

    /// Whether no entries were parsed.
    pub fn is_empty(&self) -> bool {
        self.0.count == 0
    }

    /// Returns an owned entry at `index`, if present.
    pub fn get(&self, index: usize) -> Option<AddedToken> {
        if index >= self.0.count {
            return None;
        }
        // SAFETY: All pointer arguments refer to live storage for this call.
        let item = unsafe { n::iree_tokenizer_huggingface_added_tokens_get(&self.0, index) };
        if item.is_null() {
            return None;
        }
        // SAFETY: All pointer arguments refer to live storage for this call.
        let content = unsafe { n::iree_tokenizer_huggingface_added_token_content(&self.0, item) };
        let content = copy_string_view(content);
        Some(AddedToken {
            // SAFETY: The checked entry pointer remains valid while AddedTokens is borrowed.
            id: unsafe { (*item).id },
            // SAFETY: The checked entry pointer remains valid while AddedTokens is borrowed.
            flags: AddedTokenFlags::from_bits_retain(unsafe { (*item).flags }),
            content,
        })
    }
}

/// Outcome of an incremental special-token match attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialTokenMatch {
    /// No special-token spelling begins at the current position.
    NoMatch,
    /// A special token was recognized, with its consumed bytes, ID, and match flags.
    Matched {
        /// Bytes consumed from this input chunk.
        length: usize,
        /// Matched vocabulary token ID.
        id: TokenId,
        /// Special-token matching flags.
        flags: SpecialTokenFlags,
    },
    /// The input is a prefix and the matcher needs another chunk.
    NeedMore,
}

/// Mutable matcher for special tokens, with partial-match state across chunks.
pub struct SpecialTokenMatcher<'a> {
    parent: &'a SpecialTokens,
    raw: n::iree_tokenizer_special_tokens_encode_state_t,
}
impl SpecialTokens {
    /// Creates a streaming matcher borrowing this collection.
    pub fn matcher(&self) -> SpecialTokenMatcher<'_> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: The successful native call initialized this output value.
        unsafe { n::iree_tokenizer_special_tokens_encode_state_initialize(raw.as_mut_ptr()) };
        SpecialTokenMatcher {
            parent: self,
            // SAFETY: The successful native call initialized this output value.
            raw: unsafe { raw.assume_init() },
        }
    }

    /// Returns a safe input prefix that cannot begin a special-token spelling.
    pub fn safe_prefix_length(&self, input: &[u8]) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe {
            n::iree_tokenizer_special_tokens_safe_prefix_length(&self.0, const_bytes_view(input))
        }
    }
}
impl SpecialTokenMatcher<'_> {
    /// Attempts a match at the beginning of `input`, retaining partial state.
    pub fn match_prefix(&mut self, input: &[u8]) -> SpecialTokenMatch {
        let mut length = 0;
        let mut id = -1;
        // SAFETY: All pointer arguments refer to live storage for this call.
        match unsafe {
            n::iree_tokenizer_special_tokens_match(
                &self.parent.0,
                const_bytes_view(input),
                &mut length,
                &mut id,
                &mut self.raw,
            )
        } {
            n::IREE_TOKENIZER_SPECIAL_TOKENS_NO_MATCH => SpecialTokenMatch::NoMatch,
            n::IREE_TOKENIZER_SPECIAL_TOKENS_MATCHED => SpecialTokenMatch::Matched {
                length,
                id,
                flags: SpecialTokenFlags::from_bits_retain(self.raw.matched_flags),
            },
            _ => SpecialTokenMatch::NeedMore,
        }
    }

    /// Whether the current input ended with a partial token match.
    pub fn has_partial(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_special_tokens_encode_state_has_partial(&self.raw) }
    }

    /// Updates preceding-byte context after ordinary text is consumed.
    pub fn update_previous_byte(&mut self, byte: u8) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe {
            n::iree_tokenizer_special_tokens_encode_state_update_prev_byte(&mut self.raw, byte)
        }
    }

    /// Returns whether a byte is a token word-boundary byte.
    pub fn is_word_boundary_byte(byte: u8) -> bool {
        // SAFETY: This C helper uses only a scalar byte and does not dereference pointers.
        unsafe { n::iree_tokenizer_is_word_boundary_byte(byte) }
    }

    /// Reconstructs buffered partial bytes after a failed match.
    pub fn partial_bytes(&self) -> Vec<u8> {
        let capacity = self.raw.match_position as usize;
        let mut bytes = vec![0; capacity];
        // SAFETY: All pointer arguments refer to live storage for this call.
        let written = unsafe {
            n::iree_tokenizer_special_tokens_encode_state_get_partial(
                &self.raw,
                &self.parent.0,
                bytes.as_mut_ptr(),
            )
        };
        bytes.truncate(written);
        bytes
    }

    /// Clears any pending prefix match.
    pub fn clear_partial(&mut self) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_special_tokens_encode_state_clear_partial(&mut self.raw) }
    }
}
