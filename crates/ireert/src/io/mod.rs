//! I/O: files, streams, parameter indexes and providers.
//!
//! Laws:
//! - File handles, mappings and streams are always available.
//! - Parameter indexes require `io-index`; providers and the VM
//!   I/O-parameters module require `io-parameters`.
//! - Format parsers and IRPA archive construction are compiled only when their
//!   corresponding `io-format-*` feature is enabled.

#[cfg(feature = "io-format-irpa")]
mod archive;
mod file_contents;
mod file_handle;
mod file_mapping;
#[cfg(feature = "io-index")]
mod parameters;
#[cfg(feature = "io-parameters")]
mod provider;
mod stream;

#[cfg(feature = "io-format-irpa")]
pub use archive::{ParameterArchiveBuilder, build_parameter_archive};
pub use file_contents::FileContents;
pub use file_handle::{FileAccess, FileHandle, FileHandlePrimitive, FileMode};
pub use file_mapping::{FileMapping, MappingFlags};
#[cfg(feature = "io-index")]
pub use parameters::{ParameterEntry, ParameterIndex, ParameterStorage};
#[cfg(feature = "io-parameters")]
pub use provider::{ParameterProvider, ParameterSpan, ProviderSignal, Submission};
pub use stream::{StdioMode, Stream, StreamMode};

pub(crate) fn path_str(path: &std::path::Path) -> crate::Result<&str> {
    path.to_str().ok_or_else(|| {
        crate::Error::new(
            crate::Code::InvalidArgument,
            "IREE file APIs require UTF-8 paths",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::path_str;

    #[test]
    fn path_conversion_borrows_utf8_path() {
        let path = std::path::PathBuf::from("weights/model.vmfb");
        assert_eq!(path_str(&path).unwrap(), "weights/model.vmfb");
    }

    #[cfg(unix)]
    #[test]
    fn path_conversion_rejects_non_utf8_path() {
        use std::os::unix::ffi::OsStrExt;

        let path = std::path::Path::new(std::ffi::OsStr::from_bytes(b"\xff"));
        assert_eq!(
            path_str(path).unwrap_err().code(),
            crate::Code::InvalidArgument
        );
    }
}
