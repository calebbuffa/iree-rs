//! Value-level laws for the minimum shared IREE ABI support layer.

use iree_native::{self as n, Code, Error};
use proptest::prelude::*;
use std::time::Duration;

proptest! {
    #[test]
    fn status_codes_round_trip(code in 0i32..32) {
        let value = Code::from_raw(code).unwrap();
        prop_assert_eq!(value.to_raw(), code);
    }
    #[test]
    fn invalid_codes_never_mask_into_success(code in any::<i32>().prop_filter("outside status encoding", |&c| !(0..32).contains(&c))) {
        prop_assert!(Code::from_raw(code).is_none());
    }
    #[test]
    fn string_views_preserve_arbitrary_utf8(value in any::<String>()) {
        let view = n::string_view(&value);
        prop_assert_eq!(view.size, value.len());
        // SAFETY: view borrows the live String bytes and its length matches the allocation.
        let restored = unsafe { n::view_bytes(view) };
        prop_assert_eq!(restored, value.as_bytes());
    }
    #[test]
    fn finite_duration_conversion_is_exact(nanos in 0u64..i64::MAX as u64) {
        let timeout = n::timeout(Some(Duration::from_nanos(nanos))).unwrap();
        prop_assert_eq!(timeout.nanos, nanos as i64);
        prop_assert_eq!(timeout.type_, n::IREE_TIMEOUT_RELATIVE);
    }
    #[test]
    fn oversized_duration_never_becomes_finite(nanos in i64::MAX as u64..=u64::MAX) {
        prop_assert!(n::timeout(Some(Duration::from_nanos(nanos))).is_err());
    }
    #[test]
    fn callback_status_preserves_error_code(index in 0usize..Code::ALL.len(), message in any::<String>()) {
        let code = Code::ALL[index];
        let status = n::callback_status(|| Err(Error::new(code, message.clone())));
        // SAFETY: callback_status returns one owned status for consume.
        let restored = unsafe { n::consume(status) }.unwrap_err();
        prop_assert_eq!(restored.code(), code);
        if n::IREE_STATUS_FEATURES & n::IREE_STATUS_FEATURE_ANNOTATIONS != 0 {
            prop_assert_eq!(restored.message(), message);
        }
    }
}

#[test]
fn callback_panics_return_internal_without_dropping_hostile_payloads() {
    struct Hostile;
    impl Drop for Hostile {
        fn drop(&mut self) {
            panic!("payload destructor must not run across C")
        }
    }
    let status = n::callback_status(|| std::panic::panic_any(Hostile));
    assert_eq!(
        // SAFETY: callback_status returns one owned status for consume.
        unsafe { n::consume(status) }.unwrap_err().code(),
        Code::Internal
    );
}
#[test]
fn infinite_timeout_is_explicit_and_success_cannot_be_an_error() {
    // SAFETY: timeout(None) constructs a valid by-value native timeout.
    assert!(unsafe { n::iree_timeout_is_infinite(n::timeout(None).unwrap()) });
    assert_ne!(Error::new(Code::Ok, "invalid failure").code(), Code::Ok);
    // SAFETY: callback_status returns one owned success status for consume.
    assert!(unsafe { n::consume(n::callback_status(|| Ok(()))) }.is_ok());
}

#[test]
fn status_preserves_long_diagnostics_and_owned_payloads() {
    let message = "diagnostic\0".repeat(2048);
    let status =
        // SAFETY: into_status creates one owned status.
        unsafe { n::Status::from_raw(Error::new(Code::DataLoss, message.clone()).into_status()) };
    let annotation = "payload-diagnostic".repeat(513);
    // SAFETY: status is owned, annotation borrows a live string, and returns a new owned status.
    let status = unsafe {
        n::Status::from_raw(n::iree_status_annotate(
            status.into_raw(),
            n::string_view(&annotation),
        ))
    };
    let payloads = status.payloads().unwrap();
    let error = status.into_error().unwrap();
    assert_eq!(error.code(), Code::DataLoss);
    assert_eq!(error.payload(), payloads.as_slice());
    if n::IREE_STATUS_FEATURES & n::IREE_STATUS_FEATURE_ANNOTATIONS != 0 {
        assert_eq!(error.message(), message);
        assert!(
            payloads
                .iter()
                .any(|payload| payload.text.contains(&annotation))
        );
    }
}

proptest! {
    #[test]
    fn io_errors_preserve_classification_and_message(index in 0usize..9, message in any::<String>()) {
        use std::io::ErrorKind;
        let cases = [
            (ErrorKind::NotFound, Code::NotFound),
            (ErrorKind::PermissionDenied, Code::PermissionDenied),
            (ErrorKind::AlreadyExists, Code::AlreadyExists),
            (ErrorKind::InvalidInput, Code::InvalidArgument),
            (ErrorKind::TimedOut, Code::DeadlineExceeded),
            (ErrorKind::Interrupted, Code::Aborted),
            (ErrorKind::Unsupported, Code::Unimplemented),
            (ErrorKind::OutOfMemory, Code::ResourceExhausted),
            (ErrorKind::Other, Code::Unknown),
        ];
        let (kind, code) = cases[index];
        let error = Error::from(std::io::Error::new(kind, message.clone()));
        prop_assert_eq!(error.code(), code);
        prop_assert_eq!(error.message(), message);
    }
}
