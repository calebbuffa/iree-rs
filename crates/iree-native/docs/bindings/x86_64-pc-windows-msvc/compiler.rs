use super::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_compiler_error_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_compiler_session_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_compiler_invocation_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_compiler_source_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_compiler_output_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn ireeCompilerErrorDestroy(error: *mut iree_compiler_error_t);
}
unsafe extern "C" {
    pub fn ireeCompilerErrorGetMessage(
        error: *mut iree_compiler_error_t,
    ) -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn ireeCompilerGetAPIVersion() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ireeCompilerGlobalInitialize();
}
unsafe extern "C" {
    pub fn ireeCompilerGetRevision() -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn ireeCompilerGetProcessCLArgs(
        argc: *mut ::core::ffi::c_int,
        argv: *mut *mut *const ::core::ffi::c_char,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerSetupGlobalCL(
        argc: ::core::ffi::c_int,
        argv: *mut *const ::core::ffi::c_char,
        banner: *const ::core::ffi::c_char,
        installSignalHandlers: bool,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerGlobalShutdown();
}
unsafe extern "C" {
    pub fn ireeCompilerEnumerateRegisteredHALTargetBackends(
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                backend: *const ::core::ffi::c_char,
                userData: *mut ::core::ffi::c_void,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerEnumeratePlugins(
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                pluginName: *const ::core::ffi::c_char,
                userData: *mut ::core::ffi::c_void,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerSessionCreate() -> *mut iree_compiler_session_t;
}
unsafe extern "C" {
    pub fn ireeCompilerSessionDestroy(session: *mut iree_compiler_session_t);
}
unsafe extern "C" {
    pub fn ireeCompilerSessionSetFlags(
        session: *mut iree_compiler_session_t,
        argc: ::core::ffi::c_int,
        argv: *const *const ::core::ffi::c_char,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerSessionGetFlags(
        session: *mut iree_compiler_session_t,
        nonDefaultOnly: bool,
        onFlag: ::core::option::Option<
            unsafe extern "C" fn(
                flag: *const ::core::ffi::c_char,
                length: usize,
                arg1: *mut ::core::ffi::c_void,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
pub const IREE_COMPILER_DIAGNOSTIC_SEVERITY_NOTE: iree_compiler_diagnostic_severity_t = 0;
pub const IREE_COMPILER_DIAGNOSTIC_SEVERITY_WARNING: iree_compiler_diagnostic_severity_t = 1;
pub const IREE_COMPILER_DIAGNOSTIC_SEVERITY_ERROR: iree_compiler_diagnostic_severity_t = 2;
pub const IREE_COMPILER_DIAGNOSTIC_SEVERITY_REMARK: iree_compiler_diagnostic_severity_t = 3;
pub type iree_compiler_diagnostic_severity_t = ::core::ffi::c_int;
unsafe extern "C" {
    pub fn ireeCompilerInvocationCreate(
        session: *mut iree_compiler_session_t,
    ) -> *mut iree_compiler_invocation_t;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationEnableCallbackDiagnostics(
        inv: *mut iree_compiler_invocation_t,
        flags: ::core::ffi::c_int,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                severity: iree_compiler_diagnostic_severity_t,
                message: *const ::core::ffi::c_char,
                messageSize: usize,
                userData: *mut ::core::ffi::c_void,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationEnableConsoleDiagnostics(
        inv: *mut iree_compiler_invocation_t,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationDestroy(inv: *mut iree_compiler_invocation_t);
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationSetCrashHandler(
        inv: *mut iree_compiler_invocation_t,
        genLocalReproducer: bool,
        onCrashCallback: ::core::option::Option<
            unsafe extern "C" fn(
                outOutput: *mut *mut iree_compiler_output_t,
                userData: *mut ::core::ffi::c_void,
            ) -> *mut iree_compiler_error_t,
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationParseSource(
        inv: *mut iree_compiler_invocation_t,
        source: *mut iree_compiler_source_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationSetCompileFromPhase(
        inv: *mut iree_compiler_invocation_t,
        phase: *const ::core::ffi::c_char,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationSetCompileToPhase(
        inv: *mut iree_compiler_invocation_t,
        phase: *const ::core::ffi::c_char,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationSetDumpCompilationPhasesTo(
        inv: *mut iree_compiler_invocation_t,
        path: *const ::core::ffi::c_char,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationSetVerifyIR(
        inv: *mut iree_compiler_invocation_t,
        enable: bool,
    );
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationSetupRemarks(
        inv: *mut iree_compiler_invocation_t,
        remarksFilter: *const ::core::ffi::c_char,
        remarksOutputFile: *const ::core::ffi::c_char,
    );
}
pub const IREE_COMPILER_PIPELINE_STD: iree_compiler_pipeline_t = 0;
pub const IREE_COMPILER_PIPELINE_HAL_EXECUTABLE: iree_compiler_pipeline_t = 1;
pub const IREE_COMPILER_PIPELINE_PRECOMPILE: iree_compiler_pipeline_t = 2;
pub const IREE_COMPILER_PIPELINE_VM: iree_compiler_pipeline_t = 3;
pub type iree_compiler_pipeline_t = ::core::ffi::c_int;
unsafe extern "C" {
    pub fn ireeCompilerInvocationPipeline(
        inv: *mut iree_compiler_invocation_t,
        pipeline: iree_compiler_pipeline_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationRunPassPipeline(
        inv: *mut iree_compiler_invocation_t,
        textPassPipeline: *const ::core::ffi::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationOutputIR(
        inv: *mut iree_compiler_invocation_t,
        output: *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationOutputIRBytecode(
        inv: *mut iree_compiler_invocation_t,
        output: *mut iree_compiler_output_t,
        bytecodeVersion: ::core::ffi::c_int,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationOutputVMBytecode(
        inv: *mut iree_compiler_invocation_t,
        output: *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationOutputVMCSource(
        inv: *mut iree_compiler_invocation_t,
        output: *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationOutputHALExecutable(
        inv: *mut iree_compiler_invocation_t,
        output: *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerSourceDestroy(source: *mut iree_compiler_source_t);
}
unsafe extern "C" {
    pub fn ireeCompilerSourceOpenFile(
        session: *mut iree_compiler_session_t,
        filePath: *const ::core::ffi::c_char,
        out_source: *mut *mut iree_compiler_source_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerSourceWrapBuffer(
        session: *mut iree_compiler_session_t,
        bufferName: *const ::core::ffi::c_char,
        buffer: *const ::core::ffi::c_char,
        length: usize,
        isNullTerminated: bool,
        out_source: *mut *mut iree_compiler_source_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerSourceSplit(
        source: *mut iree_compiler_source_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                source: *mut iree_compiler_source_t,
                userData: *mut ::core::ffi::c_void,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerOutputDestroy(output: *mut iree_compiler_output_t);
}
unsafe extern "C" {
    pub fn ireeCompilerOutputOpenFile(
        filePath: *const ::core::ffi::c_char,
        out_output: *mut *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerOutputOpenFD(
        fd: ::core::ffi::c_int,
        out_output: *mut *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerOutputOpenMembuffer(
        out_output: *mut *mut iree_compiler_output_t,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerOutputMapMemory(
        output: *mut iree_compiler_output_t,
        contents: *mut *mut ::core::ffi::c_void,
        size: *mut u64,
    ) -> *mut iree_compiler_error_t;
}
unsafe extern "C" {
    pub fn ireeCompilerOutputKeep(output: *mut iree_compiler_output_t);
}
unsafe extern "C" {
    pub fn ireeCompilerOutputWrite(
        output: *mut iree_compiler_output_t,
        data: *const ::core::ffi::c_void,
        length: usize,
    ) -> *mut iree_compiler_error_t;
}
/// Re-export llvm::ThreadPool so as to avoid including the LLVM C API directly.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirLlvmThreadPool {
    /// Re-export llvm::ThreadPool so as to avoid including the LLVM C API directly.
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirLlvmThreadPool",
    ][::core::mem::size_of::<MlirLlvmThreadPool>() - 8usize];
    [
        "Alignment of MlirLlvmThreadPool",
    ][::core::mem::align_of::<MlirLlvmThreadPool>() - 8usize];
    [
        "Offset of field: MlirLlvmThreadPool::ptr",
    ][::core::mem::offset_of!(MlirLlvmThreadPool, ptr) - 0usize];
};
impl Default for MlirLlvmThreadPool {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** Re-export llvm::raw_fd_ostream so as to avoid including the LLVM C API
 directly.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirLlvmRawFdOStream {
    /** Re-export llvm::raw_fd_ostream so as to avoid including the LLVM C API
 directly.*/
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirLlvmRawFdOStream",
    ][::core::mem::size_of::<MlirLlvmRawFdOStream>() - 8usize];
    [
        "Alignment of MlirLlvmRawFdOStream",
    ][::core::mem::align_of::<MlirLlvmRawFdOStream>() - 8usize];
    [
        "Offset of field: MlirLlvmRawFdOStream::ptr",
    ][::core::mem::offset_of!(MlirLlvmRawFdOStream, ptr) - 0usize];
};
impl Default for MlirLlvmRawFdOStream {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirTypeID {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirTypeID"][::core::mem::size_of::<MlirTypeID>() - 8usize];
    ["Alignment of MlirTypeID"][::core::mem::align_of::<MlirTypeID>() - 8usize];
    [
        "Offset of field: MlirTypeID::ptr",
    ][::core::mem::offset_of!(MlirTypeID, ptr) - 0usize];
};
impl Default for MlirTypeID {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirTypeIDAllocator {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirTypeIDAllocator",
    ][::core::mem::size_of::<MlirTypeIDAllocator>() - 8usize];
    [
        "Alignment of MlirTypeIDAllocator",
    ][::core::mem::align_of::<MlirTypeIDAllocator>() - 8usize];
    [
        "Offset of field: MlirTypeIDAllocator::ptr",
    ][::core::mem::offset_of!(MlirTypeIDAllocator, ptr) - 0usize];
};
impl Default for MlirTypeIDAllocator {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** A pointer to a sized fragment of a string, not necessarily null-terminated.
 Does not own the underlying string. This is equivalent to llvm::StringRef.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirStringRef {
    ///< Pointer to the first symbol.
    pub data: *const ::core::ffi::c_char,
    ///< Length of the fragment.
    pub length: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirStringRef"][::core::mem::size_of::<MlirStringRef>() - 16usize];
    ["Alignment of MlirStringRef"][::core::mem::align_of::<MlirStringRef>() - 8usize];
    [
        "Offset of field: MlirStringRef::data",
    ][::core::mem::offset_of!(MlirStringRef, data) - 0usize];
    [
        "Offset of field: MlirStringRef::length",
    ][::core::mem::offset_of!(MlirStringRef, length) - 8usize];
};
impl Default for MlirStringRef {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /** Constructs a string reference from the pointer and length. The pointer need
 not reference to a null-terminated string.*/
    #[link_name = "mlirStringRefCreate__extern"]
    pub fn mlirStringRefCreate(
        str_: *const ::core::ffi::c_char,
        length: usize,
    ) -> MlirStringRef;
}
unsafe extern "C" {
    /** Constructs a string reference from a null-terminated C string. Prefer
 mlirStringRefCreate if the length of the string is known.*/
    pub fn mlirStringRefCreateFromCString(
        str_: *const ::core::ffi::c_char,
    ) -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns true if two string references are equal, false otherwise.
    pub fn mlirStringRefEqual(string: MlirStringRef, other: MlirStringRef) -> bool;
}
/** A callback for returning string references.

 This function is called back by the functions that need to return a
 reference to the portion of the string with the following arguments:
  - an MlirStringRef representing the current portion of the string
  - a pointer to user data forwarded from the printing call.*/
pub type MlirStringCallback = ::core::option::Option<
    unsafe extern "C" fn(arg1: MlirStringRef, arg2: *mut ::core::ffi::c_void),
>;
/** A logical result value, essentially a boolean with named states. LLVM
 convention for using boolean values to designate success or failure of an
 operation is a moving target, so MLIR opted for an explicit class.
 Instances of MlirLogicalResult must only be inspected using the associated
 functions.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MlirLogicalResult {
    pub value: i8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirLogicalResult"][::core::mem::size_of::<MlirLogicalResult>() - 1usize];
    [
        "Alignment of MlirLogicalResult",
    ][::core::mem::align_of::<MlirLogicalResult>() - 1usize];
    [
        "Offset of field: MlirLogicalResult::value",
    ][::core::mem::offset_of!(MlirLogicalResult, value) - 0usize];
};
unsafe extern "C" {
    /** Create an LLVM thread pool. This is reexported here to avoid directly
 pulling in the LLVM headers directly.*/
    pub fn mlirLlvmThreadPoolCreate() -> MlirLlvmThreadPool;
}
unsafe extern "C" {
    /// Destroy an LLVM thread pool.
    pub fn mlirLlvmThreadPoolDestroy(pool: MlirLlvmThreadPool);
}
unsafe extern "C" {
    /// Returns the maximum number of threads in the thread pool.
    pub fn mlirLlvmThreadPoolGetMaxConcurrency(
        pool: MlirLlvmThreadPool,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /** Create a raw_fd_ostream for the given path. This wrapper is needed because
 std::ostream does not provide the file sharing semantics required on
 Windows.
 - `path`: output file path.
 - `binary`: controls text vs binary mode.
 - `errorCallback`: called with an error message on failure (optional).
 - `userData`: forwarded to `errorCallback` so it can copy the error message
   into caller-owned storage (e.g., a `std::string`).

 On failure, returns a null stream and invokes the optional error callback
 with the error message.*/
    pub fn mlirLlvmRawFdOStreamCreate(
        path: *const ::core::ffi::c_char,
        binary: bool,
        errorCallback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLlvmRawFdOStream;
}
unsafe extern "C" {
    /// Write a string to a raw_fd_ostream created with mlirLlvmRawFdOStreamCreate.
    pub fn mlirLlvmRawFdOStreamWrite(
        stream: MlirLlvmRawFdOStream,
        string: MlirStringRef,
    );
}
unsafe extern "C" {
    /// Checks if a raw_fd_ostream is null.
    pub fn mlirLlvmRawFdOStreamIsNull(stream: MlirLlvmRawFdOStream) -> bool;
}
unsafe extern "C" {
    /// Destroy a raw_fd_ostream created with mlirLlvmRawFdOStreamCreate.
    pub fn mlirLlvmRawFdOStreamDestroy(stream: MlirLlvmRawFdOStream);
}
unsafe extern "C" {
    /** `ptr` must be 8 byte aligned and unique to a type valid for the duration of
 the returned type id's usage*/
    pub fn mlirTypeIDCreate(ptr: *const ::core::ffi::c_void) -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether a type id is null.
    #[link_name = "mlirTypeIDIsNull__extern"]
    pub fn mlirTypeIDIsNull(typeID: MlirTypeID) -> bool;
}
unsafe extern "C" {
    /// Checks if two type ids are equal.
    pub fn mlirTypeIDEqual(typeID1: MlirTypeID, typeID2: MlirTypeID) -> bool;
}
unsafe extern "C" {
    /// Returns the hash value of the type id.
    pub fn mlirTypeIDHashValue(typeID: MlirTypeID) -> usize;
}
unsafe extern "C" {
    /// Creates a type id allocator for dynamic type id creation
    pub fn mlirTypeIDAllocatorCreate() -> MlirTypeIDAllocator;
}
unsafe extern "C" {
    /// Deallocates the allocator and all allocated type ids
    pub fn mlirTypeIDAllocatorDestroy(allocator: MlirTypeIDAllocator);
}
unsafe extern "C" {
    /// Allocates a type id that is valid for the lifetime of the allocator
    pub fn mlirTypeIDAllocatorAllocateTypeID(
        allocator: MlirTypeIDAllocator,
    ) -> MlirTypeID;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirAsmState {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirAsmState"][::core::mem::size_of::<MlirAsmState>() - 8usize];
    ["Alignment of MlirAsmState"][::core::mem::align_of::<MlirAsmState>() - 8usize];
    [
        "Offset of field: MlirAsmState::ptr",
    ][::core::mem::offset_of!(MlirAsmState, ptr) - 0usize];
};
impl Default for MlirAsmState {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirBytecodeWriterConfig {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirBytecodeWriterConfig",
    ][::core::mem::size_of::<MlirBytecodeWriterConfig>() - 8usize];
    [
        "Alignment of MlirBytecodeWriterConfig",
    ][::core::mem::align_of::<MlirBytecodeWriterConfig>() - 8usize];
    [
        "Offset of field: MlirBytecodeWriterConfig::ptr",
    ][::core::mem::offset_of!(MlirBytecodeWriterConfig, ptr) - 0usize];
};
impl Default for MlirBytecodeWriterConfig {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirContext {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirContext"][::core::mem::size_of::<MlirContext>() - 8usize];
    ["Alignment of MlirContext"][::core::mem::align_of::<MlirContext>() - 8usize];
    [
        "Offset of field: MlirContext::ptr",
    ][::core::mem::offset_of!(MlirContext, ptr) - 0usize];
};
impl Default for MlirContext {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirDialect {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirDialect"][::core::mem::size_of::<MlirDialect>() - 8usize];
    ["Alignment of MlirDialect"][::core::mem::align_of::<MlirDialect>() - 8usize];
    [
        "Offset of field: MlirDialect::ptr",
    ][::core::mem::offset_of!(MlirDialect, ptr) - 0usize];
};
impl Default for MlirDialect {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirDialectRegistry {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirDialectRegistry",
    ][::core::mem::size_of::<MlirDialectRegistry>() - 8usize];
    [
        "Alignment of MlirDialectRegistry",
    ][::core::mem::align_of::<MlirDialectRegistry>() - 8usize];
    [
        "Offset of field: MlirDialectRegistry::ptr",
    ][::core::mem::offset_of!(MlirDialectRegistry, ptr) - 0usize];
};
impl Default for MlirDialectRegistry {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirOperation {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirOperation"][::core::mem::size_of::<MlirOperation>() - 8usize];
    ["Alignment of MlirOperation"][::core::mem::align_of::<MlirOperation>() - 8usize];
    [
        "Offset of field: MlirOperation::ptr",
    ][::core::mem::offset_of!(MlirOperation, ptr) - 0usize];
};
impl Default for MlirOperation {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirOpOperand {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirOpOperand"][::core::mem::size_of::<MlirOpOperand>() - 8usize];
    ["Alignment of MlirOpOperand"][::core::mem::align_of::<MlirOpOperand>() - 8usize];
    [
        "Offset of field: MlirOpOperand::ptr",
    ][::core::mem::offset_of!(MlirOpOperand, ptr) - 0usize];
};
impl Default for MlirOpOperand {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirOpPrintingFlags {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirOpPrintingFlags",
    ][::core::mem::size_of::<MlirOpPrintingFlags>() - 8usize];
    [
        "Alignment of MlirOpPrintingFlags",
    ][::core::mem::align_of::<MlirOpPrintingFlags>() - 8usize];
    [
        "Offset of field: MlirOpPrintingFlags::ptr",
    ][::core::mem::offset_of!(MlirOpPrintingFlags, ptr) - 0usize];
};
impl Default for MlirOpPrintingFlags {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirBlock {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirBlock"][::core::mem::size_of::<MlirBlock>() - 8usize];
    ["Alignment of MlirBlock"][::core::mem::align_of::<MlirBlock>() - 8usize];
    [
        "Offset of field: MlirBlock::ptr",
    ][::core::mem::offset_of!(MlirBlock, ptr) - 0usize];
};
impl Default for MlirBlock {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirRegion {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirRegion"][::core::mem::size_of::<MlirRegion>() - 8usize];
    ["Alignment of MlirRegion"][::core::mem::align_of::<MlirRegion>() - 8usize];
    [
        "Offset of field: MlirRegion::ptr",
    ][::core::mem::offset_of!(MlirRegion, ptr) - 0usize];
};
impl Default for MlirRegion {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirSymbolTable {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirSymbolTable"][::core::mem::size_of::<MlirSymbolTable>() - 8usize];
    [
        "Alignment of MlirSymbolTable",
    ][::core::mem::align_of::<MlirSymbolTable>() - 8usize];
    [
        "Offset of field: MlirSymbolTable::ptr",
    ][::core::mem::offset_of!(MlirSymbolTable, ptr) - 0usize];
};
impl Default for MlirSymbolTable {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirIRMapping {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirIRMapping"][::core::mem::size_of::<MlirIRMapping>() - 8usize];
    ["Alignment of MlirIRMapping"][::core::mem::align_of::<MlirIRMapping>() - 8usize];
    [
        "Offset of field: MlirIRMapping::ptr",
    ][::core::mem::offset_of!(MlirIRMapping, ptr) - 0usize];
};
impl Default for MlirIRMapping {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirAttribute {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirAttribute"][::core::mem::size_of::<MlirAttribute>() - 8usize];
    ["Alignment of MlirAttribute"][::core::mem::align_of::<MlirAttribute>() - 8usize];
    [
        "Offset of field: MlirAttribute::ptr",
    ][::core::mem::offset_of!(MlirAttribute, ptr) - 0usize];
};
impl Default for MlirAttribute {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirIdentifier {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirIdentifier"][::core::mem::size_of::<MlirIdentifier>() - 8usize];
    ["Alignment of MlirIdentifier"][::core::mem::align_of::<MlirIdentifier>() - 8usize];
    [
        "Offset of field: MlirIdentifier::ptr",
    ][::core::mem::offset_of!(MlirIdentifier, ptr) - 0usize];
};
impl Default for MlirIdentifier {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirLocation {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirLocation"][::core::mem::size_of::<MlirLocation>() - 8usize];
    ["Alignment of MlirLocation"][::core::mem::align_of::<MlirLocation>() - 8usize];
    [
        "Offset of field: MlirLocation::ptr",
    ][::core::mem::offset_of!(MlirLocation, ptr) - 0usize];
};
impl Default for MlirLocation {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirModule {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirModule"][::core::mem::size_of::<MlirModule>() - 8usize];
    ["Alignment of MlirModule"][::core::mem::align_of::<MlirModule>() - 8usize];
    [
        "Offset of field: MlirModule::ptr",
    ][::core::mem::offset_of!(MlirModule, ptr) - 0usize];
};
impl Default for MlirModule {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirType {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirType"][::core::mem::size_of::<MlirType>() - 8usize];
    ["Alignment of MlirType"][::core::mem::align_of::<MlirType>() - 8usize];
    ["Offset of field: MlirType::ptr"][::core::mem::offset_of!(MlirType, ptr) - 0usize];
};
impl Default for MlirType {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirValue {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirValue"][::core::mem::size_of::<MlirValue>() - 8usize];
    ["Alignment of MlirValue"][::core::mem::align_of::<MlirValue>() - 8usize];
    [
        "Offset of field: MlirValue::ptr",
    ][::core::mem::offset_of!(MlirValue, ptr) - 0usize];
};
impl Default for MlirValue {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** Named MLIR attribute.

 A named attribute is essentially a (name, attribute) pair where the name is
 a string.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirNamedAttribute {
    pub name: MlirIdentifier,
    pub attribute: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirNamedAttribute",
    ][::core::mem::size_of::<MlirNamedAttribute>() - 16usize];
    [
        "Alignment of MlirNamedAttribute",
    ][::core::mem::align_of::<MlirNamedAttribute>() - 8usize];
    [
        "Offset of field: MlirNamedAttribute::name",
    ][::core::mem::offset_of!(MlirNamedAttribute, name) - 0usize];
    [
        "Offset of field: MlirNamedAttribute::attribute",
    ][::core::mem::offset_of!(MlirNamedAttribute, attribute) - 8usize];
};
impl Default for MlirNamedAttribute {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /** Creates an MLIR context and transfers its ownership to the caller.
 This sets the default multithreading option (enabled).*/
    pub fn mlirContextCreate() -> MlirContext;
}
unsafe extern "C" {
    /** Creates an MLIR context with an explicit setting of the multithreading
 setting and transfers its ownership to the caller.*/
    pub fn mlirContextCreateWithThreading(threadingEnabled: bool) -> MlirContext;
}
unsafe extern "C" {
    /** Creates an MLIR context, setting the multithreading setting explicitly and
 pre-loading the dialects from the provided DialectRegistry.*/
    pub fn mlirContextCreateWithRegistry(
        registry: MlirDialectRegistry,
        threadingEnabled: bool,
    ) -> MlirContext;
}
unsafe extern "C" {
    /// Checks if two contexts are equal.
    pub fn mlirContextEqual(ctx1: MlirContext, ctx2: MlirContext) -> bool;
}
unsafe extern "C" {
    /// Checks whether a context is null.
    #[link_name = "mlirContextIsNull__extern"]
    pub fn mlirContextIsNull(context: MlirContext) -> bool;
}
unsafe extern "C" {
    /// Takes an MLIR context owned by the caller and destroys it.
    pub fn mlirContextDestroy(context: MlirContext);
}
unsafe extern "C" {
    /// Sets whether unregistered dialects are allowed in this context.
    pub fn mlirContextSetAllowUnregisteredDialects(context: MlirContext, allow: bool);
}
unsafe extern "C" {
    /// Returns whether the context allows unregistered dialects.
    pub fn mlirContextGetAllowUnregisteredDialects(context: MlirContext) -> bool;
}
unsafe extern "C" {
    /** Returns the number of dialects registered with the given context. A
 registered dialect will be loaded if needed by the parser.*/
    pub fn mlirContextGetNumRegisteredDialects(context: MlirContext) -> isize;
}
unsafe extern "C" {
    /** Append the contents of the given dialect registry to the registry associated
 with the context.*/
    pub fn mlirContextAppendDialectRegistry(
        ctx: MlirContext,
        registry: MlirDialectRegistry,
    );
}
unsafe extern "C" {
    /// Returns the number of dialects loaded by the context.
    pub fn mlirContextGetNumLoadedDialects(context: MlirContext) -> isize;
}
unsafe extern "C" {
    /** Gets the dialect instance owned by the given context using the dialect
 namespace to identify it, loads (i.e., constructs the instance of) the
 dialect if necessary. If the dialect is not registered with the context,
 returns null. Use mlirContextLoad<Name>Dialect to load an unregistered
 dialect.*/
    pub fn mlirContextGetOrLoadDialect(
        context: MlirContext,
        name: MlirStringRef,
    ) -> MlirDialect;
}
unsafe extern "C" {
    /** Gets the dialect instance owned by the given context using the dialect
 namespace to identify it. If the dialect is not loaded by the context,
 returns null. Use mlirContextGetOrLoadDialect to load a dialect if it is
 registered with the context.*/
    pub fn mlirContextGetLoadedDialect(
        context: MlirContext,
        name: MlirStringRef,
    ) -> MlirDialect;
}
unsafe extern "C" {
    /// Set threading mode (must be set to false to mlir-print-ir-after-all).
    pub fn mlirContextEnableMultithreading(context: MlirContext, enable: bool);
}
unsafe extern "C" {
    /** Eagerly loads all available dialects registered with a context, making
 them available for use for IR construction.*/
    pub fn mlirContextLoadAllAvailableDialects(context: MlirContext);
}
unsafe extern "C" {
    /** Returns whether the given fully-qualified operation (i.e.
 'dialect.operation') is registered with the context. This will return true
 if the dialect is loaded and the operation is registered within the
 dialect.*/
    pub fn mlirContextIsRegisteredOperation(
        context: MlirContext,
        name: MlirStringRef,
    ) -> bool;
}
unsafe extern "C" {
    /** Sets the thread pool of the context explicitly, enabling multithreading in
 the process. This API should be used to avoid re-creating thread pools in
 long-running applications that perform multiple compilations, see
 the C++ documentation for MLIRContext for details.*/
    pub fn mlirContextSetThreadPool(
        context: MlirContext,
        threadPool: MlirLlvmThreadPool,
    );
}
unsafe extern "C" {
    /** Gets the number of threads of the thread pool of the context when
 multithreading is enabled. Returns 1 if no multithreading.*/
    pub fn mlirContextGetNumThreads(context: MlirContext) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /** Gets the thread pool of the context when enabled multithreading, otherwise
 an assertion is raised.*/
    pub fn mlirContextGetThreadPool(context: MlirContext) -> MlirLlvmThreadPool;
}
unsafe extern "C" {
    /** Begins a transient scope on the context, freezing the base layer (loaded
 dialects, registered operations, interface models, and existing
 types/attributes).
 Precondition: The context must not already be in a transient scope.*/
    pub fn mlirContextBeginTransientScope(context: MlirContext);
}
unsafe extern "C" {
    /** Ends the transient scope and resets the context to the base state, pruning
 transient types, attributes, affine expressions, distinct attributes, and
 unregistered operations added during the transient scope.*/
    pub fn mlirContextEndTransientScope(context: MlirContext);
}
unsafe extern "C" {
    /// Returns whether the context is currently in a transient scope.
    pub fn mlirContextIsInTransientScope(context: MlirContext) -> bool;
}
unsafe extern "C" {
    /// Returns the context that owns the dialect.
    pub fn mlirDialectGetContext(dialect: MlirDialect) -> MlirContext;
}
unsafe extern "C" {
    /// Checks if the dialect is null.
    #[link_name = "mlirDialectIsNull__extern"]
    pub fn mlirDialectIsNull(dialect: MlirDialect) -> bool;
}
unsafe extern "C" {
    /** Checks if two dialects that belong to the same context are equal. Dialects
 from different contexts will not compare equal.*/
    pub fn mlirDialectEqual(dialect1: MlirDialect, dialect2: MlirDialect) -> bool;
}
unsafe extern "C" {
    /// Returns the namespace of the given dialect.
    pub fn mlirDialectGetNamespace(dialect: MlirDialect) -> MlirStringRef;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirDialectHandle {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirDialectHandle"][::core::mem::size_of::<MlirDialectHandle>() - 8usize];
    [
        "Alignment of MlirDialectHandle",
    ][::core::mem::align_of::<MlirDialectHandle>() - 8usize];
    [
        "Offset of field: MlirDialectHandle::ptr",
    ][::core::mem::offset_of!(MlirDialectHandle, ptr) - 0usize];
};
impl Default for MlirDialectHandle {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /// Returns the namespace associated with the provided dialect handle.
    pub fn mlirDialectHandleGetNamespace(arg1: MlirDialectHandle) -> MlirStringRef;
}
unsafe extern "C" {
    /** Inserts the dialect associated with the provided dialect handle into the
 provided dialect registry*/
    pub fn mlirDialectHandleInsertDialect(
        arg1: MlirDialectHandle,
        arg2: MlirDialectRegistry,
    );
}
unsafe extern "C" {
    /// Registers the dialect associated with the provided dialect handle.
    pub fn mlirDialectHandleRegisterDialect(arg1: MlirDialectHandle, arg2: MlirContext);
}
unsafe extern "C" {
    /// Loads the dialect associated with the provided dialect handle.
    pub fn mlirDialectHandleLoadDialect(
        arg1: MlirDialectHandle,
        arg2: MlirContext,
    ) -> MlirDialect;
}
unsafe extern "C" {
    /// Creates a dialect registry and transfers its ownership to the caller.
    pub fn mlirDialectRegistryCreate() -> MlirDialectRegistry;
}
unsafe extern "C" {
    /// Checks if the dialect registry is null.
    #[link_name = "mlirDialectRegistryIsNull__extern"]
    pub fn mlirDialectRegistryIsNull(registry: MlirDialectRegistry) -> bool;
}
unsafe extern "C" {
    /// Takes a dialect registry owned by the caller and destroys it.
    pub fn mlirDialectRegistryDestroy(registry: MlirDialectRegistry);
}
unsafe extern "C" {
    /// Returns the underlying location attribute of this location.
    pub fn mlirLocationGetAttribute(location: MlirLocation) -> MlirAttribute;
}
unsafe extern "C" {
    /// Creates a location from a location attribute.
    pub fn mlirLocationFromAttribute(attribute: MlirAttribute) -> MlirLocation;
}
unsafe extern "C" {
    /// Creates an File/Line/Column location owned by the given context.
    pub fn mlirLocationFileLineColGet(
        context: MlirContext,
        filename: MlirStringRef,
        line: ::core::ffi::c_uint,
        col: ::core::ffi::c_uint,
    ) -> MlirLocation;
}
unsafe extern "C" {
    /// Creates an File/Line/Column range location owned by the given context.
    pub fn mlirLocationFileLineColRangeGet(
        context: MlirContext,
        filename: MlirStringRef,
        start_line: ::core::ffi::c_uint,
        start_col: ::core::ffi::c_uint,
        end_line: ::core::ffi::c_uint,
        end_col: ::core::ffi::c_uint,
    ) -> MlirLocation;
}
unsafe extern "C" {
    /// Getter for filename of FileLineColRange.
    pub fn mlirLocationFileLineColRangeGetFilename(
        location: MlirLocation,
    ) -> MlirIdentifier;
}
unsafe extern "C" {
    /// Getter for start_line of FileLineColRange.
    pub fn mlirLocationFileLineColRangeGetStartLine(
        location: MlirLocation,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Getter for start_column of FileLineColRange.
    pub fn mlirLocationFileLineColRangeGetStartColumn(
        location: MlirLocation,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Getter for end_line of FileLineColRange.
    pub fn mlirLocationFileLineColRangeGetEndLine(
        location: MlirLocation,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Getter for end_column of FileLineColRange.
    pub fn mlirLocationFileLineColRangeGetEndColumn(
        location: MlirLocation,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// TypeID Getter for FileLineColRange.
    pub fn mlirLocationFileLineColRangeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given location is an FileLineColRange.
    pub fn mlirLocationIsAFileLineColRange(location: MlirLocation) -> bool;
}
unsafe extern "C" {
    /// Creates a call site location with a callee and a caller.
    pub fn mlirLocationCallSiteGet(
        callee: MlirLocation,
        caller: MlirLocation,
    ) -> MlirLocation;
}
unsafe extern "C" {
    /// Getter for callee of CallSite.
    pub fn mlirLocationCallSiteGetCallee(location: MlirLocation) -> MlirLocation;
}
unsafe extern "C" {
    /// Getter for caller of CallSite.
    pub fn mlirLocationCallSiteGetCaller(location: MlirLocation) -> MlirLocation;
}
unsafe extern "C" {
    /// TypeID Getter for CallSite.
    pub fn mlirLocationCallSiteGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given location is an CallSite.
    pub fn mlirLocationIsACallSite(location: MlirLocation) -> bool;
}
unsafe extern "C" {
    /// Creates a fused location with an array of locations and metadata.
    pub fn mlirLocationFusedGet(
        ctx: MlirContext,
        nLocations: isize,
        locations: *const MlirLocation,
        metadata: MlirAttribute,
    ) -> MlirLocation;
}
unsafe extern "C" {
    /// Getter for number of locations fused together.
    pub fn mlirLocationFusedGetNumLocations(
        location: MlirLocation,
    ) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /** Getter for locations of Fused. Requires pre-allocated memory of
 #fusedLocations X sizeof(MlirLocation).*/
    pub fn mlirLocationFusedGetLocations(
        location: MlirLocation,
        locationsCPtr: *mut MlirLocation,
    );
}
unsafe extern "C" {
    /// Getter for metadata of Fused.
    pub fn mlirLocationFusedGetMetadata(location: MlirLocation) -> MlirAttribute;
}
unsafe extern "C" {
    /// TypeID Getter for Fused.
    pub fn mlirLocationFusedGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given location is an Fused.
    pub fn mlirLocationIsAFused(location: MlirLocation) -> bool;
}
unsafe extern "C" {
    /** Creates a name location owned by the given context. Providing null location
 for childLoc is allowed and if childLoc is null location, then the behavior
 is the same as having unknown child location.*/
    pub fn mlirLocationNameGet(
        context: MlirContext,
        name: MlirStringRef,
        childLoc: MlirLocation,
    ) -> MlirLocation;
}
unsafe extern "C" {
    /// Getter for name of Name.
    pub fn mlirLocationNameGetName(location: MlirLocation) -> MlirIdentifier;
}
unsafe extern "C" {
    /// Getter for childLoc of Name.
    pub fn mlirLocationNameGetChildLoc(location: MlirLocation) -> MlirLocation;
}
unsafe extern "C" {
    /// TypeID Getter for Name.
    pub fn mlirLocationNameGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given location is an Name.
    pub fn mlirLocationIsAName(location: MlirLocation) -> bool;
}
unsafe extern "C" {
    /// Creates a location with unknown position owned by the given context.
    pub fn mlirLocationUnknownGet(context: MlirContext) -> MlirLocation;
}
unsafe extern "C" {
    /// TypeID Getter for Unknown.
    pub fn mlirLocationUnknownGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given location is an Unknown.
    pub fn mlirLocationIsAUnknown(location: MlirLocation) -> bool;
}
unsafe extern "C" {
    /// Gets the context that a location was created with.
    pub fn mlirLocationGetContext(location: MlirLocation) -> MlirContext;
}
unsafe extern "C" {
    /// Checks if the location is null.
    #[link_name = "mlirLocationIsNull__extern"]
    pub fn mlirLocationIsNull(location: MlirLocation) -> bool;
}
unsafe extern "C" {
    /// Checks if two locations are equal.
    pub fn mlirLocationEqual(l1: MlirLocation, l2: MlirLocation) -> bool;
}
unsafe extern "C" {
    /** Prints a location by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirLocationPrint(
        location: MlirLocation,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Creates a new, empty module and transfers ownership to the caller.
    pub fn mlirModuleCreateEmpty(location: MlirLocation) -> MlirModule;
}
unsafe extern "C" {
    /// Parses a module from the string and transfers ownership to the caller.
    pub fn mlirModuleCreateParse(
        context: MlirContext,
        module: MlirStringRef,
    ) -> MlirModule;
}
unsafe extern "C" {
    /// Parses a module from file and transfers ownership to the caller.
    pub fn mlirModuleCreateParseFromFile(
        context: MlirContext,
        fileName: MlirStringRef,
    ) -> MlirModule;
}
unsafe extern "C" {
    /// Gets the context that a module was created with.
    pub fn mlirModuleGetContext(module: MlirModule) -> MlirContext;
}
unsafe extern "C" {
    /// Gets the body of the module, i.e. the only block it contains.
    pub fn mlirModuleGetBody(module: MlirModule) -> MlirBlock;
}
unsafe extern "C" {
    /// Checks whether a module is null.
    #[link_name = "mlirModuleIsNull__extern"]
    pub fn mlirModuleIsNull(module: MlirModule) -> bool;
}
unsafe extern "C" {
    /// Takes a module owned by the caller and deletes it.
    pub fn mlirModuleDestroy(module: MlirModule);
}
unsafe extern "C" {
    /// Views the module as a generic operation.
    pub fn mlirModuleGetOperation(module: MlirModule) -> MlirOperation;
}
unsafe extern "C" {
    /** Views the generic operation as a module.
 The returned module is null when the input operation was not a ModuleOp.*/
    pub fn mlirModuleFromOperation(op: MlirOperation) -> MlirModule;
}
unsafe extern "C" {
    /// Checks if two modules are equal.
    pub fn mlirModuleEqual(lhs: MlirModule, rhs: MlirModule) -> bool;
}
unsafe extern "C" {
    /// Compute a hash for the given module.
    pub fn mlirModuleHashValue(mod_: MlirModule) -> usize;
}
/** An auxiliary class for constructing operations.

 This class contains all the information necessary to construct the
 operation. It owns the MlirRegions it has pointers to and does not own
 anything else. By default, the state can be constructed from a name and
 location, the latter being also used to access the context, and has no other
 components. These components can be added progressively until the operation
 is constructed. Users are not expected to rely on the internals of this
 class and should use mlirOperationState* functions instead.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirOperationState {
    pub name: MlirStringRef,
    pub location: MlirLocation,
    pub nResults: isize,
    pub results: *mut MlirType,
    pub nOperands: isize,
    pub operands: *mut MlirValue,
    pub nRegions: isize,
    pub regions: *mut MlirRegion,
    pub nSuccessors: isize,
    pub successors: *mut MlirBlock,
    pub nAttributes: isize,
    pub attributes: *mut MlirNamedAttribute,
    pub enableResultTypeInference: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirOperationState",
    ][::core::mem::size_of::<MlirOperationState>() - 112usize];
    [
        "Alignment of MlirOperationState",
    ][::core::mem::align_of::<MlirOperationState>() - 8usize];
    [
        "Offset of field: MlirOperationState::name",
    ][::core::mem::offset_of!(MlirOperationState, name) - 0usize];
    [
        "Offset of field: MlirOperationState::location",
    ][::core::mem::offset_of!(MlirOperationState, location) - 16usize];
    [
        "Offset of field: MlirOperationState::nResults",
    ][::core::mem::offset_of!(MlirOperationState, nResults) - 24usize];
    [
        "Offset of field: MlirOperationState::results",
    ][::core::mem::offset_of!(MlirOperationState, results) - 32usize];
    [
        "Offset of field: MlirOperationState::nOperands",
    ][::core::mem::offset_of!(MlirOperationState, nOperands) - 40usize];
    [
        "Offset of field: MlirOperationState::operands",
    ][::core::mem::offset_of!(MlirOperationState, operands) - 48usize];
    [
        "Offset of field: MlirOperationState::nRegions",
    ][::core::mem::offset_of!(MlirOperationState, nRegions) - 56usize];
    [
        "Offset of field: MlirOperationState::regions",
    ][::core::mem::offset_of!(MlirOperationState, regions) - 64usize];
    [
        "Offset of field: MlirOperationState::nSuccessors",
    ][::core::mem::offset_of!(MlirOperationState, nSuccessors) - 72usize];
    [
        "Offset of field: MlirOperationState::successors",
    ][::core::mem::offset_of!(MlirOperationState, successors) - 80usize];
    [
        "Offset of field: MlirOperationState::nAttributes",
    ][::core::mem::offset_of!(MlirOperationState, nAttributes) - 88usize];
    [
        "Offset of field: MlirOperationState::attributes",
    ][::core::mem::offset_of!(MlirOperationState, attributes) - 96usize];
    [
        "Offset of field: MlirOperationState::enableResultTypeInference",
    ][::core::mem::offset_of!(MlirOperationState, enableResultTypeInference) - 104usize];
};
impl Default for MlirOperationState {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /// Constructs an operation state from a name and a location.
    pub fn mlirOperationStateGet(
        name: MlirStringRef,
        loc: MlirLocation,
    ) -> MlirOperationState;
}
unsafe extern "C" {
    /// Adds a list of components to the operation state.
    pub fn mlirOperationStateAddResults(
        state: *mut MlirOperationState,
        n: isize,
        results: *const MlirType,
    );
}
unsafe extern "C" {
    pub fn mlirOperationStateAddOperands(
        state: *mut MlirOperationState,
        n: isize,
        operands: *const MlirValue,
    );
}
unsafe extern "C" {
    pub fn mlirOperationStateAddOwnedRegions(
        state: *mut MlirOperationState,
        n: isize,
        regions: *const MlirRegion,
    );
}
unsafe extern "C" {
    pub fn mlirOperationStateAddSuccessors(
        state: *mut MlirOperationState,
        n: isize,
        successors: *const MlirBlock,
    );
}
unsafe extern "C" {
    pub fn mlirOperationStateAddAttributes(
        state: *mut MlirOperationState,
        n: isize,
        attributes: *const MlirNamedAttribute,
    );
}
unsafe extern "C" {
    /** Enables result type inference for the operation under construction. If
 enabled, then the caller must not have called
 mlirOperationStateAddResults(). Note that if enabled, the
 mlirOperationCreate() call is failable: it will return a null operation
 on inference failure and will emit diagnostics.*/
    pub fn mlirOperationStateEnableResultTypeInference(state: *mut MlirOperationState);
}
unsafe extern "C" {
    /** Creates new AsmState, as with AsmState the IR should not be mutated
 in-between using this state.
 Must be freed with a call to mlirAsmStateDestroy().*/
    pub fn mlirAsmStateCreateForOperation(
        op: MlirOperation,
        flags: MlirOpPrintingFlags,
    ) -> MlirAsmState;
}
unsafe extern "C" {
    /** Creates new AsmState from value.
 Must be freed with a call to mlirAsmStateDestroy().*/
    pub fn mlirAsmStateCreateForValue(
        value: MlirValue,
        flags: MlirOpPrintingFlags,
    ) -> MlirAsmState;
}
unsafe extern "C" {
    /// Destroys printing flags created with mlirAsmStateCreate.
    pub fn mlirAsmStateDestroy(state: MlirAsmState);
}
unsafe extern "C" {
    /** Creates new printing flags with defaults, intended for customization.
 Must be freed with a call to mlirOpPrintingFlagsDestroy().*/
    pub fn mlirOpPrintingFlagsCreate() -> MlirOpPrintingFlags;
}
unsafe extern "C" {
    /// Destroys printing flags created with mlirOpPrintingFlagsCreate.
    pub fn mlirOpPrintingFlagsDestroy(flags: MlirOpPrintingFlags);
}
unsafe extern "C" {
    /** Enables the elision of large elements attributes by printing a lexically
 valid but otherwise meaningless form instead of the element data. The
 `largeElementLimit` is used to configure what is considered to be a "large"
 ElementsAttr by providing an upper limit to the number of elements.*/
    pub fn mlirOpPrintingFlagsElideLargeElementsAttrs(
        flags: MlirOpPrintingFlags,
        largeElementLimit: isize,
    );
}
unsafe extern "C" {
    /** Enables the elision of large resources strings by omitting them from the
 `dialect_resources` section. The `largeResourceLimit` is used to configure
 what is considered to be a "large" resource by providing an upper limit to
 the string size.*/
    pub fn mlirOpPrintingFlagsElideLargeResourceString(
        flags: MlirOpPrintingFlags,
        largeResourceLimit: isize,
    );
}
unsafe extern "C" {
    /** Enable or disable printing of debug information (based on `enable`). If
 'prettyForm' is set to true, debug information is printed in a more readable
 'pretty' form. Note: The IR generated with 'prettyForm' is not parsable.*/
    pub fn mlirOpPrintingFlagsEnableDebugInfo(
        flags: MlirOpPrintingFlags,
        enable: bool,
        prettyForm: bool,
    );
}
unsafe extern "C" {
    /// Always print operations in the generic form.
    pub fn mlirOpPrintingFlagsPrintGenericOpForm(flags: MlirOpPrintingFlags);
}
unsafe extern "C" {
    /// Print the name and location, if NamedLoc, as a prefix to the SSA ID.
    pub fn mlirOpPrintingFlagsPrintNameLocAsPrefix(flags: MlirOpPrintingFlags);
}
unsafe extern "C" {
    /** Use local scope when printing the operation. This allows for using the
 printer in a more localized and thread-safe setting, but may not
 necessarily be identical to what the IR will look like when dumping
 the full module.*/
    pub fn mlirOpPrintingFlagsUseLocalScope(flags: MlirOpPrintingFlags);
}
unsafe extern "C" {
    /// Do not verify the operation when using custom operation printers.
    pub fn mlirOpPrintingFlagsAssumeVerified(flags: MlirOpPrintingFlags);
}
unsafe extern "C" {
    /// Skip printing regions.
    pub fn mlirOpPrintingFlagsSkipRegions(flags: MlirOpPrintingFlags);
}
unsafe extern "C" {
    /** Creates new printing flags with defaults, intended for customization.
 Must be freed with a call to mlirBytecodeWriterConfigDestroy().*/
    pub fn mlirBytecodeWriterConfigCreate() -> MlirBytecodeWriterConfig;
}
unsafe extern "C" {
    /// Destroys printing flags created with mlirBytecodeWriterConfigCreate.
    pub fn mlirBytecodeWriterConfigDestroy(config: MlirBytecodeWriterConfig);
}
unsafe extern "C" {
    /// Sets the version to emit in the writer config.
    pub fn mlirBytecodeWriterConfigDesiredEmitVersion(
        flags: MlirBytecodeWriterConfig,
        version: i64,
    );
}
unsafe extern "C" {
    /** Creates an operation and transfers ownership to the caller.
 Note that caller owned child objects are transferred in this call and must
 not be further used. Particularly, this applies to any regions added to
 the state (the implementation may invalidate any such pointers).

 This call can fail under the following conditions, in which case, it will
 return a null operation and emit diagnostics:
   - Result type inference is enabled and cannot be performed.*/
    pub fn mlirOperationCreate(state: *mut MlirOperationState) -> MlirOperation;
}
unsafe extern "C" {
    /** Parses an operation, giving ownership to the caller. If parsing fails a null
 operation will be returned, and an error diagnostic emitted.

 `sourceStr` may be either the text assembly format, or binary bytecode
 format. `sourceName` is used as the file name of the source; any IR without
 locations will get a `FileLineColLoc` location with `sourceName` as the file
 name.*/
    pub fn mlirOperationCreateParse(
        context: MlirContext,
        sourceStr: MlirStringRef,
        sourceName: MlirStringRef,
    ) -> MlirOperation;
}
unsafe extern "C" {
    /** Creates a deep copy of an operation. The operation is not inserted and
 ownership is transferred to the caller.*/
    pub fn mlirOperationClone(op: MlirOperation) -> MlirOperation;
}
unsafe extern "C" {
    /// Takes an operation owned by the caller and destroys it.
    pub fn mlirOperationDestroy(op: MlirOperation);
}
unsafe extern "C" {
    /** Removes the given operation from its parent block. The operation is not
 destroyed. The ownership of the operation is transferred to the caller.*/
    pub fn mlirOperationRemoveFromParent(op: MlirOperation);
}
unsafe extern "C" {
    /// Checks whether the underlying operation is null.
    #[link_name = "mlirOperationIsNull__extern"]
    pub fn mlirOperationIsNull(op: MlirOperation) -> bool;
}
unsafe extern "C" {
    /** Checks whether two operation handles point to the same operation. This does
 not perform deep comparison.*/
    pub fn mlirOperationEqual(op: MlirOperation, other: MlirOperation) -> bool;
}
unsafe extern "C" {
    /** Compute a hash for the given operation. Operand and result SSA values are
 hashed by identity and locations are significant, so equivalent-but-distinct
 operations hash differently; use mlirOperationStructuralHashValue for a hash
 that pairs with mlirOperationIsStructurallyEquivalent.*/
    pub fn mlirOperationHashValue(op: MlirOperation) -> usize;
}
unsafe extern "C" {
    /** Checks whether two operations are structurally equivalent, i.e. they have
 the same name, attributes, operand and result types, and recursively
 equivalent regions. Operand equivalence is tracked structurally while
 recursing into regions, so operands defined inside the compared regions need
 not be the exact same SSA values; operands defined outside must be. `flags`
 is a bitwise OR of MlirOperationEquivalenceFlags values.*/
    pub fn mlirOperationIsStructurallyEquivalent(
        lhs: MlirOperation,
        rhs: MlirOperation,
        flags: u32,
    ) -> bool;
}
unsafe extern "C" {
    /** Computes a hash for the given operation that pairs with
 mlirOperationIsStructurallyEquivalent: two operations that are structurally
 equivalent under the same `flags` hash equally. Operands are hashed by
 identity, results are not hashed at all, and regions do not participate in
 the hash. `flags` is a bitwise OR of MlirOperationEquivalenceFlags values.*/
    pub fn mlirOperationStructuralHashValue(op: MlirOperation, flags: u32) -> usize;
}
unsafe extern "C" {
    /// Gets the context this operation is associated with
    pub fn mlirOperationGetContext(op: MlirOperation) -> MlirContext;
}
unsafe extern "C" {
    /// Checks if the operation name has a trait identified by the given type id.
    pub fn mlirOperationNameHasTrait(
        opName: MlirStringRef,
        traitTypeID: MlirTypeID,
        context: MlirContext,
    ) -> bool;
}
unsafe extern "C" {
    /// Gets the location of the operation.
    pub fn mlirOperationGetLocation(op: MlirOperation) -> MlirLocation;
}
unsafe extern "C" {
    /// Sets the location of the operation.
    pub fn mlirOperationSetLocation(op: MlirOperation, loc: MlirLocation);
}
unsafe extern "C" {
    /** Gets the type id of the operation.
 Returns null if the operation does not have a registered operation
 description.*/
    pub fn mlirOperationGetTypeID(op: MlirOperation) -> MlirTypeID;
}
unsafe extern "C" {
    /// Gets the name of the operation as an identifier.
    pub fn mlirOperationGetName(op: MlirOperation) -> MlirIdentifier;
}
unsafe extern "C" {
    /** Gets the block that owns this operation, returning null if the operation is
 not owned.*/
    pub fn mlirOperationGetBlock(op: MlirOperation) -> MlirBlock;
}
unsafe extern "C" {
    /** Gets the operation that owns this operation, returning null if the operation
 is not owned.*/
    pub fn mlirOperationGetParentOperation(op: MlirOperation) -> MlirOperation;
}
unsafe extern "C" {
    /** Returns true if `a` is an ancestor of `b`, i.e. `a` contains `b` or
 `a == b`.*/
    pub fn mlirOperationIsAncestor(a: MlirOperation, b: MlirOperation) -> bool;
}
unsafe extern "C" {
    /** Returns true if `a` is a proper ancestor of `b`, i.e. `a` contains `b` but
 `a != b`.*/
    pub fn mlirOperationIsProperAncestor(a: MlirOperation, b: MlirOperation) -> bool;
}
unsafe extern "C" {
    /// Returns the number of regions attached to the given operation.
    pub fn mlirOperationGetNumRegions(op: MlirOperation) -> isize;
}
unsafe extern "C" {
    /// Returns `pos`-th region attached to the operation.
    pub fn mlirOperationGetRegion(op: MlirOperation, pos: isize) -> MlirRegion;
}
unsafe extern "C" {
    /** Returns an operation immediately following the given operation it its
 enclosing block.*/
    pub fn mlirOperationGetNextInBlock(op: MlirOperation) -> MlirOperation;
}
unsafe extern "C" {
    /// Returns the number of operands of the operation.
    pub fn mlirOperationGetNumOperands(op: MlirOperation) -> isize;
}
unsafe extern "C" {
    /// Returns `pos`-th operand of the operation.
    pub fn mlirOperationGetOperand(op: MlirOperation, pos: isize) -> MlirValue;
}
unsafe extern "C" {
    /// Returns `pos`-th OpOperand of the operation.
    pub fn mlirOperationGetOpOperand(op: MlirOperation, pos: isize) -> MlirOpOperand;
}
unsafe extern "C" {
    /// Sets the `pos`-th operand of the operation.
    pub fn mlirOperationSetOperand(op: MlirOperation, pos: isize, newValue: MlirValue);
}
unsafe extern "C" {
    /// Replaces the operands of the operation.
    pub fn mlirOperationSetOperands(
        op: MlirOperation,
        nOperands: isize,
        operands: *const MlirValue,
    );
}
unsafe extern "C" {
    /// Returns the number of results of the operation.
    pub fn mlirOperationGetNumResults(op: MlirOperation) -> isize;
}
unsafe extern "C" {
    /// Returns `pos`-th result of the operation.
    pub fn mlirOperationGetResult(op: MlirOperation, pos: isize) -> MlirValue;
}
unsafe extern "C" {
    /// Returns the number of successor blocks of the operation.
    pub fn mlirOperationGetNumSuccessors(op: MlirOperation) -> isize;
}
unsafe extern "C" {
    /// Returns `pos`-th successor of the operation.
    pub fn mlirOperationGetSuccessor(op: MlirOperation, pos: isize) -> MlirBlock;
}
unsafe extern "C" {
    /// Set `pos`-th successor of the operation.
    pub fn mlirOperationSetSuccessor(op: MlirOperation, pos: isize, block: MlirBlock);
}
unsafe extern "C" {
    /** Returns true if this operation defines an inherent attribute with this name.
 Note: the attribute can be optional, so
 `mlirOperationGetInherentAttributeByName` can still return a null attribute.*/
    pub fn mlirOperationHasInherentAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
    ) -> bool;
}
unsafe extern "C" {
    /// Returns an inherent attribute attached to the operation given its name.
    pub fn mlirOperationGetInherentAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Sets an inherent attribute by name, replacing the existing if it exists.
 This has no effect if "name" does not match an inherent attribute.*/
    pub fn mlirOperationSetInherentAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
        attr: MlirAttribute,
    );
}
unsafe extern "C" {
    /// Returns the number of discardable attributes attached to the operation.
    pub fn mlirOperationGetNumDiscardableAttributes(op: MlirOperation) -> isize;
}
unsafe extern "C" {
    /// Return `pos`-th discardable attribute of the operation.
    pub fn mlirOperationGetDiscardableAttribute(
        op: MlirOperation,
        pos: isize,
    ) -> MlirNamedAttribute;
}
unsafe extern "C" {
    /// Returns a discardable attribute attached to the operation given its name.
    pub fn mlirOperationGetDiscardableAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Sets a discardable attribute by name, replacing the existing if it exists or
 adding a new one otherwise. The new `attr` Attribute is not allowed to be
 null, use `mlirOperationRemoveDiscardableAttributeByName` to remove an
 Attribute instead.*/
    pub fn mlirOperationSetDiscardableAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
        attr: MlirAttribute,
    );
}
unsafe extern "C" {
    /** Removes a discardable attribute by name. Returns false if the attribute was
 not found and true if removed.*/
    pub fn mlirOperationRemoveDiscardableAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
    ) -> bool;
}
unsafe extern "C" {
    /** Returns the number of attributes attached to the operation.
 Deprecated, please use `mlirOperationGetNumInherentAttributes` or
 `mlirOperationGetNumDiscardableAttributes`.*/
    pub fn mlirOperationGetNumAttributes(op: MlirOperation) -> isize;
}
unsafe extern "C" {
    /** Return `pos`-th attribute of the operation.
 Deprecated, please use `mlirOperationGetInherentAttribute` or
 `mlirOperationGetDiscardableAttribute`.*/
    pub fn mlirOperationGetAttribute(
        op: MlirOperation,
        pos: isize,
    ) -> MlirNamedAttribute;
}
unsafe extern "C" {
    /** Returns an attribute attached to the operation given its name.
 Deprecated, please use `mlirOperationGetInherentAttributeByName` or
 `mlirOperationGetDiscardableAttributeByName`.*/
    pub fn mlirOperationGetAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Sets an attribute by name, replacing the existing if it exists or
 adding a new one otherwise.
 Deprecated, please use `mlirOperationSetInherentAttributeByName` or
 `mlirOperationSetDiscardableAttributeByName`.*/
    pub fn mlirOperationSetAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
        attr: MlirAttribute,
    );
}
unsafe extern "C" {
    /** Removes an attribute by name. Returns false if the attribute was not found
 and true if removed.
 Deprecated, please use `mlirOperationRemoveInherentAttributeByName` or
 `mlirOperationRemoveDiscardableAttributeByName`.*/
    pub fn mlirOperationRemoveAttributeByName(
        op: MlirOperation,
        name: MlirStringRef,
    ) -> bool;
}
unsafe extern "C" {
    /** Prints an operation by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirOperationPrint(
        op: MlirOperation,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /** Same as mlirOperationPrint but accepts flags controlling the printing
 behavior.*/
    pub fn mlirOperationPrintWithFlags(
        op: MlirOperation,
        flags: MlirOpPrintingFlags,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /** Same as mlirOperationPrint but accepts AsmState controlling the printing
 behavior as well as caching computed names.*/
    pub fn mlirOperationPrintWithState(
        op: MlirOperation,
        state: MlirAsmState,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Same as mlirOperationPrint but writing the bytecode format.
    pub fn mlirOperationWriteBytecode(
        op: MlirOperation,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /** Same as mlirOperationWriteBytecode but with writer config and returns
 failure only if desired bytecode could not be honored.*/
    pub fn mlirOperationWriteBytecodeWithConfig(
        op: MlirOperation,
        config: MlirBytecodeWriterConfig,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLogicalResult;
}
unsafe extern "C" {
    /// Prints an operation to stderr.
    pub fn mlirOperationDump(op: MlirOperation);
}
unsafe extern "C" {
    /// Verify the operation and return true if it passes, false if it fails.
    pub fn mlirOperationVerify(op: MlirOperation) -> bool;
}
unsafe extern "C" {
    /** Moves the given operation immediately after the other operation in its
 parent block. The given operation may be owned by the caller or by its
 current block. The other operation must belong to a block. In any case, the
 ownership is transferred to the block of the other operation.*/
    pub fn mlirOperationMoveAfter(op: MlirOperation, other: MlirOperation);
}
unsafe extern "C" {
    /** Moves the given operation immediately before the other operation in its
 parent block. The given operation may be owner by the caller or by its
 current block. The other operation must belong to a block. In any case, the
 ownership is transferred to the block of the other operation.*/
    pub fn mlirOperationMoveBefore(op: MlirOperation, other: MlirOperation);
}
unsafe extern "C" {
    /** Given an operation 'other' that is within the same parent block, return
 whether the current operation is before 'other' in the operation list
 of the parent block.
 Note: This function has an average complexity of O(1), but worst case may
 take O(N) where N is the number of operations within the parent block.*/
    pub fn mlirOperationIsBeforeInBlock(op: MlirOperation, other: MlirOperation) -> bool;
}
pub const MlirWalkResultAdvance: MlirWalkResult = 0;
pub const MlirWalkResultInterrupt: MlirWalkResult = 1;
pub const MlirWalkResultSkip: MlirWalkResult = 2;
/// Operation walk result.
pub type MlirWalkResult = ::core::ffi::c_int;
pub const MlirWalkPreOrder: MlirWalkOrder = 0;
pub const MlirWalkPostOrder: MlirWalkOrder = 1;
/// Traversal order for operation walk.
pub type MlirWalkOrder = ::core::ffi::c_int;
/** Operation walker type. The handler is passed an (opaque) reference to an
 operation and a pointer to a `userData`.*/
pub type MlirOperationWalkCallback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: MlirOperation,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirWalkResult,
>;
unsafe extern "C" {
    /** Walks operation `op` in `walkOrder` and calls `callback` on that operation.
 `*userData` is passed to the callback as well and can be used to tunnel some
 context or other data into the callback.*/
    pub fn mlirOperationWalk(
        op: MlirOperation,
        callback: MlirOperationWalkCallback,
        userData: *mut ::core::ffi::c_void,
        walkOrder: MlirWalkOrder,
    );
}
unsafe extern "C" {
    /// Replace uses of 'of' value with the 'with' value inside the 'op' operation.
    pub fn mlirOperationReplaceUsesOfWith(
        op: MlirOperation,
        of: MlirValue,
        with: MlirValue,
    );
}
unsafe extern "C" {
    /// Creates a new empty region and transfers ownership to the caller.
    pub fn mlirRegionCreate() -> MlirRegion;
}
unsafe extern "C" {
    /// Takes a region owned by the caller and destroys it.
    pub fn mlirRegionDestroy(region: MlirRegion);
}
unsafe extern "C" {
    /// Checks whether a region is null.
    #[link_name = "mlirRegionIsNull__extern"]
    pub fn mlirRegionIsNull(region: MlirRegion) -> bool;
}
unsafe extern "C" {
    /** Checks whether two region handles point to the same region. This does not
 perform deep comparison.*/
    pub fn mlirRegionEqual(region: MlirRegion, other: MlirRegion) -> bool;
}
unsafe extern "C" {
    /// Gets the first block in the region.
    pub fn mlirRegionGetFirstBlock(region: MlirRegion) -> MlirBlock;
}
unsafe extern "C" {
    /// Takes a block owned by the caller and appends it to the given region.
    pub fn mlirRegionAppendOwnedBlock(region: MlirRegion, block: MlirBlock);
}
unsafe extern "C" {
    /** Takes a block owned by the caller and inserts it at `pos` to the given
 region. This is an expensive operation that linearly scans the region,
 prefer insertAfter/Before instead.*/
    pub fn mlirRegionInsertOwnedBlock(region: MlirRegion, pos: isize, block: MlirBlock);
}
unsafe extern "C" {
    /** Takes a block owned by the caller and inserts it after the (non-owned)
 reference block in the given region. The reference block must belong to the
 region. If the reference block is null, prepends the block to the region.*/
    pub fn mlirRegionInsertOwnedBlockAfter(
        region: MlirRegion,
        reference: MlirBlock,
        block: MlirBlock,
    );
}
unsafe extern "C" {
    /** Takes a block owned by the caller and inserts it before the (non-owned)
 reference block in the given region. The reference block must belong to the
 region. If the reference block is null, appends the block to the region.*/
    pub fn mlirRegionInsertOwnedBlockBefore(
        region: MlirRegion,
        reference: MlirBlock,
        block: MlirBlock,
    );
}
unsafe extern "C" {
    /// Returns first region attached to the operation.
    pub fn mlirOperationGetFirstRegion(op: MlirOperation) -> MlirRegion;
}
unsafe extern "C" {
    /** Returns the region immediately following the given region in its parent
 operation.*/
    pub fn mlirRegionGetNextInOperation(region: MlirRegion) -> MlirRegion;
}
unsafe extern "C" {
    /// Moves the entire content of the source region to the target region.
    pub fn mlirRegionTakeBody(target: MlirRegion, source: MlirRegion);
}
unsafe extern "C" {
    /** Creates a new empty block with the given argument types and transfers
 ownership to the caller.*/
    pub fn mlirBlockCreate(
        nArgs: isize,
        args: *const MlirType,
        locs: *const MlirLocation,
    ) -> MlirBlock;
}
unsafe extern "C" {
    /// Takes a block owned by the caller and destroys it.
    pub fn mlirBlockDestroy(block: MlirBlock);
}
unsafe extern "C" {
    /// Detach a block from the owning region and assume ownership.
    pub fn mlirBlockDetach(block: MlirBlock);
}
unsafe extern "C" {
    /// Checks whether a block is null.
    #[link_name = "mlirBlockIsNull__extern"]
    pub fn mlirBlockIsNull(block: MlirBlock) -> bool;
}
unsafe extern "C" {
    /** Checks whether two blocks handles point to the same block. This does not
 perform deep comparison.*/
    pub fn mlirBlockEqual(block: MlirBlock, other: MlirBlock) -> bool;
}
unsafe extern "C" {
    /// Returns the closest surrounding operation that contains this block.
    pub fn mlirBlockGetParentOperation(arg1: MlirBlock) -> MlirOperation;
}
unsafe extern "C" {
    /// Returns the region that contains this block.
    pub fn mlirBlockGetParentRegion(block: MlirBlock) -> MlirRegion;
}
unsafe extern "C" {
    /** Returns the block immediately following the given block in its parent
 region.*/
    pub fn mlirBlockGetNextInRegion(block: MlirBlock) -> MlirBlock;
}
unsafe extern "C" {
    /// Returns the first operation in the block.
    pub fn mlirBlockGetFirstOperation(block: MlirBlock) -> MlirOperation;
}
unsafe extern "C" {
    /// Returns the terminator operation in the block or null if no terminator.
    pub fn mlirBlockGetTerminator(block: MlirBlock) -> MlirOperation;
}
unsafe extern "C" {
    /// Takes an operation owned by the caller and appends it to the block.
    pub fn mlirBlockAppendOwnedOperation(block: MlirBlock, operation: MlirOperation);
}
unsafe extern "C" {
    /** Takes an operation owned by the caller and inserts it as `pos` to the block.
 This is an expensive operation that scans the block linearly, prefer
 insertBefore/After instead.*/
    pub fn mlirBlockInsertOwnedOperation(
        block: MlirBlock,
        pos: isize,
        operation: MlirOperation,
    );
}
unsafe extern "C" {
    /** Takes an operation owned by the caller and inserts it after the (non-owned)
 reference operation in the given block. If the reference is null, prepends
 the operation. Otherwise, the reference must belong to the block.*/
    pub fn mlirBlockInsertOwnedOperationAfter(
        block: MlirBlock,
        reference: MlirOperation,
        operation: MlirOperation,
    );
}
unsafe extern "C" {
    /** Takes an operation owned by the caller and inserts it before the (non-owned)
 reference operation in the given block. If the reference is null, appends
 the operation. Otherwise, the reference must belong to the block.*/
    pub fn mlirBlockInsertOwnedOperationBefore(
        block: MlirBlock,
        reference: MlirOperation,
        operation: MlirOperation,
    );
}
unsafe extern "C" {
    /// Returns the number of arguments of the block.
    pub fn mlirBlockGetNumArguments(block: MlirBlock) -> isize;
}
unsafe extern "C" {
    /** Appends an argument of the specified type to the block. Returns the newly
 added argument.*/
    pub fn mlirBlockAddArgument(
        block: MlirBlock,
        type_: MlirType,
        loc: MlirLocation,
    ) -> MlirValue;
}
unsafe extern "C" {
    /// Erase the argument at 'index' and remove it from the argument list.
    pub fn mlirBlockEraseArgument(block: MlirBlock, index: ::core::ffi::c_uint);
}
unsafe extern "C" {
    /** Inserts an argument of the specified type at a specified index to the block.
 Returns the newly added argument.*/
    pub fn mlirBlockInsertArgument(
        block: MlirBlock,
        pos: isize,
        type_: MlirType,
        loc: MlirLocation,
    ) -> MlirValue;
}
unsafe extern "C" {
    /// Returns `pos`-th argument of the block.
    pub fn mlirBlockGetArgument(block: MlirBlock, pos: isize) -> MlirValue;
}
unsafe extern "C" {
    /** Prints a block by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirBlockPrint(
        block: MlirBlock,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Returns the number of successor blocks of the block.
    pub fn mlirBlockGetNumSuccessors(block: MlirBlock) -> isize;
}
unsafe extern "C" {
    /// Returns `pos`-th successor of the block.
    pub fn mlirBlockGetSuccessor(block: MlirBlock, pos: isize) -> MlirBlock;
}
unsafe extern "C" {
    /// Returns the number of predecessor blocks of the block.
    pub fn mlirBlockGetNumPredecessors(block: MlirBlock) -> isize;
}
unsafe extern "C" {
    /** Returns `pos`-th predecessor of the block.

 WARNING: This getter is more expensive than the others here because
 the impl actually iterates the use-def chain (of block operands) anew for
 each indexed access.*/
    pub fn mlirBlockGetPredecessor(block: MlirBlock, pos: isize) -> MlirBlock;
}
unsafe extern "C" {
    /// Returns whether the value is null.
    #[link_name = "mlirValueIsNull__extern"]
    pub fn mlirValueIsNull(value: MlirValue) -> bool;
}
unsafe extern "C" {
    /// Returns 1 if two values are equal, 0 otherwise.
    pub fn mlirValueEqual(value1: MlirValue, value2: MlirValue) -> bool;
}
unsafe extern "C" {
    /// Returns 1 if the value is a block argument, 0 otherwise.
    pub fn mlirValueIsABlockArgument(value: MlirValue) -> bool;
}
unsafe extern "C" {
    /// Returns 1 if the value is an operation result, 0 otherwise.
    pub fn mlirValueIsAOpResult(value: MlirValue) -> bool;
}
unsafe extern "C" {
    /** Returns the block in which this value is defined as an argument. Asserts if
 the value is not a block argument.*/
    pub fn mlirBlockArgumentGetOwner(value: MlirValue) -> MlirBlock;
}
unsafe extern "C" {
    /// Returns the position of the value in the argument list of its block.
    pub fn mlirBlockArgumentGetArgNumber(value: MlirValue) -> isize;
}
unsafe extern "C" {
    /// Sets the type of the block argument to the given type.
    pub fn mlirBlockArgumentSetType(value: MlirValue, type_: MlirType);
}
unsafe extern "C" {
    /// Sets the location of the block argument to the given location.
    pub fn mlirBlockArgumentSetLocation(value: MlirValue, loc: MlirLocation);
}
unsafe extern "C" {
    /** Returns an operation that produced this value as its result. Asserts if the
 value is not an op result.*/
    pub fn mlirOpResultGetOwner(value: MlirValue) -> MlirOperation;
}
unsafe extern "C" {
    /** Returns the position of the value in the list of results of the operation
 that produced it.*/
    pub fn mlirOpResultGetResultNumber(value: MlirValue) -> isize;
}
unsafe extern "C" {
    /// Returns the type of the value.
    pub fn mlirValueGetType(value: MlirValue) -> MlirType;
}
unsafe extern "C" {
    /// Set the type of the value.
    pub fn mlirValueSetType(value: MlirValue, type_: MlirType);
}
unsafe extern "C" {
    /// Prints the value to the standard error stream.
    pub fn mlirValueDump(value: MlirValue);
}
unsafe extern "C" {
    /** Prints a value by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirValuePrint(
        value: MlirValue,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Prints a value as an operand (i.e., the ValueID).
    pub fn mlirValuePrintAsOperand(
        value: MlirValue,
        state: MlirAsmState,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /** Returns an op operand representing the first use of the value, or a null op
 operand if there are no uses.*/
    pub fn mlirValueGetFirstUse(value: MlirValue) -> MlirOpOperand;
}
unsafe extern "C" {
    /** Replace all uses of 'of' value with the 'with' value, updating anything in
 the IR that uses 'of' to use the other value instead.  When this returns
 there are zero uses of 'of'.*/
    pub fn mlirValueReplaceAllUsesOfWith(of: MlirValue, with: MlirValue);
}
unsafe extern "C" {
    /** Replace all uses of 'of' value with 'with' value, updating anything in the
 IR that uses 'of' to use 'with' instead, except if the user is listed in
 'exceptions'. The 'exceptions' parameter is an array of MlirOperation
 pointers with a length of 'numExceptions'.*/
    pub fn mlirValueReplaceAllUsesExcept(
        of: MlirValue,
        with: MlirValue,
        numExceptions: isize,
        exceptions: *mut MlirOperation,
    );
}
/** Callback deciding whether a particular use should be replaced. It is passed
 the use as an MlirOpOperand (from which the owner operation, operand number
 and value can be queried) and the user-provided `userData`. Returns true to
 replace this use.*/
pub type MlirOpOperandReplaceFilterCallback = ::core::option::Option<
    unsafe extern "C" fn(
        opOperand: MlirOpOperand,
        userData: *mut ::core::ffi::c_void,
    ) -> bool,
>;
unsafe extern "C" {
    /** Replace uses of 'of' value with 'with' value, but only for the uses for
 which the `filter` callback returns true. `filter` must not be NULL; this is
 only checked by an assertion, i.e. in builds with assertions enabled.*/
    pub fn mlirValueReplaceUsesWithIf(
        of: MlirValue,
        with: MlirValue,
        filter: MlirOpOperandReplaceFilterCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Gets the location of the value.
    pub fn mlirValueGetLocation(v: MlirValue) -> MlirLocation;
}
unsafe extern "C" {
    /// Gets the context that a value was created with.
    pub fn mlirValueGetContext(v: MlirValue) -> MlirContext;
}
unsafe extern "C" {
    /// Returns whether the op operand is null.
    pub fn mlirOpOperandIsNull(opOperand: MlirOpOperand) -> bool;
}
unsafe extern "C" {
    /// Returns the value of an op operand.
    pub fn mlirOpOperandGetValue(opOperand: MlirOpOperand) -> MlirValue;
}
unsafe extern "C" {
    /// Returns the owner operation of an op operand.
    pub fn mlirOpOperandGetOwner(opOperand: MlirOpOperand) -> MlirOperation;
}
unsafe extern "C" {
    /// Returns the operand number of an op operand.
    pub fn mlirOpOperandGetOperandNumber(
        opOperand: MlirOpOperand,
    ) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /** Returns an op operand representing the next use of the value, or a null op
 operand if there is no next use.*/
    pub fn mlirOpOperandGetNextUse(opOperand: MlirOpOperand) -> MlirOpOperand;
}
unsafe extern "C" {
    /// Parses a type. The type is owned by the context.
    pub fn mlirTypeParseGet(context: MlirContext, type_: MlirStringRef) -> MlirType;
}
unsafe extern "C" {
    /// Gets the context that a type was created with.
    pub fn mlirTypeGetContext(type_: MlirType) -> MlirContext;
}
unsafe extern "C" {
    /// Gets the type ID of the type.
    pub fn mlirTypeGetTypeID(type_: MlirType) -> MlirTypeID;
}
unsafe extern "C" {
    /// Gets the dialect a type belongs to.
    pub fn mlirTypeGetDialect(type_: MlirType) -> MlirDialect;
}
unsafe extern "C" {
    /// Checks whether a type is null.
    #[link_name = "mlirTypeIsNull__extern"]
    pub fn mlirTypeIsNull(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Checks if two types are equal.
    pub fn mlirTypeEqual(t1: MlirType, t2: MlirType) -> bool;
}
unsafe extern "C" {
    /** Prints a location by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirTypePrint(
        type_: MlirType,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Prints the type to the standard error stream.
    pub fn mlirTypeDump(type_: MlirType);
}
unsafe extern "C" {
    /// Parses an attribute. The attribute is owned by the context.
    pub fn mlirAttributeParseGet(
        context: MlirContext,
        attr: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /// Gets the context that an attribute was created with.
    pub fn mlirAttributeGetContext(attribute: MlirAttribute) -> MlirContext;
}
unsafe extern "C" {
    /// Gets the type of this attribute.
    pub fn mlirAttributeGetType(attribute: MlirAttribute) -> MlirType;
}
unsafe extern "C" {
    /// Gets the type id of the attribute.
    pub fn mlirAttributeGetTypeID(attribute: MlirAttribute) -> MlirTypeID;
}
unsafe extern "C" {
    /// Gets the dialect of the attribute.
    pub fn mlirAttributeGetDialect(attribute: MlirAttribute) -> MlirDialect;
}
unsafe extern "C" {
    /// Checks whether an attribute is null.
    #[link_name = "mlirAttributeIsNull__extern"]
    pub fn mlirAttributeIsNull(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Checks if two attributes are equal.
    pub fn mlirAttributeEqual(a1: MlirAttribute, a2: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Prints an attribute by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirAttributePrint(
        attr: MlirAttribute,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Prints the attribute to the standard error stream.
    pub fn mlirAttributeDump(attr: MlirAttribute);
}
unsafe extern "C" {
    /// Associates an attribute with the name. Takes ownership of neither.
    pub fn mlirNamedAttributeGet(
        name: MlirIdentifier,
        attr: MlirAttribute,
    ) -> MlirNamedAttribute;
}
unsafe extern "C" {
    /// Gets an identifier with the given string value.
    pub fn mlirIdentifierGet(
        context: MlirContext,
        str_: MlirStringRef,
    ) -> MlirIdentifier;
}
unsafe extern "C" {
    /// Returns the context associated with this identifier
    pub fn mlirIdentifierGetContext(arg1: MlirIdentifier) -> MlirContext;
}
unsafe extern "C" {
    /// Checks whether two identifiers are the same.
    pub fn mlirIdentifierEqual(ident: MlirIdentifier, other: MlirIdentifier) -> bool;
}
unsafe extern "C" {
    /// Gets the string value of the identifier.
    pub fn mlirIdentifierStr(ident: MlirIdentifier) -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the name of the attribute used to store symbol names compatible with
 symbol tables.*/
    pub fn mlirSymbolTableGetSymbolAttributeName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the name of the attribute used by default to store symbol
 visibility.*/
    pub fn mlirSymbolTableGetDefaultVisibilityAttributeName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Creates a symbol table for the given operation. If the operation does not
 have the SymbolTable trait, returns a null symbol table.*/
    pub fn mlirSymbolTableCreate(operation: MlirOperation) -> MlirSymbolTable;
}
unsafe extern "C" {
    /// Returns true if the symbol table is null.
    #[link_name = "mlirSymbolTableIsNull__extern"]
    pub fn mlirSymbolTableIsNull(symbolTable: MlirSymbolTable) -> bool;
}
unsafe extern "C" {
    /** Destroys the symbol table created with mlirSymbolTableCreate. This does not
 affect the operations in the table.*/
    pub fn mlirSymbolTableDestroy(symbolTable: MlirSymbolTable);
}
unsafe extern "C" {
    /** Looks up a symbol with the given name in the given symbol table and returns
 the operation that corresponds to the symbol. If the symbol cannot be found,
 returns a null operation.*/
    pub fn mlirSymbolTableLookup(
        symbolTable: MlirSymbolTable,
        name: MlirStringRef,
    ) -> MlirOperation;
}
unsafe extern "C" {
    /** Inserts the given operation into the given symbol table. The operation must
 have the symbol trait. If the symbol table already has a symbol with the
 same name, renames the symbol being inserted to ensure name uniqueness. Note
 that this does not move the operation itself into the block of the symbol
 table operation, this should be done separately. Returns the name of the
 symbol after insertion.*/
    pub fn mlirSymbolTableInsert(
        symbolTable: MlirSymbolTable,
        operation: MlirOperation,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /// Removes the given operation from the symbol table and erases it.
    pub fn mlirSymbolTableErase(symbolTable: MlirSymbolTable, operation: MlirOperation);
}
unsafe extern "C" {
    /** Attempt to replace all uses that are nested within the given operation
 of the given symbol 'oldSymbol' with the provided 'newSymbol'. This does
 not traverse into nested symbol tables. Will fail atomically if there are
 any unknown operations that may be potential symbol tables.*/
    pub fn mlirSymbolTableReplaceAllSymbolUses(
        oldSymbol: MlirStringRef,
        newSymbol: MlirStringRef,
        from: MlirOperation,
    ) -> MlirLogicalResult;
}
unsafe extern "C" {
    /** Walks all symbol table operations nested within, and including, `op`. For
 each symbol table operation, the provided callback is invoked with the op
 and a boolean signifying if the symbols within that symbol table can be
 treated as if all uses within the IR are visible to the caller.
 `allSymUsesVisible` identifies whether all of the symbol uses of symbols
 within `op` are visible.*/
    pub fn mlirSymbolTableWalkSymbolTables(
        from: MlirOperation,
        allSymUsesVisible: bool,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: MlirOperation,
                arg2: bool,
                userData: *mut ::core::ffi::c_void,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Creates a new empty IRMapping.
    pub fn mlirIRMappingCreate() -> MlirIRMapping;
}
unsafe extern "C" {
    /// Destroys the given IRMapping.
    pub fn mlirIRMappingDestroy(mapping: MlirIRMapping);
}
unsafe extern "C" {
    /// Checks whether an IRMapping is null.
    #[link_name = "mlirIRMappingIsNull__extern"]
    pub fn mlirIRMappingIsNull(mapping: MlirIRMapping) -> bool;
}
unsafe extern "C" {
    /// Maps a Value in the mapping.
    pub fn mlirIRMappingMapValue(mapping: MlirIRMapping, from: MlirValue, to: MlirValue);
}
unsafe extern "C" {
    /// Maps a Block in the mapping.
    pub fn mlirIRMappingMapBlock(mapping: MlirIRMapping, from: MlirBlock, to: MlirBlock);
}
unsafe extern "C" {
    /// Maps an Operation in the mapping.
    pub fn mlirIRMappingMapOperation(
        mapping: MlirIRMapping,
        from: MlirOperation,
        to: MlirOperation,
    );
}
unsafe extern "C" {
    /// Clears all mappings.
    pub fn mlirIRMappingClear(mapping: MlirIRMapping);
}
unsafe extern "C" {
    /** Looks up a mapped Value. Returns the mapped value, or the input value if
 no mapping exists.*/
    pub fn mlirIRMappingLookupOrDefaultValue(
        mapping: MlirIRMapping,
        from: MlirValue,
    ) -> MlirValue;
}
unsafe extern "C" {
    /// Looks up a mapped Value. Returns a null MlirValue if no mapping exists.
    pub fn mlirIRMappingLookupOrNullValue(
        mapping: MlirIRMapping,
        from: MlirValue,
    ) -> MlirValue;
}
unsafe extern "C" {
    /** Looks up a mapped Block. Returns the mapped block, or the input block if
 no mapping exists.*/
    pub fn mlirIRMappingLookupOrDefaultBlock(
        mapping: MlirIRMapping,
        from: MlirBlock,
    ) -> MlirBlock;
}
unsafe extern "C" {
    /// Looks up a mapped Block. Returns a null MlirBlock if no mapping exists.
    pub fn mlirIRMappingLookupOrNullBlock(
        mapping: MlirIRMapping,
        from: MlirBlock,
    ) -> MlirBlock;
}
unsafe extern "C" {
    /** Looks up a mapped Operation. Returns the mapped operation, or the input
 operation if no mapping exists.*/
    pub fn mlirIRMappingLookupOrDefaultOperation(
        mapping: MlirIRMapping,
        from: MlirOperation,
    ) -> MlirOperation;
}
unsafe extern "C" {
    /** Looks up a mapped Operation. Returns a null MlirOperation if no mapping
 exists.*/
    pub fn mlirIRMappingLookupOrNullOperation(
        mapping: MlirIRMapping,
        from: MlirOperation,
    ) -> MlirOperation;
}
unsafe extern "C" {
    /// Returns true if the mapping contains a mapping for the given value.
    pub fn mlirIRMappingContainsValue(mapping: MlirIRMapping, value: MlirValue) -> bool;
}
unsafe extern "C" {
    /// Returns true if the mapping contains a mapping for the given block.
    pub fn mlirIRMappingContainsBlock(mapping: MlirIRMapping, block: MlirBlock) -> bool;
}
unsafe extern "C" {
    /// Returns true if the mapping contains a mapping for the given operation.
    pub fn mlirIRMappingContainsOperation(
        mapping: MlirIRMapping,
        op: MlirOperation,
    ) -> bool;
}
unsafe extern "C" {
    /// Erases a value mapping.
    pub fn mlirIRMappingEraseValue(mapping: MlirIRMapping, value: MlirValue);
}
unsafe extern "C" {
    /// Erases a block mapping.
    pub fn mlirIRMappingEraseBlock(mapping: MlirIRMapping, block: MlirBlock);
}
unsafe extern "C" {
    /// Erases an operation mapping.
    pub fn mlirIRMappingEraseOperation(mapping: MlirIRMapping, op: MlirOperation);
}
unsafe extern "C" {
    /** Clones the operation with the given mapping. The mapping is updated with
 the cloned operation's results and regions.*/
    pub fn mlirOperationCloneWithMapping(
        op: MlirOperation,
        mapping: MlirIRMapping,
    ) -> MlirOperation;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirPass {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirPass"][::core::mem::size_of::<MlirPass>() - 8usize];
    ["Alignment of MlirPass"][::core::mem::align_of::<MlirPass>() - 8usize];
    ["Offset of field: MlirPass::ptr"][::core::mem::offset_of!(MlirPass, ptr) - 0usize];
};
impl Default for MlirPass {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirExternalPass {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirExternalPass"][::core::mem::size_of::<MlirExternalPass>() - 8usize];
    [
        "Alignment of MlirExternalPass",
    ][::core::mem::align_of::<MlirExternalPass>() - 8usize];
    [
        "Offset of field: MlirExternalPass::ptr",
    ][::core::mem::offset_of!(MlirExternalPass, ptr) - 0usize];
};
impl Default for MlirExternalPass {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirPassManager {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirPassManager"][::core::mem::size_of::<MlirPassManager>() - 8usize];
    [
        "Alignment of MlirPassManager",
    ][::core::mem::align_of::<MlirPassManager>() - 8usize];
    [
        "Offset of field: MlirPassManager::ptr",
    ][::core::mem::offset_of!(MlirPassManager, ptr) - 0usize];
};
impl Default for MlirPassManager {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirOpPassManager {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirOpPassManager"][::core::mem::size_of::<MlirOpPassManager>() - 8usize];
    [
        "Alignment of MlirOpPassManager",
    ][::core::mem::align_of::<MlirOpPassManager>() - 8usize];
    [
        "Offset of field: MlirOpPassManager::ptr",
    ][::core::mem::offset_of!(MlirOpPassManager, ptr) - 0usize];
};
impl Default for MlirOpPassManager {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /// Create a new top-level PassManager with the default anchor.
    pub fn mlirPassManagerCreate(ctx: MlirContext) -> MlirPassManager;
}
unsafe extern "C" {
    /// Create a new top-level PassManager anchored on `anchorOp`.
    pub fn mlirPassManagerCreateOnOperation(
        ctx: MlirContext,
        anchorOp: MlirStringRef,
    ) -> MlirPassManager;
}
unsafe extern "C" {
    /// Destroy the provided PassManager.
    pub fn mlirPassManagerDestroy(passManager: MlirPassManager);
}
unsafe extern "C" {
    /// Checks if a PassManager is null.
    #[link_name = "mlirPassManagerIsNull__extern"]
    pub fn mlirPassManagerIsNull(passManager: MlirPassManager) -> bool;
}
unsafe extern "C" {
    /// Cast a top-level PassManager to a generic OpPassManager.
    pub fn mlirPassManagerGetAsOpPassManager(
        passManager: MlirPassManager,
    ) -> MlirOpPassManager;
}
unsafe extern "C" {
    /// Run the provided `passManager` on the given `op`.
    pub fn mlirPassManagerRunOnOp(
        passManager: MlirPassManager,
        op: MlirOperation,
    ) -> MlirLogicalResult;
}
unsafe extern "C" {
    /** Enable IR printing.
 The treePrintingPath argument is an optional path to a directory
 where the dumps will be produced. If it isn't provided then dumps
 are produced to stderr.*/
    pub fn mlirPassManagerEnableIRPrinting(
        passManager: MlirPassManager,
        printBeforeAll: bool,
        printAfterAll: bool,
        printModuleScope: bool,
        printAfterOnlyOnChange: bool,
        printAfterOnlyOnFailure: bool,
        flags: MlirOpPrintingFlags,
        treePrintingPath: MlirStringRef,
    );
}
unsafe extern "C" {
    /// Enable / disable verify-each.
    pub fn mlirPassManagerEnableVerifier(passManager: MlirPassManager, enable: bool);
}
unsafe extern "C" {
    /// Enable pass timing.
    pub fn mlirPassManagerEnableTiming(passManager: MlirPassManager);
}
pub const MLIR_PASS_DISPLAY_MODE_LIST: MlirPassDisplayMode = 0;
pub const MLIR_PASS_DISPLAY_MODE_PIPELINE: MlirPassDisplayMode = 1;
/** Enumerated type of pass display modes.
 Mainly used in mlirPassManagerEnableStatistics.*/
pub type MlirPassDisplayMode = ::core::ffi::c_int;
unsafe extern "C" {
    /// Enable pass statistics.
    pub fn mlirPassManagerEnableStatistics(
        passManager: MlirPassManager,
        displayMode: MlirPassDisplayMode,
    );
}
unsafe extern "C" {
    /** Nest an OpPassManager under the top-level PassManager, the nested
 passmanager will only run on operations matching the provided name.
 The returned OpPassManager will be destroyed when the parent is destroyed.
 To further nest more OpPassManager under the newly returned one, see
 `mlirOpPassManagerNest` below.*/
    pub fn mlirPassManagerGetNestedUnder(
        passManager: MlirPassManager,
        operationName: MlirStringRef,
    ) -> MlirOpPassManager;
}
unsafe extern "C" {
    /** Nest an OpPassManager under the provided OpPassManager, the nested
 passmanager will only run on operations matching the provided name.
 The returned OpPassManager will be destroyed when the parent is destroyed.*/
    pub fn mlirOpPassManagerGetNestedUnder(
        passManager: MlirOpPassManager,
        operationName: MlirStringRef,
    ) -> MlirOpPassManager;
}
unsafe extern "C" {
    /** Add a pass and transfer ownership to the provided top-level mlirPassManager.
 If the pass is not a generic operation pass or a ModulePass, a new
 OpPassManager is implicitly nested under the provided PassManager.*/
    pub fn mlirPassManagerAddOwnedPass(passManager: MlirPassManager, pass: MlirPass);
}
unsafe extern "C" {
    /** Add a pass and transfer ownership to the provided mlirOpPassManager. If the
 pass is not a generic operation pass or matching the type of the provided
 PassManager, a new OpPassManager is implicitly nested under the provided
 PassManager.*/
    pub fn mlirOpPassManagerAddOwnedPass(passManager: MlirOpPassManager, pass: MlirPass);
}
unsafe extern "C" {
    /** Parse a sequence of textual MLIR pass pipeline elements and add them to the
 provided OpPassManager. If parsing fails an error message is reported using
 the provided callback.*/
    pub fn mlirOpPassManagerAddPipeline(
        passManager: MlirOpPassManager,
        pipelineElements: MlirStringRef,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLogicalResult;
}
unsafe extern "C" {
    /** Print a textual MLIR pass pipeline by sending chunks of the string
 representation and forwarding `userData to `callback`. Note that the
 callback may be called several times with consecutive chunks of the string.*/
    pub fn mlirPrintPassPipeline(
        passManager: MlirOpPassManager,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /** Parse a textual MLIR pass pipeline and assign it to the provided
 OpPassManager. If parsing fails an error message is reported using the
 provided callback.*/
    pub fn mlirParsePassPipeline(
        passManager: MlirOpPassManager,
        pipeline: MlirStringRef,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLogicalResult;
}
/** Structure of external `MlirPass` callbacks.
 All callbacks are required to be set unless otherwise specified.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MlirExternalPassCallbacks {
    /** This callback is called from the pass is created.
 This is analogous to a C++ pass constructor.*/
    pub construct: ::core::option::Option<
        unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
    >,
    /** This callback is called when the pass is destroyed
 This is analogous to a C++ pass destructor.*/
    pub destruct: ::core::option::Option<
        unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
    >,
    /** This callback is optional.
 The callback is called before the pass is run, allowing a chance to
 initialize any complex state necessary for running the pass.
 See Pass::initialize(MLIRContext *).*/
    pub initialize: ::core::option::Option<
        unsafe extern "C" fn(
            ctx: MlirContext,
            userData: *mut ::core::ffi::c_void,
        ) -> MlirLogicalResult,
    >,
    /** This callback is called when the pass is cloned.
 See Pass::clonePass().*/
    pub clone: ::core::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    /** This callback is called when the pass is run.
 See Pass::runOnOperation().*/
    pub run: ::core::option::Option<
        unsafe extern "C" fn(
            op: MlirOperation,
            pass: MlirExternalPass,
            userData: *mut ::core::ffi::c_void,
        ),
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirExternalPassCallbacks",
    ][::core::mem::size_of::<MlirExternalPassCallbacks>() - 40usize];
    [
        "Alignment of MlirExternalPassCallbacks",
    ][::core::mem::align_of::<MlirExternalPassCallbacks>() - 8usize];
    [
        "Offset of field: MlirExternalPassCallbacks::construct",
    ][::core::mem::offset_of!(MlirExternalPassCallbacks, construct) - 0usize];
    [
        "Offset of field: MlirExternalPassCallbacks::destruct",
    ][::core::mem::offset_of!(MlirExternalPassCallbacks, destruct) - 8usize];
    [
        "Offset of field: MlirExternalPassCallbacks::initialize",
    ][::core::mem::offset_of!(MlirExternalPassCallbacks, initialize) - 16usize];
    [
        "Offset of field: MlirExternalPassCallbacks::clone",
    ][::core::mem::offset_of!(MlirExternalPassCallbacks, clone) - 24usize];
    [
        "Offset of field: MlirExternalPassCallbacks::run",
    ][::core::mem::offset_of!(MlirExternalPassCallbacks, run) - 32usize];
};
unsafe extern "C" {
    /** Creates an external `MlirPass` that calls the supplied `callbacks` using the
 supplied `userData`. If `opName` is empty, the pass is a generic operation
 pass. Otherwise it is an operation pass specific to the specified pass name.*/
    pub fn mlirCreateExternalPass(
        passID: MlirTypeID,
        name: MlirStringRef,
        argument: MlirStringRef,
        description: MlirStringRef,
        opName: MlirStringRef,
        nDependentDialects: isize,
        dependentDialects: *mut MlirDialectHandle,
        callbacks: MlirExternalPassCallbacks,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirPass;
}
unsafe extern "C" {
    /** This signals that the pass has failed. This is only valid to call during
 the `run` callback of `MlirExternalPassCallbacks`.
 See Pass::signalPassFailure().*/
    pub fn mlirExternalPassSignalFailure(pass: MlirExternalPass);
}
unsafe extern "C" {
    pub fn ireeCompilerRegisterDialects(registry: MlirDialectRegistry);
}
unsafe extern "C" {
    pub fn ireeCompilerInitializeContext(context: MlirContext);
}
unsafe extern "C" {
    pub fn ireeCompilerSessionBorrowContext(
        session: *mut iree_compiler_session_t,
    ) -> MlirContext;
}
unsafe extern "C" {
    pub fn ireeCompilerSessionStealContext(
        session: *mut iree_compiler_session_t,
    ) -> MlirContext;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationImportBorrowModule(
        inv: *mut iree_compiler_invocation_t,
        moduleOp: MlirOperation,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationImportStealModule(
        inv: *mut iree_compiler_invocation_t,
        moduleOp: MlirOperation,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeCompilerInvocationExportStealModule(
        inv: *mut iree_compiler_invocation_t,
    ) -> MlirOperation;
}
unsafe extern "C" {
    pub fn iree_native_load_compiler(path: *const u16) -> u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirAffineExpr {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirAffineExpr"][::core::mem::size_of::<MlirAffineExpr>() - 8usize];
    ["Alignment of MlirAffineExpr"][::core::mem::align_of::<MlirAffineExpr>() - 8usize];
    [
        "Offset of field: MlirAffineExpr::ptr",
    ][::core::mem::offset_of!(MlirAffineExpr, ptr) - 0usize];
};
impl Default for MlirAffineExpr {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /// Gets the context that owns the affine expression.
    pub fn mlirAffineExprGetContext(affineExpr: MlirAffineExpr) -> MlirContext;
}
unsafe extern "C" {
    /// Returns `true` if the two affine expressions are equal.
    pub fn mlirAffineExprEqual(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /** Returns `true` if the given affine expression is a null expression. Note
 constant zero is not a null expression.*/
    #[link_name = "mlirAffineExprIsNull__extern"]
    pub fn mlirAffineExprIsNull(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /** Prints an affine expression by sending chunks of the string representation
 and forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirAffineExprPrint(
        affineExpr: MlirAffineExpr,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Prints the affine expression to the standard error stream.
    pub fn mlirAffineExprDump(affineExpr: MlirAffineExpr);
}
unsafe extern "C" {
    /** Checks whether the given affine expression is made out of only symbols and
 constants.*/
    pub fn mlirAffineExprIsSymbolicOrConstant(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /** Checks whether the given affine expression is a pure affine expression, i.e.
 mul, floordiv, ceildic, and mod is only allowed w.r.t constants.*/
    pub fn mlirAffineExprIsPureAffine(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /** Returns the greatest known integral divisor of this affine expression. The
 result is always positive.*/
    pub fn mlirAffineExprGetLargestKnownDivisor(affineExpr: MlirAffineExpr) -> i64;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is a multiple of 'factor'.
    pub fn mlirAffineExprIsMultipleOf(affineExpr: MlirAffineExpr, factor: i64) -> bool;
}
unsafe extern "C" {
    /** Checks whether the given affine expression involves AffineDimExpr
 'position'.*/
    pub fn mlirAffineExprIsFunctionOfDim(
        affineExpr: MlirAffineExpr,
        position: isize,
    ) -> bool;
}
unsafe extern "C" {
    /// Composes the given map with the given expression.
    pub fn mlirAffineExprCompose(
        affineExpr: MlirAffineExpr,
        affineMap: MlirAffineMap,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /** Replace dims[offset ... numDims)
 by dims[offset + shift ... shift + numDims).*/
    pub fn mlirAffineExprShiftDims(
        affineExpr: MlirAffineExpr,
        numDims: u32,
        shift: u32,
        offset: u32,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /** Replace symbols[offset ... numSymbols)
 by symbols[offset + shift ... shift + numSymbols).*/
    pub fn mlirAffineExprShiftSymbols(
        affineExpr: MlirAffineExpr,
        numSymbols: u32,
        shift: u32,
        offset: u32,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /** Simplify an affine expression by flattening and some amount of simple
 analysis. This has complexity linear in the number of nodes in 'expr'.
 Returns the simplified expression, which is the same as the input expression
 if it can't be simplified. When `expr` is semi-affine, a simplified
 semi-affine expression is constructed in the sorted order of dimension and
 symbol positions.*/
    pub fn mlirSimplifyAffineExpr(
        expr: MlirAffineExpr,
        numDims: u32,
        numSymbols: u32,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is a dimension expression.
    pub fn mlirAffineExprIsADim(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine dimension expression with 'position' in the context.
    pub fn mlirAffineDimExprGet(ctx: MlirContext, position: isize) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Returns the position of the given affine dimension expression.
    pub fn mlirAffineDimExprGetPosition(affineExpr: MlirAffineExpr) -> isize;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is a symbol expression.
    pub fn mlirAffineExprIsASymbol(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine symbol expression with 'position' in the context.
    pub fn mlirAffineSymbolExprGet(ctx: MlirContext, position: isize) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Returns the position of the given affine symbol expression.
    pub fn mlirAffineSymbolExprGetPosition(affineExpr: MlirAffineExpr) -> isize;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is a constant expression.
    pub fn mlirAffineExprIsAConstant(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine constant expression with 'constant' in the context.
    pub fn mlirAffineConstantExprGet(ctx: MlirContext, constant: i64) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Returns the value of the given affine constant expression.
    pub fn mlirAffineConstantExprGetValue(affineExpr: MlirAffineExpr) -> i64;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is an add expression.
    pub fn mlirAffineExprIsAAdd(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine add expression with 'lhs' and 'rhs'.
    pub fn mlirAffineAddExprGet(
        lhs: MlirAffineExpr,
        rhs: MlirAffineExpr,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is an mul expression.
    pub fn mlirAffineExprIsAMul(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine mul expression with 'lhs' and 'rhs'.
    pub fn mlirAffineMulExprGet(
        lhs: MlirAffineExpr,
        rhs: MlirAffineExpr,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is an mod expression.
    pub fn mlirAffineExprIsAMod(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine mod expression with 'lhs' and 'rhs'.
    pub fn mlirAffineModExprGet(
        lhs: MlirAffineExpr,
        rhs: MlirAffineExpr,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is an floordiv expression.
    pub fn mlirAffineExprIsAFloorDiv(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine floordiv expression with 'lhs' and 'rhs'.
    pub fn mlirAffineFloorDivExprGet(
        lhs: MlirAffineExpr,
        rhs: MlirAffineExpr,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is an ceildiv expression.
    pub fn mlirAffineExprIsACeilDiv(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /// Creates an affine ceildiv expression with 'lhs' and 'rhs'.
    pub fn mlirAffineCeilDivExprGet(
        lhs: MlirAffineExpr,
        rhs: MlirAffineExpr,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /// Checks whether the given affine expression is binary.
    pub fn mlirAffineExprIsABinary(affineExpr: MlirAffineExpr) -> bool;
}
unsafe extern "C" {
    /** Returns the left hand side affine expression of the given affine binary
 operation expression.*/
    pub fn mlirAffineBinaryOpExprGetLHS(affineExpr: MlirAffineExpr) -> MlirAffineExpr;
}
unsafe extern "C" {
    /** Returns the right hand side affine expression of the given affine binary
 operation expression.*/
    pub fn mlirAffineBinaryOpExprGetRHS(affineExpr: MlirAffineExpr) -> MlirAffineExpr;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirAffineMap {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirAffineMap"][::core::mem::size_of::<MlirAffineMap>() - 8usize];
    ["Alignment of MlirAffineMap"][::core::mem::align_of::<MlirAffineMap>() - 8usize];
    [
        "Offset of field: MlirAffineMap::ptr",
    ][::core::mem::offset_of!(MlirAffineMap, ptr) - 0usize];
};
impl Default for MlirAffineMap {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /// Gets the context that the given affine map was created with
    pub fn mlirAffineMapGetContext(affineMap: MlirAffineMap) -> MlirContext;
}
unsafe extern "C" {
    /// Checks whether an affine map is null.
    #[link_name = "mlirAffineMapIsNull__extern"]
    pub fn mlirAffineMapIsNull(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /// Checks if two affine maps are equal.
    pub fn mlirAffineMapEqual(a1: MlirAffineMap, a2: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /** Prints an affine map by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirAffineMapPrint(
        affineMap: MlirAffineMap,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Prints the affine map to the standard error stream.
    pub fn mlirAffineMapDump(affineMap: MlirAffineMap);
}
unsafe extern "C" {
    /** Creates a zero result affine map with no dimensions or symbols in the
 context. The affine map is owned by the context.*/
    pub fn mlirAffineMapEmptyGet(ctx: MlirContext) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Creates a zero result affine map of the given dimensions and symbols in the
 context. The affine map is owned by the context.*/
    pub fn mlirAffineMapZeroResultGet(
        ctx: MlirContext,
        dimCount: isize,
        symbolCount: isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Creates an affine map with results defined by the given list of affine
 expressions. The map resulting map also has the requested number of input
 dimensions and symbols, regardless of them being used in the results.*/
    pub fn mlirAffineMapGet(
        ctx: MlirContext,
        dimCount: isize,
        symbolCount: isize,
        nAffineExprs: isize,
        affineExprs: *mut MlirAffineExpr,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Creates a single constant result affine map in the context. The affine map
 is owned by the context.*/
    pub fn mlirAffineMapConstantGet(ctx: MlirContext, val: i64) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Creates an affine map with 'numDims' identity in the context. The affine map
 is owned by the context.*/
    pub fn mlirAffineMapMultiDimIdentityGet(
        ctx: MlirContext,
        numDims: isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Creates an identity affine map on the most minor dimensions in the context.
 The affine map is owned by the context. The function asserts that the number
 of dimensions is greater or equal to the number of results.*/
    pub fn mlirAffineMapMinorIdentityGet(
        ctx: MlirContext,
        dims: isize,
        results: isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Creates an affine map with a permutation expression and its size in the
 context. The permutation expression is a non-empty vector of integers.
 The elements of the permutation vector must be continuous from 0 and cannot
 be repeated (i.e. `[1,2,0]` is a valid permutation. `[2,0]` or `[1,1,2]` is
 an invalid permutation.) The affine map is owned by the context.*/
    pub fn mlirAffineMapPermutationGet(
        ctx: MlirContext,
        size: isize,
        permutation: *mut ::core::ffi::c_uint,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Checks whether the given affine map is an identity affine map. The function
 asserts that the number of dimensions is greater or equal to the number of
 results.*/
    pub fn mlirAffineMapIsIdentity(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given affine map is a minor identity affine map.
    pub fn mlirAffineMapIsMinorIdentity(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given affine map is an empty affine map.
    pub fn mlirAffineMapIsEmpty(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /** Checks whether the given affine map is a single result constant affine
 map.*/
    pub fn mlirAffineMapIsSingleConstant(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /** Returns the constant result of the given affine map. The function asserts
 that the map has a single constant result.*/
    pub fn mlirAffineMapGetSingleConstantResult(affineMap: MlirAffineMap) -> i64;
}
unsafe extern "C" {
    /// Returns the number of dimensions of the given affine map.
    pub fn mlirAffineMapGetNumDims(affineMap: MlirAffineMap) -> isize;
}
unsafe extern "C" {
    /// Returns the number of symbols of the given affine map.
    pub fn mlirAffineMapGetNumSymbols(affineMap: MlirAffineMap) -> isize;
}
unsafe extern "C" {
    /// Returns the number of results of the given affine map.
    pub fn mlirAffineMapGetNumResults(affineMap: MlirAffineMap) -> isize;
}
unsafe extern "C" {
    /// Returns the result at the given position.
    pub fn mlirAffineMapGetResult(
        affineMap: MlirAffineMap,
        pos: isize,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /** Returns the number of inputs (dimensions + symbols) of the given affine
 map.*/
    pub fn mlirAffineMapGetNumInputs(affineMap: MlirAffineMap) -> isize;
}
unsafe extern "C" {
    /** Checks whether the given affine map represents a subset of a symbol-less
 permutation map.*/
    pub fn mlirAffineMapIsProjectedPermutation(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /** Checks whether the given affine map represents a symbol-less permutation
 map.*/
    pub fn mlirAffineMapIsPermutation(affineMap: MlirAffineMap) -> bool;
}
unsafe extern "C" {
    /// Returns the affine map consisting of the `resultPos` subset.
    pub fn mlirAffineMapGetSubMap(
        affineMap: MlirAffineMap,
        size: isize,
        resultPos: *mut isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Returns the affine map consisting of the most major `numResults` results.
 Returns the null AffineMap if the `numResults` is equal to zero.
 Returns the `affineMap` if `numResults` is greater or equals to number of
 results of the given affine map.*/
    pub fn mlirAffineMapGetMajorSubMap(
        affineMap: MlirAffineMap,
        numResults: isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Returns the affine map consisting of the most minor `numResults` results.
 Returns the null AffineMap if the `numResults` is equal to zero.
 Returns the `affineMap` if `numResults` is greater or equals to number of
 results of the given affine map.*/
    pub fn mlirAffineMapGetMinorSubMap(
        affineMap: MlirAffineMap,
        numResults: isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Apply AffineExpr::replace(`map`) to each of the results and return a new
 new AffineMap with the new results and the specified number of dims and
 symbols.*/
    pub fn mlirAffineMapReplace(
        affineMap: MlirAffineMap,
        expression: MlirAffineExpr,
        replacement: MlirAffineExpr,
        numResultDims: isize,
        numResultSyms: isize,
    ) -> MlirAffineMap;
}
unsafe extern "C" {
    /** Returns the simplified affine map resulting from dropping the symbols that
 do not appear in any of the individual maps in `affineMaps`.
 Asserts that all maps in `affineMaps` are normalized to the same number of
 dims and symbols.
 Takes a callback `populateResult` to fill the `res` container with value
 `m` at entry `idx`. This allows returning without worrying about ownership
 considerations.*/
    pub fn mlirAffineMapCompressUnusedSymbols(
        affineMaps: *mut MlirAffineMap,
        size: isize,
        result: *mut ::core::ffi::c_void,
        populateResult: ::core::option::Option<
            unsafe extern "C" fn(
                res: *mut ::core::ffi::c_void,
                idx: isize,
                m: MlirAffineMap,
            ),
        >,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirIntegerSet {
    pub ptr: *const ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirIntegerSet"][::core::mem::size_of::<MlirIntegerSet>() - 8usize];
    ["Alignment of MlirIntegerSet"][::core::mem::align_of::<MlirIntegerSet>() - 8usize];
    [
        "Offset of field: MlirIntegerSet::ptr",
    ][::core::mem::offset_of!(MlirIntegerSet, ptr) - 0usize];
};
impl Default for MlirIntegerSet {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /// Gets the context in which the given integer set lives.
    pub fn mlirIntegerSetGetContext(set: MlirIntegerSet) -> MlirContext;
}
unsafe extern "C" {
    /// Checks whether an integer set is a null object.
    #[link_name = "mlirIntegerSetIsNull__extern"]
    pub fn mlirIntegerSetIsNull(set: MlirIntegerSet) -> bool;
}
unsafe extern "C" {
    /** Checks if two integer set objects are equal. This is a "shallow" comparison
 of two objects. Only the sets with some small number of constraints are
 uniqued and compare equal here. Set objects that represent the same integer
 set with different constraints may be considered non-equal by this check.
 Set difference followed by an (expensive) emptiness check should be used to
 check equivalence of the underlying integer sets.*/
    pub fn mlirIntegerSetEqual(s1: MlirIntegerSet, s2: MlirIntegerSet) -> bool;
}
unsafe extern "C" {
    /** Prints an integer set by sending chunks of the string representation and
 forwarding `userData to `callback`. Note that the callback may be called
 several times with consecutive chunks of the string.*/
    pub fn mlirIntegerSetPrint(
        set: MlirIntegerSet,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Prints an integer set to the standard error stream.
    pub fn mlirIntegerSetDump(set: MlirIntegerSet);
}
unsafe extern "C" {
    /** Gets or creates a new canonically empty integer set with the give number of
 dimensions and symbols in the given context.*/
    pub fn mlirIntegerSetEmptyGet(
        context: MlirContext,
        numDims: isize,
        numSymbols: isize,
    ) -> MlirIntegerSet;
}
unsafe extern "C" {
    /** Gets or creates a new integer set in the given context. The set is defined
 by a list of affine constraints, with the given number of input dimensions
 and symbols, which are treated as either equalities (eqFlags is 1) or
 inequalities (eqFlags is 0). Both `constraints` and `eqFlags` are expected
 to point to at least `numConstraint` consecutive values.*/
    pub fn mlirIntegerSetGet(
        context: MlirContext,
        numDims: isize,
        numSymbols: isize,
        numConstraints: isize,
        constraints: *const MlirAffineExpr,
        eqFlags: *const bool,
    ) -> MlirIntegerSet;
}
unsafe extern "C" {
    /** Gets or creates a new integer set in which the values and dimensions of the
 given set are replaced with the given affine expressions. `dimReplacements`
 and `symbolReplacements` are expected to point to at least as many
 consecutive expressions as the given set has dimensions and symbols,
 respectively. The new set will have `numResultDims` and `numResultSymbols`
 dimensions and symbols, respectively.*/
    pub fn mlirIntegerSetReplaceGet(
        set: MlirIntegerSet,
        dimReplacements: *const MlirAffineExpr,
        symbolReplacements: *const MlirAffineExpr,
        numResultDims: isize,
        numResultSymbols: isize,
    ) -> MlirIntegerSet;
}
unsafe extern "C" {
    /** Checks whether the given set is a canonical empty set, e.g., the set
 returned by mlirIntegerSetEmptyGet.*/
    pub fn mlirIntegerSetIsCanonicalEmpty(set: MlirIntegerSet) -> bool;
}
unsafe extern "C" {
    /// Returns the number of dimensions in the given set.
    pub fn mlirIntegerSetGetNumDims(set: MlirIntegerSet) -> isize;
}
unsafe extern "C" {
    /// Returns the number of symbols in the given set.
    pub fn mlirIntegerSetGetNumSymbols(set: MlirIntegerSet) -> isize;
}
unsafe extern "C" {
    /// Returns the number of inputs (dimensions + symbols) in the given set.
    pub fn mlirIntegerSetGetNumInputs(set: MlirIntegerSet) -> isize;
}
unsafe extern "C" {
    /** Returns the number of constraints (equalities + inequalities) in the given
 set.*/
    pub fn mlirIntegerSetGetNumConstraints(set: MlirIntegerSet) -> isize;
}
unsafe extern "C" {
    /// Returns the number of equalities in the given set.
    pub fn mlirIntegerSetGetNumEqualities(set: MlirIntegerSet) -> isize;
}
unsafe extern "C" {
    /// Returns the number of inequalities in the given set.
    pub fn mlirIntegerSetGetNumInequalities(set: MlirIntegerSet) -> isize;
}
unsafe extern "C" {
    /// Returns `pos`-th constraint of the set.
    pub fn mlirIntegerSetGetConstraint(
        set: MlirIntegerSet,
        pos: isize,
    ) -> MlirAffineExpr;
}
unsafe extern "C" {
    /** Returns `true` of the `pos`-th constraint of the set is an equality
 constraint, `false` otherwise.*/
    pub fn mlirIntegerSetIsConstraintEq(set: MlirIntegerSet, pos: isize) -> bool;
}
unsafe extern "C" {
    /// Returns an empty attribute.
    pub fn mlirAttributeGetNull() -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirAttributeIsALocation(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given attribute is an affine map attribute.
    pub fn mlirAttributeIsAAffineMap(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates an affine map attribute wrapping the given map. The attribute
 belongs to the same context as the affine map.*/
    pub fn mlirAffineMapAttrGet(map: MlirAffineMap) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirAffineMapAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the affine map wrapped in the given affine map attribute.
    pub fn mlirAffineMapAttrGetValue(attr: MlirAttribute) -> MlirAffineMap;
}
unsafe extern "C" {
    /// Returns the typeID of an AffineMap attribute.
    pub fn mlirAffineMapAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is an array attribute.
    pub fn mlirAttributeIsAArray(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates an array element containing the given list of elements in the given
 context.*/
    pub fn mlirArrayAttrGet(
        ctx: MlirContext,
        numElements: isize,
        elements: *const MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirArrayAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the number of elements stored in the given array attribute.
    pub fn mlirArrayAttrGetNumElements(attr: MlirAttribute) -> isize;
}
unsafe extern "C" {
    /// Returns pos-th element stored in the given array attribute.
    pub fn mlirArrayAttrGetElement(attr: MlirAttribute, pos: isize) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the typeID of an Array attribute.
    pub fn mlirArrayAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a dictionary attribute.
    pub fn mlirAttributeIsADictionary(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates a dictionary attribute containing the given list of elements in the
 provided context.*/
    pub fn mlirDictionaryAttrGet(
        ctx: MlirContext,
        numElements: isize,
        elements: *const MlirNamedAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDictionaryAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the number of attributes contained in a dictionary attribute.
    pub fn mlirDictionaryAttrGetNumElements(attr: MlirAttribute) -> isize;
}
unsafe extern "C" {
    /// Returns pos-th element of the given dictionary attribute.
    pub fn mlirDictionaryAttrGetElement(
        attr: MlirAttribute,
        pos: isize,
    ) -> MlirNamedAttribute;
}
unsafe extern "C" {
    /** Returns the dictionary attribute element with the given name or NULL if the
 given name does not exist in the dictionary.*/
    pub fn mlirDictionaryAttrGetElementByName(
        attr: MlirAttribute,
        name: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the typeID of a Dictionary attribute.
    pub fn mlirDictionaryAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a floating point attribute.
    pub fn mlirAttributeIsAFloat(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirFloatAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Creates a floating point attribute in the given context with the given
 double value and double-precision FP semantics.*/
    pub fn mlirFloatAttrDoubleGet(
        ctx: MlirContext,
        type_: MlirType,
        value: f64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Same as "mlirFloatAttrDoubleGet", but if the type is not valid for a
 construction of a FloatAttr, returns a null MlirAttribute.*/
    pub fn mlirFloatAttrDoubleGetChecked(
        loc: MlirLocation,
        type_: MlirType,
        value: f64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the value stored in the given floating point attribute, interpreting
 the value as double.*/
    pub fn mlirFloatAttrGetValueDouble(attr: MlirAttribute) -> f64;
}
unsafe extern "C" {
    /// Returns the typeID of a Float attribute.
    pub fn mlirFloatAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is an integer attribute.
    pub fn mlirAttributeIsAInteger(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates an integer attribute of the given type with the given integer
 value.*/
    pub fn mlirIntegerAttrGet(type_: MlirType, value: i64) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirIntegerAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the value stored in the given integer attribute, assuming the value
 is of signless type and fits into a signed 64-bit integer.*/
    pub fn mlirIntegerAttrGetValueInt(attr: MlirAttribute) -> i64;
}
unsafe extern "C" {
    /** Returns the value stored in the given integer attribute, assuming the value
 is of signed type and fits into a signed 64-bit integer.*/
    pub fn mlirIntegerAttrGetValueSInt(attr: MlirAttribute) -> i64;
}
unsafe extern "C" {
    /** Returns the value stored in the given integer attribute, assuming the value
 is of unsigned type and fits into an unsigned 64-bit integer.*/
    pub fn mlirIntegerAttrGetValueUInt(attr: MlirAttribute) -> u64;
}
unsafe extern "C" {
    /** Returns the bit width of the integer attribute's underlying APInt value.
 This is useful for determining the size of the integer, especially for
 values larger than 64 bits.*/
    pub fn mlirIntegerAttrGetValueBitWidth(attr: MlirAttribute) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /** Returns the number of 64-bit words that make up the integer attribute's
 underlying APInt value. For integers <= 64 bits, this returns 1.*/
    pub fn mlirIntegerAttrGetValueNumWords(attr: MlirAttribute) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /** Copies the 64-bit words making up the integer attribute's APInt value into
 the provided buffer. The buffer must have space for at least
 mlirIntegerAttrGetValueNumWords(attr) elements. Words are stored in
 little-endian order (least significant word first). The sign information
 is not encoded in the words themselves; use the type's signedness to
 interpret the value correctly.*/
    pub fn mlirIntegerAttrGetValueWords(attr: MlirAttribute, words: *mut u64);
}
unsafe extern "C" {
    /** Creates an integer attribute of the given type from an array of 64-bit
 words. This is useful for creating integer attributes with values with
 widths larger than 64 bits. Words are in little-endian order (least
 significant word first). The number of words must match the bit width of the
 type: numWords = ceil(bitWidth / 64).*/
    pub fn mlirIntegerAttrGetFromWords(
        type_: MlirType,
        numWords: ::core::ffi::c_uint,
        words: *const u64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the typeID of an Integer attribute.
    pub fn mlirIntegerAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a bool attribute.
    pub fn mlirAttributeIsABool(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Creates a bool attribute in the given context with the given value.
    pub fn mlirBoolAttrGet(ctx: MlirContext, value: ::core::ffi::c_int) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the value stored in the given bool attribute.
    pub fn mlirBoolAttrGetValue(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given attribute is an integer set attribute.
    pub fn mlirAttributeIsAIntegerSet(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates an integer set attribute wrapping the given set. The attribute
 belongs to the same context as the integer set.*/
    pub fn mlirIntegerSetAttrGet(set: MlirIntegerSet) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirIntegerSetAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the integer set wrapped in the given integer set attribute.
    pub fn mlirIntegerSetAttrGetValue(attr: MlirAttribute) -> MlirIntegerSet;
}
unsafe extern "C" {
    /// Returns the typeID of an IntegerSet attribute.
    pub fn mlirIntegerSetAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is an opaque attribute.
    pub fn mlirAttributeIsAOpaque(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates an opaque attribute in the given context associated with the dialect
 identified by its namespace. The attribute contains opaque byte data of the
 specified length (data need not be null-terminated).*/
    pub fn mlirOpaqueAttrGet(
        ctx: MlirContext,
        dialectNamespace: MlirStringRef,
        dataLength: isize,
        data: *const ::core::ffi::c_char,
        type_: MlirType,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirOpaqueAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the namespace of the dialect with which the given opaque attribute
 is associated. The namespace string is owned by the context.*/
    pub fn mlirOpaqueAttrGetDialectNamespace(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the raw data as a string reference. The data remains live as long as
 the context in which the attribute lives.*/
    pub fn mlirOpaqueAttrGetData(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Opaque attribute.
    pub fn mlirOpaqueAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a string attribute.
    pub fn mlirAttributeIsAString(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Creates a string attribute in the given context containing the given string.
    pub fn mlirStringAttrGet(ctx: MlirContext, str_: MlirStringRef) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirStringAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Creates a string attribute in the given context containing the given string.
 Additionally, the attribute has the given type.*/
    pub fn mlirStringAttrTypedGet(type_: MlirType, str_: MlirStringRef) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the attribute values as a string reference. The data remains live as
 long as the context in which the attribute lives.*/
    pub fn mlirStringAttrGetValue(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of a String attribute.
    pub fn mlirStringAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a symbol reference attribute.
    pub fn mlirAttributeIsASymbolRef(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates a symbol reference attribute in the given context referencing a
 symbol identified by the given string inside a list of nested references.
 Each of the references in the list must not be nested.*/
    pub fn mlirSymbolRefAttrGet(
        ctx: MlirContext,
        symbol: MlirStringRef,
        numReferences: isize,
        references: *const MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirSymbolRefAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the string reference to the root referenced symbol. The data remains
 live as long as the context in which the attribute lives.*/
    pub fn mlirSymbolRefAttrGetRootReference(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the string reference to the leaf referenced symbol. The data remains
 live as long as the context in which the attribute lives.*/
    pub fn mlirSymbolRefAttrGetLeafReference(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the number of references nested in the given symbol reference
 attribute.*/
    pub fn mlirSymbolRefAttrGetNumNestedReferences(attr: MlirAttribute) -> isize;
}
unsafe extern "C" {
    /// Returns pos-th reference nested in the given symbol reference attribute.
    pub fn mlirSymbolRefAttrGetNestedReference(
        attr: MlirAttribute,
        pos: isize,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the typeID of an SymbolRef attribute.
    pub fn mlirSymbolRefAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Creates a DistinctAttr with the referenced attribute.
    pub fn mlirDistinctAttrCreate(referencedAttr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a flat symbol reference attribute.
    pub fn mlirAttributeIsAFlatSymbolRef(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates a flat symbol reference attribute in the given context referencing a
 symbol identified by the given string.*/
    pub fn mlirFlatSymbolRefAttrGet(
        ctx: MlirContext,
        symbol: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirFlatSymbolRefAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the referenced symbol as a string reference. The data remains live
 as long as the context in which the attribute lives.*/
    pub fn mlirFlatSymbolRefAttrGetValue(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a type attribute.
    pub fn mlirAttributeIsAType(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates a type attribute wrapping the given type in the same context as the
 type.*/
    pub fn mlirTypeAttrGet(type_: MlirType) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirTypeAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the type stored in the given type attribute.
    pub fn mlirTypeAttrGetValue(attr: MlirAttribute) -> MlirType;
}
unsafe extern "C" {
    /// Returns the typeID of a Type attribute.
    pub fn mlirTypeAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a unit attribute.
    pub fn mlirAttributeIsAUnit(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Creates a unit attribute in the given context.
    pub fn mlirUnitAttrGet(ctx: MlirContext) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnitAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of a Unit attribute.
    pub fn mlirUnitAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is an elements attribute.
    pub fn mlirAttributeIsAElements(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Returns the element at the given rank-dimensional index.
    pub fn mlirElementsAttrGetValue(
        attr: MlirAttribute,
        rank: isize,
        idxs: *mut u64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Checks whether the given rank-dimensional index is valid in the given
 elements attribute.*/
    pub fn mlirElementsAttrIsValidIndex(
        attr: MlirAttribute,
        rank: isize,
        idxs: *mut u64,
    ) -> bool;
}
unsafe extern "C" {
    /** Gets the total number of elements in the given elements attribute. In order
 to iterate over the attribute, obtain its type, which must be a statically
 shaped type and use its sizes to build a multi-dimensional index.*/
    pub fn mlirElementsAttrGetNumElements(attr: MlirAttribute) -> i64;
}
unsafe extern "C" {
    pub fn mlirDenseArrayAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a dense array attribute.
    pub fn mlirAttributeIsADenseBoolArray(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseI8Array(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseI16Array(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseI32Array(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseI64Array(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseF32Array(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseF64Array(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Create a dense array attribute with the given elements.
    pub fn mlirDenseBoolArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const ::core::ffi::c_int,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseI8ArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const i8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseI16ArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const i16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseI32ArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const i32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseI64ArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const i64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseF32ArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const f32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseF64ArrayGet(
        ctx: MlirContext,
        size: isize,
        values: *const f64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /// Get the size of a dense array.
    pub fn mlirDenseArrayGetNumElements(attr: MlirAttribute) -> isize;
}
unsafe extern "C" {
    /// Get an element of a dense array.
    pub fn mlirDenseBoolArrayGetElement(attr: MlirAttribute, pos: isize) -> bool;
}
unsafe extern "C" {
    pub fn mlirDenseI8ArrayGetElement(attr: MlirAttribute, pos: isize) -> i8;
}
unsafe extern "C" {
    pub fn mlirDenseI16ArrayGetElement(attr: MlirAttribute, pos: isize) -> i16;
}
unsafe extern "C" {
    pub fn mlirDenseI32ArrayGetElement(attr: MlirAttribute, pos: isize) -> i32;
}
unsafe extern "C" {
    pub fn mlirDenseI64ArrayGetElement(attr: MlirAttribute, pos: isize) -> i64;
}
unsafe extern "C" {
    pub fn mlirDenseF32ArrayGetElement(attr: MlirAttribute, pos: isize) -> f32;
}
unsafe extern "C" {
    pub fn mlirDenseF64ArrayGetElement(attr: MlirAttribute, pos: isize) -> f64;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a dense elements attribute.
    pub fn mlirAttributeIsADenseElements(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseIntElements(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseFPElements(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /// Returns the typeID of a DenseTypedElements attribute.
    pub fn mlirDenseTypedElementsAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Deprecated API. Will be removed in the future.
    pub fn mlirDenseIntOrFPElementsAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /** Creates a dense elements attribute with the given Shaped type and elements
 in the same context as the type.*/
    pub fn mlirDenseElementsAttrGet(
        shapedType: MlirType,
        numElements: isize,
        elements: *const MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Creates a dense elements attribute with the given Shaped type and elements
 populated from a packed, row-major opaque buffer of contents.

 The format of the raw buffer is a densely packed array of values that
 can be bitcast to the storage format of the element type specified.
 Types that are not byte aligned will be:
   - For bitwidth > 1: Rounded up to the next byte.
   - For bitwidth = 1: Packed into 8bit bytes with bits corresponding to
     the linear order of the shape type from MSB to LSB, padded to on the
     right.

 A raw buffer of a single element (or for 1-bit, a byte of value 0 or 255)
 will be interpreted as a splat. User code should be prepared for additional,
 conformant patterns to be identified as splats in the future.*/
    pub fn mlirDenseElementsAttrRawBufferGet(
        shapedType: MlirType,
        rawBufferSize: usize,
        rawBuffer: *const ::core::ffi::c_void,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Creates a dense elements attribute with the given Shaped type containing a
 single replicated element (splat).*/
    pub fn mlirDenseElementsAttrSplatGet(
        shapedType: MlirType,
        element: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrBoolSplatGet(
        shapedType: MlirType,
        element: bool,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt8SplatGet(
        shapedType: MlirType,
        element: u8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt8SplatGet(
        shapedType: MlirType,
        element: i8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt32SplatGet(
        shapedType: MlirType,
        element: u32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt32SplatGet(
        shapedType: MlirType,
        element: i32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt64SplatGet(
        shapedType: MlirType,
        element: u64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt64SplatGet(
        shapedType: MlirType,
        element: i64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrFloatSplatGet(
        shapedType: MlirType,
        element: f32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrDoubleSplatGet(
        shapedType: MlirType,
        element: f64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Creates a dense elements attribute with the given shaped type from elements
 of a specific type. Expects the element type of the shaped type to match the
 data element type.*/
    pub fn mlirDenseElementsAttrBoolGet(
        shapedType: MlirType,
        numElements: isize,
        elements: *const ::core::ffi::c_int,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt8Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const u8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt8Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const i8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt16Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const u16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt16Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const i16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt32Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const u32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt32Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const i32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrUInt64Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const u64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrInt64Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const i64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrFloatGet(
        shapedType: MlirType,
        numElements: isize,
        elements: *const f32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrDoubleGet(
        shapedType: MlirType,
        numElements: isize,
        elements: *const f64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrBFloat16Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const u16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrFloat16Get(
        shapedType: MlirType,
        numElements: isize,
        elements: *const u16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Creates a dense elements attribute with the given shaped type from string
 elements.*/
    pub fn mlirDenseElementsAttrStringGet(
        shapedType: MlirType,
        numElements: isize,
        strs: *mut MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Creates a dense elements attribute that has the same data as the given dense
 elements attribute and a different shaped type. The new type must have the
 same total number of elements.*/
    pub fn mlirDenseElementsAttrReshapeGet(
        attr: MlirAttribute,
        shapedType: MlirType,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Checks whether the given dense elements attribute contains a single
 replicated value (splat).*/
    pub fn mlirDenseElementsAttrIsSplat(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Returns the single replicated value (splat) of a specific type contained by
 the given dense elements attribute.*/
    pub fn mlirDenseElementsAttrGetSplatValue(attr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetBoolSplatValue(
        attr: MlirAttribute,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt8SplatValue(attr: MlirAttribute) -> i8;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt8SplatValue(attr: MlirAttribute) -> u8;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt32SplatValue(attr: MlirAttribute) -> i32;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt32SplatValue(attr: MlirAttribute) -> u32;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt64SplatValue(attr: MlirAttribute) -> i64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt64SplatValue(attr: MlirAttribute) -> u64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetFloatSplatValue(attr: MlirAttribute) -> f32;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetDoubleSplatValue(attr: MlirAttribute) -> f64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetStringSplatValue(
        attr: MlirAttribute,
    ) -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the pos-th value (flat contiguous indexing) of a specific type
 contained by the given dense elements attribute.*/
    pub fn mlirDenseElementsAttrGetBoolValue(attr: MlirAttribute, pos: isize) -> bool;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt8Value(attr: MlirAttribute, pos: isize) -> i8;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt8Value(attr: MlirAttribute, pos: isize) -> u8;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt16Value(attr: MlirAttribute, pos: isize) -> i16;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt16Value(attr: MlirAttribute, pos: isize) -> u16;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt32Value(attr: MlirAttribute, pos: isize) -> i32;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt32Value(attr: MlirAttribute, pos: isize) -> u32;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetInt64Value(attr: MlirAttribute, pos: isize) -> i64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetUInt64Value(attr: MlirAttribute, pos: isize) -> u64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetIndexValue(attr: MlirAttribute, pos: isize) -> u64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetFloatValue(attr: MlirAttribute, pos: isize) -> f32;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetDoubleValue(attr: MlirAttribute, pos: isize) -> f64;
}
unsafe extern "C" {
    pub fn mlirDenseElementsAttrGetStringValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the raw data of the given dense elements attribute.
    pub fn mlirDenseElementsAttrGetRawData(
        attr: MlirAttribute,
    ) -> *const ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn mlirAttributeIsADenseResourceElements(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Unlike the typed accessors below, constructs the attribute with a raw
 data buffer and no type/alignment checking. Use a more strongly typed
 accessor if possible. If dataIsMutable is false, then an immutable
 AsmResourceBlob will be created and that passed data contents will be
 treated as const.
 If the deleter is non NULL, then it will be called when the data buffer
 can no longer be accessed (passing userData to it).*/
    pub fn mlirUnmanagedDenseResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        data: *mut ::core::ffi::c_void,
        dataLength: usize,
        dataAlignment: usize,
        dataIsMutable: bool,
        deleter: ::core::option::Option<
            unsafe extern "C" fn(
                userData: *mut ::core::ffi::c_void,
                data: *const ::core::ffi::c_void,
                size: usize,
                align: usize,
            ),
        >,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirDenseResourceElementsAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseBoolResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const ::core::ffi::c_int,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseUInt8ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const u8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseInt8ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const i8,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseUInt16ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const u16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseInt16ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const i16,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseUInt32ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const u32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseInt32ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const i32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseUInt64ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const u64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseInt64ResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const i64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseFloatResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const f32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirUnmanagedDenseDoubleResourceElementsAttrGet(
        shapedType: MlirType,
        name: MlirStringRef,
        numElements: isize,
        elements: *const f64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the pos-th value (flat contiguous indexing) of a specific type
 contained by the given dense resource elements attribute.*/
    pub fn mlirDenseBoolResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mlirDenseInt8ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> i8;
}
unsafe extern "C" {
    pub fn mlirDenseUInt8ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> u8;
}
unsafe extern "C" {
    pub fn mlirDenseInt16ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> i16;
}
unsafe extern "C" {
    pub fn mlirDenseUInt16ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> u16;
}
unsafe extern "C" {
    pub fn mlirDenseInt32ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> i32;
}
unsafe extern "C" {
    pub fn mlirDenseUInt32ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> u32;
}
unsafe extern "C" {
    pub fn mlirDenseInt64ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> i64;
}
unsafe extern "C" {
    pub fn mlirDenseUInt64ResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> u64;
}
unsafe extern "C" {
    pub fn mlirDenseFloatResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> f32;
}
unsafe extern "C" {
    pub fn mlirDenseDoubleResourceElementsAttrGetValue(
        attr: MlirAttribute,
        pos: isize,
    ) -> f64;
}
unsafe extern "C" {
    /// Checks whether the given attribute is a sparse elements attribute.
    pub fn mlirAttributeIsASparseElements(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    /** Creates a sparse elements attribute of the given shape from a list of
 indices and a list of associated values. Both lists are expected to be dense
 elements attributes with the same number of elements. The list of indices is
 expected to contain 64-bit integers. The attribute is created in the same
 context as the type.*/
    pub fn mlirSparseElementsAttribute(
        shapedType: MlirType,
        denseIndices: MlirAttribute,
        denseValues: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the dense elements attribute containing 64-bit integer indices of
 non-null elements in the given sparse elements attribute.*/
    pub fn mlirSparseElementsAttrGetIndices(attr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the dense elements attribute containing the non-null elements in the
 given sparse elements attribute.*/
    pub fn mlirSparseElementsAttrGetValues(attr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the typeID of a SparseElements attribute.
    pub fn mlirSparseElementsAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn mlirAttributeIsAStridedLayout(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn mlirStridedLayoutAttrGet(
        ctx: MlirContext,
        offset: i64,
        numStrides: isize,
        strides: *const i64,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirStridedLayoutAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    pub fn mlirStridedLayoutAttrGetOffset(attr: MlirAttribute) -> i64;
}
unsafe extern "C" {
    pub fn mlirStridedLayoutAttrGetNumStrides(attr: MlirAttribute) -> isize;
}
unsafe extern "C" {
    pub fn mlirStridedLayoutAttrGetStride(attr: MlirAttribute, pos: isize) -> i64;
}
unsafe extern "C" {
    /// Returns the typeID of a StridedLayout attribute.
    pub fn mlirStridedLayoutAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Returns the typeID of an Integer type.
    pub fn mlirIntegerTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an integer type.
    pub fn mlirTypeIsAInteger(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a signless integer type of the given bitwidth in the context. The
 type is owned by the context.*/
    pub fn mlirIntegerTypeGet(
        ctx: MlirContext,
        bitwidth: ::core::ffi::c_uint,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirIntegerTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Creates a signed integer type of the given bitwidth in the context. The type
 is owned by the context.*/
    pub fn mlirIntegerTypeSignedGet(
        ctx: MlirContext,
        bitwidth: ::core::ffi::c_uint,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Creates an unsigned integer type of the given bitwidth in the context. The
 type is owned by the context.*/
    pub fn mlirIntegerTypeUnsignedGet(
        ctx: MlirContext,
        bitwidth: ::core::ffi::c_uint,
    ) -> MlirType;
}
unsafe extern "C" {
    /// Returns the bitwidth of an integer type.
    pub fn mlirIntegerTypeGetWidth(type_: MlirType) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /// Checks whether the given integer type is signless.
    pub fn mlirIntegerTypeIsSignless(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given integer type is signed.
    pub fn mlirIntegerTypeIsSigned(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given integer type is unsigned.
    pub fn mlirIntegerTypeIsUnsigned(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the typeID of an Index type.
    pub fn mlirIndexTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an index type.
    pub fn mlirTypeIsAIndex(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an index type in the given context. The type is owned by the
 context.*/
    pub fn mlirIndexTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirIndexTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Checks whether the given type is a floating-point type.
    pub fn mlirTypeIsAFloat(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the bitwidth of a floating-point type.
    pub fn mlirFloatTypeGetWidth(type_: MlirType) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    /// Returns the typeID of an Float4E2M1FN type.
    pub fn mlirFloat4E2M1FNTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f4E2M1FN type.
    pub fn mlirTypeIsAFloat4E2M1FN(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f4E2M1FN type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat4E2M1FNTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat4E2M1FNTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float6E2M3FN type.
    pub fn mlirFloat6E2M3FNTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f6E2M3FN type.
    pub fn mlirTypeIsAFloat6E2M3FN(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f6E2M3FN type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat6E2M3FNTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat6E2M3FNTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float6E3M2FN type.
    pub fn mlirFloat6E3M2FNTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f6E3M2FN type.
    pub fn mlirTypeIsAFloat6E3M2FN(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f6E3M2FN type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat6E3M2FNTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat6E3M2FNTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E5M2 type.
    pub fn mlirFloat8E5M2TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E5M2 type.
    pub fn mlirTypeIsAFloat8E5M2(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E5M2 type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E5M2TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E5M2TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E4M3 type.
    pub fn mlirFloat8E4M3TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E4M3 type.
    pub fn mlirTypeIsAFloat8E4M3(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E4M3 type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E4M3TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E4M3TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E4M3FN type.
    pub fn mlirFloat8E4M3FNTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E4M3FN type.
    pub fn mlirTypeIsAFloat8E4M3FN(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E4M3FN type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E4M3FNTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E4M3FNTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E5M2FNUZ type.
    pub fn mlirFloat8E5M2FNUZTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E5M2FNUZ type.
    pub fn mlirTypeIsAFloat8E5M2FNUZ(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E5M2FNUZ type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E5M2FNUZTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E5M2FNUZTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E4M3FNUZ type.
    pub fn mlirFloat8E4M3FNUZTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E4M3FNUZ type.
    pub fn mlirTypeIsAFloat8E4M3FNUZ(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E4M3FNUZ type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E4M3FNUZTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E4M3FNUZTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E4M3B11FNUZ type.
    pub fn mlirFloat8E4M3B11FNUZTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E4M3B11FNUZ type.
    pub fn mlirTypeIsAFloat8E4M3B11FNUZ(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E4M3B11FNUZ type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E4M3B11FNUZTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E4M3B11FNUZTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E3M4 type.
    pub fn mlirFloat8E3M4TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E3M4 type.
    pub fn mlirTypeIsAFloat8E3M4(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E3M4 type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E3M4TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E3M4TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float8E8M0FNU type.
    pub fn mlirFloat8E8M0FNUTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E8M0FNU type.
    pub fn mlirTypeIsAFloat8E8M0FNU(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E8M0FNU type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E8M0FNUTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E8M0FNUTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of a Float8E5M3FNU type.
    pub fn mlirFloat8E5M3FNUTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f8E5M3FNU type.
    pub fn mlirTypeIsAFloat8E5M3FNU(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f8E5M3FNU type in the given context. The type is owned by the
 context.*/
    pub fn mlirFloat8E5M3FNUTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFloat8E5M3FNUTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an BFloat16 type.
    pub fn mlirBFloat16TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a bf16 type.
    pub fn mlirTypeIsABF16(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a bf16 type in the given context. The type is owned by the
 context.*/
    pub fn mlirBF16TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirBF16TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float16 type.
    pub fn mlirFloat16TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f16 type.
    pub fn mlirTypeIsAF16(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f16 type in the given context. The type is owned by the
 context.*/
    pub fn mlirF16TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirF16TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float32 type.
    pub fn mlirFloat32TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f32 type.
    pub fn mlirTypeIsAF32(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an f32 type in the given context. The type is owned by the
 context.*/
    pub fn mlirF32TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirF32TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Float64 type.
    pub fn mlirFloat64TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an f64 type.
    pub fn mlirTypeIsAF64(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a f64 type in the given context. The type is owned by the
 context.*/
    pub fn mlirF64TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirF64TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of a TF32 type.
    pub fn mlirFloatTF32TypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an TF32 type.
    pub fn mlirTypeIsATF32(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a TF32 type in the given context. The type is owned by the
 context.*/
    pub fn mlirTF32TypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirTF32TypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an None type.
    pub fn mlirNoneTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a None type.
    pub fn mlirTypeIsANone(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a None type in the given context. The type is owned by the
 context.*/
    pub fn mlirNoneTypeGet(ctx: MlirContext) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirNoneTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the typeID of an Complex type.
    pub fn mlirComplexTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a Complex type.
    pub fn mlirTypeIsAComplex(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a complex type with the given element type in the same context as
 the element type. The type is owned by the context.*/
    pub fn mlirComplexTypeGet(elementType: MlirType) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirComplexTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the element type of the given complex type.
    pub fn mlirComplexTypeGetElementType(type_: MlirType) -> MlirType;
}
unsafe extern "C" {
    /// Checks whether the given type is a Shaped type.
    pub fn mlirTypeIsAShaped(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the element type of the shaped type.
    pub fn mlirShapedTypeGetElementType(type_: MlirType) -> MlirType;
}
unsafe extern "C" {
    /// Checks whether the given shaped type is ranked.
    pub fn mlirShapedTypeHasRank(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the rank of the given ranked shaped type.
    pub fn mlirShapedTypeGetRank(type_: MlirType) -> i64;
}
unsafe extern "C" {
    /// Checks whether the given shaped type has a static shape.
    pub fn mlirShapedTypeHasStaticShape(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Checks whether the dim-th dimension of the given shaped type is dynamic.
    pub fn mlirShapedTypeIsDynamicDim(type_: MlirType, dim: isize) -> bool;
}
unsafe extern "C" {
    /// Checks whether the dim-th dimension of the given shaped type is static.
    pub fn mlirShapedTypeIsStaticDim(type_: MlirType, dim: isize) -> bool;
}
unsafe extern "C" {
    /// Returns the dim-th dimension of the given ranked shaped type.
    pub fn mlirShapedTypeGetDimSize(type_: MlirType, dim: isize) -> i64;
}
unsafe extern "C" {
    /** Checks whether the given value is used as a placeholder for dynamic sizes
 in shaped types.*/
    pub fn mlirShapedTypeIsDynamicSize(size: i64) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given shaped type dimension value is statically-sized.
    pub fn mlirShapedTypeIsStaticSize(size: i64) -> bool;
}
unsafe extern "C" {
    /** Returns the value indicating a dynamic size in a shaped type. Prefer
 mlirShapedTypeIsDynamicSize and mlirShapedTypeIsStaticSize to direct
 comparisons with this value.*/
    pub fn mlirShapedTypeGetDynamicSize() -> i64;
}
unsafe extern "C" {
    /** Checks whether the given value is used as a placeholder for dynamic strides
 and offsets in shaped types.*/
    pub fn mlirShapedTypeIsDynamicStrideOrOffset(val: i64) -> bool;
}
unsafe extern "C" {
    /** Checks whether the given dimension value of a stride or an offset is
 statically-sized.*/
    pub fn mlirShapedTypeIsStaticStrideOrOffset(val: i64) -> bool;
}
unsafe extern "C" {
    /** Returns the value indicating a dynamic stride or offset in a shaped type.
 Prefer mlirShapedTypeIsDynamicStrideOrOffset and
 mlirShapedTypeIsStaticStrideOrOffset to direct comparisons with this value.*/
    pub fn mlirShapedTypeGetDynamicStrideOrOffset() -> i64;
}
unsafe extern "C" {
    /// Returns the typeID of an Vector type.
    pub fn mlirVectorTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a Vector type.
    pub fn mlirTypeIsAVector(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a vector type of the shape identified by its rank and dimensions,
 with the given element type in the same context as the element type. The
 type is owned by the context.*/
    pub fn mlirVectorTypeGet(
        rank: isize,
        shape: *const i64,
        elementType: MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirVectorTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Same as "mlirVectorTypeGet" but returns a nullptr wrapping MlirType on
 illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirVectorTypeGetChecked(
        loc: MlirLocation,
        rank: isize,
        shape: *const i64,
        elementType: MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Creates a scalable vector type with the shape identified by its rank and
 dimensions. A subset of dimensions may be marked as scalable via the
 corresponding flag list, which is expected to have as many entries as the
 rank of the vector. The vector is created in the same context as the element
 type.*/
    pub fn mlirVectorTypeGetScalable(
        rank: isize,
        shape: *const i64,
        scalable: *const bool,
        elementType: MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Same as "mlirVectorTypeGetScalable" but returns a nullptr wrapping MlirType
 on illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirVectorTypeGetScalableChecked(
        loc: MlirLocation,
        rank: isize,
        shape: *const i64,
        scalable: *const bool,
        elementType: MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Checks whether the given vector type is scalable, i.e., has at least one
 scalable dimension.*/
    pub fn mlirVectorTypeIsScalable(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Checks whether the "dim"-th dimension of the given vector is scalable.
    pub fn mlirVectorTypeIsDimScalable(type_: MlirType, dim: isize) -> bool;
}
unsafe extern "C" {
    /// Checks whether the given type is a Tensor type.
    pub fn mlirTypeIsATensor(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the typeID of an RankedTensor type.
    pub fn mlirRankedTensorTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a ranked tensor type.
    pub fn mlirTypeIsARankedTensor(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the typeID of an UnrankedTensor type.
    pub fn mlirUnrankedTensorTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an unranked tensor type.
    pub fn mlirTypeIsAUnrankedTensor(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a tensor type of a fixed rank with the given shape, element type,
 and optional encoding in the same context as the element type. The type is
 owned by the context. Tensor types without any specific encoding field
 should assign mlirAttributeGetNull() to this parameter.*/
    pub fn mlirRankedTensorTypeGet(
        rank: isize,
        shape: *const i64,
        elementType: MlirType,
        encoding: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirRankedTensorTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Same as "mlirRankedTensorTypeGet" but returns a nullptr wrapping MlirType on
 illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirRankedTensorTypeGetChecked(
        loc: MlirLocation,
        rank: isize,
        shape: *const i64,
        elementType: MlirType,
        encoding: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Gets the 'encoding' attribute from the ranked tensor type, returning a null
 attribute if none.*/
    pub fn mlirRankedTensorTypeGetEncoding(type_: MlirType) -> MlirAttribute;
}
unsafe extern "C" {
    /** Creates an unranked tensor type with the given element type in the same
 context as the element type. The type is owned by the context.*/
    pub fn mlirUnrankedTensorTypeGet(elementType: MlirType) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirUnrankedTensorTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Same as "mlirUnrankedTensorTypeGet" but returns a nullptr wrapping MlirType
 on illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirUnrankedTensorTypeGetChecked(
        loc: MlirLocation,
        elementType: MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    /// Returns the typeID of an MemRef type.
    pub fn mlirMemRefTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a MemRef type.
    pub fn mlirTypeIsAMemRef(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Returns the typeID of an UnrankedMemRef type.
    pub fn mlirUnrankedMemRefTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an UnrankedMemRef type.
    pub fn mlirTypeIsAUnrankedMemRef(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a MemRef type with the given rank and shape, a potentially empty
 list of affine layout maps, the given memory space and element type, in the
 same context as element type. The type is owned by the context.*/
    pub fn mlirMemRefTypeGet(
        elementType: MlirType,
        rank: isize,
        shape: *const i64,
        layout: MlirAttribute,
        memorySpace: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirMemRefTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Same as "mlirMemRefTypeGet" but returns a nullptr-wrapping MlirType o
 illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirMemRefTypeGetChecked(
        loc: MlirLocation,
        elementType: MlirType,
        rank: isize,
        shape: *const i64,
        layout: MlirAttribute,
        memorySpace: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Creates a MemRef type with the given rank, shape, memory space and element
 type in the same context as the element type. The type has no affine maps,
 i.e. represents a default row-major contiguous memref. The type is owned by
 the context.*/
    pub fn mlirMemRefTypeContiguousGet(
        elementType: MlirType,
        rank: isize,
        shape: *const i64,
        memorySpace: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Same as "mlirMemRefTypeContiguousGet" but returns a nullptr wrapping
 MlirType on illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirMemRefTypeContiguousGetChecked(
        loc: MlirLocation,
        elementType: MlirType,
        rank: isize,
        shape: *const i64,
        memorySpace: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    /** Creates an Unranked MemRef type with the given element type and in the given
 memory space. The type is owned by the context of element type.*/
    pub fn mlirUnrankedMemRefTypeGet(
        elementType: MlirType,
        memorySpace: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirUnrankedMemRefTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Same as "mlirUnrankedMemRefTypeGet" but returns a nullptr wrapping
 MlirType on illegal arguments, emitting appropriate diagnostics.*/
    pub fn mlirUnrankedMemRefTypeGetChecked(
        loc: MlirLocation,
        elementType: MlirType,
        memorySpace: MlirAttribute,
    ) -> MlirType;
}
unsafe extern "C" {
    /// Returns the layout of the given MemRef type.
    pub fn mlirMemRefTypeGetLayout(type_: MlirType) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the affine map of the given MemRef type.
    pub fn mlirMemRefTypeGetAffineMap(type_: MlirType) -> MlirAffineMap;
}
unsafe extern "C" {
    /// Returns the memory space of the given MemRef type.
    pub fn mlirMemRefTypeGetMemorySpace(type_: MlirType) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the strides of the MemRef if the layout map is in strided form.
 Both strides and offset are out params. strides must point to pre-allocated
 memory of length equal to the rank of the memref.*/
    pub fn mlirMemRefTypeGetStridesAndOffset(
        type_: MlirType,
        strides: *mut i64,
        offset: *mut i64,
    ) -> MlirLogicalResult;
}
unsafe extern "C" {
    /// Returns the memory spcae of the given Unranked MemRef type.
    pub fn mlirUnrankedMemrefGetMemorySpace(type_: MlirType) -> MlirAttribute;
}
unsafe extern "C" {
    /// Returns the typeID of an Tuple type.
    pub fn mlirTupleTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a tuple type.
    pub fn mlirTypeIsATuple(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates a tuple type that consists of the given list of elemental types. The
 type is owned by the context.*/
    pub fn mlirTupleTypeGet(
        ctx: MlirContext,
        numElements: isize,
        elements: *const MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirTupleTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the number of types contained in a tuple.
    pub fn mlirTupleTypeGetNumTypes(type_: MlirType) -> isize;
}
unsafe extern "C" {
    /// Returns the pos-th type in the tuple type.
    pub fn mlirTupleTypeGetType(type_: MlirType, pos: isize) -> MlirType;
}
unsafe extern "C" {
    /// Returns the typeID of an Function type.
    pub fn mlirFunctionTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is a function type.
    pub fn mlirTypeIsAFunction(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /// Creates a function type, mapping a list of input types to result types.
    pub fn mlirFunctionTypeGet(
        ctx: MlirContext,
        numInputs: isize,
        inputs: *const MlirType,
        numResults: isize,
        results: *const MlirType,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirFunctionTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /// Returns the number of input types.
    pub fn mlirFunctionTypeGetNumInputs(type_: MlirType) -> isize;
}
unsafe extern "C" {
    /// Returns the number of result types.
    pub fn mlirFunctionTypeGetNumResults(type_: MlirType) -> isize;
}
unsafe extern "C" {
    /// Returns the pos-th input type.
    pub fn mlirFunctionTypeGetInput(type_: MlirType, pos: isize) -> MlirType;
}
unsafe extern "C" {
    /// Returns the pos-th result type.
    pub fn mlirFunctionTypeGetResult(type_: MlirType, pos: isize) -> MlirType;
}
unsafe extern "C" {
    /// Returns the typeID of an Opaque type.
    pub fn mlirOpaqueTypeGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    /// Checks whether the given type is an opaque type.
    pub fn mlirTypeIsAOpaque(type_: MlirType) -> bool;
}
unsafe extern "C" {
    /** Creates an opaque type in the given context associated with the dialect
 identified by its namespace. The type contains opaque byte data of the
 specified length (data need not be null-terminated).*/
    pub fn mlirOpaqueTypeGet(
        ctx: MlirContext,
        dialectNamespace: MlirStringRef,
        typeData: MlirStringRef,
    ) -> MlirType;
}
unsafe extern "C" {
    pub fn mlirOpaqueTypeGetName() -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the namespace of the dialect with which the given opaque type
 is associated. The namespace string is owned by the context.*/
    pub fn mlirOpaqueTypeGetDialectNamespace(type_: MlirType) -> MlirStringRef;
}
unsafe extern "C" {
    /** Returns the raw data as a string reference. The data remains live as long as
 the context in which the type lives.*/
    pub fn mlirOpaqueTypeGetData(type_: MlirType) -> MlirStringRef;
}
unsafe extern "C" {
    pub fn mlirGetDialectHandle__irdl__() -> MlirDialectHandle;
}
unsafe extern "C" {
    /** Loads all IRDL dialects in the provided module, registering the dialects in
 the module's associated context.*/
    pub fn mlirLoadIRDLDialects(module: MlirModule) -> MlirLogicalResult;
}
unsafe extern "C" {
    pub fn mlirIRDLVariadicityAttrGet(
        ctx: MlirContext,
        value: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirIRDLVariadicityAttrGetName() -> MlirStringRef;
}
unsafe extern "C" {
    pub fn mlirIRDLVariadicityArrayAttrGet(
        ctx: MlirContext,
        nValues: isize,
        values: *const MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirIRDLVariadicityArrayAttrGetName() -> MlirStringRef;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirMemoryEffect {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirMemoryEffect"][::core::mem::size_of::<MlirMemoryEffect>() - 8usize];
    [
        "Alignment of MlirMemoryEffect",
    ][::core::mem::align_of::<MlirMemoryEffect>() - 8usize];
    [
        "Offset of field: MlirMemoryEffect::ptr",
    ][::core::mem::offset_of!(MlirMemoryEffect, ptr) - 0usize];
};
impl Default for MlirMemoryEffect {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirMemoryEffectInstance {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirMemoryEffectInstance",
    ][::core::mem::size_of::<MlirMemoryEffectInstance>() - 8usize];
    [
        "Alignment of MlirMemoryEffectInstance",
    ][::core::mem::align_of::<MlirMemoryEffectInstance>() - 8usize];
    [
        "Offset of field: MlirMemoryEffectInstance::ptr",
    ][::core::mem::offset_of!(MlirMemoryEffectInstance, ptr) - 0usize];
};
impl Default for MlirMemoryEffectInstance {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirSideEffectResource {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirSideEffectResource",
    ][::core::mem::size_of::<MlirSideEffectResource>() - 8usize];
    [
        "Alignment of MlirSideEffectResource",
    ][::core::mem::align_of::<MlirSideEffectResource>() - 8usize];
    [
        "Offset of field: MlirSideEffectResource::ptr",
    ][::core::mem::offset_of!(MlirSideEffectResource, ptr) - 0usize];
};
impl Default for MlirSideEffectResource {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /** Returns `true` if the given operation implements an interface identified by
 its TypeID.*/
    pub fn mlirOperationImplementsInterface(
        operation: MlirOperation,
        interfaceTypeID: MlirTypeID,
    ) -> bool;
}
unsafe extern "C" {
    /** Returns `true` if the operation identified by its canonical string name
 implements the interface identified by its TypeID in the given context.
 Note that interfaces may be attached to operations in some contexts and not
 others.*/
    pub fn mlirOperationImplementsInterfaceStatic(
        operationName: MlirStringRef,
        context: MlirContext,
        interfaceTypeID: MlirTypeID,
    ) -> bool;
}
unsafe extern "C" {
    /// Returns the interface TypeID of the InferTypeOpInterface.
    pub fn mlirInferTypeOpInterfaceTypeID() -> MlirTypeID;
}
/** These callbacks are used to return multiple types from functions while
 transferring ownership to the caller. The first argument is the number of
 consecutive elements pointed to by the second argument. The third argument
 is an opaque pointer forwarded to the callback by the caller.*/
pub type MlirTypesCallback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: isize,
        arg2: *mut MlirType,
        arg3: *mut ::core::ffi::c_void,
    ),
>;
unsafe extern "C" {
    /** Infers the return types of the operation identified by its canonical given
 the arguments that will be supplied to its generic builder. Calls `callback`
 with the types of inferred arguments, potentially several times, on success.
 Returns failure otherwise.*/
    pub fn mlirInferTypeOpInterfaceInferReturnTypes(
        opName: MlirStringRef,
        context: MlirContext,
        location: MlirLocation,
        nOperands: isize,
        operands: *mut MlirValue,
        attributes: MlirAttribute,
        properties: *mut ::core::ffi::c_void,
        nRegions: isize,
        regions: *mut MlirRegion,
        callback: MlirTypesCallback,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLogicalResult;
}
unsafe extern "C" {
    /// Returns the interface TypeID of the InferShapedTypeOpInterface.
    pub fn mlirInferShapedTypeOpInterfaceTypeID() -> MlirTypeID;
}
/** These callbacks are used to return multiple shaped type components from
 functions while transferring ownership to the caller. The first argument is
 the has rank boolean followed by the the rank and a pointer to the shape
 (if applicable). The next argument is the element type, then the attribute.
 The last argument is an opaque pointer forwarded to the callback by the
 caller. This callback will be called potentially multiple times for each
 shaped type components.*/
pub type MlirShapedTypeComponentsCallback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: bool,
        arg2: isize,
        arg3: *const i64,
        arg4: MlirType,
        arg5: MlirAttribute,
        arg6: *mut ::core::ffi::c_void,
    ),
>;
unsafe extern "C" {
    /** Infers the return shaped type components of the operation. Calls `callback`
 with the types of inferred arguments on success. Returns failure otherwise.*/
    pub fn mlirInferShapedTypeOpInterfaceInferReturnTypes(
        opName: MlirStringRef,
        context: MlirContext,
        location: MlirLocation,
        nOperands: isize,
        operands: *mut MlirValue,
        attributes: MlirAttribute,
        properties: *mut ::core::ffi::c_void,
        nRegions: isize,
        regions: *mut MlirRegion,
        callback: MlirShapedTypeComponentsCallback,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLogicalResult;
}
/// The operation is not speculatable.
pub const MlirSpeculatabilityNotSpeculatable: MlirSpeculatability = 0;
/// The operation is speculatable.
pub const MlirSpeculatabilitySpeculatable: MlirSpeculatability = 1;
/// The operation is speculatable if all nested operations are speculatable.
pub const MlirSpeculatabilityRecursivelySpeculatable: MlirSpeculatability = 2;
/// Enum representing the speculatability of an operation.
pub type MlirSpeculatability = ::core::ffi::c_int;
unsafe extern "C" {
    /// Returns the interface TypeID of the ConditionallySpeculatable interface.
    pub fn mlirConditionallySpeculatableOpInterfaceTypeID() -> MlirTypeID;
}
/// Callbacks for implementing ConditionallySpeculatable from external code.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirConditionallySpeculatableOpInterfaceCallbacks {
    /// Optional constructor for user data. Set to nullptr to disable it.
    pub construct: ::core::option::Option<
        unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
    >,
    /// Optional destructor for user data. Set to nullptr to disable it.
    pub destruct: ::core::option::Option<
        unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
    >,
    /// Returns the speculatability of the given operation.
    pub getSpeculatability: ::core::option::Option<
        unsafe extern "C" fn(
            op: MlirOperation,
            userData: *mut ::core::ffi::c_void,
        ) -> MlirSpeculatability,
    >,
    pub userData: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirConditionallySpeculatableOpInterfaceCallbacks",
    ][::core::mem::size_of::<MlirConditionallySpeculatableOpInterfaceCallbacks>()
        - 32usize];
    [
        "Alignment of MlirConditionallySpeculatableOpInterfaceCallbacks",
    ][::core::mem::align_of::<MlirConditionallySpeculatableOpInterfaceCallbacks>()
        - 8usize];
    [
        "Offset of field: MlirConditionallySpeculatableOpInterfaceCallbacks::construct",
    ][::core::mem::offset_of!(
        MlirConditionallySpeculatableOpInterfaceCallbacks, construct
    ) - 0usize];
    [
        "Offset of field: MlirConditionallySpeculatableOpInterfaceCallbacks::destruct",
    ][::core::mem::offset_of!(
        MlirConditionallySpeculatableOpInterfaceCallbacks, destruct
    ) - 8usize];
    [
        "Offset of field: MlirConditionallySpeculatableOpInterfaceCallbacks::getSpeculatability",
    ][::core::mem::offset_of!(
        MlirConditionallySpeculatableOpInterfaceCallbacks, getSpeculatability
    ) - 16usize];
    [
        "Offset of field: MlirConditionallySpeculatableOpInterfaceCallbacks::userData",
    ][::core::mem::offset_of!(
        MlirConditionallySpeculatableOpInterfaceCallbacks, userData
    ) - 24usize];
};
impl Default for MlirConditionallySpeculatableOpInterfaceCallbacks {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /** Attach a new FallbackModel for the ConditionallySpeculatable interface to
 the named operation. The FallbackModel will call the provided callbacks.*/
    pub fn mlirConditionallySpeculatableOpInterfaceAttachFallbackModel(
        ctx: MlirContext,
        opName: MlirStringRef,
        callbacks: MlirConditionallySpeculatableOpInterfaceCallbacks,
    );
}
unsafe extern "C" {
    /** Returns the speculatability of the given operation.

 The operation must implement the ConditionallySpeculatable interface.*/
    pub fn mlirConditionallySpeculatableOpInterfaceGetSpeculatability(
        operation: MlirOperation,
    ) -> MlirSpeculatability;
}
unsafe extern "C" {
    /// Returns the singleton instance of the allocate memory effect.
    pub fn mlirMemoryEffectsAllocateGet() -> MlirMemoryEffect;
}
unsafe extern "C" {
    /// Returns the singleton instance of the free memory effect.
    pub fn mlirMemoryEffectsFreeGet() -> MlirMemoryEffect;
}
unsafe extern "C" {
    /// Returns the singleton instance of the read memory effect.
    pub fn mlirMemoryEffectsReadGet() -> MlirMemoryEffect;
}
unsafe extern "C" {
    /// Returns the singleton instance of the write memory effect.
    pub fn mlirMemoryEffectsWriteGet() -> MlirMemoryEffect;
}
unsafe extern "C" {
    /// Returns the TypeID identifying the concrete type of the given memory effect.
    pub fn mlirMemoryEffectGetEffectID(effect: MlirMemoryEffect) -> MlirTypeID;
}
unsafe extern "C" {
    /// Returns the singleton instance of the default side effect resource.
    pub fn mlirSideEffectsDefaultResourceGet() -> MlirSideEffectResource;
}
unsafe extern "C" {
    /** Creates a memory effect instance without an associated IR entity.
 `parameters` may be a null attribute. The caller owns the returned instance
 and must destroy it with `mlirMemoryEffectInstanceDestroy`.*/
    pub fn mlirMemoryEffectInstanceCreate(
        effect: MlirMemoryEffect,
        parameters: MlirAttribute,
        stage: ::core::ffi::c_int,
        effectOnFullRegion: bool,
        resource: MlirSideEffectResource,
    ) -> MlirMemoryEffectInstance;
}
unsafe extern "C" {
    /** Creates a memory effect instance associated with an operation operand.
 `parameters` may be a null attribute. The caller owns the returned instance
 and must destroy it with `mlirMemoryEffectInstanceDestroy`.*/
    pub fn mlirMemoryEffectInstanceCreateForOpOperand(
        effect: MlirMemoryEffect,
        opOperand: MlirOpOperand,
        parameters: MlirAttribute,
        stage: ::core::ffi::c_int,
        effectOnFullRegion: bool,
        resource: MlirSideEffectResource,
    ) -> MlirMemoryEffectInstance;
}
unsafe extern "C" {
    /** Creates a memory effect instance associated with an operation result.
 `result` must wrap an OpResult. `parameters` may be a null attribute. The
 caller owns the returned instance and must destroy it with
 `mlirMemoryEffectInstanceDestroy`.*/
    pub fn mlirMemoryEffectInstanceCreateForOpResult(
        effect: MlirMemoryEffect,
        result: MlirValue,
        parameters: MlirAttribute,
        stage: ::core::ffi::c_int,
        effectOnFullRegion: bool,
        resource: MlirSideEffectResource,
    ) -> MlirMemoryEffectInstance;
}
unsafe extern "C" {
    /** Creates a memory effect instance associated with a block argument.
 `blockArgument` must wrap a BlockArgument. `parameters` may be a null
 attribute. The caller owns the returned instance and must destroy it with
 `mlirMemoryEffectInstanceDestroy`.*/
    pub fn mlirMemoryEffectInstanceCreateForBlockArgument(
        effect: MlirMemoryEffect,
        blockArgument: MlirValue,
        parameters: MlirAttribute,
        stage: ::core::ffi::c_int,
        effectOnFullRegion: bool,
        resource: MlirSideEffectResource,
    ) -> MlirMemoryEffectInstance;
}
unsafe extern "C" {
    /** Creates a memory effect instance associated with a symbol. `symbol` must be
 a SymbolRefAttr. `parameters` may be a null attribute. The caller owns the
 returned instance and must destroy it with
 `mlirMemoryEffectInstanceDestroy`.*/
    pub fn mlirMemoryEffectInstanceCreateForSymbol(
        effect: MlirMemoryEffect,
        symbol: MlirAttribute,
        parameters: MlirAttribute,
        stage: ::core::ffi::c_int,
        effectOnFullRegion: bool,
        resource: MlirSideEffectResource,
    ) -> MlirMemoryEffectInstance;
}
unsafe extern "C" {
    /** Creates a copy of a memory effect instance. The caller must destroy
 the returned instance with `mlirMemoryEffectInstanceDestroy`.*/
    pub fn mlirMemoryEffectInstanceClone(
        instance: MlirMemoryEffectInstance,
    ) -> MlirMemoryEffectInstance;
}
unsafe extern "C" {
    /// Destroys a memory effect instance created or cloned by APIs above.
    pub fn mlirMemoryEffectInstanceDestroy(instance: MlirMemoryEffectInstance);
}
unsafe extern "C" {
    /// Returns the memory effect of the given instance.
    pub fn mlirMemoryEffectInstanceGetEffect(
        instance: MlirMemoryEffectInstance,
    ) -> MlirMemoryEffect;
}
unsafe extern "C" {
    /// Returns the side effect resource of the given instance.
    pub fn mlirMemoryEffectInstanceGetResource(
        instance: MlirMemoryEffectInstance,
    ) -> MlirSideEffectResource;
}
unsafe extern "C" {
    /// Returns the stage of the given instance.
    pub fn mlirMemoryEffectInstanceGetStage(
        instance: MlirMemoryEffectInstance,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /** Returns true if the given instance has effect on every single value of
 the resource.*/
    pub fn mlirMemoryEffectInstanceGetEffectOnFullRegion(
        instance: MlirMemoryEffectInstance,
    ) -> bool;
}
unsafe extern "C" {
    /** Returns the parameters of the given instance, or a null attribute if there
 are no parameters.*/
    pub fn mlirMemoryEffectInstanceGetParameters(
        instance: MlirMemoryEffectInstance,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    /** Returns the value (OpOperand, OpResult, or BlockArgument) of the given
 instance, or a null value if there is no associated value.*/
    pub fn mlirMemoryEffectInstanceGetValue(
        instance: MlirMemoryEffectInstance,
    ) -> MlirValue;
}
unsafe extern "C" {
    /** Returns the symbol reference of the given instance, or a null attribute if
 there is no associated symbol.*/
    pub fn mlirMemoryEffectInstanceGetSymbolRef(
        instance: MlirMemoryEffectInstance,
    ) -> MlirAttribute;
}
/** Callback for receiving a batch of memory effect instances. `effects` points
 to `numEffects` consecutive instances. Ownership is not transferred, and
 the instances are valid only while `callback` is executing. The
 caller-provided `userData` is forwarded to the callback.*/
pub type MlirMemoryEffectInstancesCallback = ::core::option::Option<
    unsafe extern "C" fn(
        numEffects: isize,
        effects: *mut MlirMemoryEffectInstance,
        userData: *mut ::core::ffi::c_void,
    ),
>;
unsafe extern "C" {
    /// Returns the interface TypeID of the MemoryEffectsOpInterface.
    pub fn mlirMemoryEffectsOpInterfaceTypeID() -> MlirTypeID;
}
/// Callbacks for implementing MemoryEffectsOpInterface from external code.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirMemoryEffectsOpInterfaceCallbacks {
    /// Optional constructor for user data. Set to nullptr to disable it.
    pub construct: ::core::option::Option<
        unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
    >,
    /// Optional destructor for user data. Set to nullptr to disable it.
    pub destruct: ::core::option::Option<
        unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
    >,
    /** Get memory effects callback. Implementations report effects by invoking
 `callback` before returning. The supplied callback copies the instances,
 so implementations retain ownership of the instances and only need to
 keep them valid until `callback` returns.*/
    pub getEffects: ::core::option::Option<
        unsafe extern "C" fn(
            op: MlirOperation,
            callback: MlirMemoryEffectInstancesCallback,
            callbackUserData: *mut ::core::ffi::c_void,
            userData: *mut ::core::ffi::c_void,
        ),
    >,
    pub userData: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirMemoryEffectsOpInterfaceCallbacks",
    ][::core::mem::size_of::<MlirMemoryEffectsOpInterfaceCallbacks>() - 32usize];
    [
        "Alignment of MlirMemoryEffectsOpInterfaceCallbacks",
    ][::core::mem::align_of::<MlirMemoryEffectsOpInterfaceCallbacks>() - 8usize];
    [
        "Offset of field: MlirMemoryEffectsOpInterfaceCallbacks::construct",
    ][::core::mem::offset_of!(MlirMemoryEffectsOpInterfaceCallbacks, construct)
        - 0usize];
    [
        "Offset of field: MlirMemoryEffectsOpInterfaceCallbacks::destruct",
    ][::core::mem::offset_of!(MlirMemoryEffectsOpInterfaceCallbacks, destruct) - 8usize];
    [
        "Offset of field: MlirMemoryEffectsOpInterfaceCallbacks::getEffects",
    ][::core::mem::offset_of!(MlirMemoryEffectsOpInterfaceCallbacks, getEffects)
        - 16usize];
    [
        "Offset of field: MlirMemoryEffectsOpInterfaceCallbacks::userData",
    ][::core::mem::offset_of!(MlirMemoryEffectsOpInterfaceCallbacks, userData)
        - 24usize];
};
impl Default for MlirMemoryEffectsOpInterfaceCallbacks {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    /** Attach a new FallbackModel for the MemoryEffectsOpInterface to the named
 operation. The FallbackModel will call the provided callbacks.*/
    pub fn mlirMemoryEffectsOpInterfaceAttachFallbackModel(
        ctx: MlirContext,
        opName: MlirStringRef,
        callbacks: MlirMemoryEffectsOpInterfaceCallbacks,
    );
}
unsafe extern "C" {
    /** Gets the memory effects of the given operation. The operation must
 implement the MemoryEffectsOpInterface. Invokes `callback` once with all
 effects. Ownership is not transferred; call
 `mlirMemoryEffectInstanceClone` from the callback to keep a copy after the
 callback returns.*/
    pub fn mlirMemoryEffectsOpInterfaceGetEffects(
        operation: MlirOperation,
        callback: MlirMemoryEffectInstancesCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /** Runs the IREE compiler main function. This is used to build
 iree-compile-like binaries that link against a common shared library.*/
    pub fn ireeCompilerRunMain(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Runs the iree-opt main function.
    pub fn ireeOptRunMain(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Runs the iree-mlir-lsp-server main function.
    pub fn ireeMlirLspServerRunMain(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /** Runs LLD in "generic" mode (i.e. as `lld`, requiring a -flavor command line
 option). This does *not* mean that we support invoking LLD as a library,
 but we do support creating busybox style tools that invoke it standalone
 by linking against the CAPI.*/
    pub fn ireeCompilerRunLldMain(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Runs the IREE module linker main function.
    pub fn ireeLinkRunMain(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    /// Runs the IREE reducer main function.
    pub fn ireeReduceRunMain(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
/** An opaque reference to a diagnostic, always owned by the diagnostics engine
 (context). Must not be stored outside of the diagnostic handler.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirDiagnostic {
    pub ptr: *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MlirDiagnostic"][::core::mem::size_of::<MlirDiagnostic>() - 8usize];
    ["Alignment of MlirDiagnostic"][::core::mem::align_of::<MlirDiagnostic>() - 8usize];
    [
        "Offset of field: MlirDiagnostic::ptr",
    ][::core::mem::offset_of!(MlirDiagnostic, ptr) - 0usize];
};
impl Default for MlirDiagnostic {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const MlirDiagnosticError: MlirDiagnosticSeverity = 0;
pub const MlirDiagnosticWarning: MlirDiagnosticSeverity = 1;
pub const MlirDiagnosticNote: MlirDiagnosticSeverity = 2;
pub const MlirDiagnosticRemark: MlirDiagnosticSeverity = 3;
/// Severity of a diagnostic.
pub type MlirDiagnosticSeverity = ::core::ffi::c_int;
/// Opaque identifier of a diagnostic handler, useful to detach a handler.
pub type MlirDiagnosticHandlerID = u64;
/** Diagnostic handler type. Accepts a reference to a diagnostic, which is only
 guaranteed to be live during the call. The handler is passed the `userData`
 that was provided when the handler was attached to a context. If the handler
 processed the diagnostic completely, it is expected to return success.
 Otherwise, it is expected to return failure to indicate that other handlers
 should attempt to process the diagnostic.*/
pub type MlirDiagnosticHandler = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: MlirDiagnostic,
        userData: *mut ::core::ffi::c_void,
    ) -> MlirLogicalResult,
>;
unsafe extern "C" {
    /// Prints a diagnostic using the provided callback.
    pub fn mlirDiagnosticPrint(
        diagnostic: MlirDiagnostic,
        callback: MlirStringCallback,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    /// Returns the location at which the diagnostic is reported.
    pub fn mlirDiagnosticGetLocation(diagnostic: MlirDiagnostic) -> MlirLocation;
}
unsafe extern "C" {
    /// Returns the severity of the diagnostic.
    pub fn mlirDiagnosticGetSeverity(
        diagnostic: MlirDiagnostic,
    ) -> MlirDiagnosticSeverity;
}
unsafe extern "C" {
    /// Returns the number of notes attached to the diagnostic.
    pub fn mlirDiagnosticGetNumNotes(diagnostic: MlirDiagnostic) -> isize;
}
unsafe extern "C" {
    /** Returns `pos`-th note attached to the diagnostic. Expects `pos` to be a
 valid zero-based index into the list of notes.*/
    pub fn mlirDiagnosticGetNote(
        diagnostic: MlirDiagnostic,
        pos: isize,
    ) -> MlirDiagnostic;
}
unsafe extern "C" {
    /** Attaches the diagnostic handler to the context. Handlers are invoked in the
 reverse order of attachment until one of them processes the diagnostic
 completely. When a handler is invoked it is passed the `userData` that was
 provided when it was attached. If non-NULL, `deleteUserData` is called once
 the system no longer needs to call the handler (for instance after the
 handler is detached or the context is destroyed). Returns an identifier that
 can be used to detach the handler.*/
    pub fn mlirContextAttachDiagnosticHandler(
        context: MlirContext,
        handler: MlirDiagnosticHandler,
        userData: *mut ::core::ffi::c_void,
        deleteUserData: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void),
        >,
    ) -> MlirDiagnosticHandlerID;
}
unsafe extern "C" {
    /** Detaches an attached diagnostic handler from the context given its
 identifier.*/
    pub fn mlirContextDetachDiagnosticHandler(
        context: MlirContext,
        id: MlirDiagnosticHandlerID,
    );
}
unsafe extern "C" {
    /** Emits an error at the given location through the diagnostics engine. Used
 for testing purposes.*/
    pub fn mlirEmitError(location: MlirLocation, message: *const ::core::ffi::c_char);
}
unsafe extern "C" {
    /** Emits a warning at the given location through the diagnostics engine. Used
 for testing purposes.*/
    pub fn mlirEmitWarning(location: MlirLocation, message: *const ::core::ffi::c_char);
}
unsafe extern "C" {
    /** Emits a remark at the given location through the diagnostics engine. Used
 for testing purposes.*/
    pub fn mlirEmitRemark(location: MlirLocation, message: *const ::core::ffi::c_char);
}
unsafe extern "C" {
    /** Apply the special region builder for the builtin named Linalg op.
 Assert that `mlirOp` is a builtin named Linalg op.*/
    pub fn mlirLinalgFillBuiltinNamedOpRegion(mlirOp: MlirOperation);
}
unsafe extern "C" {
    pub fn mlirLinalgIsAContractionOp(op: MlirOperation) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirLinalgContractionDimensions {
    pub batch: MlirAttribute,
    pub m: MlirAttribute,
    pub n: MlirAttribute,
    pub k: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirLinalgContractionDimensions",
    ][::core::mem::size_of::<MlirLinalgContractionDimensions>() - 32usize];
    [
        "Alignment of MlirLinalgContractionDimensions",
    ][::core::mem::align_of::<MlirLinalgContractionDimensions>() - 8usize];
    [
        "Offset of field: MlirLinalgContractionDimensions::batch",
    ][::core::mem::offset_of!(MlirLinalgContractionDimensions, batch) - 0usize];
    [
        "Offset of field: MlirLinalgContractionDimensions::m",
    ][::core::mem::offset_of!(MlirLinalgContractionDimensions, m) - 8usize];
    [
        "Offset of field: MlirLinalgContractionDimensions::n",
    ][::core::mem::offset_of!(MlirLinalgContractionDimensions, n) - 16usize];
    [
        "Offset of field: MlirLinalgContractionDimensions::k",
    ][::core::mem::offset_of!(MlirLinalgContractionDimensions, k) - 24usize];
};
impl Default for MlirLinalgContractionDimensions {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn mlirLinalgInferContractionDimensions(
        op: MlirOperation,
    ) -> MlirLinalgContractionDimensions;
}
unsafe extern "C" {
    pub fn mlirLinalgInferContractionDimensionsFromMaps(
        indexingMaps: *const MlirAffineMap,
        numMaps: usize,
    ) -> MlirLinalgContractionDimensions;
}
unsafe extern "C" {
    pub fn mlirLinalgIsAConvolutionOp(op: MlirOperation) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MlirLinalgConvolutionDimensions {
    pub batch: MlirAttribute,
    pub outputImage: MlirAttribute,
    pub outputChannel: MlirAttribute,
    pub filterLoop: MlirAttribute,
    pub inputChannel: MlirAttribute,
    pub depth: MlirAttribute,
    pub strides: MlirAttribute,
    pub dilations: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MlirLinalgConvolutionDimensions",
    ][::core::mem::size_of::<MlirLinalgConvolutionDimensions>() - 64usize];
    [
        "Alignment of MlirLinalgConvolutionDimensions",
    ][::core::mem::align_of::<MlirLinalgConvolutionDimensions>() - 8usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::batch",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, batch) - 0usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::outputImage",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, outputImage) - 8usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::outputChannel",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, outputChannel) - 16usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::filterLoop",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, filterLoop) - 24usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::inputChannel",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, inputChannel) - 32usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::depth",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, depth) - 40usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::strides",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, strides) - 48usize];
    [
        "Offset of field: MlirLinalgConvolutionDimensions::dilations",
    ][::core::mem::offset_of!(MlirLinalgConvolutionDimensions, dilations) - 56usize];
};
impl Default for MlirLinalgConvolutionDimensions {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn mlirLinalgInferConvolutionDimensions(
        op: MlirOperation,
    ) -> MlirLinalgConvolutionDimensions;
}
unsafe extern "C" {
    pub fn mlirLinalgInferConvolutionDimensionsFromMaps(
        indexingMaps: *const MlirAffineMap,
        numMaps: usize,
    ) -> MlirLinalgConvolutionDimensions;
}
unsafe extern "C" {
    pub fn mlirLinalgGetIndexingMapsAttribute(op: MlirOperation) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn mlirGetDialectHandle__linalg__() -> MlirDialectHandle;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgPasses();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgConvertElementwiseToLinalgPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgConvertElementwiseToLinalgPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgConvertLinalgToAffineLoopsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgConvertLinalgToAffineLoopsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgConvertLinalgToLoopsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgConvertLinalgToLoopsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgConvertLinalgToParallelLoopsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgConvertLinalgToParallelLoopsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgBlockPackMatmul() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgBlockPackMatmul();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgCategorizeOpsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgCategorizeOpsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgElementwiseOpFusionPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgElementwiseOpFusionPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgFoldIntoElementwisePass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgFoldIntoElementwisePass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgFoldUnitExtentDimsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgFoldUnitExtentDimsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgGeneralizeNamedOpsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgGeneralizeNamedOpsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgInlineScalarOperandsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgInlineScalarOperandsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgMorphOpsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgMorphOpsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgLinalgSpecializeGenericOpsPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgLinalgSpecializeGenericOpsPass();
}
unsafe extern "C" {
    pub fn mlirCreateLinalgSimplifyDepthwiseConvPass() -> MlirPass;
}
unsafe extern "C" {
    pub fn mlirRegisterLinalgSimplifyDepthwiseConvPass();
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenVMVXPipelineAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenVMVXPipelineAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenVMVXPipelineAttrGet(mlirCtx: MlirContext) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenTransformDialectCodegenPipelineAttr(
        attr: MlirAttribute,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenTransformDialectCodegenPipelineAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenTransformDialectCodegenPipelineAttrGet(
        mlirCtx: MlirContext,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenNoPipelineAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenNoPipelineAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenNoPipelineAttrGet(mlirCtx: MlirContext) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenPassPipelineAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenPassPipelineAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenPassPipelineAttrGet(
        mlirCtx: MlirContext,
        pipeline: MlirStringRef,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeCodegenPassPipelineAttrGetPipeline(attr: MlirAttribute) -> MlirStringRef;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenTranslationInfoAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenTranslationInfoAttrGetTypeID() -> MlirTypeID;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeCodegenTranslationInfoParameters {
    pub passPipeline: MlirAttribute,
    pub codegenSpec: MlirAttribute,
    pub workgroupSize: *const i64,
    pub numWorkgroupSizeElements: usize,
    pub subgroupSize: i64,
    pub configuration: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeCodegenTranslationInfoParameters",
    ][::core::mem::size_of::<ireeCodegenTranslationInfoParameters>() - 48usize];
    [
        "Alignment of ireeCodegenTranslationInfoParameters",
    ][::core::mem::align_of::<ireeCodegenTranslationInfoParameters>() - 8usize];
    [
        "Offset of field: ireeCodegenTranslationInfoParameters::passPipeline",
    ][::core::mem::offset_of!(ireeCodegenTranslationInfoParameters, passPipeline)
        - 0usize];
    [
        "Offset of field: ireeCodegenTranslationInfoParameters::codegenSpec",
    ][::core::mem::offset_of!(ireeCodegenTranslationInfoParameters, codegenSpec)
        - 8usize];
    [
        "Offset of field: ireeCodegenTranslationInfoParameters::workgroupSize",
    ][::core::mem::offset_of!(ireeCodegenTranslationInfoParameters, workgroupSize)
        - 16usize];
    [
        "Offset of field: ireeCodegenTranslationInfoParameters::numWorkgroupSizeElements",
    ][::core::mem::offset_of!(
        ireeCodegenTranslationInfoParameters, numWorkgroupSizeElements
    ) - 24usize];
    [
        "Offset of field: ireeCodegenTranslationInfoParameters::subgroupSize",
    ][::core::mem::offset_of!(ireeCodegenTranslationInfoParameters, subgroupSize)
        - 32usize];
    [
        "Offset of field: ireeCodegenTranslationInfoParameters::configuration",
    ][::core::mem::offset_of!(ireeCodegenTranslationInfoParameters, configuration)
        - 40usize];
};
impl Default for ireeCodegenTranslationInfoParameters {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeCodegenTranslationInfoAttrGet(
        mlirCtx: MlirContext,
        parameters: ireeCodegenTranslationInfoParameters,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeCodegenTranslationInfoAttrGetParameters(
        attr: MlirAttribute,
    ) -> ireeCodegenTranslationInfoParameters;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenCompilationInfoAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenCompilationInfoAttrGetTypeID() -> MlirTypeID;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeCodegenCompilationInfoParameters {
    pub loweringConfig: MlirAttribute,
    pub translationInfo: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeCodegenCompilationInfoParameters",
    ][::core::mem::size_of::<ireeCodegenCompilationInfoParameters>() - 16usize];
    [
        "Alignment of ireeCodegenCompilationInfoParameters",
    ][::core::mem::align_of::<ireeCodegenCompilationInfoParameters>() - 8usize];
    [
        "Offset of field: ireeCodegenCompilationInfoParameters::loweringConfig",
    ][::core::mem::offset_of!(ireeCodegenCompilationInfoParameters, loweringConfig)
        - 0usize];
    [
        "Offset of field: ireeCodegenCompilationInfoParameters::translationInfo",
    ][::core::mem::offset_of!(ireeCodegenCompilationInfoParameters, translationInfo)
        - 8usize];
};
impl Default for ireeCodegenCompilationInfoParameters {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeCodegenCompilationInfoAttrGet(
        mlirCtx: MlirContext,
        parameters: ireeCodegenCompilationInfoParameters,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeCodegenCompilationInfoAttrGetParameters(
        attr: MlirAttribute,
    ) -> ireeCodegenCompilationInfoParameters;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenRootOpAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenRootOpAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenRootOpAttrGet(mlirCtx: MlirContext, set: i64) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeCodegenRootOpAttrGetSet(attr: MlirAttribute) -> i64;
}
unsafe extern "C" {
    pub fn ireeCodegenGetExecutableVariantOps(
        module: MlirModule,
        numOps: *mut usize,
        executableOps: *mut MlirOperation,
    );
}
unsafe extern "C" {
    pub fn ireeCodegenGetTunerRootOps(
        module: MlirModule,
        numOps: *mut usize,
        rootOps: *mut MlirOperation,
    );
}
unsafe extern "C" {
    pub fn ireeCodegenConvertConstraintsOpToSMTLIB(
        constraintsOp: MlirOperation,
        emitReset: bool,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeCodegenMaterializeConfigurationAttrFromConstraintsOp(
        constraintsOp: MlirOperation,
        configurationAttrName: MlirStringRef,
        numAssignments: usize,
        assignmentNames: *const MlirStringRef,
        assignmentValues: *const i64,
        diagnosticMessage: *mut MlirAttribute,
    ) -> MlirAttribute;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeCodegenAttentionOpDetail {
    pub batch: MlirAttribute,
    pub m: MlirAttribute,
    pub k1: MlirAttribute,
    pub k2: MlirAttribute,
    pub n: MlirAttribute,
    pub domainRank: i64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeCodegenAttentionOpDetail",
    ][::core::mem::size_of::<ireeCodegenAttentionOpDetail>() - 48usize];
    [
        "Alignment of ireeCodegenAttentionOpDetail",
    ][::core::mem::align_of::<ireeCodegenAttentionOpDetail>() - 8usize];
    [
        "Offset of field: ireeCodegenAttentionOpDetail::batch",
    ][::core::mem::offset_of!(ireeCodegenAttentionOpDetail, batch) - 0usize];
    [
        "Offset of field: ireeCodegenAttentionOpDetail::m",
    ][::core::mem::offset_of!(ireeCodegenAttentionOpDetail, m) - 8usize];
    [
        "Offset of field: ireeCodegenAttentionOpDetail::k1",
    ][::core::mem::offset_of!(ireeCodegenAttentionOpDetail, k1) - 16usize];
    [
        "Offset of field: ireeCodegenAttentionOpDetail::k2",
    ][::core::mem::offset_of!(ireeCodegenAttentionOpDetail, k2) - 24usize];
    [
        "Offset of field: ireeCodegenAttentionOpDetail::n",
    ][::core::mem::offset_of!(ireeCodegenAttentionOpDetail, n) - 32usize];
    [
        "Offset of field: ireeCodegenAttentionOpDetail::domainRank",
    ][::core::mem::offset_of!(ireeCodegenAttentionOpDetail, domainRank) - 40usize];
};
impl Default for ireeCodegenAttentionOpDetail {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeCodegenGetAttentionOpDetail(
        qMap: MlirAffineMap,
        kMap: MlirAffineMap,
        vMap: MlirAffineMap,
        oMap: MlirAffineMap,
    ) -> ireeCodegenAttentionOpDetail;
}
unsafe extern "C" {
    pub fn ireeCodegenMlirOperationIsACodegenAttentionOp(op: MlirOperation) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeCodegenIGEMMGenericConvDetails {
    pub igemmContractionMaps: MlirAttribute,
    pub igemmLoopBounds: MlirAttribute,
    pub igemmLoopIterators: MlirAttribute,
    pub im2colOutputPerm: MlirAttribute,
    pub filterReassocIndices: MlirAttribute,
    pub isOutputChannelFirst: bool,
    pub convToIgemmDimMap: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeCodegenIGEMMGenericConvDetails",
    ][::core::mem::size_of::<ireeCodegenIGEMMGenericConvDetails>() - 56usize];
    [
        "Alignment of ireeCodegenIGEMMGenericConvDetails",
    ][::core::mem::align_of::<ireeCodegenIGEMMGenericConvDetails>() - 8usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::igemmContractionMaps",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, igemmContractionMaps)
        - 0usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::igemmLoopBounds",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, igemmLoopBounds)
        - 8usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::igemmLoopIterators",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, igemmLoopIterators)
        - 16usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::im2colOutputPerm",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, im2colOutputPerm)
        - 24usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::filterReassocIndices",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, filterReassocIndices)
        - 32usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::isOutputChannelFirst",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, isOutputChannelFirst)
        - 40usize];
    [
        "Offset of field: ireeCodegenIGEMMGenericConvDetails::convToIgemmDimMap",
    ][::core::mem::offset_of!(ireeCodegenIGEMMGenericConvDetails, convToIgemmDimMap)
        - 48usize];
};
impl Default for ireeCodegenIGEMMGenericConvDetails {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeCodegenHasIGEMMGenericConvDetails(op: MlirOperation) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenGetIGEMMGenericConvDetails(
        op: MlirOperation,
    ) -> ireeCodegenIGEMMGenericConvDetails;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeCodegenScaledContractionDimensions {
    pub batch: MlirAttribute,
    pub m: MlirAttribute,
    pub n: MlirAttribute,
    pub k: MlirAttribute,
    pub kB: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeCodegenScaledContractionDimensions",
    ][::core::mem::size_of::<ireeCodegenScaledContractionDimensions>() - 40usize];
    [
        "Alignment of ireeCodegenScaledContractionDimensions",
    ][::core::mem::align_of::<ireeCodegenScaledContractionDimensions>() - 8usize];
    [
        "Offset of field: ireeCodegenScaledContractionDimensions::batch",
    ][::core::mem::offset_of!(ireeCodegenScaledContractionDimensions, batch) - 0usize];
    [
        "Offset of field: ireeCodegenScaledContractionDimensions::m",
    ][::core::mem::offset_of!(ireeCodegenScaledContractionDimensions, m) - 8usize];
    [
        "Offset of field: ireeCodegenScaledContractionDimensions::n",
    ][::core::mem::offset_of!(ireeCodegenScaledContractionDimensions, n) - 16usize];
    [
        "Offset of field: ireeCodegenScaledContractionDimensions::k",
    ][::core::mem::offset_of!(ireeCodegenScaledContractionDimensions, k) - 24usize];
    [
        "Offset of field: ireeCodegenScaledContractionDimensions::kB",
    ][::core::mem::offset_of!(ireeCodegenScaledContractionDimensions, kB) - 32usize];
};
impl Default for ireeCodegenScaledContractionDimensions {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeCodegenMlirOperationIsAScaledContractionOp(op: MlirOperation) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenInferScaledContractionDimensions(
        op: MlirOperation,
    ) -> ireeCodegenScaledContractionDimensions;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenIntKnobAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenIntKnobAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenIntKnobAttrGetName(attr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeAttributeIsACodegenOneOfKnobAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeCodegenOneOfKnobAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeCodegenOneOfKnobAttrGetName(attr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeCodegenOneOfKnobAttrGetOptions(
        attr: MlirAttribute,
        numOptions: *mut isize,
        options: *mut MlirAttribute,
    );
}
pub type mma_intrinsic_enum_t = u32;
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUReorderWorkgroupsStrategyAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUReorderWorkgroupsStrategyAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPUReorderWorkgroupsStrategyAttrGet(
        mlirCtx: MlirContext,
        value: u32,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUReorderWorkgroupsStrategyAttrGetValue(attr: MlirAttribute) -> u32;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUPipelineAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineAttrGet(mlirCtx: MlirContext, value: u32) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineAttrGetValue(attr: MlirAttribute) -> u32;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUPipelineOptionsAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineOptionsAttrGet(
        mlirCtx: MlirContext,
        prefetchNumStages: *mut i64,
        noReduceSharedMemoryBankConflicts: *mut bool,
        useIgemmConvolution: *mut bool,
        reorderWorkgroupsStrategy: *mut MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineOptionsAttrGetPrefetchNumStages(
        attr: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineOptionsAttrGetNoReduceSharedMemoryBankConflicts(
        attr: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineOptionsAttrGetUseIgemmConvolution(
        attr: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineOptionsAttrGetReorderWorkgroupsStrategy(
        attr: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUPipelineOptionsAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUMMAIntrinsicAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUMMAIntrinsicAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPUMMAIntrinsicAttrGet(
        mlirCtx: MlirContext,
        value: mma_intrinsic_enum_t,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUMMAIntrinsicAttrGetValue(attr: MlirAttribute) -> mma_intrinsic_enum_t;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUMMAAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUMMAAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPUMMAAttrGet(
        mlirCtx: MlirContext,
        value: mma_intrinsic_enum_t,
        colMajor: bool,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUMMAAttrGetColMajor(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUVirtualMMAIntrinsicAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUVirtualMMAIntrinsicAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPUVirtualMMAIntrinsicAttrGet(
        mlirCtx: MlirContext,
        value: mma_intrinsic_enum_t,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUVirtualMMAIntrinsicAttrGetValue(
        attr: MlirAttribute,
    ) -> mma_intrinsic_enum_t;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPUVirtualMMAAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUVirtualMMAAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPUVirtualMMAAttrGet(
        mlirCtx: MlirContext,
        value: mma_intrinsic_enum_t,
        colMajor: bool,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPUVirtualMMAAttrGetColMajor(attr: MlirAttribute) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeGPUMMAInfo {
    pub aElementType: MlirType,
    pub bElementType: MlirType,
    pub cElementType: MlirType,
    pub aVectorType: MlirType,
    pub bVectorType: MlirType,
    pub cVectorType: MlirType,
    pub mElements: i64,
    pub nElements: i64,
    pub kElements: i64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ireeGPUMMAInfo"][::core::mem::size_of::<ireeGPUMMAInfo>() - 72usize];
    ["Alignment of ireeGPUMMAInfo"][::core::mem::align_of::<ireeGPUMMAInfo>() - 8usize];
    [
        "Offset of field: ireeGPUMMAInfo::aElementType",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, aElementType) - 0usize];
    [
        "Offset of field: ireeGPUMMAInfo::bElementType",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, bElementType) - 8usize];
    [
        "Offset of field: ireeGPUMMAInfo::cElementType",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, cElementType) - 16usize];
    [
        "Offset of field: ireeGPUMMAInfo::aVectorType",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, aVectorType) - 24usize];
    [
        "Offset of field: ireeGPUMMAInfo::bVectorType",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, bVectorType) - 32usize];
    [
        "Offset of field: ireeGPUMMAInfo::cVectorType",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, cVectorType) - 40usize];
    [
        "Offset of field: ireeGPUMMAInfo::mElements",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, mElements) - 48usize];
    [
        "Offset of field: ireeGPUMMAInfo::nElements",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, nElements) - 56usize];
    [
        "Offset of field: ireeGPUMMAInfo::kElements",
    ][::core::mem::offset_of!(ireeGPUMMAInfo, kElements) - 64usize];
};
impl Default for ireeGPUMMAInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeGPUMMAAttrGetInfo(attr: MlirAttribute) -> ireeGPUMMAInfo;
}
unsafe extern "C" {
    pub fn ireeGPUMMAAttrGetVirtualMMAIntrinsic(attr: MlirAttribute) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeAttributeIsAGPULoweringConfigAttr(attr: MlirAttribute) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPULoweringConfigAttrGetTypeID() -> MlirTypeID;
}
unsafe extern "C" {
    pub fn ireeGPULoweringConfigAttrGet(
        mlirCtx: MlirContext,
        attributesDictionary: MlirAttribute,
    ) -> MlirAttribute;
}
unsafe extern "C" {
    pub fn ireeGPULoweringConfigAttrGetAttributes(attr: MlirAttribute) -> MlirAttribute;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeGPUTileSizes {
    pub workgroupAttr: MlirAttribute,
    pub reductionAttr: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ireeGPUTileSizes"][::core::mem::size_of::<ireeGPUTileSizes>() - 16usize];
    [
        "Alignment of ireeGPUTileSizes",
    ][::core::mem::align_of::<ireeGPUTileSizes>() - 8usize];
    [
        "Offset of field: ireeGPUTileSizes::workgroupAttr",
    ][::core::mem::offset_of!(ireeGPUTileSizes, workgroupAttr) - 0usize];
    [
        "Offset of field: ireeGPUTileSizes::reductionAttr",
    ][::core::mem::offset_of!(ireeGPUTileSizes, reductionAttr) - 8usize];
};
impl Default for ireeGPUTileSizes {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeGPULoweringConfigAttrGetTileSizes(
        attr: MlirAttribute,
    ) -> ireeGPUTileSizes;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeGPUSubgroupBasisInfo {
    pub countsAttr: MlirAttribute,
    pub mappingAttr: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeGPUSubgroupBasisInfo",
    ][::core::mem::size_of::<ireeGPUSubgroupBasisInfo>() - 16usize];
    [
        "Alignment of ireeGPUSubgroupBasisInfo",
    ][::core::mem::align_of::<ireeGPUSubgroupBasisInfo>() - 8usize];
    [
        "Offset of field: ireeGPUSubgroupBasisInfo::countsAttr",
    ][::core::mem::offset_of!(ireeGPUSubgroupBasisInfo, countsAttr) - 0usize];
    [
        "Offset of field: ireeGPUSubgroupBasisInfo::mappingAttr",
    ][::core::mem::offset_of!(ireeGPUSubgroupBasisInfo, mappingAttr) - 8usize];
};
impl Default for ireeGPUSubgroupBasisInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeGPULoweringConfigAttrGetSubgroupBasis(
        attr: MlirAttribute,
    ) -> ireeGPUSubgroupBasisInfo;
}
unsafe extern "C" {
    pub fn ireeGPULoweringConfigAttrGetMmaKind(attr: MlirAttribute) -> MlirAttribute;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeGPUMMASingleSubgroupLayout {
    pub outer: MlirAttribute,
    pub thread: MlirAttribute,
    pub tstrides: MlirAttribute,
    pub element: MlirAttribute,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ireeGPUMMASingleSubgroupLayout",
    ][::core::mem::size_of::<ireeGPUMMASingleSubgroupLayout>() - 32usize];
    [
        "Alignment of ireeGPUMMASingleSubgroupLayout",
    ][::core::mem::align_of::<ireeGPUMMASingleSubgroupLayout>() - 8usize];
    [
        "Offset of field: ireeGPUMMASingleSubgroupLayout::outer",
    ][::core::mem::offset_of!(ireeGPUMMASingleSubgroupLayout, outer) - 0usize];
    [
        "Offset of field: ireeGPUMMASingleSubgroupLayout::thread",
    ][::core::mem::offset_of!(ireeGPUMMASingleSubgroupLayout, thread) - 8usize];
    [
        "Offset of field: ireeGPUMMASingleSubgroupLayout::tstrides",
    ][::core::mem::offset_of!(ireeGPUMMASingleSubgroupLayout, tstrides) - 16usize];
    [
        "Offset of field: ireeGPUMMASingleSubgroupLayout::element",
    ][::core::mem::offset_of!(ireeGPUMMASingleSubgroupLayout, element) - 24usize];
};
impl Default for ireeGPUMMASingleSubgroupLayout {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeGPUGetSingleSubgroupLayout(
        attr: MlirAttribute,
        fragment: u32,
    ) -> ireeGPUMMASingleSubgroupLayout;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ireeGPUTargetInfo {
    pub arch: MlirIdentifier,
    pub subgroupSizeChoices: MlirAttribute,
    pub maxWorkgroupSizes: MlirAttribute,
    pub maxThreadCountPerWorkgroup: i32,
    pub maxWorkgroupMemoryBytes: i32,
    pub mmaIntrinsics: MlirAttribute,
    pub wgpCount: u32,
    pub simdsPerWgp: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ireeGPUTargetInfo"][::core::mem::size_of::<ireeGPUTargetInfo>() - 48usize];
    [
        "Alignment of ireeGPUTargetInfo",
    ][::core::mem::align_of::<ireeGPUTargetInfo>() - 8usize];
    [
        "Offset of field: ireeGPUTargetInfo::arch",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, arch) - 0usize];
    [
        "Offset of field: ireeGPUTargetInfo::subgroupSizeChoices",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, subgroupSizeChoices) - 8usize];
    [
        "Offset of field: ireeGPUTargetInfo::maxWorkgroupSizes",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, maxWorkgroupSizes) - 16usize];
    [
        "Offset of field: ireeGPUTargetInfo::maxThreadCountPerWorkgroup",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, maxThreadCountPerWorkgroup) - 24usize];
    [
        "Offset of field: ireeGPUTargetInfo::maxWorkgroupMemoryBytes",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, maxWorkgroupMemoryBytes) - 28usize];
    [
        "Offset of field: ireeGPUTargetInfo::mmaIntrinsics",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, mmaIntrinsics) - 32usize];
    [
        "Offset of field: ireeGPUTargetInfo::wgpCount",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, wgpCount) - 40usize];
    [
        "Offset of field: ireeGPUTargetInfo::simdsPerWgp",
    ][::core::mem::offset_of!(ireeGPUTargetInfo, simdsPerWgp) - 44usize];
};
impl Default for ireeGPUTargetInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn ireeHALExecutableTargetAttrGetGPUTargetInfo(
        attr: MlirAttribute,
    ) -> ireeGPUTargetInfo;
}
unsafe extern "C" {
    pub fn ireeGPUTargetInfoGet(
        mlirCtx: MlirContext,
        arch: *const ::core::ffi::c_char,
        subgroupChoices: *const i32,
        numSubgroupChoices: usize,
        workgroupSizes: *const i32,
        numWorkgroupSizes: usize,
        threadCount: i32,
        memoryBytes: i32,
        wgpCount: u32,
        simdsPerWgp: i32,
        mmaIntrinsics: *const mma_intrinsic_enum_t,
        numMmaIntrinsics: usize,
    ) -> ireeGPUTargetInfo;
}
unsafe extern "C" {
    pub fn ireeGPUTargetInfoGetMMAIntrinsics(
        mmaIntrinsics: MlirAttribute,
        mmaIntrinsicVals: *mut mma_intrinsic_enum_t,
        virtualMmaIntrinsicTags: *mut u8,
    );
}
unsafe extern "C" {
    pub fn ireeGPUGetXorShuffleBounds(
        mmaIntrinsic: MlirAttribute,
        operandIndex: i32,
        minAccessElems: *mut i64,
        totalTileElems: *mut i64,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ireeGPUIsXORShuffleValid(
        numRowElems: i64,
        numAccessElems: i64,
        totalTileElems: i64,
    ) -> bool;
}
