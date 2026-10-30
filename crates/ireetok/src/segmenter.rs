//! Pre-tokenizer segmenters and split behavior.
use crate::{RegexDfa, Result, check, n, view};

/// Properties exposed by a HuggingFace pre-tokenizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PreTokenizerFlags(u32);
impl PreTokenizerFlags {
    /// No flags.
    pub const NONE: Self = Self(0);
    /// Byte-level segmentation is active.
    pub const BYTE_LEVEL: Self = Self(1);
    /// Add a synthetic prefix space.
    pub const ADD_PREFIX_SPACE: Self = Self(2);
    /// Metaspace prepending is active.
    pub const METASPACE_PREPEND: Self = Self(4);
    /// Metaspace replacement is active.
    pub const METASPACE_REPLACE: Self = Self(8);
    /// Word-level splitting is active.
    pub const WORD_LEVEL_SPLIT: Self = Self(16);
    /// Offsets are trimmed by the postprocessor.
    pub const TRIM_OFFSETS: Self = Self(32);
    /// Whitespace splitting is present.
    pub const HAS_WHITESPACE_SPLIT: Self = Self(64);
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
/// Behavior used when a segmenter splits around a matched pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum SplitBehavior {
    /// Remove the matched pattern.
    Removed = 0,
    /// Emit the matched pattern as its own segment.
    Isolated = 1,
    /// Append the pattern to the preceding segment.
    MergedWithPrevious = 2,
    /// Prepend the pattern to the following segment.
    MergedWithNext = 3,
    /// Keep adjacent matches as one contiguous segment.
    Contiguous = 4,
}

impl SplitBehavior {
    /// Whether this split mode creates boundaries between adjacent BPE symbols.
    pub fn creates_bpe_boundaries(self) -> bool {
        // SAFETY: The referenced DFA and buffers remain live for this call.
        unsafe { n::iree_tokenizer_regex_split_creates_bpe_boundaries(self as i32) }
    }
}
use std::ptr::{self, NonNull};

/// An owned pre-tokenizer/segmenter component.
pub struct Segmenter(pub(crate) NonNull<n::iree_tokenizer_segmenter_t>);
impl Drop for Segmenter {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native segmenter and drops it once.
        unsafe { n::iree_tokenizer_segmenter_free(self.0.as_ptr()) }
    }
}
impl Segmenter {
    fn allocate_with(
        allocate: impl FnOnce(
            n::iree_allocator_t,
            *mut *mut n::iree_tokenizer_segmenter_t,
        ) -> n::iree_status_t,
    ) -> Result<Self> {
        let mut raw = ptr::null_mut();
        check(allocate(crate::status::system(), &mut raw))?;
        Ok(Self(crate::status::nonnull(
            raw,
            "segmenter allocation succeeded with a null output",
        )?))
    }

    /// Parses a HuggingFace pre-tokenizer JSON value.
    pub fn from_huggingface_json(json: &str) -> Result<(Option<Self>, PreTokenizerFlags)> {
        let mut raw = ptr::null_mut();
        let mut flags = 0;
        // SAFETY: Input/configuration storage and every output slot outlive the native call.
        check(unsafe {
            n::iree_tokenizer_huggingface_parse_segmenter(
                view(json),
                crate::status::system(),
                &mut raw,
                &mut flags,
            )
        })?;
        let segmenter = NonNull::new(raw).map(Self);
        Ok((segmenter, PreTokenizerFlags::from_bits_retain(flags)))
    }

    /// Creates a whitespace segmenter.
    pub fn whitespace() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_whitespace_allocate(allocator, out)
        })
    }

    /// Creates a segmenter that passes each input through unchanged.
    pub fn passthrough() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_passthrough_allocate(allocator, out)
        })
    }

    /// Creates a BERT punctuation segmenter.
    pub fn bert() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_bert_allocate(allocator, out)
        })
    }

    /// Creates a segmenter that separates digits.
    pub fn digits(individual_digits: bool) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_digits_allocate(individual_digits, allocator, out)
        })
    }

    /// Creates a Unicode punctuation segmenter.
    pub fn punctuation(behavior: SplitBehavior) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_punctuation_allocate(behavior as i32, allocator, out)
        })
    }

    /// Creates a metaspace segmenter.
    pub fn metaspace(replacement: char, split_enabled: bool) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_metaspace_allocate(
                replacement as u32,
                split_enabled,
                allocator,
                out,
            )
        })
    }

    /// Creates a regex segmenter and consumes its compiled DFA on success.
    pub fn split(dfa: RegexDfa, behavior: SplitBehavior, invert: bool) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: DFA storage stays live; success transfers its ownership to the segmenter.
        check(unsafe {
            n::iree_tokenizer_segmenter_split_allocate(
                dfa.raw,
                dfa.storage.as_ptr(),
                behavior as i32,
                invert,
                crate::status::system(),
                &mut raw,
            )
        })?;
        let segmenter = crate::status::nonnull(
            raw,
            "regex segmenter allocation succeeded with a null output",
        )?;
        std::mem::forget(dfa);
        Ok(Self(segmenter))
    }

    /// Creates a literal-pattern segmenter.
    pub fn split_literal(pattern: &str, behavior: SplitBehavior, invert: bool) -> Result<Self> {
        let pattern = view(pattern);
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_segmenter_split_literal_allocate(
                pattern,
                behavior as i32,
                invert,
                allocator,
                out,
            )
        })
    }

    /// Chains at least two segmenters in order.
    ///
    /// The sequence owns its children on success; on error they are dropped.
    pub fn sequence(children: Vec<Self>) -> Result<Self> {
        let raw_children = children
            .iter()
            .map(|child| child.0.as_ptr())
            .collect::<Vec<_>>();
        let mut raw = ptr::null_mut();
        // SAFETY: Child pointers stay live; success transfers child ownership to the sequence.
        check(unsafe {
            n::iree_tokenizer_segmenter_sequence_allocate(
                raw_children.as_ptr(),
                raw_children.len(),
                crate::status::system(),
                &mut raw,
            )
        })?;
        for child in children {
            std::mem::forget(child);
        }
        Ok(Self(crate::status::nonnull(
            raw,
            "segmenter sequence allocation succeeded with a null output",
        )?))
    }

    /// Returns the required streaming-state storage size.
    pub(crate) fn state_size(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_segmenter_state_size(self.0.as_ptr()) }
    }
}

/// A half-open byte range emitted by a segmenter.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Segment {
    /// Inclusive starting byte offset.
    pub start: usize,
    /// Exclusive ending byte offset.
    pub end: usize,
}
