//! Compiler invocations and their configuration lifecycle.
use crate::{Error, Output, Result, Source, cstring, lock, path_cstring, take};
use iree_native as n;
use std::{
    ffi::{c_char, c_int, c_void},
    marker::PhantomData,
    path::Path,
    ptr::{self, NonNull},
    sync::{Mutex, PoisonError},
};

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Informational note.
    Note,
    /// Warning.
    Warning,
    /// Error.
    Error,
    /// Optimization remark.
    Remark,
}

/// A message emitted by the compiler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity of the message.
    pub severity: Severity,
    /// Message text.
    pub message: String,
}

/// Compilation pipeline selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipeline {
    /// Standard full pipeline.
    Std,
    /// HAL executable pipeline.
    HalExecutable,
    /// Pre-compile pipeline.
    Precompile,
    /// VM pipeline.
    Vm,
}

/// A named phase in IREE's standard VM compilation pipeline.
///
/// These spellings match `enumerateIREEVMPipelinePhases` in upstream
/// `compiler/src/iree/compiler/Pipelines/Pipelines.h`; use the `_raw` setters
/// to access a phase added by a different pipeline or compiler version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    /// Input processing and lowering into core IREE input dialects.
    Input,
    /// Adjusts the program ABI for the execution environment.
    Abi,
    /// Preprocessing phase.
    Preprocessing,
    /// Global optimization phase.
    GlobalOptimization,
    /// Dispatch creation phase.
    DispatchCreation,
    /// Flow dialect phase.
    Flow,
    /// Stream dialect phase.
    Stream,
    /// Before executable configuration and code generation.
    ExecutableSources,
    /// Selects executable translation strategies.
    ExecutableConfigurations,
    /// Translates executable targets, including code generation.
    ExecutableTargets,
    /// HAL dialect phase.
    Hal,
    /// VM dialect phase.
    Vm,
    /// End of the complete compilation pipeline.
    End,
}

impl Phase {
    /// The stable mnemonic used by `ireeCompilerInvocationSetCompile*Phase`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Abi => "abi",
            Self::Preprocessing => "preprocessing",
            Self::GlobalOptimization => "global-optimization",
            Self::DispatchCreation => "dispatch-creation",
            Self::Flow => "flow",
            Self::Stream => "stream",
            Self::ExecutableSources => "executable-sources",
            Self::ExecutableConfigurations => "executable-configurations",
            Self::ExecutableTargets => "executable-targets",
            Self::Hal => "hal",
            Self::Vm => "vm",
            Self::End => "end",
        }
    }
}

impl Pipeline {
    fn raw(self) -> n::iree_compiler_pipeline_t {
        match self {
            Pipeline::Std => 0,
            Pipeline::HalExecutable => 1,
            Pipeline::Precompile => 2,
            Pipeline::Vm => 3,
        }
    }
}

type CrashHandler = Box<dyn FnMut() -> Option<Output> + Send>;

struct Shared {
    diagnostics: Mutex<Vec<Diagnostic>>,
    crash: Mutex<Option<CrashHandler>>,
}

/// One compilation (`iree_compiler_invocation_t`).
///
/// Borrows its session; the parsed source is kept alive until drop.
pub struct Invocation<'s> {
    raw: NonNull<n::iree_compiler_invocation_t>,
    session: NonNull<n::iree_compiler_session_t>,
    shared: Box<Shared>,
    source: Option<Source<'s>>,
    _session: PhantomData<&'s ()>,
}

impl<'s> Invocation<'s> {
    pub(crate) fn create(session: *mut n::iree_compiler_session_t) -> Result<Invocation<'s>> {
        let session = NonNull::new(session).ok_or_else(|| {
            Error::Native(n::Error::new(
                n::Code::InvalidArgument,
                "compiler returned an invocation without a session",
            ))
        })?;
        let shared = Box::new(Shared {
            diagnostics: Mutex::new(Vec::new()),
            crash: Mutex::new(None),
        });
        let guard = lock();
        // SAFETY: session is live and this process-wide compiler call is serialized.
        let raw = unsafe { n::ireeCompilerInvocationCreate(session.as_ptr()) };
        let raw = NonNull::new(raw).ok_or_else(|| {
            Error::Native(iree_native::Error::new(
                iree_native::Code::Unknown,
                "compiler returned a null invocation",
            ))
        })?;
        // SAFETY: raw is live; shared remains alive with the invocation and is its callback context.
        unsafe {
            n::ireeCompilerInvocationEnableCallbackDiagnostics(
                raw.as_ptr(),
                0,
                Some(diagnostic),
                ptr::from_ref(&*shared).cast_mut().cast(),
            )
        };
        drop(guard);
        Ok(Invocation {
            raw,
            session,
            shared,
            source: None,
            _session: PhantomData,
        })
    }

    fn run(&self, f: impl FnOnce() -> bool) -> Result<()> {
        let ok = {
            let _guard = lock();
            f()
        };
        if ok {
            Ok(())
        } else {
            Err(Error::Compile(self.diagnostics()))
        }
    }

    /// The underlying compiler invocation handle for interop with companion crates.
    ///
    /// The handle is borrowed and remains valid only while this invocation is alive.
    pub fn as_raw(&self) -> *mut n::iree_compiler_invocation_t {
        self.raw.as_ptr()
    }

    /// The underlying session handle for same-instance MLIR interop.
    ///
    /// The pointer is borrowed and valid only while this invocation is alive.
    pub fn session_as_raw(&self) -> *mut n::iree_compiler_session_t {
        self.session.as_ptr()
    }

    /// Diagnostics collected so far.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.shared
            .diagnostics
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clone()
    }

    /// Also print diagnostics to the console.
    pub fn enable_console_diagnostics(&mut self) {
        let _guard = lock();
        // SAFETY: self owns a live invocation and compiler calls are serialized.
        unsafe { n::ireeCompilerInvocationEnableConsoleDiagnostics(self.raw.as_ptr()) }
    }

    /// Install a crash handler returning an output for a crash reproducer.
    /// Pass `None` to remove it; otherwise pass `Some(Box::new(handler))`.
    pub fn set_crash_handler(
        &mut self,
        handler: Option<Box<dyn FnMut() -> Option<Output> + Send + 'static>>,
    ) {
        let enabled = handler.is_some();
        *self.shared.crash.lock().unwrap_or_else(|p| p.into_inner()) =
            handler.map(|h| Box::new(h) as CrashHandler);
        let _guard = lock();
        // SAFETY: self owns the invocation and its shared callback state remains live until drop.
        unsafe {
            n::ireeCompilerInvocationSetCrashHandler(
                self.raw.as_ptr(),
                enabled,
                enabled.then_some(crash as _),
                ptr::from_ref(&*self.shared).cast_mut().cast(),
            )
        }
    }

    /// Parse `source`, which the invocation then owns.
    pub fn parse_source(&mut self, source: Source<'s>) -> Result<()> {
        let raw = source.as_raw();
        // SAFETY: both handles are live during the call; source is retained after successful parsing.
        self.run(|| unsafe { n::ireeCompilerInvocationParseSource(self.raw.as_ptr(), raw) })?;
        self.source = Some(source);
        Ok(())
    }

    /// Start compilation from a standard IREE pipeline phase.
    pub fn set_compile_from_phase(&mut self, phase: crate::Phase) -> Result<()> {
        self.set_compile_from_phase_raw(phase.as_str())
    }

    /// Start compilation from a raw phase mnemonic.
    ///
    /// This escape hatch supports custom pipelines and phase names introduced
    /// by compiler versions newer than this crate.
    pub fn set_compile_from_phase_raw(&mut self, phase: &str) -> Result<()> {
        let phase = cstring(phase)?;
        let _guard = lock();
        // SAFETY: the invocation is live and phase C string outlives this synchronous call.
        unsafe { n::ireeCompilerInvocationSetCompileFromPhase(self.raw.as_ptr(), phase.as_ptr()) };
        Ok(())
    }

    /// Stop compilation after a standard IREE pipeline phase.
    pub fn set_compile_to_phase(&mut self, phase: crate::Phase) -> Result<()> {
        self.set_compile_to_phase_raw(phase.as_str())
    }

    /// Stop compilation after a raw phase mnemonic.
    ///
    /// This escape hatch supports custom pipelines and phase names introduced
    /// by compiler versions newer than this crate.
    pub fn set_compile_to_phase_raw(&mut self, phase: &str) -> Result<()> {
        let phase = cstring(phase)?;
        let _guard = lock();
        // SAFETY: the invocation is live and phase C string outlives this synchronous call.
        unsafe { n::ireeCompilerInvocationSetCompileToPhase(self.raw.as_ptr(), phase.as_ptr()) };
        Ok(())
    }

    /// Dump IR after each phase into `path`.
    pub fn set_dump_compilation_phases_to(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_cstring(path.as_ref())?;
        let _guard = lock();
        // SAFETY: the invocation is live and path C string outlives this synchronous call.
        unsafe {
            n::ireeCompilerInvocationSetDumpCompilationPhasesTo(self.raw.as_ptr(), path.as_ptr())
        };
        Ok(())
    }

    /// Enable or disable IR verification between passes.
    pub fn set_verify_ir(&mut self, enable: bool) {
        let _guard = lock();
        // SAFETY: self owns a live invocation and compiler calls are serialized.
        unsafe { n::ireeCompilerInvocationSetVerifyIR(self.raw.as_ptr(), enable) }
    }

    /// Emit optimization remarks matching the regexes into `output_file`.
    pub fn setup_remarks(&mut self, output_file: impl AsRef<Path>, filter: &str) -> Result<()> {
        let (file, filter) = (path_cstring(output_file.as_ref())?, cstring(filter)?);
        let _guard = lock();
        // SAFETY: the invocation and both C strings remain live for this synchronous call.
        unsafe {
            n::ireeCompilerInvocationSetupRemarks(self.raw.as_ptr(), file.as_ptr(), filter.as_ptr())
        };
        Ok(())
    }

    /// Run a predefined pipeline.
    pub fn pipeline(&mut self, pipeline: Pipeline) -> Result<()> {
        // SAFETY: self owns a live invocation and the pipeline value is a valid selector.
        self.run(|| unsafe { n::ireeCompilerInvocationPipeline(self.raw.as_ptr(), pipeline.raw()) })
    }

    /// Run a textual MLIR pass pipeline.
    pub fn run_pass_pipeline(&mut self, text: &str) -> Result<()> {
        let text = cstring(text)?;
        // SAFETY: the invocation is live and the pass-pipeline C string outlives the call.
        self.run(|| unsafe {
            n::ireeCompilerInvocationRunPassPipeline(self.raw.as_ptr(), text.as_ptr())
        })
    }

    fn emit(
        &self,
        output: &mut Output,
        f: impl FnOnce(
            *mut n::iree_compiler_invocation_t,
            *mut n::iree_compiler_output_t,
        ) -> *mut n::iree_compiler_error_t,
    ) -> Result<()> {
        let error = {
            let _guard = lock();
            f(self.raw.as_ptr(), output.as_raw())
        };
        // SAFETY: the native emit function returns a null or owned compiler error.
        unsafe { take(error) }.map_err(|e| match self.diagnostics() {
            ds if ds.is_empty() => e,
            ds => Error::Compile(ds),
        })
    }

    /// Print the IR as text.
    pub fn output_ir(&self, output: &mut Output) -> Result<()> {
        // SAFETY: emit holds the live invocation and mutable output handles through the call.
        self.emit(output, |inv, out| unsafe {
            n::ireeCompilerInvocationOutputIR(inv, out)
        })
    }

    /// Emit the IR as MLIR bytecode; `version` of `None` selects the latest.
    pub fn output_ir_bytecode(&self, output: &mut Output, version: Option<u32>) -> Result<()> {
        let version = version
            .map(c_int::try_from)
            .transpose()
            .map_err(|_| Error::BytecodeVersion)?
            .unwrap_or(-1);
        // SAFETY: emit holds the live invocation and mutable output handles through the call.
        self.emit(output, |inv, out| unsafe {
            n::ireeCompilerInvocationOutputIRBytecode(inv, out, version)
        })
    }

    /// Emit the VM bytecode module (VMFB) to an in-memory or caller-managed output.
    ///
    /// Use [`Output::memory`] followed by [`Output::bytes`] to retrieve the
    /// artifact in memory, or [`Invocation::output_vm_bytecode_to`] to persist
    /// it directly to disk.
    pub fn output_vm_bytecode(&self, output: &mut Output) -> Result<()> {
        // SAFETY: emit holds the live invocation and mutable output handles through the call.
        self.emit(output, |inv, out| unsafe {
            n::ireeCompilerInvocationOutputVMBytecode(inv, out)
        })
    }

    /// Compile to a VMFB file, committing it only after successful output.
    ///
    /// If compilation fails, dropping the unkept native file output removes
    /// the partial artifact.
    pub fn output_vm_bytecode_to(&self, path: impl AsRef<Path>) -> Result<()> {
        let compiler = crate::Compiler::new()?;
        let mut output = Output::file(&compiler, path)?;
        self.output_vm_bytecode(&mut output)?;
        output.keep();
        Ok(())
    }

    /// Emit the VM module as C source.
    pub fn output_vm_c_source(&self, output: &mut Output) -> Result<()> {
        // SAFETY: emit holds the live invocation and mutable output handles through the call.
        self.emit(output, |inv, out| unsafe {
            n::ireeCompilerInvocationOutputVMCSource(inv, out)
        })
    }

    /// Emit the single HAL executable.
    pub fn output_hal_executable(&self, output: &mut Output) -> Result<()> {
        // SAFETY: emit holds the live invocation and mutable output handles through the call.
        self.emit(output, |inv, out| unsafe {
            n::ireeCompilerInvocationOutputHALExecutable(inv, out)
        })
    }
}

impl Drop for Invocation<'_> {
    fn drop(&mut self) {
        {
            let _guard = lock();
            // SAFETY: drop uniquely owns this live invocation.
            unsafe { n::ireeCompilerInvocationDestroy(self.raw.as_ptr()) }
        }
        self.source = None;
    }
}

unsafe extern "C" fn diagnostic(
    severity: n::iree_compiler_diagnostic_severity_t,
    message: *const c_char,
    length: usize,
    user: *mut c_void,
) {
    // SAFETY: the invocation registered its live `Shared` state as `user`.
    let shared = unsafe { &*user.cast::<Shared>() };
    // SAFETY: the compiler supplies `length` readable message bytes.
    let Some(message) = (unsafe { crate::lossy(message, length) }) else {
        return;
    };
    n::contain(|| {
        let severity = match severity {
            n::IREE_COMPILER_DIAGNOSTIC_SEVERITY_NOTE => Severity::Note,
            n::IREE_COMPILER_DIAGNOSTIC_SEVERITY_WARNING => Severity::Warning,
            n::IREE_COMPILER_DIAGNOSTIC_SEVERITY_REMARK => Severity::Remark,
            _ => Severity::Error,
        };
        shared
            .diagnostics
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(Diagnostic { severity, message });
    });
}

unsafe extern "C" fn crash(
    out: *mut *mut n::iree_compiler_output_t,
    user: *mut c_void,
) -> *mut n::iree_compiler_error_t {
    // SAFETY: the invocation registered its live `Shared` state as `user`.
    let shared = unsafe { &*user.cast::<Shared>() };
    let produced = n::contain(|| {
        let mut handler = shared.crash.lock().unwrap_or_else(PoisonError::into_inner);
        handler.as_mut().and_then(|handler| handler())
    })
    .flatten();
    if let Some(output) = produced {
        // SAFETY: the compiler passes a writable output slot; ownership transfers to it.
        unsafe { *out = output.as_raw() };
        std::mem::forget(output);
    }
    ptr::null_mut()
}

#[cfg(test)]
mod phase_tests {
    use super::Phase;

    #[test]
    fn standard_phase_names_match_upstream_pipeline() {
        let phases = [
            (Phase::Input, "input"),
            (Phase::Abi, "abi"),
            (Phase::Preprocessing, "preprocessing"),
            (Phase::GlobalOptimization, "global-optimization"),
            (Phase::DispatchCreation, "dispatch-creation"),
            (Phase::Flow, "flow"),
            (Phase::Stream, "stream"),
            (Phase::ExecutableSources, "executable-sources"),
            (Phase::ExecutableConfigurations, "executable-configurations"),
            (Phase::ExecutableTargets, "executable-targets"),
            (Phase::Hal, "hal"),
            (Phase::Vm, "vm"),
            (Phase::End, "end"),
        ];
        for (phase, expected) in phases {
            assert_eq!(phase.as_str(), expected);
        }
    }
}
