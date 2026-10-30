//! Errors produced by the safe MLIR bindings.
use iree_native as n;

/// Errors from the compiler embedding and MLIR C APIs.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The compiler embedding wrapper failed or a native compiler API failed.
    #[error(transparent)]
    Compiler(#[from] ireec::Error),
    /// MLIR rejected the supplied text.
    #[error("failed to parse MLIR {0}")]
    Parse(&'static str),
    /// A native API returned a null handle.
    #[error("native API returned a null {0}")]
    Null(&'static str),
    /// A Rust argument violates a native API precondition.
    #[error("{0}")]
    Invalid(String),
    /// Writing printed output failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

impl From<n::Error> for Error {
    fn from(error: n::Error) -> Self {
        Self::Compiler(error.into())
    }
}
