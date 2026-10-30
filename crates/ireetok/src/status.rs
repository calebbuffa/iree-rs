//! Native status checking and borrowed byte/string view helpers.
use crate::{Code, Error, Result, n};
use std::ptr::NonNull;

pub(crate) use n::system_allocator as system;

pub(crate) fn check(status: n::iree_status_t) -> Result<()> {
    // SAFETY: every status handed to `check` is freshly returned and owned.
    unsafe { n::consume(status) }
}

pub(crate) use n::{bytes_view as const_bytes_view, string_view as view};

pub(crate) fn bytes_view(s: &mut [u8]) -> n::iree_mutable_string_view_t {
    n::iree_mutable_string_view_t {
        data: s.as_mut_ptr().cast(),
        size: s.len(),
    }
}

pub(crate) fn copy_string_view(value: n::iree_string_view_t) -> String {
    // SAFETY: callers pass views returned by IREE while the backing object is live.
    unsafe { n::view_string(value) }
}

/// Initial output estimate for `size` input units; callers grow it with
/// [`grow_capacity`] when the native call reports exhaustion.
pub(crate) fn capacity_for(size: usize) -> Result<usize> {
    size.checked_mul(8)
        .and_then(|v| v.checked_add(32))
        .ok_or_else(|| Error::new(Code::OutOfRange, "tokenizer buffer size overflow"))
}

/// Times a one-shot call may double its output buffer before the exhaustion
/// is returned to the caller.
///
/// Upstream also reports zero decode progress as exhaustion when the pipeline
/// cannot make progress at all, so growth must be bounded.
pub(crate) const MAX_OUTPUT_GROWTHS: u32 = 8;

/// Doubles `capacity` for a retry after `growths` previous retries, or
/// returns `exhausted` once [`MAX_OUTPUT_GROWTHS`] is reached.
pub(crate) fn grow_capacity(capacity: usize, growths: &mut u32, exhausted: Error) -> Result<usize> {
    if *growths >= MAX_OUTPUT_GROWTHS {
        return Err(exhausted);
    }
    *growths += 1;
    capacity
        .checked_mul(2)
        .ok_or_else(|| Error::new(Code::OutOfRange, "tokenizer buffer size overflow"))
}

pub(crate) fn word_storage(bytes: usize) -> Vec<usize> {
    vec![0; bytes.saturating_add(std::mem::size_of::<usize>() - 1) / std::mem::size_of::<usize>()]
}

pub(crate) fn span(words: &mut [usize]) -> n::iree_byte_span_t {
    n::iree_byte_span_t {
        data: words.as_mut_ptr().cast(),
        data_length: std::mem::size_of_val(words),
    }
}

pub(crate) fn nonnull<T>(raw: *mut T, context: &'static str) -> Result<NonNull<T>> {
    NonNull::new(raw).ok_or_else(|| Error::new(Code::Internal, context))
}
