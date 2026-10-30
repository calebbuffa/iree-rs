//! Low-level Rust bindings to the pinned IREE C API.
//!
//! Generated C declarations are re-exported at the crate root. The [`abi`]
//! module contains value-level shims; [`Code`], [`Error`], and [`Status`]
//! provide shared status handling for the higher-level runtime crates.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#[cfg(any(panic = "unwind", test, not(target_os = "none")))]
extern crate std;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
#[cfg(feature = "compiler")]
/// Build-local compiler deployment directory, also staged under the Cargo profile.
pub const IREE_COMPILER_BIN_DIR: &str = env!("IREE_COMPILER_BIN_DIR");
#[cfg(feature = "compiler")]
/// Content fingerprint used to identify the matching compiler deployment.
pub const IREE_COMPILER_DEPLOYMENT_ID: &str = env!("IREE_COMPILER_DEPLOYMENT_ID");
#[allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    improper_ctypes,
    unsafe_op_in_unsafe_fn,
    unused_imports,
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_codeblock_attributes
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use generated::*;

extern crate alloc;
pub mod abi;
#[cfg(all(
    feature = "compiler",
    any(not(iree_native_docs), not(target_arch = "wasm32"))
))]
pub mod compiler;
mod error;
pub use error::{Code, Error, Result, Status, StatusPayload, callback_status, consume, contain};

use core::time::Duration;

/// Returns IREE's process-wide system allocator.
///
/// The returned value is a copyable allocator descriptor, not an owned
/// allocation. It can be passed to native APIs that accept an allocator.
pub fn system_allocator() -> crate::iree_allocator_t {
    // SAFETY: IREE's system allocator accessor returns a plain descriptor.
    unsafe { crate::iree_allocator_system() }
}

/// Borrow UTF-8 bytes for a synchronous native call; C must not retain them.
pub const fn string_view(value: &str) -> crate::iree_string_view_t {
    crate::iree_string_view_t {
        data: value.as_ptr().cast(),
        size: value.len(),
    }
}
/// Borrow arbitrary bytes as a counted native string; no UTF-8 is required.
pub const fn bytes_view(value: &[u8]) -> crate::iree_string_view_t {
    crate::iree_string_view_t {
        data: value.as_ptr().cast(),
        size: value.len(),
    }
}
/// Borrow bytes as a native constant byte span for a synchronous call.
pub const fn const_span(value: &[u8]) -> crate::iree_const_byte_span_t {
    crate::iree_const_byte_span_t {
        data: value.as_ptr(),
        data_length: value.len(),
    }
}
/// Borrow a native array, accepting a null pointer for an empty range.
///
/// # Safety
/// When `len` is nonzero, `data` must be nonnull, aligned, and point to `len`
/// initialized values within one allocation whose total size is at most
/// `isize::MAX` bytes. The values must remain valid and unmodified for `'a`.
pub unsafe fn slice<'a, T>(data: *const T, len: usize) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        // SAFETY: guaranteed by the caller.
        unsafe { core::slice::from_raw_parts(data, len) }
    }
}
/// Mutably borrow a native array, accepting a null pointer for an empty range.
///
/// # Safety
/// When `len` is nonzero, `data` must be nonnull, aligned, and point to `len`
/// initialized values within one allocation whose total size is at most
/// `isize::MAX` bytes. The values must remain valid and exclusively accessible
/// for `'a`.
pub unsafe fn slice_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        // SAFETY: guaranteed by the caller.
        unsafe { core::slice::from_raw_parts_mut(data, len) }
    }
}
/// Borrow the bytes of a native string view.
///
/// # Safety
/// The view must describe readable bytes that stay valid for `'a`. For a
/// nonempty view, its pointer must be nonnull and aligned, and its length must
/// not exceed `isize::MAX`.
pub unsafe fn view_bytes<'a>(value: crate::iree_string_view_t) -> &'a [u8] {
    // SAFETY: guaranteed by the caller.
    unsafe { slice(value.data.cast(), value.size) }
}
/// Copy a native string view into an owned string, replacing invalid UTF-8.
///
/// # Safety
/// The view must describe readable bytes for the duration of the call. For a
/// nonempty view, its pointer must be nonnull and aligned, and its length must
/// not exceed `isize::MAX`.
pub unsafe fn view_string(value: crate::iree_string_view_t) -> alloc::string::String {
    // SAFETY: guaranteed by the caller.
    alloc::string::String::from_utf8_lossy(unsafe { view_bytes(value) }).into_owned()
}
/// Convert a finite relative duration, or `None` for an infinite native timeout.
pub fn timeout(value: Option<Duration>) -> Result<crate::iree_timeout_t> {
    let Some(value) = value else {
        // SAFETY: IREE's timeout constructor has no preconditions.
        return Ok(unsafe { crate::iree_infinite_timeout() });
    };
    let nanos = i64::try_from(value.as_nanos())
        .ok()
        .filter(|&nanos| nanos != i64::MAX)
        .ok_or_else(|| {
            Error::new(
                Code::OutOfRange,
                "duration exceeds finite native nanoseconds",
            )
        })?;
    // SAFETY: nanos is finite and excludes the reserved infinite-timeout sentinel.
    Ok(unsafe { crate::iree_make_timeout_ns(nanos) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_native_slices_accept_null_pointers() {
        // SAFETY: an empty range does not dereference its null pointer.
        assert!(unsafe { slice::<u8>(core::ptr::null(), 0) }.is_empty());
        // SAFETY: an empty range does not dereference its null pointer.
        assert!(unsafe { slice_mut::<u8>(core::ptr::null_mut(), 0) }.is_empty());
    }

    #[test]
    fn native_slices_borrow_the_requested_elements() {
        let mut values = [1_u32, 2, 3];
        // SAFETY: values contains three initialized elements and remains live.
        assert_eq!(unsafe { slice(values.as_ptr(), values.len()) }, values);
        // SAFETY: values contains three initialized elements and is exclusively borrowed.
        let borrowed = unsafe { slice_mut(values.as_mut_ptr(), values.len()) };
        borrowed[1] = 4;
        assert_eq!(values, [1, 4, 3]);
    }
}
