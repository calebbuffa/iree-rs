//! Safe, owned Rust interfaces for IREE's in-process compiler.
//!
//! [`Compiler`] creates compiler sessions; [`Invocation`] parses sources,
//! selects pipelines, and emits [`Output`]. Native resources are released on
//! drop. Compiler calls are serialized because upstream compiler state is
//! process-global.
//!
//! The unpublished `ireec-ir` crate provides in-process MLIR bindings when IR
//! construction or Rust-authored passes are needed. Compiler output is
//! executable code: compile and run only trusted input. See the crate README
//! for feature selection and native deployment requirements.
#![deny(missing_docs, clippy::all)]

mod global_options;
mod invocation;
mod options;
mod output;
mod session;
mod source;
#[cfg(feature = "tools")]
mod tools;

pub use global_options::GlobalOptions;
pub use invocation::{Diagnostic, Invocation, Phase, Pipeline, Severity};
pub use iree_native as native;
pub use options::{
    Device, InputConversion, InputType, OptLevel, Options, OutputFormat, Target, VmBytecodeFormat,
    VmCFormat,
};
pub use output::Output;
pub use session::Session;
pub use source::Source;
#[cfg(feature = "tools")]
pub use tools::{run_lld_main, run_main};

use iree_native as n;
use std::{
    ffi::{CStr, CString, c_char, c_int, c_void},
    path::Path,
    ptr,
    sync::{Mutex, MutexGuard, Once},
};
#[cfg(all(windows, target_env = "msvc"))]
use std::{path::PathBuf, sync::OnceLock};

/// Compiler calls are serialized process-wide; the C API's global state is
/// not documented as thread-safe.
static LOCK: Mutex<()> = Mutex::new(());
static INIT: Once = Once::new();

pub(crate) fn lock() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn format_compile_diagnostics(diagnostics: &[Diagnostic]) -> String {
    let mut message = String::new();
    for diagnostic in diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Error)
    {
        message.push_str("\n  ");
        message.push_str(&diagnostic.message);
    }
    message
}

/// Compilation failure.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The compiler API itself reported an error.
    #[error(transparent)]
    Native(#[from] n::Error),
    /// The input was rejected or a pass failed; carries the diagnostics.
    #[error("compilation failed{}", format_compile_diagnostics(.0))]
    Compile(Vec<Diagnostic>),
    /// An argument contained an interior NUL byte.
    #[error("argument contained an interior NUL byte")]
    Nul,
    /// The argument count exceeded the C `int` range.
    #[error("too many command-line arguments")]
    ArgumentCount,
    /// The bytecode version exceeded the C `int` range.
    #[error("bytecode version exceeds the native range")]
    BytecodeVersion,
    /// A typed compiler option is invalid or conflicting.
    #[error("invalid compiler options: {0}")]
    InvalidOptions(String),
    /// The compiler image has not registered a requested target backend.
    #[error("compiler target backend is not registered: {0}")]
    UnregisteredBackend(String),
    /// Process-wide compiler options were already configured differently.
    #[error("process-wide compiler options are already configured differently")]
    ConflictingGlobalOptions,
    /// A path was not valid UTF-8.
    #[error("path is not valid UTF-8")]
    Path,
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn cstring(s: &str) -> Result<CString> {
    CString::new(s).map_err(|_| Error::Nul)
}

pub(crate) fn path_cstring(path: &Path) -> Result<CString> {
    cstring(path.to_str().ok_or(Error::Path)?)
}

/// Take ownership of a compiler error, releasing it.
///
/// # Safety
/// `error` is null or an owned `iree_compiler_error_t`.
pub(crate) unsafe fn take(error: *mut n::iree_compiler_error_t) -> Result<()> {
    if error.is_null() {
        return Ok(());
    }
    let message = {
        let _guard = lock();
        // SAFETY: error is an owned compiler error whose API exposes a NUL-terminated message.
        let raw_message = unsafe { n::ireeCompilerErrorGetMessage(error) };
        let message = if raw_message.is_null() {
            "compiler returned an error without a message".to_owned()
        } else {
            // SAFETY: the error API returns a NUL-terminated message when non-null.
            unsafe { CStr::from_ptr(raw_message) }
                .to_string_lossy()
                .into_owned()
        };
        // SAFETY: take consumes this owned compiler error exactly once.
        unsafe { n::ireeCompilerErrorDestroy(error) };
        message
    };
    Err(n::Error::new(n::Code::Unknown, message).into())
}

/// Collect a C enumeration callback into a vector of strings.
pub(crate) fn collect_strings(
    run: impl FnOnce(Option<unsafe extern "C" fn(*const c_char, *mut c_void)>, *mut c_void),
) -> Vec<String> {
    unsafe extern "C" fn push(name: *const c_char, out: *mut c_void) {
        if name.is_null() {
            return;
        }
        // SAFETY: `collect_strings` passes its vector; `name` is NUL-terminated.
        let (out, name) = unsafe { (&mut *out.cast::<Vec<String>>(), CStr::from_ptr(name)) };
        n::contain(|| out.push(name.to_string_lossy().into_owned()));
    }
    let mut names = Vec::new();
    run(Some(push), ptr::from_mut(&mut names).cast());
    names
}

/// Copy a native (pointer, length) text span, replacing invalid UTF-8.
///
/// # Safety
/// A non-null `text` must point to `len` readable bytes.
pub(crate) unsafe fn lossy(text: *const c_char, len: usize) -> Option<String> {
    let bytes = match len {
        0 => &[][..],
        _ if text.is_null() => return None,
        // SAFETY: guaranteed by the caller.
        _ => unsafe { n::slice(text.cast::<u8>(), len) },
    };
    Some(String::from_utf8_lossy(bytes).into_owned())
}

/// Argument vector handed to `ireeCompilerSetupGlobalCL`; the compiler keeps
/// it until shutdown, which never happens, so every vector is kept for the
/// life of the process, including those of earlier calls.
struct Argv {
    _strings: Vec<CString>,
    _banner: CString,
    _pointers: Vec<*const c_char>,
}

// SAFETY: the pointers refer into the owned `CString`s alongside them and are
// never read or written from Rust after being stored.
unsafe impl Send for Argv {}

static ARGV: Mutex<Vec<Argv>> = Mutex::new(Vec::new());

/// Handle to the process-wide compiler.
///
/// The compiler is initialised on first use and never shut down: shutdown
/// permanently disables it for the process.
#[derive(Clone, Copy, Debug)]
pub struct Compiler(());

impl Compiler {
    /// Load and initialize the compiler selected by the build.
    ///
    /// On Windows, the runtime DLL is resolved from `IREE_COMPILER_RUNTIME_DIR`,
    /// the executable's adjacent `iree-native/<content-id>` directory, or the
    /// build-local deployment, in that order.
    pub fn new() -> Result<Compiler> {
        load_compiler_runtime()?;
        INIT.call_once(|| {
            let _guard = lock();
            // SAFETY: called once, before any other compiler call.
            unsafe { n::ireeCompilerGlobalInitialize() }
        });
        Ok(Compiler(()))
    }

    /// The compiler's API version as `(major, minor)`.
    pub fn api_version(&self) -> (u16, u16) {
        let _guard = lock();
        // SAFETY: the version query takes no arguments and only reads constants.
        let v = unsafe { n::ireeCompilerGetAPIVersion() } as u32;
        ((v >> 16) as u16, v as u16)
    }

    /// The build revision: the release tag, or empty for development builds.
    pub fn revision(&self) -> String {
        let _guard = lock();
        // SAFETY: compiler is initialized and the revision query returns a static C string.
        let raw = unsafe { n::ireeCompilerGetRevision() };
        if raw.is_null() {
            return String::new();
        }
        // SAFETY: the revision API returns a static NUL-terminated string.
        unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned()
    }

    /// Names of the registered HAL target backends.
    pub fn target_backends(&self) -> Vec<String> {
        let _guard = lock();
        // SAFETY: enumeration is synchronous and collect_strings provides a valid callback context.
        collect_strings(|callback, user| unsafe {
            n::ireeCompilerEnumerateRegisteredHALTargetBackends(callback, user)
        })
    }

    /// Names of the plugins linked into the compiler.
    pub fn plugins(&self) -> Vec<String> {
        let _guard = lock();
        // SAFETY: enumeration is synchronous and collect_strings provides a valid callback context.
        collect_strings(|callback, user| unsafe { n::ireeCompilerEnumeratePlugins(callback, user) })
    }

    /// Initialize the process-wide LLVM/MLIR command-line environment from
    /// `args`, as a tool would from its `argv`. `banner` is shown in help
    /// output.
    ///
    /// This is not session configuration: flags parsed here affect process-wide
    /// compiler state and cannot be varied between sessions. Prefer
    /// [`Session::set_flags`] or [`Session::configure`] for session options.
    /// Output selection is performed on the invocation with `output_*`.
    pub fn setup_global_cl(
        &self,
        args: &[&str],
        banner: &str,
        install_signal_handlers: bool,
    ) -> Result<()> {
        let strings = args
            .iter()
            .map(|a| cstring(a))
            .collect::<Result<Vec<_>>>()?;
        let argc = c_int::try_from(strings.len()).map_err(|_| Error::ArgumentCount)?;
        let mut pointers: Vec<*const c_char> = strings.iter().map(|s| s.as_ptr()).collect();
        pointers.push(ptr::null());
        let banner = cstring(banner)?;
        let _guard = lock();
        // SAFETY: all argv strings remain alive and the pointer array is kept for the process lifetime.
        unsafe {
            n::ireeCompilerSetupGlobalCL(
                argc,
                pointers.as_mut_ptr(),
                banner.as_ptr(),
                install_signal_handlers,
            )
        };
        ARGV.lock().unwrap_or_else(|p| p.into_inner()).push(Argv {
            _strings: strings,
            _banner: banner,
            _pointers: pointers,
        });
        Ok(())
    }

    /// A new session with default flags.
    pub fn session(&self) -> Result<Session> {
        Session::create(*self)
    }
}

#[cfg(all(windows, target_env = "msvc"))]
static COMPILER_LOAD: OnceLock<core::result::Result<(), u32>> = OnceLock::new();

#[cfg(all(windows, target_env = "msvc"))]
fn load_compiler_runtime() -> Result<()> {
    use std::os::windows::ffi::OsStrExt;

    let runtime_dir = std::env::var_os("IREE_COMPILER_RUNTIME_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|path| path.parent().map(Path::to_path_buf))
                .map(|path| {
                    path.join("iree-native")
                        .join(n::IREE_COMPILER_DEPLOYMENT_ID)
                })
                .filter(|path| path.join("IREECompiler.dll").is_file())
        })
        .unwrap_or_else(|| PathBuf::from(n::IREE_COMPILER_BIN_DIR));
    let library_path = runtime_dir.join("IREECompiler.dll");
    if !library_path.is_absolute() {
        return Err(n::Error::new(
            n::Code::InvalidArgument,
            "IREE compiler runtime directory must be absolute",
        )
        .into());
    }
    let wide_path = library_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let load_result = COMPILER_LOAD.get_or_init(|| {
        // SAFETY: the path is NUL-terminated UTF-16 and remains live through
        // the synchronous call; the native loader retains its module handle.
        let error = unsafe { n::iree_native_load_compiler(wide_path.as_ptr()) };
        if error == 0 { Ok(()) } else { Err(error) }
    });
    match load_result {
        Ok(()) => Ok(()),
        Err(code) => Err(n::Error::new(
            n::Code::Unavailable,
            format!(
                "failed to load {} (Windows error {code})",
                library_path.display()
            ),
        )
        .into()),
    }
}

#[cfg(not(all(windows, target_env = "msvc")))]
fn load_compiler_runtime() -> Result<()> {
    Ok(())
}
