use crate::{Code, Error, Result, n};
use core::{mem::size_of, str};

/// Borrows the bytes from a native string view.
///
/// # Safety
/// A nonempty view must reference readable storage for its declared size.
pub(super) unsafe fn input_bytes<'a>(input: n::iree_string_view_t) -> Result<&'a [u8]> {
    if input.size == 0 {
        Ok(&[])
    } else if input.data.is_null() {
        Err(Error::new(
            Code::InvalidArgument,
            "extension input storage is null",
        ))
    } else {
        // SAFETY: IREE's string-view contract guarantees this nonempty input is
        // readable for the supplied length during the callback.
        Ok(unsafe { n::slice(input.data.cast::<u8>(), input.size) })
    }
}

/// Borrows UTF-8 text from a native string view.
///
/// # Safety
/// A nonempty view must reference readable storage for its declared size.
pub(super) unsafe fn input_str<'a>(input: n::iree_string_view_t) -> Result<&'a str> {
    // SAFETY: the caller guarantees the native view points to readable bytes.
    let bytes = unsafe { input_bytes(input) }?;
    str::from_utf8(bytes)
        .map_err(|_| Error::new(Code::InvalidArgument, "extension input is not valid UTF-8"))
}

pub(super) unsafe fn output_bytes<'a>(
    output: n::iree_mutable_string_view_t,
) -> Result<&'a mut [u8]> {
    if output.size == 0 {
        Ok(&mut [])
    } else if output.data.is_null() {
        Err(Error::new(
            Code::InvalidArgument,
            "extension output storage is null",
        ))
    } else {
        // SAFETY: IREE's mutable view contract guarantees writable storage for
        // the supplied length during the callback.
        Ok(unsafe { n::slice_mut(output.data.cast::<u8>(), output.size) })
    }
}

pub(super) unsafe fn state_slot<B, T>(state: *mut B) -> *mut *mut T {
    // SAFETY: caller provides storage sized for the base followed by one pointer.
    unsafe { state.cast::<u8>().add(size_of::<B>()).cast() }
}
