//! Construction and inspection of MLIR's logical-result value.
//!
//! Law: success is any nonzero value; failure is zero. These operations
//! preserve the public C representation without native allocation.
use crate::MlirLogicalResult;

/// Construct MLIR logical success, preserving its public C representation.
#[inline]
pub const fn success() -> MlirLogicalResult {
    MlirLogicalResult { value: 1 }
}
/// Construct MLIR logical failure, preserving its public C representation.
#[inline]
pub const fn failure() -> MlirLogicalResult {
    MlirLogicalResult { value: 0 }
}
/// Whether a public MLIR logical result denotes success.
#[inline]
pub const fn is_success(value: MlirLogicalResult) -> bool {
    value.value != 0
}
/// Whether a public MLIR logical result denotes failure.
#[inline]
pub const fn is_failure(value: MlirLogicalResult) -> bool {
    value.value == 0
}
