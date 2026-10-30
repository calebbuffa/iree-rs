//! RAII wrappers for the public vocabulary index APIs.
use crate::{Code, Error, Result, TokenId, VocabularyRef, check, const_bytes_view, n};
use std::ptr::NonNull;

/// An owned hash index borrowing its vocabulary's token and string storage.
pub struct VocabularyHash<'a> {
    raw: NonNull<n::iree_tokenizer_vocab_hash_t>,
    _vocabulary: VocabularyRef<'a>,
    backing: HashBacking,
}
enum HashBacking {
    Allocated,
    CallerStorage { _slab: Vec<usize> },
}
impl Drop for VocabularyHash<'_> {
    fn drop(&mut self) {
        match &mut self.backing {
            HashBacking::Allocated => {
                // SAFETY: The native hash was allocated by the matching build function.
                unsafe { n::iree_tokenizer_vocab_hash_free(self.raw.as_ptr()) }
            }
            HashBacking::CallerStorage { .. } => {
                // SAFETY: The hash occupies the retained aligned slab and is deinitialized once.
                unsafe { n::iree_tokenizer_vocab_hash_deinitialize(self.raw.as_ptr()) }
            }
        }
    }
}
impl<'a> VocabularyHash<'a> {
    /// Builds an immutable hash index using IREE's system allocator.
    pub fn new(vocabulary: VocabularyRef<'a>, load_percent: u8) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: Vocabulary token/string data outlives this index; output is local and valid.
        check(unsafe {
            n::iree_tokenizer_vocab_hash_build(
                n::iree_tokenizer_vocab_tokens(vocabulary.raw()),
                vocabulary.capacity(),
                n::iree_tokenizer_vocab_string_table(vocabulary.raw()),
                load_percent,
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw: crate::status::nonnull(raw, "vocabulary hash build returned null")?,
            _vocabulary: vocabulary,
            backing: HashBacking::Allocated,
        })
    }

    /// Returns required caller-storage bytes for a vocabulary index.
    pub fn storage_size(vocabulary: &VocabularyRef<'_>, load_percent: u8) -> Result<usize> {
        let mut size = 0;
        // SAFETY: The output slot is valid for the duration of this call.
        check(unsafe {
            n::iree_tokenizer_vocab_hash_storage_size(
                vocabulary.capacity(),
                load_percent,
                &mut size,
            )
        })?;
        Ok(size)
    }

    /// Initializes a hash index in an aligned caller-owned word slab.
    pub fn with_storage(
        vocabulary: VocabularyRef<'a>,
        load_percent: u8,
        mut storage: Vec<usize>,
    ) -> Result<Self> {
        let required = Self::storage_size(&vocabulary, load_percent)?;
        let available = storage
            .len()
            .checked_mul(std::mem::size_of::<usize>())
            .ok_or_else(|| Error::new(Code::OutOfRange, "hash storage size overflow"))?;
        if available < required {
            return Err(Error::new(
                Code::InvalidArgument,
                "caller storage is too small for the vocabulary hash",
            ));
        }
        let raw = storage
            .as_mut_ptr()
            .cast::<n::iree_tokenizer_vocab_hash_t>();
        // SAFETY: Vec<usize> provides aligned, writable storage of the validated size;
        // the vocabulary's token/string data outlives the index.
        unsafe {
            n::iree_tokenizer_vocab_hash_initialize(
                n::iree_tokenizer_vocab_tokens(vocabulary.raw()),
                vocabulary.capacity(),
                n::iree_tokenizer_vocab_string_table(vocabulary.raw()),
                load_percent,
                raw,
            )
        };
        Ok(Self {
            raw: crate::status::nonnull(raw, "vocabulary hash storage is empty")?,
            _vocabulary: vocabulary,
            backing: HashBacking::CallerStorage { _slab: storage },
        })
    }

    /// Looks up a token by its exact byte spelling.
    pub fn lookup(&self, text: &[u8]) -> Option<TokenId> {
        // SAFETY: The view and index's retained vocabulary storage are valid.
        let id = unsafe {
            n::iree_tokenizer_vocab_hash_lookup(self.raw.as_ptr(), const_bytes_view(text))
        };
        (id >= 0).then_some(id)
    }
}

/// An owned prefix trie borrowing its vocabulary's token and string storage.
pub struct VocabularyTrie<'a> {
    raw: NonNull<n::iree_tokenizer_vocab_trie_t>,
    _vocabulary: VocabularyRef<'a>,
}
impl Drop for VocabularyTrie<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native trie and drops it once.
        unsafe { n::iree_tokenizer_vocab_trie_free(self.raw.as_ptr()) }
    }
}
impl<'a> VocabularyTrie<'a> {
    /// Builds an immutable prefix trie using IREE's system allocator.
    pub fn new(vocabulary: VocabularyRef<'a>) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: Vocabulary token/string data outlives this trie; output is local and valid.
        check(unsafe {
            n::iree_tokenizer_vocab_trie_build(
                n::iree_tokenizer_vocab_tokens(vocabulary.raw()),
                vocabulary.capacity(),
                n::iree_tokenizer_vocab_string_table(vocabulary.raw()),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw: crate::status::nonnull(raw, "vocabulary trie build returned null")?,
            _vocabulary: vocabulary,
        })
    }

    /// Returns the maximum token length represented in the trie.
    pub fn max_depth(&self) -> usize {
        // SAFETY: The trie remains live for this call.
        unsafe { n::iree_tokenizer_vocab_trie_max_depth(self.raw.as_ptr()) }
    }

    /// Returns the number of trie states.
    pub fn node_count(&self) -> usize {
        // SAFETY: The trie remains live for this call.
        unsafe { n::iree_tokenizer_vocab_trie_node_count(self.raw.as_ptr()) }
    }

    /// Creates a cursor borrowing this trie.
    pub fn cursor(&self) -> TrieCursor<'_, 'a> {
        let mut raw = std::mem::MaybeUninit::uninit();
        // SAFETY: Reset initializes the cursor and the trie outlives the returned borrow.
        unsafe { n::iree_tokenizer_trie_cursor_reset(raw.as_mut_ptr(), self.raw.as_ptr()) };
        TrieCursor {
            // SAFETY: trie_cursor_reset initialized raw immediately above.
            raw: unsafe { raw.assume_init() },
            trie: self,
        }
    }
}

/// A byte-at-a-time cursor borrowing a vocabulary trie.
pub struct TrieCursor<'a, 'v> {
    raw: n::iree_tokenizer_trie_cursor_t,
    trie: &'a VocabularyTrie<'v>,
}
impl TrieCursor<'_, '_> {
    /// Resets traversal to the trie root.
    pub fn reset(&mut self) {
        // SAFETY: The cursor retains a borrow of the trie it references.
        unsafe { n::iree_tokenizer_trie_cursor_reset(&mut self.raw, self.trie.raw.as_ptr()) }
    }

    /// Advances by one byte, returning false when no further prefix can match.
    pub fn advance(&mut self, byte: u8) -> bool {
        // SAFETY: The cursor retains a borrow of the trie it references.
        unsafe { n::iree_tokenizer_trie_cursor_advance(&mut self.raw, byte) }
    }

    /// Returns the number of bytes consumed since the last reset.
    pub fn depth(&self) -> usize {
        // SAFETY: The cursor was initialized by the native reset function.
        unsafe { n::iree_tokenizer_trie_cursor_depth(&self.raw) }
    }

    /// Returns the token ending at the current position, if any.
    pub fn token_id(&self) -> Option<TokenId> {
        // SAFETY: The cursor retains a borrow of the trie it references.
        let id = unsafe { n::iree_tokenizer_trie_cursor_token_id(&self.raw) };
        (id >= 0).then_some(id)
    }
}

/// The rank and resulting token of a BPE merge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MergeResult {
    /// Merge priority, with zero applied first.
    pub rank: u32,
    /// Token produced by the merge.
    pub token_id: TokenId,
}

/// An owned immutable BPE merge lookup table.
pub struct MergeHash(NonNull<n::iree_tokenizer_vocab_merge_hash_t>);
impl Drop for MergeHash {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native merge hash and drops it once.
        unsafe { n::iree_tokenizer_vocab_merge_hash_free(self.0.as_ptr()) }
    }
}
impl MergeHash {
    /// Builds a merge lookup table from a vocabulary.
    pub fn new(vocabulary: &VocabularyRef<'_>) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: The native build copies merge lookup data from the live vocabulary.
        check(unsafe {
            n::iree_tokenizer_vocab_merge_hash_build(
                vocabulary.raw(),
                crate::status::system(),
                &mut raw,
            )
        })?;
        Ok(Self(crate::status::nonnull(
            raw,
            "vocabulary merge hash build returned null",
        )?))
    }

    /// Finds the highest-priority merge for an adjacent token pair.
    pub fn lookup(&self, left: TokenId, right: TokenId) -> Option<MergeResult> {
        // SAFETY: The merge table is immutable and remains live for this call.
        let value =
            unsafe { n::iree_tokenizer_vocab_merge_hash_lookup(self.0.as_ptr(), left, right) };
        // SAFETY: This helper only inspects the value returned above.
        unsafe { n::iree_tokenizer_merge_hash_result_is_valid(value) }.then_some(MergeResult {
            rank: value.rank,
            token_id: value.result_id,
        })
    }
}
