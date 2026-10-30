//! Mutable streaming states and caller-provided token output buffers.
use crate::{Code, Error, Offset, Result, TokenId, n};
use std::mem::{align_of, offset_of, size_of};
use std::ptr;

/// One offset-mapping run used by streaming offset tracking.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct OffsetRun {
    /// Position in transformed text.
    transform_position: usize,
    /// Corresponding byte position in original input.
    original_offset: usize,
}

/// A caller-owned output view for streaming encoding.
pub struct TokenOutput<'a> {
    ids: &'a mut [TokenId],
    offsets: Option<&'a mut [Offset]>,
    type_ids: Option<&'a mut [u8]>,
}

const _: () = assert!(
    size_of::<Offset>() == size_of::<n::iree_tokenizer_offset_t>()
        && align_of::<Offset>() == align_of::<n::iree_tokenizer_offset_t>()
        && offset_of!(Offset, start) == offset_of!(n::iree_tokenizer_offset_t, start)
        && offset_of!(Offset, end) == offset_of!(n::iree_tokenizer_offset_t, end)
);

impl<'a> TokenOutput<'a> {
    /// Creates an output view. Auxiliary arrays, when provided, must match the ID capacity.
    pub fn new(
        ids: &'a mut [TokenId],
        offsets: Option<&'a mut [Offset]>,
        type_ids: Option<&'a mut [u8]>,
    ) -> Result<Self> {
        if offsets.as_ref().is_some_and(|x| x.len() != ids.len())
            || type_ids.as_ref().is_some_and(|x| x.len() != ids.len())
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "token output arrays must have equal capacities",
            ));
        }
        Ok(Self {
            ids,
            offsets,
            type_ids,
        })
    }

    /// Borrows native output buffers for one custom model callback.
    ///
    /// # Safety
    /// Every non-null pointer must be writable for `capacity` elements and
    /// remain live for `'a`. Non-null offset storage must use the native
    /// `iree_tokenizer_offset_t` layout.
    pub(crate) unsafe fn from_raw(
        output: n::iree_tokenizer_token_output_t,
    ) -> Result<TokenOutput<'a>> {
        let capacity = output.capacity;
        if capacity > 0 && output.token_ids.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "model token output storage is null",
            ));
        }
        let ids = if capacity == 0 {
            &mut []
        } else {
            // SAFETY: required by this method's caller contract.
            unsafe { n::slice_mut(output.token_ids, capacity) }
        };
        let offsets = if capacity == 0 || output.token_offsets.is_null() {
            None
        } else {
            // SAFETY: layout and capacity are required by the caller contract.
            Some(unsafe { n::slice_mut(output.token_offsets.cast::<Offset>(), capacity) })
        };
        let type_ids = if capacity == 0 || output.type_ids.is_null() {
            None
        } else {
            // SAFETY: capacity and pointer validity are required by the caller contract.
            Some(unsafe { n::slice_mut(output.type_ids, capacity) })
        };
        Self::new(ids, offsets, type_ids)
    }

    pub(crate) fn capacity(&self) -> usize {
        self.ids.len()
    }

    /// Returns the writable token identifier buffer.
    pub fn ids_mut(&mut self) -> &mut [TokenId] {
        self.ids
    }

    /// Returns the optional writable byte-offset buffer.
    pub fn offsets_mut(&mut self) -> Option<&mut [Offset]> {
        self.offsets.as_deref_mut()
    }

    /// Returns the optional writable token type-ID buffer.
    pub fn type_ids_mut(&mut self) -> Option<&mut [u8]> {
        self.type_ids.as_deref_mut()
    }

    pub(crate) fn raw(&mut self) -> n::iree_tokenizer_token_output_t {
        n::iree_tokenizer_token_output_t {
            capacity: self.ids.len(),
            token_ids: self.ids.as_mut_ptr(),
            token_offsets: self
                .offsets
                .as_mut()
                .map_or(ptr::null_mut(), |x| x.as_mut_ptr().cast()),
            type_ids: self
                .type_ids
                .as_mut()
                .map_or(ptr::null_mut(), |x| x.as_mut_ptr()),
        }
    }
}

/// The amount of progress made by a streaming operation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    /// Bytes or tokens consumed from the provided input.
    pub consumed: usize,
    /// Tokens or bytes written, depending on operation.
    pub written: usize,
}

mod decoder;
mod model;
mod normalizer;
mod postprocessor;
mod segmenter;
mod tokenizer;

pub use decoder::DecoderState;
pub use model::ModelState;
pub use normalizer::NormalizerState;
pub use postprocessor::PostprocessorState;
pub use segmenter::SegmenterState;
pub use tokenizer::{DecodeState, EncodeState};
