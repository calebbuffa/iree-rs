//! Formatting through `iree_string_builder_t`.
use crate::{Result, check};
use iree_native as n;

/// Runs a native formatter into a fresh string builder and returns its text.
pub(crate) fn format(
    write: impl FnOnce(*mut n::iree_string_builder_t) -> n::iree_status_t,
) -> Result<String> {
    struct Builder(n::iree_string_builder_t);
    impl Drop for Builder {
        fn drop(&mut self) {
            // SAFETY: the builder was initialized exactly once.
            unsafe { n::iree_string_builder_deinitialize(&mut self.0) }
        }
    }
    let mut builder = Builder(n::iree_string_builder_t::default());
    // SAFETY: the storage is writable; initialization does not allocate.
    unsafe { n::iree_string_builder_initialize(crate::system(), &mut builder.0) };
    check(write(&mut builder.0))?;
    // SAFETY: the builder owns the viewed bytes until it is dropped.
    Ok(unsafe { n::view_string(n::iree_string_builder_view(&builder.0)) })
}
