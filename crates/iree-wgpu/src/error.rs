//! Construction of crate errors.

use ireert::{Code, Error};

/// An [`Error`] with [`Code::InvalidArgument`] and a crate-prefixed message.
pub(crate) fn invalid(error: impl std::fmt::Display) -> Error {
    Error::new(Code::InvalidArgument, format!("iree-wgpu: {error}"))
}
