//! Value-only operations for public IREE reference counters.

use crate as n;

// MSVC's C path, GCC, and disabled atomics typedef int32_t; Clang typedefs
// _Atomic int32_t. Bindgen must expose their actual size/alignment as i32.
const _: [(); core::mem::size_of::<i32>()] =
    [(); core::mem::size_of::<n::iree_atomic_ref_count_t>()];
const _: [(); core::mem::align_of::<i32>()] =
    [(); core::mem::align_of::<n::iree_atomic_ref_count_t>()];
#[cfg(target_has_atomic = "32")]
const _: [(); core::mem::align_of::<i32>()] =
    [(); core::mem::align_of::<core::sync::atomic::AtomicI32>()];

/// Initialize an unpublished native reference counter to one.
///
/// # Safety
/// `count` must be nonnull, aligned, and writable native refcount storage. It
/// must not be accessed nonatomically or deallocated during this call. On targets
/// without 32-bit atomics the caller must have exclusive access, matching IREE's
/// synchronization-disabled single-threaded contract.
#[inline]
pub unsafe fn init(count: *mut n::iree_atomic_ref_count_t) {
    let count: *mut i32 = count;
    #[cfg(target_has_atomic = "32")]
    {
        use core::sync::atomic::{AtomicI32, Ordering};
        // MSVC C's InterlockedExchange strengthens native relaxed initialization
        // to sequential consistency. Clang/GCC implement the requested relaxed store.
        let ordering = if cfg!(target_env = "msvc") {
            Ordering::SeqCst
        } else {
            Ordering::Relaxed
        };
        // SAFETY: the caller guarantees lifetime/access; ABI assertions above
        // establish the alignment and representation required by from_ptr.
        unsafe { AtomicI32::from_ptr(count) }.store(1, ordering);
    }
    #[cfg(not(target_has_atomic = "32"))]
    // SAFETY: no concurrent accesses are permitted on the disabled-atomics path.
    unsafe {
        count.write(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_sets_the_native_initial_reference() {
        let mut count = 0;
        // SAFETY: count is a writable, correctly aligned ref-count slot.
        unsafe { init(&mut count) };
        assert_eq!(count, 1);
    }
}
