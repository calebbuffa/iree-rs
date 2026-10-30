//! Parameter indexes: insert-only maps from keys to splat or file storage.
//!
//! Laws:
//! - Entries are never removed; `len` only grows and `get(i)` is stable.
//! - The index retains every file handle it references, so only
//!   `FileHandle<'static>` (no borrowed host memory) may be inserted or parsed.
use super::file_handle::{FileAccess, FileHandle};
#[cfg(feature = "io-parameters")]
use super::provider::ParameterProvider;
use crate::{Code, Error, Result, check};
use iree_native as n;
use n::{const_span, slice, view_bytes};

/// The storage representation of an indexed parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterStorage<'a> {
    /// A repeated byte pattern.
    Splat(&'a [u8]),
    /// Bytes held in a file, with the offset within that handle.
    File {
        /// Byte offset of the parameter within its file.
        offset: u64,
    },
    /// A storage kind added by a newer native library.
    Unknown(n::iree_io_parameter_index_entry_storage_type_t),
}

/// A borrowed parameter index entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParameterEntry<'a> {
    /// UTF-8 parameter key.
    pub key: &'a str,
    /// Optional opaque metadata.
    pub metadata: &'a [u8],
    /// Byte length of the parameter.
    pub length: u64,
    /// Backing storage description.
    pub storage: ParameterStorage<'a>,
}

/// A retained, insert-only parameter index.
pub struct ParameterIndex(*mut n::iree_io_parameter_index_t);

impl ParameterIndex {
    /// Creates an empty parameter index.
    pub fn new() -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: output storage is writable and the system allocator is valid.
        check(unsafe { n::iree_io_parameter_index_create(crate::system(), &mut raw) })?;
        // SAFETY: successful creation returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned parameter-index reference.
    ///
    /// # Safety
    /// `raw` must be a live pointer with one owned reference.
    pub unsafe fn from_raw(raw: *mut n::iree_io_parameter_index_t) -> Self {
        Self(raw)
    }

    /// Borrows the native index pointer.
    pub fn as_raw(&self) -> *mut n::iree_io_parameter_index_t {
        self.0
    }

    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_io_parameter_index_t {
        core::mem::ManuallyDrop::new(self).0
    }

    /// Returns the current entry count.
    pub fn len(&self) -> usize {
        // SAFETY: self owns a live index reference.
        unsafe { n::iree_io_parameter_index_count(self.0) }
    }

    /// Returns whether the index currently has no entries.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserves capacity for at least `capacity` entries.
    pub fn reserve(&mut self, capacity: usize) -> Result<()> {
        // SAFETY: the index is live.
        check(unsafe { n::iree_io_parameter_index_reserve(self.0, capacity) })
    }

    /// Adds a synthetic entry repeating `pattern` (1, 2, 4, 8 or 16 bytes).
    pub fn add_splat(
        &mut self,
        key: &str,
        metadata: &[u8],
        pattern: &[u8],
        length: u64,
    ) -> Result<()> {
        let mut splat = n::iree_io_parameter_index_entry_t__bindgen_ty_1__bindgen_ty_1 {
            pattern_length: u8::try_from(pattern.len())
                .map_err(|_| Error::new(Code::InvalidArgument, "splat pattern is too long"))?,
            ..Default::default()
        };
        splat
            .pattern
            .get_mut(..pattern.len())
            .ok_or_else(|| Error::new(Code::InvalidArgument, "splat pattern is too long"))?
            .copy_from_slice(pattern);
        let mut entry = Self::entry(key, metadata, length);
        entry.type_ = n::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_SPLAT;
        entry.storage.splat = splat;
        // SAFETY: native validates the pattern length and copies key and metadata.
        check(unsafe { n::iree_io_parameter_index_add(self.0, &entry) })
    }

    /// Adds an entry backed by `length` bytes of `handle` at `offset`.
    pub fn add_file(
        &mut self,
        key: &str,
        metadata: &[u8],
        handle: &FileHandle<'static>,
        offset: u64,
        length: u64,
    ) -> Result<()> {
        // Native assumes file entries are readable and maps host allocations
        // without checking the range, so both are enforced here.
        handle.require_access(FileAccess::READ)?;
        let end = offset
            .checked_add(length)
            .ok_or_else(|| Error::new(Code::OutOfRange, "parameter file range overflows"))?;
        if let super::file_handle::FileHandlePrimitive::HostAllocation { length: capacity } =
            handle.primitive()
            && u64::try_from(capacity).is_ok_and(|capacity| end > capacity)
        {
            return Err(Error::new(
                Code::OutOfRange,
                "parameter file range exceeds host allocation",
            ));
        }
        let mut entry = Self::entry(key, metadata, length);
        entry.type_ = n::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_FILE;
        entry.storage.file = n::iree_io_parameter_index_entry_t__bindgen_ty_1__bindgen_ty_2 {
            handle: handle.as_raw(),
            offset,
        };
        // SAFETY: native retains the live handle and copies key and metadata.
        check(unsafe { n::iree_io_parameter_index_add(self.0, &entry) })
    }

    /// Returns the entry at `index`, or `None` past the end.
    pub fn get(&self, index: usize) -> Result<Option<ParameterEntry<'_>>> {
        if index >= self.len() {
            return Ok(None);
        }
        let mut entry = core::ptr::null();
        // SAFETY: the index is live and the output pointer is writable.
        check(unsafe { n::iree_io_parameter_index_get(self.0, index, &mut entry) })?;
        // SAFETY: success returns an entry that lives as long as the index.
        entry_view(unsafe { &*entry }).map(Some)
    }

    /// Finds an entry by key, returning the native not-found error when absent.
    pub fn lookup(&self, key: &str) -> Result<ParameterEntry<'_>> {
        let mut entry = core::ptr::null();
        // SAFETY: the key view and output pointer are valid during the lookup.
        check(unsafe { n::iree_io_parameter_index_lookup(self.0, crate::view(key), &mut entry) })?;
        // SAFETY: success returns an entry that lives as long as the index.
        entry_view(unsafe { &*entry })
    }

    /// Appends the entries of a GGUF file.
    #[cfg(feature = "io-format-gguf")]
    pub fn parse_gguf(&mut self, handle: &FileHandle<'static>) -> Result<()> {
        handle.require_access(FileAccess::READ)?;
        // SAFETY: the handle and index remain live for the parser call.
        check(unsafe { n::iree_io_parse_gguf_index(handle.as_raw(), self.0, crate::system()) })
    }

    /// Appends the entries of an IREE parameter archive.
    #[cfg(feature = "io-format-irpa")]
    pub fn parse_irpa(&mut self, handle: &FileHandle<'static>) -> Result<()> {
        handle.require_access(FileAccess::READ)?;
        // SAFETY: the handle and index remain live for the parser call.
        check(unsafe { n::iree_io_parse_irpa_index(handle.as_raw(), self.0, crate::system()) })
    }

    /// Appends the entries of a SafeTensors file.
    #[cfg(feature = "io-format-safetensors")]
    pub fn parse_safetensors(&mut self, handle: &FileHandle<'static>) -> Result<()> {
        handle.require_access(FileAccess::READ)?;
        // SAFETY: the handle and index remain live for the parser call.
        check(unsafe {
            n::iree_io_parse_safetensors_index(handle.as_raw(), self.0, crate::system())
        })
    }

    /// Formats this index with IREE's native parameter-index dump.
    pub fn dump(&self, scope: &str) -> Result<String> {
        // SAFETY: the scope and the index remain valid while the formatter runs.
        crate::base::string_builder::format(|builder| unsafe {
            n::iree_io_parameter_index_dump(crate::view(scope), self.0, builder)
        })
    }

    /// Creates a provider serving this index under `scope`.
    #[cfg(feature = "io-parameters")]
    pub fn provider(
        &self,
        scope: &str,
        max_concurrent_operations: usize,
    ) -> Result<ParameterProvider> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: native retains the index for the provider lifetime.
        check(unsafe {
            n::iree_io_parameter_index_provider_create(
                crate::view(scope),
                self.0,
                max_concurrent_operations,
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: successful creation returns one owned reference.
        Ok(unsafe { ParameterProvider::from_raw(raw) })
    }

    fn entry(key: &str, metadata: &[u8], length: u64) -> n::iree_io_parameter_index_entry_t {
        n::iree_io_parameter_index_entry_t {
            key: crate::view(key),
            metadata: const_span(metadata),
            length,
            ..Default::default()
        }
    }
}

impl Clone for ParameterIndex {
    fn clone(&self) -> Self {
        // SAFETY: `self.0` is a live ref-counted index.
        unsafe { n::iree_io_parameter_index_retain(self.0) };
        Self(self.0)
    }
}

impl Drop for ParameterIndex {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one index reference.
        unsafe { n::iree_io_parameter_index_release(self.0) }
    }
}

fn entry_view(entry: &n::iree_io_parameter_index_entry_t) -> Result<ParameterEntry<'_>> {
    // SAFETY: the index owns the key bytes for the lifetime of `entry`.
    let key = core::str::from_utf8(unsafe { view_bytes(entry.key) })
        .map_err(|_| Error::new(Code::InvalidArgument, "parameter key is not UTF-8"))?;
    // SAFETY: the index owns the metadata bytes for the lifetime of `entry`.
    let metadata = unsafe { slice(entry.metadata.data, entry.metadata.data_length) };
    let storage = match entry.type_ {
        n::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_SPLAT => {
            // SAFETY: the SPLAT union member is active.
            let splat = unsafe { &entry.storage.splat };
            ParameterStorage::Splat(
                splat
                    .pattern
                    .get(..usize::from(splat.pattern_length))
                    .unwrap_or(&splat.pattern),
            )
        }
        n::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_FILE => ParameterStorage::File {
            // SAFETY: the FILE union member is active.
            offset: unsafe { entry.storage.file.offset },
        },
        other => ParameterStorage::Unknown(other),
    };
    Ok(ParameterEntry {
        key,
        metadata,
        length: entry.length,
        storage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::BTreeSet;

    proptest! {
        #[test]
        fn parameter_index_preserves_counts_and_keys(keys in proptest::collection::btree_set("[a-z]{1,16}", 0..32)) {
            let mut index = ParameterIndex::new().unwrap();
            for key in &keys {
                index.add_splat(key, &[], &[0xA5], 8).unwrap();
            }
            prop_assert_eq!(index.len(), keys.len());
            let actual = (0..index.len())
                .map(|position| index.get(position).unwrap().unwrap().key.to_owned())
                .collect::<BTreeSet<_>>();
            prop_assert_eq!(actual, keys);
        }
    }

    #[test]
    fn splat_pattern_longer_than_native_storage_is_rejected() {
        let mut index = ParameterIndex::new().unwrap();
        assert!(index.add_splat("a", &[], &[0; 17], 8).is_err());
    }
}
