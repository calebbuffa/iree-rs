//! Command-line entry points embedded in the compiler library.

use crate::{Compiler, Error, Result, cstring};
use iree_native as n;
use std::ffi::{c_char, c_int};

fn run(
    args: &[&str],
    entry: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
) -> Result<i32> {
    let argc = c_int::try_from(args.len()).map_err(|_| Error::ArgumentCount)?;
    let mut strings = args
        .iter()
        .map(|arg| cstring(arg).map(|arg| arg.into_bytes_with_nul()))
        .collect::<Result<Vec<_>>>()?;
    let mut pointers: Vec<*mut c_char> = strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr().cast())
        .collect();
    pointers.push(std::ptr::null_mut());
    let _guard = super::lock();
    // SAFETY: argv pointers refer to writable NUL-terminated buffers that
    // remain alive for the entire call, and argc matches the pointer array.
    Ok(unsafe { entry(argc, pointers.as_mut_ptr()) })
}

/// Run the embedded `iree-compile` command with C-style arguments.
///
/// Include the program name as the first argument, as with `main`'s `argv`.
pub fn run_main(args: &[&str]) -> Result<i32> {
    let _compiler = Compiler::new()?;
    run(args, n::ireeCompilerRunMain)
}

/// Run the embedded LLD command with C-style arguments.
///
/// Include the program name as the first argument, as with `main`'s `argv`.
pub fn run_lld_main(args: &[&str]) -> Result<i32> {
    let _compiler = Compiler::new()?;
    run(args, n::ireeCompilerRunLldMain)
}
