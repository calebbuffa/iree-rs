//! Text normalizers and normalization flags.
use crate::{Code, Result, check, n, view};

/// Flags used to request normalizer segment-boundary behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct NormalizerFlags(u32);
impl NormalizerFlags {
    /// No special behavior.
    pub const NONE: Self = Self(0);
    /// Flushes lookahead at the current logical boundary.
    pub const SEGMENT_END: Self = Self(1);
    /// Indicates an input special token consumed the first position.
    pub const FIRST_CONSUMED: Self = Self(2);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns the underlying flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}
/// Configuration flags for the BERT normalizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BertNormalizerFlags(u32);
impl BertNormalizerFlags {
    /// No BERT normalization operations.
    pub const NONE: Self = Self(0);
    /// Remove control characters and normalize whitespace.
    pub const CLEAN_TEXT: Self = Self(1);
    /// Add spaces around CJK ideographs.
    pub const HANDLE_CHINESE_CHARS: Self = Self(2);
    /// Decompose and remove nonspacing accents.
    pub const STRIP_ACCENTS: Self = Self(4);
    /// Apply Unicode lowercase folding.
    pub const LOWERCASE: Self = Self(8);
    /// Default HuggingFace BERT behavior.
    pub const DEFAULT: Self = Self(15);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}
use std::ptr::{self, NonNull};

/// An owned normalizer component.
pub struct Normalizer(pub(crate) NonNull<n::iree_tokenizer_normalizer_t>);
impl Drop for Normalizer {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native normalizer and drops it once.
        unsafe { n::iree_tokenizer_normalizer_free(self.0.as_ptr()) }
    }
}
impl Normalizer {
    fn allocate_with(
        allocate: impl FnOnce(
            n::iree_allocator_t,
            *mut *mut n::iree_tokenizer_normalizer_t,
        ) -> n::iree_status_t,
    ) -> Result<Self> {
        let mut raw = ptr::null_mut();
        check(allocate(crate::status::system(), &mut raw))?;
        Ok(Self(crate::status::nonnull(
            raw,
            "normalizer allocation succeeded with a null output",
        )?))
    }

    /// Parses a HuggingFace normalizer JSON value.
    pub fn from_huggingface_json(json: &str) -> Result<Option<Self>> {
        let mut raw = ptr::null_mut();
        // SAFETY: Input/configuration storage and every output slot outlive the native call.
        check(unsafe {
            n::iree_tokenizer_huggingface_parse_normalizer(
                view(json),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(std::ptr::NonNull::new(raw).map(Self))
    }

    /// Creates a passthrough normalizer.
    pub fn passthrough() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_passthrough_allocate(allocator, out)
        })
    }

    /// Creates a deferred normalizer for incremental configuration.
    pub fn deferred() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_deferred_allocate(allocator, out)
        })
    }

    /// Creates a lowercase normalizer.
    pub fn lowercase() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_lowercase_allocate(allocator, out)
        })
    }

    /// Creates an NFC normalizer.
    pub fn nfc() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_nfc_allocate(allocator, out)
        })
    }

    /// Creates an NFD normalizer.
    pub fn nfd() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_nfd_allocate(allocator, out)
        })
    }

    /// Creates an NFKD normalizer.
    pub fn nfkd() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_nfkd_allocate(allocator, out)
        })
    }

    /// Creates a normalizer that strips combining accents.
    pub fn strip_accents() -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_strip_accents_allocate(allocator, out)
        })
    }

    /// Creates a BERT normalizer with the selected operations.
    pub fn bert(flags: BertNormalizerFlags) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_bert_allocate(flags.bits(), allocator, out)
        })
    }

    /// Creates a whitespace-strip normalizer.
    pub fn strip(strip_left: bool, strip_right: bool) -> Result<Self> {
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_strip_allocate(strip_left, strip_right, allocator, out)
        })
    }

    /// Creates a normalizer that prepends `prefix` to non-empty input.
    pub fn prepend(prefix: &str, skip_if_prefix_matches: bool) -> Result<Self> {
        let prefix = view(prefix);
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_prepend_allocate(
                prefix,
                skip_if_prefix_matches,
                allocator,
                out,
            )
        })
    }

    /// Creates a literal replacement normalizer.
    pub fn replace(pattern: &str, content: &str) -> Result<Self> {
        let pattern = view(pattern);
        let content = view(content);
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_replace_allocate(pattern, content, allocator, out)
        })
    }

    /// Creates a regex-based replacement normalizer.
    pub fn regex_replace(pattern: &str, content: &str) -> Result<Self> {
        let pattern = view(pattern);
        let content = view(content);
        // SAFETY: The referenced DFA and buffers remain live for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_normalizer_regex_replace_allocate(pattern, content, allocator, out)
        })
    }

    /// Creates a normalizer from a compiled HuggingFace character map.
    pub fn precompiled(charsmap: &[u8]) -> Result<Self> {
        let span = n::iree_const_byte_span_t {
            data: charsmap.as_ptr(),
            data_length: charsmap.len(),
        };
        // SAFETY: All pointer arguments refer to live storage for this call.
        Self::allocate_with(|allocator, out| unsafe {
            n::iree_tokenizer_precompiled_normalizer_allocate(span, allocator, out)
        })
    }

    /// Chains at least two normalizers in order, flattening nested sequences.
    ///
    /// The sequence owns its children on success; on error they are dropped.
    pub fn sequence(children: Vec<Self>) -> Result<Self> {
        // SAFETY: the system allocator is valid for the resulting sequence's lifetime.
        unsafe { Self::sequence_with_allocator(children, crate::status::system()) }
    }

    /// # Safety
    /// `allocator` must be valid for the lifetime of the resulting sequence and
    /// report allocation failure with `ResourceExhausted`, as the system allocator does.
    unsafe fn sequence_with_allocator(
        children: Vec<Self>,
        allocator: n::iree_allocator_t,
    ) -> Result<Self> {
        let raw_children = children
            .iter()
            .map(|child| child.0.as_ptr())
            .collect::<Vec<_>>();
        let mut raw = ptr::null_mut();
        // SAFETY: children are live and uniquely owned; the caller guarantees the allocator.
        let result = check(unsafe {
            n::iree_tokenizer_normalizer_sequence_allocate(
                raw_children.as_ptr(),
                raw_children.len(),
                allocator,
                &mut raw,
            )
        });
        // Upstream validates before taking ownership, but frees the flattened
        // children itself on allocation failure, contrary to its header contract.
        // Forget only the handles, not the Vec's backing allocation.
        if result.is_ok()
            || result
                .as_ref()
                .is_err_and(|error| error.code() == Code::ResourceExhausted)
        {
            for child in children {
                std::mem::forget(child);
            }
        }
        result?;
        Ok(Self(crate::status::nonnull(
            raw,
            "normalizer sequence allocation succeeded with a null output",
        )?))
    }

    /// Returns the required streaming-state storage size.
    pub(crate) fn state_size(&self) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_normalizer_state_size(self.0.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Progress, extension::NormalizerExtension};
    use std::{ffi::c_void, sync::Arc};

    struct Tracked {
        _owner: Arc<()>,
    }

    impl NormalizerExtension for Tracked {
        type State = ();

        fn process(
            &self,
            _: &mut (),
            _: &str,
            _: &mut [u8],
            _: NormalizerFlags,
        ) -> Result<Progress> {
            Ok(Progress::default())
        }
    }

    unsafe extern "C" fn fail_allocation(
        _: *mut c_void,
        _: n::iree_allocator_command_t,
        _: *const c_void,
        _: *mut *mut c_void,
    ) -> n::iree_status_t {
        n::callback_status(|| {
            Err(crate::Error::new(
                Code::ResourceExhausted,
                "injected sequence allocation failure",
            ))
        })
    }

    #[test]
    fn sequence_allocation_failure_releases_flattened_children_once() -> Result<()> {
        let owner = Arc::new(());
        let child = || {
            Normalizer::from_extension(Tracked {
                _owner: owner.clone(),
            })
        };
        let nested = Normalizer::sequence(vec![child(), child()])?;
        let allocator = n::iree_allocator_t {
            self_: ptr::null_mut(),
            ctl: Some(fail_allocation),
        };
        // SAFETY: this stateless allocator always reports ResourceExhausted,
        // so no sequence allocation can outlive the call.
        let result =
            unsafe { Normalizer::sequence_with_allocator(vec![nested, child()], allocator) };
        assert!(matches!(result, Err(error) if error.code() == Code::ResourceExhausted));
        assert_eq!(Arc::strong_count(&owner), 1);
        Ok(())
    }
}
