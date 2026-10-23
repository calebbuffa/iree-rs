use crate::{Error, Result, sys};
use std::{
    ffi::{CStr, CString, c_char, c_void},
    marker::PhantomData,
    ptr::{self, NonNull},
    rc::Rc,
    sync::{Arc, Mutex, Once},
};

/// Compilation target. Artifacts are target-specific, not interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Native CPU code specialized to the machine running the compiler.
    Cpu,
    /// Portable Vulkan SPIR-V, subject to the selected target capabilities.
    Vulkan,
    /// NVIDIA CUDA executables; requires `target-cuda` and an explicit GPU target.
    Cuda,
    /// AMD GPU executables; requires `target-rocm` and an explicit GPU target.
    Rocm,
    /// Metal executables; requires `target-metal`.
    Metal,
    /// WebGPU WGSL executables; requires `target-webgpu`.
    WebGpu,
    /// Portable VM vector bytecode, available in every compiler build.
    Vmvx,
}

impl Target {
    fn backend(self) -> (&'static str, bool) {
        match self {
            Self::Cpu => (
                "llvm-cpu",
                cfg!(any(
                    feature = "cpu",
                    feature = "local-sync",
                    feature = "local-task",
                    feature = "target-llvm-cpu"
                )),
            ),
            Self::Vulkan => (
                "vulkan-spirv",
                cfg!(any(feature = "vulkan", feature = "target-vulkan")),
            ),
            Self::Cuda => ("cuda", cfg!(feature = "target-cuda")),
            Self::Rocm => ("rocm", cfg!(feature = "target-rocm")),
            Self::Metal => ("metal-spirv", cfg!(feature = "target-metal")),
            Self::WebGpu => ("webgpu-spirv", cfg!(feature = "target-webgpu")),
            Self::Vmvx => ("vmvx", true),
        }
    }
}

/// Compiler input dialect pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    /// Let IREE's registered input plugins detect the source dialects.
    Auto,
    /// StableHLO and CHLO input; requires `input-stablehlo`.
    StableHlo,
    /// TOSA input; requires `input-tosa`.
    Tosa,
    /// Torch input; requires `input-torch`.
    Torch,
    /// Already legal IREE input; do not run an external dialect conversion.
    None,
}

/// Host/device synchronization ABI for exported functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionModel {
    /// Functions wait for device work before returning.
    #[default]
    Synchronous,
    /// Functions accept wait/signal fences and return after native submission.
    ///
    /// Use [`crate::Session::call_async`] to await completion safely.
    AsyncExternal,
}

/// Reusable session-local compiler configuration.
///
/// Every compilation creates fresh native session state. Explicit flags extend
/// these presets and are passed directly to IREE, never to a shell.
#[derive(Debug, Clone)]
pub struct CompileOptions {
    target: Target,
    input: Input,
    cpu: Option<String>,
    flags: Vec<String>,
    execution_model: ExecutionModel,
}

impl CompileOptions {
    /// Configure one backend with automatic input dialect detection.
    ///
    /// Unlike [`Compiler::compile`], this does not silently specialize CPU
    /// output to the compiler's host. Select [`Self::cpu`] explicitly when
    /// targeting a particular CPU, including `"host"` for local execution.
    pub fn new(target: Target) -> Self {
        Self {
            target,
            input: Input::Auto,
            cpu: None,
            flags: Vec::new(),
            execution_model: ExecutionModel::default(),
        }
    }
    /// Select a registered input conversion pipeline.
    pub fn input(mut self, input: Input) -> Self {
        self.input = input;
        self
    }
    /// Set the LLVM CPU target, such as `"generic"`, `"host"`, or `"znver4"`.
    ///
    /// Applying this to a non-CPU backend is an error at compilation time.
    pub fn cpu(mut self, cpu: impl Into<String>) -> Self {
        self.cpu = Some(cpu.into());
        self
    }
    /// Append an upstream compiler option as one argument.
    ///
    /// Explicit options follow the presets and may override them according to
    /// IREE's option parser. Unknown options fail with native diagnostics.
    pub fn flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }
    /// Select synchronous or externally fenced asynchronous device execution.
    pub fn execution_model(mut self, model: ExecutionModel) -> Self {
        self.execution_model = model;
        self
    }

    fn arguments(&self) -> Result<Vec<String>> {
        let (backend, enabled) = self.target.backend();
        if !enabled {
            return Err(Error::invalid(format!(
                "compiler backend {backend} was not enabled at build time"
            )));
        }
        let (input, enabled) = match self.input {
            Input::Auto => ("auto", true),
            Input::StableHlo => ("stablehlo", cfg!(feature = "input-stablehlo")),
            Input::Tosa => ("tosa", cfg!(feature = "input-tosa")),
            Input::Torch => ("torch", cfg!(feature = "input-torch")),
            Input::None => ("none", true),
        };
        if !enabled {
            return Err(Error::invalid(format!(
                "compiler input {input} was not enabled at build time"
            )));
        }
        let mut flags = vec![
            format!("--iree-input-type={input}"),
            format!("--iree-hal-target-backends={backend}"),
        ];
        flags.push(format!(
            "--iree-execution-model={}",
            match self.execution_model {
                ExecutionModel::Synchronous => "async-internal",
                ExecutionModel::AsyncExternal => "async-external",
            }
        ));
        if let Some(cpu) = &self.cpu {
            if self.target != Target::Cpu {
                return Err(Error::invalid(
                    "CPU specialization requires the CPU backend",
                ));
            }
            flags.push(format!("--iree-llvmcpu-target-cpu={cpu}"));
        }
        flags.extend(self.flags.iter().cloned());
        Ok(flags)
    }
}

/// Owned VM bytecode and its embedded native executables.
#[derive(Clone)]
pub struct Artifact {
    pub(crate) bytes: Arc<[u8]>,
}

impl Artifact {
    /// Borrow the complete VMFB bytes, including embedded executable data.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Load a previously compiled artifact.
    ///
    /// # Safety
    ///
    /// The artifact must be trusted, compatible IREE compiler output. It may
    /// contain native machine code; the runtime is not a security sandbox.
    pub unsafe fn from_bytes(bytes: impl Into<Arc<[u8]>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }
}

/// A thread-local compiler handle. Each compilation has independent session state.
///
/// This wrapper is deliberately thread-local; use separate compiler sessions
/// on separate threads. Calls are serialized across handles to avoid concurrent
/// embedding deadlocks observed with the pinned compiler.
pub struct Compiler {
    _thread_local: PhantomData<Rc<()>>,
}

struct CompilerSession(NonNull<sys::iree_compiler_session_t>);

impl Drop for CompilerSession {
    fn drop(&mut self) {
        unsafe { sys::ireeCompilerSessionDestroy(self.0.as_ptr()) };
    }
}

fn compiler_result(error: *mut sys::iree_compiler_error_t) -> Result<()> {
    if error.is_null() {
        return Ok(());
    }
    // SAFETY: an error returned by the compiler owns a live message until destroy.
    let message = unsafe {
        let message = CStr::from_ptr(sys::ireeCompilerErrorGetMessage(error))
            .to_string_lossy()
            .into_owned();
        sys::ireeCompilerErrorDestroy(error);
        message
    };
    Err(Error::new(crate::StatusCode::INTERNAL, message))
}

struct Invocation {
    raw: *mut sys::iree_compiler_invocation_t,
    diagnostics: Box<Mutex<String>>,
}

impl Invocation {
    fn failure(&self, phase: &str) -> Error {
        let diagnostics = self.diagnostics.lock().unwrap_or_else(|e| e.into_inner());
        Error::new(
            crate::StatusCode::INTERNAL,
            format!("IREE {phase} failed:\n{diagnostics}"),
        )
    }
}

impl Drop for Invocation {
    fn drop(&mut self) {
        // SAFETY: invocation is owned, and callbacks stop before diagnostics drop.
        unsafe { sys::ireeCompilerInvocationDestroy(self.raw) };
    }
}

unsafe extern "C" fn diagnostic(
    _severity: sys::iree_compiler_diagnostic_severity_t,
    message: *const c_char,
    length: usize,
    user_data: *mut c_void,
) {
    // SAFETY: IREE supplies length valid bytes and the registered boxed mutex.
    let bytes = unsafe { std::slice::from_raw_parts(message.cast::<u8>(), length) };
    let diagnostics = unsafe { &*user_data.cast::<Mutex<String>>() };
    let mut text = diagnostics.lock().unwrap_or_else(|e| e.into_inner());
    text.push_str(&String::from_utf8_lossy(bytes));
    text.push('\n');
}

struct Source(*mut sys::iree_compiler_source_t);
impl Drop for Source {
    fn drop(&mut self) {
        // SAFETY: this source is owned and its backing CString remains live.
        unsafe { sys::ireeCompilerSourceDestroy(self.0) };
    }
}

struct Output(*mut sys::iree_compiler_output_t);
impl Drop for Output {
    fn drop(&mut self) {
        // SAFETY: the memory mapping has been copied before destruction.
        unsafe { sys::ireeCompilerOutputDestroy(self.0) };
    }
}

impl Compiler {
    /// Initialize the process-wide compiler and create a thread-local handle.
    ///
    /// Global state remains alive until process exit. All compilations through
    /// this wrapper are serialized; the compiler may parallelize each invocation.
    pub fn new() -> Result<Self> {
        static INITIALIZE: Once = Once::new();
        INITIALIZE.call_once(|| {
            // Keep the process-global compiler alive: final shutdown is irreversible.
            unsafe { sys::ireeCompilerGlobalInitialize() };
        });
        Ok(Self {
            _thread_local: PhantomData,
        })
    }

    /// Return the source revision embedded in the native compiler.
    pub fn revision(&self) -> String {
        // SAFETY: global initialization remains live for the process lifetime.
        unsafe { CStr::from_ptr(sys::ireeCompilerGetRevision()) }
            .to_string_lossy()
            .into_owned()
    }

    /// Compile MLIR StableHLO/CHLO assembly, entirely in memory.
    pub fn compile(&mut self, source: &str, target: Target) -> Result<Artifact> {
        let (backend, enabled) = target.backend();
        if !enabled {
            return Err(Error::invalid(format!(
                "compiler backend {backend} was not enabled at build time"
            )));
        }
        if !cfg!(feature = "input-stablehlo") {
            return Err(Error::invalid(
                "compile requires input-stablehlo; use compile_with_flags for other input dialects",
            ));
        }
        let flags = match target {
            Target::Cpu => &[
                "--iree-input-type=stablehlo",
                "--iree-hal-target-backends=llvm-cpu",
                "--iree-llvmcpu-target-cpu=host",
            ][..],
            Target::Vulkan => &[
                "--iree-input-type=stablehlo",
                "--iree-hal-target-backends=vulkan-spirv",
            ][..],
            _ => {
                let backend_flag = format!("--iree-hal-target-backends={backend}");
                return self
                    .compile_with_flags(source, &["--iree-input-type=stablehlo", &backend_flag]);
            }
        };
        self.compile_with_flags(source, flags)
    }

    /// Compile with a reusable typed configuration and explicit native options.
    pub fn compile_with_options(
        &mut self,
        source: &str,
        options: &CompileOptions,
    ) -> Result<Artifact> {
        let arguments = options.arguments()?;
        let arguments: Vec<_> = arguments.iter().map(String::as_str).collect();
        self.compile_with_flags(source, &arguments)
    }

    /// Compile with explicit IREE session-local options.
    ///
    /// Flags are passed as separate arguments, not parsed as a shell command.
    /// Each call uses a fresh native session; flags never leak between calls.
    pub fn compile_with_flags(&mut self, source: &str, flags: &[&str]) -> Result<Artifact> {
        // Concurrent embedding invocations deadlocked during qualification.
        // Passes within one invocation still use IREE's internal parallelism.
        static COMPILE: Mutex<()> = Mutex::new(());
        let _guard = COMPILE
            .lock()
            .map_err(|_| Error::invalid("IREE compiler lock was poisoned"))?;
        let session = CompilerSession(
            NonNull::new(unsafe { sys::ireeCompilerSessionCreate() })
                .ok_or_else(|| Error::invalid("IREE returned a null compiler session"))?,
        );
        let flags = flags
            .iter()
            .map(|flag| CString::new(*flag).map_err(|_| Error::invalid("NUL in compiler flag")))
            .collect::<Result<Vec<_>>>()?;
        let argv = flags.iter().map(|flag| flag.as_ptr()).collect::<Vec<_>>();
        let argc =
            i32::try_from(argv.len()).map_err(|_| Error::invalid("too many compiler flags"))?;
        compiler_result(unsafe {
            sys::ireeCompilerSessionSetFlags(session.0.as_ptr(), argc, argv.as_ptr())
        })?;

        let text = CString::new(source).map_err(|_| Error::invalid("NUL in MLIR source"))?;
        let mut source = ptr::null_mut();
        compiler_result(unsafe {
            sys::ireeCompilerSourceWrapBuffer(
                session.0.as_ptr(),
                c"input.mlir".as_ptr(),
                text.as_ptr(),
                text.as_bytes_with_nul().len(),
                true,
                &mut source,
            )
        })?;
        let source = Source(source);
        let invocation_raw =
            NonNull::new(unsafe { sys::ireeCompilerInvocationCreate(session.0.as_ptr()) })
                .ok_or_else(|| Error::invalid("IREE returned a null compiler invocation"))?;
        let mut invocation = Invocation {
            raw: invocation_raw.as_ptr(),
            diagnostics: Box::new(Mutex::new(String::new())),
        };
        unsafe {
            sys::ireeCompilerInvocationEnableCallbackDiagnostics(
                invocation.raw,
                0,
                Some(diagnostic),
                (&mut *invocation.diagnostics as *mut Mutex<String>).cast(),
            );
        }
        if !unsafe { sys::ireeCompilerInvocationParseSource(invocation.raw, source.0) } {
            return Err(invocation.failure("source parsing"));
        }
        if !unsafe { sys::ireeCompilerInvocationPipeline(invocation.raw, 0) } {
            return Err(invocation.failure("compilation"));
        }
        let mut output = ptr::null_mut();
        compiler_result(unsafe { sys::ireeCompilerOutputOpenMembuffer(&mut output) })?;
        let output = Output(output);
        compiler_result(unsafe {
            sys::ireeCompilerInvocationOutputVMBytecode(invocation.raw, output.0)
        })?;
        let mut data = ptr::null_mut();
        let mut length = 0;
        compiler_result(unsafe {
            sys::ireeCompilerOutputMapMemory(output.0, &mut data, &mut length)
        })?;
        let length = usize::try_from(length)
            .map_err(|_| Error::invalid("compiled artifact exceeds address space"))?;
        if length == 0 || length > isize::MAX as usize || data.is_null() {
            return Err(Error::invalid("invalid compiler output memory range"));
        }
        // SAFETY: output owns this mapping until drop; copy detaches the artifact.
        let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), length) };
        Ok(Artifact {
            bytes: Arc::from(bytes),
        })
    }
}
