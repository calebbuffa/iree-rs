//! Vocabulary identifiers, attributes, and owned vocabulary storage.
use crate::{Result, SpecialTokenKind, check, const_bytes_view, n, view};
use std::{
    marker::PhantomData,
    ptr::{self, NonNull},
};

/// A vocabulary token identifier.
pub type TokenId = i32;

/// Vocabulary token attribute flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TokenAttributes(u16);
impl TokenAttributes {
    /// No attributes.
    pub const NONE: Self = Self(0);
    /// Unknown-token marker.
    pub const UNKNOWN: Self = Self(1);
    /// Control token marker.
    pub const CONTROL: Self = Self(2);
    /// Byte token marker.
    pub const BYTE: Self = Self(4);
    /// Special token marker.
    pub const SPECIAL: Self = Self(8);
    /// Unused sparse-vocabulary slot.
    pub const UNUSED: Self = Self(16);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u16) -> Self {
        Self(bits)
    }
    /// Returns the underlying flag bits.
    pub const fn bits(self) -> u16 {
        self.0
    }
    /// Tests whether all flags in `other` are present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
/// A borrowed, immutable view of a tokenizer vocabulary.
#[derive(Clone, Copy)]
pub struct VocabularyRef<'a> {
    raw: *const n::iree_tokenizer_vocab_t,
    _borrow: PhantomData<&'a n::iree_tokenizer_vocab_t>,
}

/// Borrows token text from vocabulary storage; empty for out-of-range or unused IDs.
///
/// # Safety
/// `raw` must point to a vocabulary that outlives `'a`.
unsafe fn token_bytes<'a>(raw: *const n::iree_tokenizer_vocab_t, id: TokenId) -> &'a [u8] {
    // SAFETY: the caller guarantees a live vocabulary; upstream bounds-checks `id`.
    let text = unsafe { n::iree_tokenizer_vocab_token_text(raw, id) };
    if text.size == 0 {
        return &[];
    }
    // SAFETY: nonempty views point into immutable vocabulary storage living for `'a`.
    unsafe { n::slice(text.data.cast::<u8>(), text.size) }
}

impl<'a> VocabularyRef<'a> {
    /// # Safety
    /// `raw` must be a live vocabulary that outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: *const n::iree_tokenizer_vocab_t) -> Self {
        Self {
            raw,
            _borrow: PhantomData,
        }
    }

    pub(crate) fn raw(&self) -> *const n::iree_tokenizer_vocab_t {
        self.raw
    }

    /// Returns the immutable vocabulary string table.
    pub fn string_table(&self) -> &'a [u8] {
        // SAFETY: The returned span is backed by storage owned by the borrowed native value.
        let span = unsafe { n::iree_tokenizer_vocab_string_table(self.raw) };
        if span.data_length == 0 {
            return &[];
        }
        // SAFETY: A nonempty span points to vocabulary storage valid for this borrow.
        unsafe { n::slice(span.data, span.data_length) }
    }

    /// Looks up a token by its exact byte string.
    pub fn lookup(&self, text: &[u8]) -> Option<TokenId> {
        // SAFETY: the byte view and borrowed vocabulary are valid.
        let id = unsafe { n::iree_tokenizer_vocab_lookup(self.raw, const_bytes_view(text)) };
        (id >= 0).then_some(id)
    }

    /// Returns token text for an identifier, or `None` when it is out of range or unused.
    pub fn token_text(&self, id: TokenId) -> Option<&'a [u8]> {
        let in_range = usize::try_from(id).is_ok_and(|index| index < self.capacity());
        (in_range && !self.token_attributes(id).contains(TokenAttributes::UNUSED))
            // SAFETY: the vocabulary outlives `'a`.
            .then(|| unsafe { token_bytes(self.raw, id) })
    }

    /// Iterates active vocabulary entries without copying token text.
    pub fn tokens(&self) -> VocabularyTokens<'a> {
        VocabularyTokens {
            vocab: *self,
            next: 0,
            end: self.capacity(),
        }
    }

    /// Returns active token count.
    pub fn token_count(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_vocab_token_count(self.raw) }
    }

    /// Returns vocabulary capacity, including sparse unused entries.
    pub fn capacity(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_vocab_capacity(self.raw) }
    }

    /// Returns the maximum token length in bytes.
    pub fn max_token_length(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_vocab_max_token_length(self.raw) }
    }

    /// Returns number of BPE merge rules.
    pub fn merge_count(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_vocab_merge_count(self.raw) }
    }

    /// Returns the score for a token, or zero when out of range or unscored.
    pub fn token_score(&self, id: TokenId) -> f32 {
        // SAFETY: upstream bounds-checks `id` against the live vocabulary.
        unsafe { n::iree_tokenizer_vocab_token_score(self.raw, id) }
    }

    /// Returns token attributes, or none when out of range.
    pub fn token_attributes(&self, id: TokenId) -> TokenAttributes {
        // SAFETY: upstream bounds-checks `id` against the live vocabulary.
        TokenAttributes(unsafe { n::iree_tokenizer_vocab_token_attrs(self.raw, id) })
    }

    /// Returns the configured special token IDs.
    pub fn special_ids(&self) -> SpecialTokenIds {
        // SAFETY: All pointer arguments refer to live storage for this call.
        let ids = unsafe { n::iree_tokenizer_vocab_special_ids(self.raw) };
        let id = |value: TokenId| (value >= 0).then_some(value);
        SpecialTokenIds {
            bos: id(ids.bos),
            eos: id(ids.eos),
            unk: id(ids.unk),
            pad: id(ids.pad),
            sep: id(ids.sep),
            cls: id(ids.cls),
            mask: id(ids.mask),
        }
    }

    /// Returns a BPE merge by rank, if present.
    pub fn merge(&self, rank: usize) -> Option<BpeMerge> {
        if rank >= self.merge_count() {
            return None;
        }
        // SAFETY: `rank` is in range for the live vocabulary.
        let merge = unsafe { n::iree_tokenizer_vocab_merge(self.raw, rank) };
        Some(BpeMerge {
            left: merge.left_id,
            right: merge.right_id,
        })
    }
}
/// A vocabulary entry borrowed from native vocabulary storage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VocabToken<'a> {
    /// Token identifier.
    pub id: TokenId,
    /// Token spelling bytes, valid for the vocabulary borrow.
    pub text: &'a [u8],
    /// Token score (BPE priority, zero when unscored).
    pub score: f32,
    /// Native token attributes.
    pub attributes: TokenAttributes,
}

/// Iterator over active vocabulary entries.
pub struct VocabularyTokens<'a> {
    vocab: VocabularyRef<'a>,
    next: usize,
    end: usize,
}
impl<'a> Iterator for VocabularyTokens<'a> {
    type Item = VocabToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.end {
            let id = TokenId::try_from(self.next).ok()?;
            self.next += 1;
            let attributes = self.vocab.token_attributes(id);
            if attributes.contains(TokenAttributes::UNUSED) {
                continue;
            }
            return Some(VocabToken {
                id,
                // SAFETY: the iterator holds the vocabulary borrow for `'a`.
                text: unsafe { token_bytes(self.vocab.raw, id) },
                score: self.vocab.token_score(id),
                attributes,
            });
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.end - self.next))
    }
}
impl std::iter::FusedIterator for VocabularyTokens<'_> {}

/// An owned vocabulary with explicit RAII destruction.
pub struct Vocabulary(pub(crate) NonNull<n::iree_tokenizer_vocab_t>);
impl Drop for Vocabulary {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native vocabulary and drops it once.
        unsafe { n::iree_tokenizer_vocab_free(self.0.as_ptr()) }
    }
}
impl Vocabulary {
    /// Returns an immutable borrowed vocabulary view.
    pub fn view(&self) -> VocabularyRef<'_> {
        // SAFETY: this owned vocabulary outlives the returned borrow.
        unsafe { VocabularyRef::from_raw(self.0.as_ptr()) }
    }
}

/// Builder for programmatically creating a vocabulary.
pub struct VocabularyBuilder(NonNull<n::iree_tokenizer_vocab_builder_t>);
impl Drop for VocabularyBuilder {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns and drops the native value once.
        unsafe { n::iree_tokenizer_vocab_builder_free(self.0.as_ptr()) }
    }
}
impl VocabularyBuilder {
    /// Allocates a builder with an optional capacity hint.
    pub fn new(capacity_hint: usize) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_vocab_builder_allocate(
                capacity_hint,
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self(crate::status::nonnull(
            raw,
            "vocabulary builder allocation succeeded with a null output",
        )?))
    }
    /// Adds a token with the next sequential ID.
    pub fn add_token(&mut self, text: &str, score: f32, attrs: TokenAttributes) -> Result<()> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_vocab_builder_add_token(
                self.0.as_ptr(),
                view(text),
                score,
                attrs.bits(),
            )
        })
    }
    /// Adds a token at an explicit ID.
    pub fn add_token_with_id(
        &mut self,
        id: TokenId,
        text: &str,
        score: f32,
        attrs: TokenAttributes,
    ) -> Result<()> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_vocab_builder_add_token_with_id(
                self.0.as_ptr(),
                id,
                view(text),
                score,
                attrs.bits(),
            )
        })
    }
    /// Adds attributes to an existing token ID.
    pub fn add_token_attributes(&mut self, id: TokenId, attrs: TokenAttributes) -> Result<()> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_vocab_builder_add_token_attrs(self.0.as_ptr(), id, attrs.bits())
        })
    }

    /// Ensures a token exists at the requested ID, inserting it if absent.
    pub fn ensure_token(
        &mut self,
        id: TokenId,
        text: &str,
        score: f32,
        attrs: TokenAttributes,
    ) -> Result<()> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_vocab_builder_ensure_token(
                self.0.as_ptr(),
                id,
                view(text),
                score,
                attrs.bits(),
            )
        })
    }

    /// Adds a BPE merge in priority order.
    pub fn add_merge(&mut self, left: u32, right: u32) -> Result<()> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe { n::iree_tokenizer_vocab_builder_add_merge(self.0.as_ptr(), left, right) })
    }
    /// Sets the vocabulary's ID for a special-token category.
    pub fn set_special_token(&mut self, kind: SpecialTokenKind, id: TokenId) -> Result<()> {
        // SAFETY: The live builder pointer is valid; this stores only a special-token ID.
        check(unsafe {
            n::iree_tokenizer_vocab_builder_set_special_token(self.0.as_ptr(), kind as i32, id)
        })
    }
    /// Finalizes the vocabulary and consumes this builder.
    pub fn build(self) -> Result<Vocabulary> {
        let mut raw = ptr::null_mut();
        let this = std::mem::ManuallyDrop::new(self);
        // SAFETY: Build consumes the builder on success or failure; output slot is valid.
        check(unsafe { n::iree_tokenizer_vocab_builder_build(this.0.as_ptr(), &mut raw) })?;
        Ok(Vocabulary(crate::status::nonnull(
            raw,
            "vocabulary build succeeded with a null output",
        )?))
    }
}

/// Well-known special-token identifiers carried by a vocabulary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpecialTokenIds {
    /// Beginning-of-sequence token ID, if configured.
    pub bos: Option<TokenId>,
    /// End-of-sequence token ID, if configured.
    pub eos: Option<TokenId>,
    /// Unknown token ID, if configured.
    pub unk: Option<TokenId>,
    /// Padding token ID, if configured.
    pub pad: Option<TokenId>,
    /// Separator token ID, if configured.
    pub sep: Option<TokenId>,
    /// Classification token ID, if configured.
    pub cls: Option<TokenId>,
    /// Mask token ID, if configured.
    pub mask: Option<TokenId>,
}

/// A pair of token IDs forming a BPE merge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BpeMerge {
    /// Left-hand token ID.
    pub left: u32,
    /// Right-hand token ID.
    pub right: u32,
}
