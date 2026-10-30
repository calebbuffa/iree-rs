use crate::{string_view, view_string};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{ffi::c_void, fmt, mem::ManuallyDrop};

/// Valid five-bit IREE status classification, including future native codes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code(u8);

macro_rules! codes {
    ($($name:ident = $value:expr),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        impl Code {
            $(#[doc = stringify!($name)] pub const $name: Self = Self($value);)*
            /// All known failure codes (success is deliberately excluded).
            pub const ALL: &'static [Self] = &[$(Self::$name),*];
        }
    };
}
codes! {
    Cancelled = 1, Unknown = 2, InvalidArgument = 3, DeadlineExceeded = 4,
    NotFound = 5, AlreadyExists = 6, PermissionDenied = 7, ResourceExhausted = 8,
    FailedPrecondition = 9, Aborted = 10, OutOfRange = 11, Unimplemented = 12,
    Internal = 13, Unavailable = 14, DataLoss = 15, Unauthenticated = 16,
    Deferred = 17, Incompatible = 18,
}
impl Code {
    /// Successful completion, not a valid error classification.
    #[allow(non_upper_case_globals)]
    pub const Ok: Self = Self(0);
    /// Validate a raw code without truncating it to a different classification.
    pub fn from_raw(value: crate::iree_status_code_t) -> Option<Self> {
        (0..=crate::IREE_STATUS_CODE_MASK)
            .contains(&value)
            .then_some(Self(value as u8))
    }
    /// Return the unchanged native classification.
    pub fn to_raw(self) -> crate::iree_status_code_t {
        self.0.into()
    }
    /// Native canonical name, or an empty name for future codes.
    pub fn name(self) -> &'static str {
        // SAFETY: IREE returns a static NUL-terminated name for every status code.
        let name = unsafe { crate::iree_status_code_string(self.to_raw()) };
        // SAFETY: the status-name API guarantees a non-null static C string.
        unsafe { core::ffi::CStr::from_ptr(name) }
            .to_str()
            .unwrap_or("")
    }
}
impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name().is_empty() {
            write!(f, "STATUS_{}", self.0)
        } else {
            f.write_str(self.name())
        }
    }
}
impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// An owned, rendered native diagnostic payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusPayload {
    /// Native payload type identifier, retained even for unknown user payloads.
    pub kind: u32,
    /// Native formatted payload text, not an interpretation of private bytes.
    pub text: String,
}

/// A failure with owned diagnostic text and formatted native payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    code: Code,
    message: String,
    payload: Vec<StatusPayload>,
}
/// Shared binding result type.
pub type Result<T, E = Error> = core::result::Result<T, E>;
impl Error {
    /// Construct a failure; success is rejected as an invalid argument.
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        let code = if code == Code::Ok {
            Code::InvalidArgument
        } else {
            code
        };
        Self {
            code,
            message: message.into(),
            payload: Vec::new(),
        }
    }
    /// Construct a rejected-input error.
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new(Code::InvalidArgument, message)
    }
    /// Machine-readable classification.
    pub fn code(&self) -> Code {
        self.code
    }
    /// Native primary message, excluding payloads.
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Native payloads in attachment order, formatted before releasing C storage.
    pub fn payload(&self) -> &[StatusPayload] {
        &self.payload
    }
    /// Transfer an owned native status to a Rust failure, or `None` for success.
    ///
    /// # Safety
    /// `raw` must be a live owned IREE status and must never be reused afterwards.
    pub unsafe fn from_raw(raw: crate::iree_status_t) -> Option<Self> {
        // SAFETY: this method delegates its live-owned-status precondition unchanged.
        unsafe { consume(raw) }.err()
    }
    /// Transfer this failure into an owned native status for a C callback.
    ///
    /// Formatted payloads are preserved as diagnostic text. IREE exposes no
    /// public API for reconstructing arbitrary user payload types from text.
    pub fn into_status(self) -> crate::iree_status_t {
        let message = if self.payload.is_empty() {
            self.message
        } else {
            self.to_string()
        };
        // SAFETY: all string views borrow live strings for this synchronous allocation call.
        unsafe {
            crate::iree_status_allocate_copy(
                self.code.to_raw(),
                string_view("rust"),
                0,
                string_view(&message),
            )
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)?;
        for payload in &self.payload {
            write!(f, "\n{}", payload.text)?;
        }
        Ok(())
    }
}
impl core::error::Error for Error {}

/// Preserve hosted I/O failures using the shared IREE classification.
#[cfg(not(target_os = "none"))]
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        use std::io::ErrorKind;
        let code = match error.kind() {
            ErrorKind::NotFound => Code::NotFound,
            ErrorKind::PermissionDenied => Code::PermissionDenied,
            ErrorKind::AlreadyExists => Code::AlreadyExists,
            ErrorKind::InvalidInput => Code::InvalidArgument,
            ErrorKind::InvalidData | ErrorKind::UnexpectedEof => Code::DataLoss,
            ErrorKind::TimedOut => Code::DeadlineExceeded,
            ErrorKind::WouldBlock => Code::Unavailable,
            ErrorKind::Interrupted => Code::Aborted,
            ErrorKind::Unsupported => Code::Unimplemented,
            ErrorKind::OutOfMemory => Code::ResourceExhausted,
            _ => Code::Unknown,
        };
        Self::new(code, error.to_string())
    }
}

/// An owned native status. Raw payloads remain intact until transfer or drop.
///
/// This owner is intentionally neither Send nor Sync: foreign payloads can
/// have thread-affine allocation and destruction callbacks.
pub struct Status {
    raw: crate::iree_status_t,
}
impl Status {
    /// Adopt exactly one status, including null success and code-only statuses.
    ///
    /// # Safety
    /// `raw` must be live, exclusively owned, and freeable through this runtime.
    pub unsafe fn from_raw(raw: crate::iree_status_t) -> Self {
        Self { raw }
    }
    /// Borrow without transferring native ownership.
    pub fn as_raw(&self) -> crate::iree_status_t {
        self.raw
    }
    /// Transfer native ownership without losing opaque payloads.
    pub fn into_raw(self) -> crate::iree_status_t {
        ManuallyDrop::new(self).raw
    }
    /// Native classification, including success.
    pub fn code(&self) -> Code {
        Code::from_raw(crate::abi::status::code(self.raw)).unwrap_or(Code::Unknown)
    }
    /// Primary message copied into Rust storage.
    pub fn message(&self) -> String {
        // SAFETY: self owns a live status and its message view remains valid until status release.
        let value = unsafe { crate::iree_status_message(self.raw) };
        // SAFETY: the returned view remains readable while `self` owns the status.
        unsafe { view_string(value) }
    }
    /// Copy formatted payloads in native attachment order.
    pub fn payloads(&self) -> Result<Vec<StatusPayload>> {
        let mut output = Vec::<StatusPayload>::new();
        // SAFETY: self owns the status; the callback context is a live, exclusively borrowed Vec.
        let status = unsafe {
            crate::iree_status_enumerate_payloads(
                self.raw,
                Some(collect_payload),
                (&mut output as *mut Vec<StatusPayload>).cast(),
            )
        };
        // SAFETY: enumerate_payloads returns one owned status for this caller.
        let failure = unsafe { Status::from_raw(status) };
        if failure.code() != Code::Ok {
            // Do not enumerate a visitor failure: repeated allocation failures
            // would recurse through this visitor indefinitely.
            return Err(Error::new(failure.code(), failure.message()));
        }
        Ok(output)
    }
    /// Consume into an owned Rust failure; success produces `None`.
    pub fn into_error(self) -> Option<Error> {
        if self.raw.is_null() {
            return None;
        }
        let mut error = Error::new(self.code(), self.message());
        match self.payloads() {
            Ok(payload) => error.payload = payload,
            Err(failure) => error.payload.push(StatusPayload {
                kind: 0,
                text: failure.to_string(),
            }),
        }
        Some(error)
    }
}
impl Drop for Status {
    fn drop(&mut self) {
        // SAFETY: Status uniquely owns raw and frees it exactly once.
        unsafe {
            crate::iree_status_free(self.raw);
        };
    }
}
impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Status")
            .field("code", &self.code())
            .finish()
    }
}
/// Consume a native status exactly once, returning its Rust diagnostic.
///
/// # Safety
/// `status` must be a valid owned IREE status and must not be used afterwards.
pub unsafe fn consume(status: crate::iree_status_t) -> Result<()> {
    // SAFETY: the caller guarantees status is live, owned, and not reused.
    match unsafe { Status::from_raw(status) }.into_error() {
        None => Ok(()),
        Some(error) => Err(error),
    }
}
/// Execute a callback body without unwinding through C, returning an owned status.
///
/// A panicking panic-payload destructor cannot be run safely at a C boundary;
/// such a payload is deliberately forgotten. With panic=abort Rust still aborts.
#[doc(hidden)]
pub fn callback_status(operation: impl FnOnce() -> Result<()>) -> crate::iree_status_t {
    match contain(operation) {
        Some(Ok(())) => core::ptr::null_mut(),
        Some(Err(error)) => error.into_status(),
        // Return a code-only status without allocating while handling panic.
        None => crate::abi::status::from_code(crate::IREE_STATUS_INTERNAL),
    }
}

/// Runs a Rust callback invoked by native code, returning `None` instead of
/// unwinding across the C boundary. The panic payload is leaked, never dropped,
/// because its destructor could panic again.
pub fn contain<T>(callback: impl FnOnce() -> T) -> Option<T> {
    #[cfg(panic = "unwind")]
    return std::panic::catch_unwind(core::panic::AssertUnwindSafe(callback))
        .map_err(core::mem::forget)
        .ok();
    #[cfg(not(panic = "unwind"))]
    Some(callback())
}
unsafe extern "C" fn collect_payload(
    data: *mut c_void,
    payload: *const crate::iree_status_payload_t,
) -> crate::iree_status_t {
    callback_status(|| {
        let mut length = 0;
        // SAFETY: the payload is valid for this visitor call and the length output is writable.
        unsafe {
            crate::iree_status_payload_format(payload, 0, core::ptr::null_mut(), &mut length)
        };
        let mut capacity = length
            .checked_add(1)
            .ok_or_else(|| Error::new(Code::OutOfRange, "payload size overflow"))?;
        let mut bytes = Vec::new();
        loop {
            bytes
                .try_reserve_exact(capacity - bytes.len())
                .map_err(|_| Error::new(Code::ResourceExhausted, "payload allocation failed"))?;
            bytes.resize(capacity, 0u8);
            // SAFETY: the payload is live; bytes and length are writable for their supplied capacity.
            unsafe {
                crate::iree_status_payload_format(
                    payload,
                    capacity,
                    bytes.as_mut_ptr().cast(),
                    &mut length,
                )
            };
            if length < capacity {
                break;
            }
            // Native lengths exclude the terminator, including on truncation.
            capacity = length
                .checked_add(1)
                .ok_or_else(|| Error::new(Code::OutOfRange, "payload size overflow"))?;
        }
        bytes.truncate(length);
        // SAFETY: enumerate_payloads receives this exact live Vec pointer as callback context.
        let output = unsafe { &mut *data.cast::<Vec<StatusPayload>>() };
        output
            .try_reserve(1)
            .map_err(|_| Error::new(Code::ResourceExhausted, "payload allocation failed"))?;
        output.push(StatusPayload {
            // SAFETY: payload is valid for the duration of this native visitor callback.
            kind: unsafe { crate::iree_status_payload_type(payload) } as u32,
            text: String::from_utf8_lossy(&bytes).into_owned(),
        });
        Ok(())
    })
}
