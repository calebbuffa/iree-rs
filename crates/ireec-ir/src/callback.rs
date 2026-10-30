//! Panic containment and text collection for MLIR C callbacks.
use iree_native as n;
use std::{
    ffi::c_void,
    io::{self, Write},
    ptr,
};

pub(crate) use n::contain;

struct Sink<'a> {
    writer: &'a mut dyn Write,
    error: Option<io::Error>,
}

unsafe extern "C" fn write_chunk(chunk: n::MlirStringRef, data: *mut c_void) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
    let sink = unsafe { &mut *data.cast::<Sink<'_>>() };
    if sink.error.is_some() || chunk.length == 0 {
        return;
    }
    let writer = &mut sink.writer;
    let written = contain(|| {
        if chunk.data.is_null() || chunk.length > isize::MAX as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MLIR supplied an invalid text chunk",
            ));
        }
        // SAFETY: the callback rejects null pointers and oversized lengths before forming this borrowed slice.
        let bytes = unsafe { n::slice(chunk.data.cast::<u8>(), chunk.length) };
        writer.write_all(bytes)
    });
    sink.error = match written {
        Some(Ok(())) => None,
        Some(Err(error)) => Some(error),
        None => Some(io::Error::other("writer panicked during MLIR printing")),
    };
}

/// Streams MLIR printer output to `writer`, returning `run`'s result and any write error.
pub(crate) fn print_to<T>(
    writer: &mut dyn Write,
    run: impl FnOnce(n::MlirStringCallback, *mut c_void) -> T,
) -> (T, io::Result<()>) {
    let mut sink = Sink {
        writer,
        error: None,
    };
    let value = run(Some(write_chunk), ptr::from_mut(&mut sink).cast());
    (value, sink.error.map_or(Ok(()), Err))
}

/// Collects MLIR printer output as text, returning `run`'s result alongside it.
pub(crate) fn capture<T>(run: impl FnOnce(n::MlirStringCallback, *mut c_void) -> T) -> (T, String) {
    let mut bytes = Vec::new();
    // Writing to a `Vec` cannot fail; invalid chunks are skipped.
    let (value, _) = print_to(&mut bytes, run);
    (value, String::from_utf8_lossy(&bytes).into_owned())
}

/// Collects MLIR printer output as text.
pub(crate) fn print_with(run: impl FnOnce(n::MlirStringCallback, *mut c_void)) -> String {
    capture(run).1
}
