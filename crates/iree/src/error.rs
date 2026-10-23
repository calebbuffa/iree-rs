use std::fmt;

/// A native IREE diagnostic or a rejected Rust API argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
    code: StatusCode,
}

/// Stable IREE status code, retained independently of diagnostic text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(i32);

impl StatusCode {
    /// Unknown failure without a more precise classification.
    pub const UNKNOWN: Self = Self(2);
    /// Invalid argument, including rejected Rust API input.
    pub const INVALID_ARGUMENT: Self = Self(3);
    /// Operation cancelled before completion.
    pub const CANCELLED: Self = Self(1);
    /// A wait deadline elapsed.
    pub const DEADLINE_EXCEEDED: Self = Self(4);
    /// A requested function, resource, or identifier does not exist.
    pub const NOT_FOUND: Self = Self(5);
    /// A resource with the requested identifier already exists.
    pub const ALREADY_EXISTS: Self = Self(6);
    /// Access to the requested operation was denied.
    pub const PERMISSION_DENIED: Self = Self(7);
    /// A resource or allocation limit was exhausted.
    pub const RESOURCE_EXHAUSTED: Self = Self(8);
    /// The operation requires a different resource or runtime state.
    pub const FAILED_PRECONDITION: Self = Self(9);
    /// The operation was aborted.
    pub const ABORTED: Self = Self(10);
    /// An index, offset, or numeric value was out of range.
    pub const OUT_OF_RANGE: Self = Self(11);
    /// The requested capability is unavailable in the selected implementation.
    pub const UNIMPLEMENTED: Self = Self(12);
    /// Internal implementation error.
    pub const INTERNAL: Self = Self(13);
    /// A service or device is temporarily unavailable.
    pub const UNAVAILABLE: Self = Self(14);
    /// Irrecoverable data loss or corruption was detected.
    pub const DATA_LOSS: Self = Self(15);
    /// Authentication is required or failed.
    pub const UNAUTHENTICATED: Self = Self(16);
    /// Operation is not yet complete.
    pub const DEFERRED: Self = Self(17);
    /// Numeric IREE status value, including future native codes.
    pub const fn as_i32(self) -> i32 {
        self.0
    }
    pub(crate) fn from_native(value: crate::sys::iree_status_code_t) -> Self {
        // C enum signedness varies by target. IREE codes occupy five bits.
        Self(i32::try_from(i64::from(value)).expect("native IREE status code exceeds i32"))
    }
}

impl Error {
    /// Consume one owned native status, returning its diagnostic on failure.
    ///
    /// A null success status returns `None`. Failure diagnostics are copied
    /// into Rust storage before releasing the native status.
    ///
    /// # Safety
    ///
    /// `status` must be a valid owned IREE status from this crate's native
    /// libraries. Ownership is transferred even if diagnostic formatting fails;
    /// do not release or reuse a non-null status after calling this method.
    pub unsafe fn from_raw(status: crate::sys::iree_status_t) -> Option<Self> {
        crate::runtime::check(status).err()
    }

    pub(crate) fn invalid(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INVALID_ARGUMENT, message)
    }

    /// Construct an error preserving its native status classification.
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }

    /// Machine-readable status classification.
    pub fn code(&self) -> StatusCode {
        self.code
    }

    /// Complete diagnostic message, excluding the status classification.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

/// Result of an owned API operation, preserving native diagnostic text.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::StatusCode;
    use crate::sys;

    #[test]
    fn native_status_codes_preserve_their_numeric_values() {
        for (native, expected) in [
            (sys::IREE_STATUS_CANCELLED, StatusCode::CANCELLED),
            (
                sys::IREE_STATUS_INVALID_ARGUMENT,
                StatusCode::INVALID_ARGUMENT,
            ),
            (sys::IREE_STATUS_NOT_FOUND, StatusCode::NOT_FOUND),
            (sys::IREE_STATUS_DEFERRED, StatusCode::DEFERRED),
        ] {
            let code = StatusCode::from_native(native);
            assert_eq!(code, expected);
            assert_eq!(i64::from(code.as_i32()), i64::from(native));
        }
    }
}
