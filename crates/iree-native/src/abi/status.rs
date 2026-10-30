//! Value-only operations for public `iree_status_t` representations.

use crate as n;

/// Construct a non-owning code-only native status using the public tag mask.
#[inline]
pub const fn from_code(code: n::iree_status_code_t) -> n::iree_status_t {
    core::ptr::without_provenance_mut((code as usize) & (n::IREE_STATUS_CODE_MASK as usize))
}

/// Read status tag bits without dereferencing or acquiring its payload.
#[inline]
pub fn code(status: n::iree_status_t) -> n::iree_status_code_t {
    (status.addr() & n::IREE_STATUS_CODE_MASK as usize) as n::iree_status_code_t
}

/// Successful native status; no ownership is acquired.
#[inline]
pub const fn ok() -> n::iree_status_t {
    from_code(n::IREE_STATUS_OK)
}

/// Whether `status` is the native success status.
#[inline]
pub fn is_ok(status: n::iree_status_t) -> bool {
    status.is_null()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_code_round_trips_without_storage() {
        for code in 0..=n::IREE_STATUS_CODE_MASK {
            assert_eq!(self::code(from_code(code)), code);
        }
        assert!(ok().is_null());
        assert!(is_ok(ok()));
        assert!(!is_ok(from_code(n::IREE_STATUS_INTERNAL)));
    }
}
