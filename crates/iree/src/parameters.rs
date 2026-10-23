//! Owned parameter storage and native archive-backed model weights.

use crate::{
    Error, Result, StatusCode,
    runtime::{check, string_view},
    sys,
};
use std::{
    ffi::c_void,
    marker::PhantomData,
    path::Path,
    ptr::{self, NonNull},
    rc::Rc,
};

struct File(NonNull<sys::iree_io_file_handle_t>);
impl Drop for File {
    fn drop(&mut self) {
        unsafe { sys::iree_io_file_handle_release(self.0.as_ptr()) };
    }
}

unsafe extern "C" fn release_bytes(
    data: *mut c_void,
    _primitive: sys::iree_io_file_handle_primitive_t,
) {
    unsafe {
        drop(Box::from_raw(data.cast::<Vec<u8>>()));
    }
}

impl File {
    fn bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut storage = Box::new(bytes);
        let span = sys::iree_byte_span_t {
            data: storage.as_mut_ptr(),
            data_length: storage.len(),
        };
        let owner = Box::into_raw(storage);
        let mut raw = ptr::null_mut();
        let status = unsafe {
            sys::iree_io_file_handle_wrap_host_allocation(
                sys::IREE_IO_FILE_ACCESS_READ as _,
                span,
                sys::iree_io_file_handle_release_callback_t {
                    fn_: Some(release_bytes),
                    user_data: owner.cast(),
                },
                sys::iree_allocator_system(),
                &mut raw,
            )
        };
        if let Err(error) = check(status) {
            unsafe {
                drop(Box::from_raw(owner));
            }
            return Err(error);
        }
        Ok(Self(NonNull::new(raw).ok_or_else(|| {
            Error::invalid("IREE returned a null file handle")
        })?))
    }

    fn open(path: &str) -> Result<Self> {
        if path.as_bytes().contains(&0) {
            return Err(Error::invalid("native parameter paths cannot contain NUL"));
        }
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_io_file_handle_open(
                (sys::IREE_IO_FILE_MODE_READ | sys::IREE_IO_FILE_MODE_SHARE_READ) as _,
                string_view(path),
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Self(NonNull::new(raw).ok_or_else(|| {
            Error::invalid("IREE returned a null file handle")
        })?))
    }
}

/// Owned metadata for one entry in a native parameter index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterInfo {
    /// Scope-local parameter key.
    pub key: String,
    /// Opaque format-specific metadata, copied out of native storage.
    pub metadata: Vec<u8>,
    /// Total parameter size in bytes.
    pub byte_length: u64,
    /// Whether the entry is represented by a repeating byte pattern.
    pub is_splat: bool,
}

/// An insert-only native parameter index retaining all backing file storage.
///
/// Byte-backed parameters take ownership of initialized Rust storage. Native
/// references retain that allocation even after this Rust index is dropped.
/// File loading uses the native GGUF, IRPA, or safetensors parser.
pub struct ParameterIndex {
    raw: NonNull<sys::iree_io_parameter_index_t>,
    _thread_local: PhantomData<Rc<()>>,
}

impl ParameterIndex {
    /// Create an empty index.
    pub fn new() -> Result<Self> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_io_parameter_index_create(sys::iree_allocator_system(), &mut raw)
        })?;
        Ok(Self {
            raw: NonNull::new(raw)
                .ok_or_else(|| Error::invalid("IREE returned a null parameter index"))?,
            _thread_local: PhantomData,
        })
    }

    /// Number of indexed parameters.
    pub fn len(&self) -> usize {
        unsafe { sys::iree_io_parameter_index_count(self.as_raw()) }
    }
    /// Whether the index contains no parameters.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Reserve native index capacity.
    pub fn reserve(&mut self, capacity: usize) -> Result<()> {
        check(unsafe { sys::iree_io_parameter_index_reserve(self.as_raw(), capacity) })
    }

    fn ensure_absent(&self, key: &str) -> Result<()> {
        match self.lookup(key) {
            Ok(_) => Err(Error::new(
                StatusCode::ALREADY_EXISTS,
                format!("parameter {key:?} already exists"),
            )),
            Err(error) if error.code() == StatusCode::NOT_FOUND => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Insert initialized, read-only bytes without copying their allocation.
    ///
    /// Keys must be unique. Metadata is copied by the native index.
    pub fn add_bytes(&mut self, key: &str, bytes: Vec<u8>, metadata: &[u8]) -> Result<()> {
        self.ensure_absent(key)?;
        let length = bytes.len() as u64;
        let file = File::bytes(bytes)?;
        let entry = sys::iree_io_parameter_index_entry_t {
            key: string_view(key),
            metadata: sys::iree_const_byte_span_t {
                data: metadata.as_ptr(),
                data_length: metadata.len(),
            },
            length,
            type_: sys::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_FILE,
            storage: sys::iree_io_parameter_index_entry_t__bindgen_ty_1 {
                file: sys::iree_io_parameter_index_entry_t__bindgen_ty_1__bindgen_ty_2 {
                    handle: file.0.as_ptr(),
                    offset: 0,
                },
            },
        };
        check(unsafe { sys::iree_io_parameter_index_add(self.as_raw(), &entry) })
    }

    /// Insert a repeated little-endian pattern without allocating tensor data.
    ///
    /// Pattern length must be a power of two in `1..=16`. The total byte
    /// length must contain a whole number of pattern repetitions.
    pub fn add_splat(&mut self, key: &str, pattern: &[u8], byte_length: u64) -> Result<()> {
        if pattern.is_empty()
            || pattern.len() > 16
            || !pattern.len().is_power_of_two()
            || !byte_length.is_multiple_of(pattern.len() as u64)
        {
            return Err(Error::invalid(
                "splat requires a 1/2/4/8/16-byte pattern and a whole number of repetitions",
            ));
        }
        self.ensure_absent(key)?;
        let mut bytes = [0; 16];
        bytes[..pattern.len()].copy_from_slice(pattern);
        let entry = sys::iree_io_parameter_index_entry_t {
            key: string_view(key),
            length: byte_length,
            type_: sys::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_SPLAT,
            storage: sys::iree_io_parameter_index_entry_t__bindgen_ty_1 {
                splat: sys::iree_io_parameter_index_entry_t__bindgen_ty_1__bindgen_ty_1 {
                    pattern_length: pattern.len() as u8,
                    pattern: bytes,
                },
            },
            ..Default::default()
        };
        check(unsafe { sys::iree_io_parameter_index_add(self.as_raw(), &entry) })
    }

    /// Append a native archive's entries from owned bytes.
    ///
    /// `format` is `"gguf"`, `"irpa"`, or `"safetensors"`. Parse into a new
    /// index when atomic replacement is required: the native parser may have
    /// inserted entries before reporting malformed later input.
    pub fn load_bytes(&mut self, format: &str, bytes: Vec<u8>) -> Result<()> {
        let file = File::bytes(bytes)?;
        self.parse(format, &file)
    }

    /// Open a read-only archive and index it without eagerly copying its data.
    ///
    /// The extension selects the native format parser. The index/provider
    /// retains the opened handle, not merely its filesystem path.
    pub fn load_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::invalid("native parameter paths must be UTF-8"))?;
        let file = File::open(path)?;
        self.parse(path, &file)
    }

    fn parse(&mut self, path: &str, file: &File) -> Result<()> {
        check(unsafe {
            sys::iree_io_parse_file_index(
                string_view(path),
                file.0.as_ptr(),
                self.as_raw(),
                sys::iree_allocator_system(),
            )
        })
    }

    /// Look up copied metadata without exposing native pointers.
    pub fn lookup(&self, key: &str) -> Result<ParameterInfo> {
        let mut entry = ptr::null();
        check(unsafe {
            sys::iree_io_parameter_index_lookup(self.as_raw(), string_view(key), &mut entry)
        })?;
        Ok(unsafe { Self::info(&*entry) })
    }

    /// Copy metadata for every indexed entry.
    pub fn entries(&self) -> Result<Vec<ParameterInfo>> {
        (0..self.len())
            .map(|index| {
                let mut entry = ptr::null();
                check(unsafe {
                    sys::iree_io_parameter_index_get(self.as_raw(), index, &mut entry)
                })?;
                Ok(unsafe { Self::info(&*entry) })
            })
            .collect()
    }

    unsafe fn info(entry: &sys::iree_io_parameter_index_entry_t) -> ParameterInfo {
        ParameterInfo {
            key: unsafe { crate::driver::owned_string(entry.key) },
            metadata: if entry.metadata.data_length == 0 {
                Vec::new()
            } else {
                unsafe {
                    std::slice::from_raw_parts(entry.metadata.data, entry.metadata.data_length)
                }
                .to_vec()
            },
            byte_length: entry.length,
            is_splat: entry.type_ == sys::IREE_IO_PARAMETER_INDEX_ENTRY_STORAGE_TYPE_SPLAT,
        }
    }

    /// Create a scoped native provider retaining this index and its files.
    ///
    /// `max_concurrent_operations` limits native I/O fan-out and must be nonzero.
    pub fn provider(
        &self,
        scope: &str,
        max_concurrent_operations: usize,
    ) -> Result<ParameterProvider> {
        if max_concurrent_operations == 0 {
            return Err(Error::invalid(
                "parameter provider concurrency must be nonzero",
            ));
        }
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_io_parameter_index_provider_create(
                string_view(scope),
                self.as_raw(),
                max_concurrent_operations,
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(ParameterProvider {
            raw: NonNull::new(raw)
                .ok_or_else(|| Error::invalid("IREE returned a null parameter provider"))?,
            _thread_local: PhantomData,
        })
    }

    /// Borrow the native index without transferring ownership.
    pub fn as_raw(&self) -> *mut sys::iree_io_parameter_index_t {
        self.raw.as_ptr()
    }
}

impl Drop for ParameterIndex {
    fn drop(&mut self) {
        unsafe { sys::iree_io_parameter_index_release(self.as_raw()) };
    }
}

/// A scoped provider of read-only file-backed or splat parameters.
///
/// Providers remain thread-local: upstream does not define overlapping
/// parameter read/write behavior across concurrent VM contexts.
pub struct ParameterProvider {
    raw: NonNull<sys::iree_io_parameter_provider_t>,
    _thread_local: PhantomData<Rc<()>>,
}

impl Clone for ParameterProvider {
    fn clone(&self) -> Self {
        unsafe { sys::iree_io_parameter_provider_retain(self.as_raw()) };
        Self {
            raw: self.raw,
            _thread_local: PhantomData,
        }
    }
}

/// Native parameter-provider lifecycle notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterSignal {
    /// Resume after suspension; resources may be reallocated.
    Resume,
    /// Suspend and release transient resources.
    Suspend,
    /// Aggressively trim caches in response to memory pressure.
    LowMemory,
}

impl ParameterProvider {
    /// Whether this provider recognizes a scope.
    pub fn supports(&self, scope: &str) -> bool {
        unsafe { sys::iree_io_parameter_provider_query_support(self.as_raw(), string_view(scope)) }
    }
    /// Notify the provider of host lifecycle or memory-pressure events.
    pub fn notify(&self, signal: ParameterSignal) -> Result<()> {
        let signal = match signal {
            ParameterSignal::Resume => sys::IREE_IO_PARAMETER_PROVIDER_SIGNAL_RESUME,
            ParameterSignal::Suspend => sys::IREE_IO_PARAMETER_PROVIDER_SIGNAL_SUSPEND,
            ParameterSignal::LowMemory => sys::IREE_IO_PARAMETER_PROVIDER_SIGNAL_LOW_MEMORY,
        };
        check(unsafe { sys::iree_io_parameter_provider_notify(self.as_raw(), signal) })
    }
    /// Borrow the native provider without transferring ownership.
    pub fn as_raw(&self) -> *mut sys::iree_io_parameter_provider_t {
        self.raw.as_ptr()
    }
}

impl Drop for ParameterProvider {
    fn drop(&mut self) {
        unsafe { sys::iree_io_parameter_provider_release(self.as_raw()) };
    }
}
