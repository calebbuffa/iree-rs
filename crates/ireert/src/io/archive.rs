//! Building IRPA parameter archives.
//!
//! Laws:
//! - A built archive parses back to the same keys, lengths and splats.
//! - Destination handles are retained by the target index, so they must be
//!   `FileHandle<'static>`.
use super::{
    file_handle::{FileAccess, FileHandle},
    parameters::ParameterIndex,
    stream::Stream,
};
use crate::{Code, Error, Result, check};
use iree_native as n;
use n::const_span;

/// An `iree_io_parameter_archive_builder_t`.
pub struct ParameterArchiveBuilder {
    raw: n::iree_io_parameter_archive_builder_t,
}

impl ParameterArchiveBuilder {
    /// Initializes an empty builder.
    pub fn new() -> Result<Self> {
        let mut raw = n::iree_io_parameter_archive_builder_t::default();
        // SAFETY: the builder storage is writable for the call.
        check(unsafe {
            n::iree_io_parameter_archive_builder_initialize(crate::system(), &mut raw)
        })?;
        Ok(Self { raw })
    }

    /// Returns whether no entries have been added.
    pub fn is_empty(&self) -> bool {
        // SAFETY: the builder is initialized.
        unsafe { n::iree_io_parameter_archive_builder_is_empty(&self.raw) }
    }

    /// Returns the serialized header size in bytes.
    pub fn header_size(&self) -> u64 {
        // SAFETY: the builder is initialized.
        unsafe { n::iree_io_parameter_archive_builder_header_size(&self.raw) }
    }

    /// Returns the complete serialized size in bytes.
    pub fn total_size(&self) -> u64 {
        // SAFETY: the builder is initialized.
        unsafe { n::iree_io_parameter_archive_builder_total_size(&self.raw) }
    }

    /// Adds an entry of `length` bytes repeating `pattern` (1, 2, 4, 8 or 16 bytes).
    pub fn add_splat(
        &mut self,
        name: &str,
        metadata: &[u8],
        pattern: &[u8],
        length: u64,
    ) -> Result<()> {
        let pattern_length = u8::try_from(pattern.len())
            .map_err(|_| Error::new(Code::InvalidArgument, "splat pattern too long"))?;
        // SAFETY: the inputs are valid for the call; native copies them.
        check(unsafe {
            n::iree_io_parameter_archive_builder_add_splat_entry(
                &mut self.raw,
                crate::view(name),
                const_span(metadata),
                pattern.as_ptr().cast(),
                pattern_length,
                length,
            )
        })
    }

    /// Adds an entry of `length` bytes whose data is written by
    /// [`ParameterArchiveBuilder::write`].
    pub fn add_data(
        &mut self,
        name: &str,
        metadata: &[u8],
        minimum_alignment: u64,
        length: u64,
    ) -> Result<()> {
        // Native aligns with a power-of-two mask and does not validate it.
        if !minimum_alignment.is_power_of_two() {
            return Err(Error::new(
                Code::InvalidArgument,
                "archive data alignment must be a power of two",
            ));
        }
        // SAFETY: the inputs are valid for the call; native copies them.
        check(unsafe {
            n::iree_io_parameter_archive_builder_add_data_entry(
                &mut self.raw,
                crate::view(name),
                const_span(metadata),
                minimum_alignment,
                length,
            )
        })
    }

    /// Writes the archive header through `output` and adds entries referencing
    /// `file` at `file_offset` to `target_index`.
    pub fn write(
        &self,
        file: &FileHandle<'static>,
        file_offset: u64,
        output: &mut Stream<'_>,
        target_index: &mut ParameterIndex,
    ) -> Result<()> {
        file.require_access(FileAccess::READ)?;
        // SAFETY: all arguments are live for the synchronous call.
        check(unsafe {
            n::iree_io_parameter_archive_builder_write(
                &self.raw,
                file.as_raw(),
                file_offset,
                output.as_raw(),
                target_index.as_raw(),
            )
        })
    }
}

impl Drop for ParameterArchiveBuilder {
    fn drop(&mut self) {
        // SAFETY: the builder was initialized exactly once.
        unsafe { n::iree_io_parameter_archive_builder_deinitialize(&mut self.raw) }
    }
}

/// Builds an archive of every entry in `source`, adding the archived entries
/// to `target`.
///
/// `open(archive_offset, archive_length)` returns the file the archive is
/// written into; it must hold at least `archive_offset + archive_length` bytes.
/// The returned file must support both writing and subsequent parameter reads.
pub fn build_parameter_archive(
    source: &ParameterIndex,
    target: &mut ParameterIndex,
    file_offset: u64,
    mut open: impl FnMut(u64, u64) -> Result<FileHandle<'static>>,
) -> Result<()> {
    unsafe extern "C" fn trampoline(
        user_data: *mut core::ffi::c_void,
        archive_offset: u64,
        archive_length: u64,
        out_file: *mut *mut n::iree_io_file_handle_t,
    ) -> n::iree_status_t {
        n::callback_status(|| {
            // SAFETY: `user_data` is the live `&mut dyn FnMut` below.
            let open = unsafe {
                &mut *user_data.cast::<&mut dyn FnMut(u64, u64) -> Result<FileHandle<'static>>>()
            };
            let file = open(archive_offset, archive_length)?;
            file.require_access(FileAccess::READ | FileAccess::WRITE)?;
            // SAFETY: native takes ownership of the returned reference.
            unsafe { *out_file = file.into_raw() };
            Ok(())
        })
    }
    let mut open: &mut dyn FnMut(u64, u64) -> Result<FileHandle<'static>> = &mut open;
    // SAFETY: the callback runs synchronously while `open` is live.
    check(unsafe {
        n::iree_io_build_parameter_archive(
            source.as_raw(),
            target.as_raw(),
            n::iree_io_parameter_archive_file_open_callback_t {
                fn_: Some(trampoline),
                user_data: (&mut open
                    as *mut &mut dyn FnMut(u64, u64) -> Result<FileHandle<'static>>)
                    .cast(),
            },
            file_offset,
            crate::system(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::FileAccess;
    use proptest::prelude::*;

    #[test]
    fn archive_builder_rejects_immutable_destination() {
        let mut source = ParameterIndex::new().unwrap();
        source.add_splat("weight", &[], &[1], 16).unwrap();
        let mut target = ParameterIndex::new().unwrap();
        let error = build_parameter_archive(&source, &mut target, 0, |_, _| {
            FileHandle::from_owner([0; 4096])
        })
        .unwrap_err();
        assert_eq!(error.code(), Code::PermissionDenied);
    }

    proptest! {
        #[test]
        fn splat_archive_build_then_parse_round_trips(
            key in "[a-z]{1,16}",
            pattern_bytes in proptest::collection::vec(any::<u8>(), 16),
            pattern_length in prop_oneof![Just(1usize), Just(2), Just(4), Just(8), Just(16)],
            length in 0u64..4096,
        ) {
            let pattern = &pattern_bytes[..pattern_length];
            let mut source = ParameterIndex::new().unwrap();
            source.add_splat(&key, &[], pattern, length).unwrap();
            let mut target = ParameterIndex::new().unwrap();
            let mut file = None;
            build_parameter_archive(&source, &mut target, 0, |offset, size| {
                let bytes = vec![0u8; (offset + size) as usize].into_boxed_slice();
                let handle = FileHandle::from_bytes(bytes, FileAccess::READ | FileAccess::WRITE)?;
                file = Some(handle.clone());
                Ok(handle)
            })
            .unwrap();
            prop_assert_eq!(target.lookup(&key).unwrap().key, key.as_str());

            let mut parsed = ParameterIndex::new().unwrap();
            parsed.parse_irpa(&file.unwrap()).unwrap();
            let entry = parsed.lookup(&key).unwrap();
            prop_assert_eq!(entry.key, key.as_str());
            prop_assert_eq!(entry.length, length);
        }
    }
}
