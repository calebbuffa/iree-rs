//! Decoder components and their construction policies.
use crate::{Result, check, n, view};

/// Metaspace decoder policy for prepending its replacement marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MetaspacePrependScheme {
    /// Always prepend the marker.
    Always = 0,
    /// Never prepend the marker.
    Never = 1,
    /// Prepend only at the beginning of a stream.
    First = 2,
}

/// Native decoder behavior flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DecoderFlags(u32);
impl DecoderFlags {
    /// No special behavior.
    pub const NONE: Self = Self(0);
    /// Decoder output may shrink as additional token strings arrive.
    pub const SHRINKING: Self = Self(n::IREE_TOKENIZER_DECODER_FLAG_SHRINKING as u32);

    /// Constructs flags from raw bits, preserving unknown native flags.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns the underlying native flags.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Tests whether all flags in `other` are present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

/// Rust-owned configuration for the WordPiece decoder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WordPieceDecoderConfig {
    /// Prefix marking a non-initial subword, usually `##`.
    pub prefix: String,
    /// Whether to clean up punctuation spacing.
    pub cleanup: bool,
}

/// Decoder optimization properties.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DecoderCapabilities(u32);
impl DecoderCapabilities {
    /// No declared capability.
    pub const NONE: Self = Self(0);
    /// Each token can be decoded independently.
    pub const STATELESS: Self = Self(1);
    /// First-token output differs from subsequent tokens.
    pub const POSITION_SENSITIVE: Self = Self(2);
    /// Only byte-fallback tokens require cross-token state.
    pub const STATELESS_EXCEPT_BYTE_TOKENS: Self = Self(4);
    /// Some tokens may contain partial UTF-8 bytes.
    pub const STATELESS_EXCEPT_PARTIAL_UTF8: Self = Self(8);
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
use std::ptr::{self, NonNull};

/// An owned decoder component.
pub struct Decoder(pub(crate) NonNull<n::iree_tokenizer_decoder_t>);
impl Drop for Decoder {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native decoder and drops it once.
        unsafe { n::iree_tokenizer_decoder_free(self.0.as_ptr()) }
    }
}
impl Decoder {
    fn allocate_with(
        allocate: impl FnOnce(
            n::iree_allocator_t,
            *mut *mut n::iree_tokenizer_decoder_t,
        ) -> n::iree_status_t,
    ) -> Result<Self> {
        let mut raw = ptr::null_mut();
        check(allocate(crate::status::system(), &mut raw))?;
        Ok(Self(crate::status::nonnull(
            raw,
            "decoder allocation succeeded with a null output",
        )?))
    }

    /// Parses a HuggingFace decoder JSON value.
    pub fn from_huggingface_json(json: &str) -> Result<Option<Self>> {
        let mut raw = ptr::null_mut();
        // SAFETY: Input/configuration storage and every output slot outlive the native call.
        check(unsafe {
            n::iree_tokenizer_huggingface_parse_decoder(
                view(json),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(std::ptr::NonNull::new(raw).map(Self))
    }

    /// Creates a passthrough decoder.
    pub fn passthrough() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_passthrough_allocate(allocator, out)
        })
    }

    /// Creates a GPT-2 byte-level decoder.
    pub fn byte_level() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_byte_level_allocate(allocator, out)
        })
    }

    /// Creates a byte-fallback decoder.
    pub fn byte_fallback() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_byte_fallback_allocate(allocator, out)
        })
    }

    /// Creates a WordPiece decoder.
    pub fn wordpiece(config: &WordPieceDecoderConfig) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        let raw_config = unsafe {
            n::iree_tokenizer_make_decoder_wordpiece_config(view(&config.prefix), config.cleanup)
        };
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_wordpiece_allocate(raw_config, allocator, out)
        })
    }

    /// Creates a decoder that removes repeated prefix and suffix strings.
    pub fn strip(content: &str, start_count: usize, stop_count: usize) -> Result<Self> {
        let content = view(content);
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_strip_allocate(
                content,
                start_count,
                stop_count,
                allocator,
                out,
            )
        })
    }

    /// Creates a literal replacement decoder.
    pub fn replace(pattern: &str, content: &str) -> Result<Self> {
        let pattern = view(pattern);
        let content = view(content);
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_replace_allocate(pattern, content, allocator, out)
        })
    }

    /// Creates a metaspace decoder.
    pub fn metaspace(replacement: char, prepend_scheme: MetaspacePrependScheme) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_metaspace_allocate(
                replacement as u32,
                prepend_scheme as i32,
                allocator,
                out,
            )
        })
    }

    /// Creates a CTC decoder.
    pub fn ctc(pad_token: &str, word_delimiter_token: &str, cleanup: bool) -> Result<Self> {
        let pad_token = view(pad_token);
        let word_delimiter_token = view(word_delimiter_token);
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_decoder_ctc_allocate(
                pad_token,
                word_delimiter_token,
                cleanup,
                allocator,
                out,
            )
        })
    }

    /// Chains at least two shrinking decoders in order.
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
            n::iree_tokenizer_decoder_sequence_allocate(
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
            "decoder sequence allocation succeeded with a null output",
        )?))
    }

    /// Returns the required streaming-state storage size.
    pub(crate) fn state_size(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_decoder_state_size(self.0.as_ptr()) }
    }

    /// Parses the byte represented by a byte-fallback token spelling.
    pub fn parse_byte_fallback_token(token: &str) -> Option<u8> {
        let mut byte = 0;
        // SAFETY: Input/configuration storage and every output slot outlive the native call.
        let valid = unsafe {
            n::iree_tokenizer_decoder_byte_fallback_parse_byte_token(view(token), &mut byte)
        };
        valid.then_some(byte)
    }

    /// Returns decoder capability flags.
    pub fn capabilities(&self) -> DecoderCapabilities {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe {
            DecoderCapabilities::from_bits_retain(n::iree_tokenizer_decoder_capabilities(
                self.0.as_ptr(),
            ))
        }
    }
}
