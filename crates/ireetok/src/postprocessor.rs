//! Sequence templates and postprocessor behavior.
use crate::{Code, Error, PostprocessorState, Result, TokenId, check, n, view};
use std::{mem::MaybeUninit, ptr};

/// A parsed value-type postprocessor configuration.
pub struct Postprocessor(pub(crate) n::iree_tokenizer_postprocessor_t);
impl Drop for Postprocessor {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native value and its backing storage.
        unsafe { n::iree_tokenizer_postprocessor_deinitialize(&mut self.0) }
    }
}

impl Postprocessor {
    /// Parses a HuggingFace postprocessor JSON value.
    pub fn from_huggingface_json(json: &str) -> Result<Self> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: The successful native call initialized this output value.
        check(unsafe {
            n::iree_tokenizer_huggingface_parse_postprocessor(view(json), raw.as_mut_ptr())
        })?;
        // SAFETY: The successful native call initialized this output value.
        Ok(Self(unsafe { raw.assume_init() }))
    }
}

/// Data for one phase of a postprocessor template.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PostprocessorTemplate {
    /// Special tokens emitted before the first sequence.
    pub prefix: Vec<(TokenId, u8)>,
    /// Special tokens emitted between paired sequences.
    pub infix: Vec<(TokenId, u8)>,
    /// Special tokens emitted after the sequence(s).
    pub suffix: Vec<(TokenId, u8)>,
    /// Type ID assigned to first-sequence tokens.
    pub sequence_a_type_id: u8,
    /// Type ID assigned to second-sequence tokens.
    pub sequence_b_type_id: u8,
}

/// Postprocessor behavior flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PostprocessorFlags(u32);
impl PostprocessorFlags {
    /// No offset-trimming or prefix-space behavior.
    pub const NONE: Self = Self(0);
    /// Trim leading/trailing whitespace from token offsets.
    pub const TRIM_OFFSETS: Self = Self(1);
    /// Preserve synthetic leading space in the first token's offsets.
    pub const ADD_PREFIX_SPACE: Self = Self(2);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the underlying flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl Postprocessor {
    /// Creates a postprocessor from Rust-owned single and optional pair templates.
    pub fn new(
        single: &PostprocessorTemplate,
        pair: Option<&PostprocessorTemplate>,
        flags: PostprocessorFlags,
    ) -> Result<Self> {
        fn convert(
            source: &PostprocessorTemplate,
        ) -> Result<n::iree_tokenizer_postprocessor_template_t> {
            let total = source
                .prefix
                .len()
                .checked_add(source.infix.len())
                .and_then(|n| n.checked_add(source.suffix.len()))
                .ok_or_else(|| {
                    Error::new(Code::OutOfRange, "postprocessor piece count overflow")
                })?;
            if total > n::IREE_TOKENIZER_POSTPROCESSOR_MAX_PIECES as usize {
                return Err(Error::new(
                    Code::OutOfRange,
                    "postprocessor template contains too many special tokens",
                ));
            }
            // Each count is bounded by the checked total above.
            let mut target = n::iree_tokenizer_postprocessor_template_t {
                prefix_count: source.prefix.len() as u8,
                infix_count: source.infix.len() as u8,
                suffix_count: source.suffix.len() as u8,
                sequence_a_type_id: source.sequence_a_type_id,
                sequence_b_type_id: source.sequence_b_type_id,
                ..Default::default()
            };
            for (index, (id, type_id)) in source
                .prefix
                .iter()
                .chain(&source.infix)
                .chain(&source.suffix)
                .enumerate()
            {
                target.token_ids[index] = *id;
                target.type_ids[index] = *type_id;
            }
            Ok(target)
        }
        let single = convert(single)?;
        let pair = pair.map(convert).transpose()?;
        let mut raw = MaybeUninit::uninit();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_postprocessor_initialize(
                &single,
                pair.as_ref().map_or(ptr::null(), |value| value),
                flags.bits(),
                raw.as_mut_ptr(),
            )
        })?;
        // SAFETY: The successful native call initialized this output value.
        Ok(Self(unsafe { raw.assume_init() }))
    }

    /// Returns postprocessor behavior flags.
    pub fn flags(&self) -> PostprocessorFlags {
        PostprocessorFlags::from_bits_retain(self.0.flags)
    }

    /// Returns the single-sequence template.
    pub fn single(&self) -> PostprocessorTemplate {
        template_from_native(&self.0.single)
    }

    /// Returns the pair template, if one is configured.
    pub fn pair(&self) -> Option<PostprocessorTemplate> {
        self.0.has_pair.then(|| template_from_native(&self.0.pair))
    }

    /// Creates mutable encode state using the single or pair template.
    pub fn state(&self, pair: bool) -> Result<PostprocessorState<'_>> {
        if pair && !self.0.has_pair {
            return Err(Error::new(
                Code::InvalidArgument,
                "pair template is not configured",
            ));
        }
        let active = if pair { &self.0.pair } else { &self.0.single };
        let mut raw = MaybeUninit::uninit();
        // SAFETY: State storage is aligned/sized and the borrowed component outlives it.
        unsafe {
            n::iree_tokenizer_postprocessor_encode_state_initialize(
                &self.0,
                active,
                raw.as_mut_ptr(),
            )
        };
        Ok(PostprocessorState {
            _parent: self,
            // SAFETY: The successful native call initialized this output value.
            raw: unsafe { raw.assume_init() },
        })
    }

    /// Whether the postprocessor has a pair template.
    pub fn supports_pair(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_supports_pair(&self.0) }
    }
}

fn template_from_native(
    value: &n::iree_tokenizer_postprocessor_template_t,
) -> PostprocessorTemplate {
    let prefix_end = value.prefix_count as usize;
    let infix_end = prefix_end + value.infix_count as usize;
    let total_end = infix_end + value.suffix_count as usize;
    let parts = (0..total_end)
        .map(|i| (value.token_ids[i], value.type_ids[i]))
        .collect::<Vec<_>>();
    PostprocessorTemplate {
        prefix: parts[..prefix_end].to_vec(),
        infix: parts[prefix_end..infix_end].to_vec(),
        suffix: parts[infix_end..total_end].to_vec(),
        sequence_a_type_id: value.sequence_a_type_id,
        sequence_b_type_id: value.sequence_b_type_id,
    }
}
