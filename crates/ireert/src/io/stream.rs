//! Streams over files and host memory.
//!
//! Laws:
//! - `Stream<'a>` never outlives memory it borrows; owned sources are `'static`.
//! - Mapped slices are only exposed for memory streams this wrapper owns
//!   exclusively, and are bounded by `&mut self`.
use super::file_handle::{FileAccess, FileHandle};
use crate::hal::bitfield::bitfield;
use crate::{Code, Error, Result, check};
use core::marker::PhantomData;
use iree_native as n;
use std::io::{self, Read, Seek, SeekFrom, Write};
#[cfg(unix)]
use std::os::fd::{AsRawFd, BorrowedFd};

bitfield! {
    /// Stream capabilities.
    StreamMode: u32 {
        /// Reading is permitted.
        READABLE = n::IREE_IO_STREAM_MODE_READABLE,
        /// Writing is permitted.
        WRITABLE = n::IREE_IO_STREAM_MODE_WRITABLE,
        /// Stream storage can grow.
        RESIZABLE = n::IREE_IO_STREAM_MODE_RESIZABLE,
        /// Arbitrary seeking is permitted.
        SEEKABLE = n::IREE_IO_STREAM_MODE_SEEKABLE,
        /// Stream data can be mapped.
        MAPPABLE = n::IREE_IO_STREAM_MODE_MAPPABLE,
    }
}

bitfield! {
    /// Standard-I/O stream opening modes.
    StdioMode: u32 {
        /// Discards all written data.
        DISCARD = n::IREE_IO_STDIO_STREAM_MODE_DISCARD,
        /// Opens for reading.
        READ = n::IREE_IO_STDIO_STREAM_MODE_READ,
        /// Opens for writing.
        WRITE = n::IREE_IO_STDIO_STREAM_MODE_WRITE,
        /// Opens for append.
        APPEND = n::IREE_IO_STDIO_STREAM_MODE_APPEND,
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Kind {
    Other,
    /// A memory stream this wrapper owns exclusively.
    Memory,
    /// An `iree_io_vec_stream_t`.
    Vector,
}

/// An owned `iree_io_stream_t` reference.
pub struct Stream<'a> {
    raw: *mut n::iree_io_stream_t,
    kind: Kind,
    _memory: PhantomData<&'a mut [u8]>,
}

impl Stream<'static> {
    /// Creates a growable vector-backed stream.
    pub fn vector(mode: StreamMode, block_size: usize) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: allocator and output pointer are valid for the call.
        check(unsafe {
            n::iree_io_vec_stream_create(mode.bits(), block_size, crate::system(), &mut raw)
        })?;
        // SAFETY: success returns one owned reference.
        let mut stream = unsafe { Self::from_raw(raw) };
        stream.kind = Kind::Vector;
        Ok(stream)
    }

    /// Opens a standard I/O file stream at `path`.
    pub fn open_stdio(path: impl AsRef<std::path::Path>, mode: StdioMode) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view, allocator, and output pointer are valid for the call.
        check(unsafe {
            n::iree_io_stdio_stream_open(mode.bits(), crate::view(path), crate::system(), &mut raw)
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Opens a standard I/O stream over a duplicate of `fd`.
    #[cfg(unix)]
    pub fn open_stdio_fd(fd: BorrowedFd<'_>, mode: StdioMode) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: native duplicates the descriptor, which is live for the call.
        check(unsafe {
            n::iree_io_stdio_stream_open_fd(mode.bits(), fd.as_raw_fd(), crate::system(), &mut raw)
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Wraps immutable bytes in a memory stream; `mode` must not be writable.
    pub fn from_owner<T: AsRef<[u8]> + Send + 'static>(owner: T, mode: StreamMode) -> Result<Self> {
        if mode.contains(StreamMode::WRITABLE) {
            return Err(crate::Error::new(
                crate::Code::InvalidArgument,
                "immutable stream owners cannot be writable",
            ));
        }
        let owner = Box::new(owner);
        let bytes = (*owner).as_ref();
        let span = n::iree_byte_span_t {
            data: bytes.as_ptr().cast_mut(),
            data_length: bytes.len(),
        };
        Self::wrap_owner(mode, span, owner)
    }

    /// Wraps exclusively owned mutable bytes in a memory stream.
    pub fn from_mut_owner<T: AsMut<[u8]> + Send + 'static>(
        owner: T,
        mode: StreamMode,
    ) -> Result<Self> {
        let mut owner = Box::new(owner);
        let bytes = (*owner).as_mut();
        let span = n::iree_byte_span_t {
            data: bytes.as_mut_ptr(),
            data_length: bytes.len(),
        };
        Self::wrap_owner(mode, span, owner)
    }

    fn wrap_owner<T: Send + 'static>(
        mode: StreamMode,
        span: n::iree_byte_span_t,
        owner: Box<T>,
    ) -> Result<Self> {
        let owner = Box::into_raw(owner);
        let mut raw = core::ptr::null_mut();
        // SAFETY: `owner` keeps `span` live until the release callback drops
        // it; native never calls the callback when wrapping fails.
        let status = check(unsafe {
            n::iree_io_memory_stream_wrap(
                mode.bits(),
                span,
                n::iree_io_stream_release_callback_t {
                    fn_: Some(release_owner::<T>),
                    user_data: owner.cast(),
                },
                crate::system(),
                &mut raw,
            )
        });
        if let Err(error) = status {
            // SAFETY: wrapping failed, so ownership never left this function.
            drop(unsafe { Box::from_raw(owner) });
            return Err(error);
        }
        // SAFETY: success returns one owned reference.
        let mut stream = unsafe { Self::from_raw(raw) };
        stream.kind = Kind::Memory;
        Ok(stream)
    }
}

fn stream_position(value: usize, description: &str) -> Result<n::iree_io_stream_pos_t> {
    n::iree_io_stream_pos_t::try_from(value).map_err(|_| {
        Error::new(
            Code::OutOfRange,
            format!("{description} exceeds IREE's stream range"),
        )
    })
}

unsafe extern "C" fn release_owner<T>(
    user_data: *mut core::ffi::c_void,
    _stream: *mut n::iree_io_stream_t,
) {
    // SAFETY: native calls this once on destroy with the boxed owner.
    n::contain(|| drop(unsafe { Box::from_raw(user_data.cast::<T>()) }));
}

impl<'a> Stream<'a> {
    /// Opens a stream over `handle` at `file_offset`; native retains the handle.
    ///
    /// Read and write capabilities must be permitted by the handle.
    pub fn open(handle: &FileHandle<'a>, mode: StreamMode, file_offset: u64) -> Result<Self> {
        let mut access = FileAccess::default();
        if mode.contains(StreamMode::READABLE) {
            access |= FileAccess::READ;
        }
        if mode.contains(StreamMode::WRITABLE) {
            access |= FileAccess::WRITE;
        }
        handle.require_access(access)?;
        let mut raw = core::ptr::null_mut();
        // SAFETY: the handle is live and output storage is writable.
        check(unsafe {
            n::iree_io_stream_open(
                mode.bits(),
                handle.as_raw(),
                file_offset,
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Wraps a mutable host buffer for the buffer's lifetime.
    pub fn memory(buffer: &'a mut [u8], mode: StreamMode) -> Result<Self> {
        let span = n::iree_byte_span_t {
            data: buffer.as_mut_ptr(),
            data_length: buffer.len(),
        };
        let mut raw = core::ptr::null_mut();
        // SAFETY: the stream lifetime is tied to the exclusive borrow.
        check(unsafe {
            n::iree_io_memory_stream_wrap(
                mode.bits(),
                span,
                n::iree_io_stream_release_callback_null(),
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned reference.
        let mut stream = unsafe { Self::from_raw(raw) };
        stream.kind = Kind::Memory;
        Ok(stream)
    }

    /// Adopts one owned stream reference.
    ///
    /// # Safety
    /// `raw` must be an owned stream reference valid for `'a`.
    pub unsafe fn from_raw(raw: *mut n::iree_io_stream_t) -> Self {
        Self {
            raw,
            kind: Kind::Other,
            _memory: PhantomData,
        }
    }

    /// Borrows the native stream.
    pub fn as_raw(&self) -> *mut n::iree_io_stream_t {
        self.raw
    }

    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_io_stream_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Returns the stream capabilities.
    pub fn mode(&self) -> StreamMode {
        // SAFETY: `self` owns a live stream.
        StreamMode::from_bits_retain(unsafe { n::iree_io_stream_mode(self.raw) })
    }

    /// Returns the cursor position.
    pub fn offset(&self) -> u64 {
        // SAFETY: `self` owns a live stream; positions are non-negative.
        unsafe { n::iree_io_stream_offset(self.raw) as u64 }
    }

    /// Returns the stream length.
    pub fn length(&self) -> u64 {
        // SAFETY: `self` owns a live stream; lengths are non-negative.
        unsafe { n::iree_io_stream_length(self.raw) as u64 }
    }

    /// Returns whether the cursor is at end-of-stream.
    pub fn is_eos(&self) -> bool {
        // SAFETY: `self` owns a live stream.
        unsafe { n::iree_io_stream_is_eos(self.raw) }
    }

    /// Advances the cursor to the next multiple of `alignment`.
    pub fn seek_to_alignment(&mut self, alignment: usize) -> Result<()> {
        let alignment = stream_position(alignment, "alignment")?;
        // SAFETY: `self` owns a live stream.
        check(unsafe { n::iree_io_stream_seek_to_alignment(self.raw, alignment) })
    }

    /// Writes `pattern` `count` times.
    pub fn fill(&mut self, count: usize, pattern: &[u8]) -> Result<()> {
        let count = stream_position(count, "fill count")?;
        // SAFETY: the pattern is valid for the call.
        check(unsafe {
            n::iree_io_stream_fill(self.raw, count, pattern.as_ptr().cast(), pattern.len())
        })
    }

    /// Borrows the next `length` bytes of a memory stream and advances the cursor.
    pub fn map_read(&mut self, length: usize) -> Result<&[u8]> {
        self.require_exclusive_memory()?;
        let mut span = n::iree_const_byte_span_t::default();
        // SAFETY: `self` owns a live stream and output storage is writable.
        check(unsafe { n::iree_io_stream_map_read(self.raw, length, &mut span) })?;
        // SAFETY: the bytes live as long as this exclusively owned stream and
        // the borrow of `self` excludes every other access.
        Ok(unsafe { n::slice(span.data, span.data_length) })
    }

    /// Mutably borrows the next `length` bytes of a memory stream and
    /// advances the cursor.
    pub fn map_write(&mut self, length: usize) -> Result<&mut [u8]> {
        self.require_exclusive_memory()?;
        let mut span = n::iree_byte_span_t::default();
        // SAFETY: `self` owns a live stream and output storage is writable.
        check(unsafe { n::iree_io_stream_map_write(self.raw, length, &mut span) })?;
        // SAFETY: as for `map_read`; native checked write access.
        Ok(unsafe { n::slice_mut(span.data, span.data_length) })
    }

    fn require_exclusive_memory(&self) -> Result<()> {
        if self.kind == Kind::Memory {
            Ok(())
        } else {
            Err(crate::Error::new(
                crate::Code::FailedPrecondition,
                "mapping requires an exclusively owned memory stream",
            ))
        }
    }

    /// Reads exactly `buffer.len()` bytes or returns an error without advancing
    /// the cursor when the full range is unavailable.
    pub fn read_exact_atomic(&mut self, buffer: &mut [u8]) -> Result<()> {
        // SAFETY: the buffer is writable for its length; a null output count requests
        // the native all-or-error read behavior.
        check(unsafe {
            n::iree_io_stream_read(
                self.raw,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
                core::ptr::null_mut(),
            )
        })
    }

    /// Copies `length` bytes from this stream into `target`.
    pub fn copy_to(&mut self, target: &mut Stream<'_>, length: usize) -> Result<()> {
        let length = stream_position(length, "copy length")?;
        // SAFETY: both streams are live.
        check(unsafe { n::iree_io_stream_copy(self.raw, target.raw, length) })
    }

    /// Writes `length` bytes of `source` starting at `offset`.
    pub fn write_file(
        &mut self,
        source: &FileHandle<'_>,
        offset: u64,
        length: usize,
    ) -> Result<()> {
        source.require_access(FileAccess::READ)?;
        let length = stream_position(length, "file write length")?;
        // SAFETY: the stream and handle are live for the call.
        check(unsafe {
            n::iree_io_stream_write_file(self.raw, source.as_raw(), offset, length, crate::system())
        })
    }

    /// Visits each block of a stream created by [`Stream::vector`] in order.
    pub fn for_each_block(&self, mut visit: impl FnMut(&[u8])) -> Result<()> {
        unsafe extern "C" fn trampoline(
            user_data: *mut core::ffi::c_void,
            block: n::iree_const_byte_span_t,
        ) -> n::iree_status_t {
            n::callback_status(|| {
                // SAFETY: `user_data` is the live `&mut dyn FnMut` below and
                // the block is readable for the callback.
                let visit = unsafe { &mut *user_data.cast::<&mut dyn FnMut(&[u8])>() };
                // SAFETY: The native visitor supplies this readable block only for the synchronous callback.
                visit(unsafe { n::slice(block.data, block.data_length) });
                Ok(())
            })
        }
        if self.kind != Kind::Vector {
            return Err(crate::Error::new(
                crate::Code::FailedPrecondition,
                "block enumeration requires a vector stream",
            ));
        }
        let mut visit: &mut dyn FnMut(&[u8]) = &mut visit;
        // SAFETY: this is a vector stream; the callback runs synchronously.
        check(unsafe {
            n::iree_io_vec_stream_enumerate_blocks(
                self.raw,
                Some(trampoline),
                (&mut visit as *mut &mut dyn FnMut(&[u8])).cast(),
            )
        })
    }
}

impl Drop for Stream<'_> {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one reference.
        unsafe { n::iree_io_stream_release(self.raw) }
    }
}

impl Read for Stream<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        // SAFETY: the buffer is writable for its length.
        check(unsafe {
            n::iree_io_stream_read(
                self.raw,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
                &mut read,
            )
        })
        .map_err(io::Error::other)?;
        Ok(read)
    }
}

impl Write for Stream<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        // SAFETY: the buffer is readable for its length.
        check(unsafe { n::iree_io_stream_write(self.raw, buffer.len(), buffer.as_ptr().cast()) })
            .map_err(io::Error::other)?;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for Stream<'_> {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let (mode, offset) = match position {
            SeekFrom::Start(value) => (
                n::IREE_IO_STREAM_SEEK_SET,
                i64::try_from(value).map_err(io::Error::other)?,
            ),
            SeekFrom::Current(value) => (n::IREE_IO_STREAM_SEEK_FROM_CURRENT, value),
            SeekFrom::End(value) => (n::IREE_IO_STREAM_SEEK_FROM_END, value),
        };
        // SAFETY: `self` owns a live stream.
        check(unsafe { n::iree_io_stream_seek(self.raw, mode, offset) })
            .map_err(io::Error::other)?;
        Ok(self.offset())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const MODE: StreamMode = StreamMode::from_bits_retain(
        StreamMode::READABLE.bits()
            | StreamMode::WRITABLE.bits()
            | StreamMode::RESIZABLE.bits()
            | StreamMode::SEEKABLE.bits(),
    );

    #[test]
    fn stream_positions_are_checked_before_ffi() {
        assert_eq!(stream_position(0, "length").unwrap(), 0);
        assert_eq!(
            stream_position(i64::MAX as usize, "length").unwrap(),
            i64::MAX
        );
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            stream_position(usize::MAX, "length").unwrap_err().code(),
            Code::OutOfRange
        );
    }

    proptest! {
        #[test]
        fn write_then_read_round_trips(bytes in proptest::collection::vec(any::<u8>(), 0..2048)) {
            let mut stream = Stream::vector(MODE, 64).unwrap();
            stream.write_all(&bytes).unwrap();
            prop_assert_eq!(stream.offset(), bytes.len() as u64);
            stream.seek(SeekFrom::Start(0)).unwrap();
            let mut actual = vec![0; bytes.len()];
            stream.read_exact(&mut actual).unwrap();
            prop_assert_eq!(actual, bytes);
        }

        #[test]
        fn blocks_concatenate_to_contents(bytes in proptest::collection::vec(any::<u8>(), 0..512)) {
            let mut stream = Stream::vector(MODE, 32).unwrap();
            stream.write_all(&bytes).unwrap();
            let mut joined = Vec::new();
            stream.for_each_block(|block| joined.extend_from_slice(block)).unwrap();
            prop_assert_eq!(joined, bytes);
        }
    }
}
