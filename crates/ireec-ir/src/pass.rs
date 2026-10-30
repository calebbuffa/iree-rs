//! Pass-manager ownership follows the native APIs: a manager owns every pass
//! transferred into it, while nested managers are borrowed from their parent.
//! Pipeline parsing and execution report native failures as [`Error`] values.

use crate::{
    Borrowed, Context, Error, OpPrintingFlags, Operation, Result,
    callback::{capture, contain, print_with},
    string_ref::string_ref,
};
use iree_native as n;
use std::{
    ffi::c_void,
    marker::PhantomData,
    ptr,
    sync::{Mutex, PoisonError},
};

/// An owned MLIR pass that is destroyed unless transferred to a manager.
pub struct Pass<'ctx> {
    raw: n::MlirPass,
    context: n::MlirContext,
    anchor: Option<Box<str>>,
    _context: PhantomData<&'ctx ()>,
}

/// A pass manager that owns its pass pipeline and is tied to its context.
pub struct PassManager<'ctx> {
    raw: n::MlirPassManager,
    context: n::MlirContext,
    anchor: Option<Box<str>>,
    _context: PhantomData<&'ctx ()>,
}

/// A nested pass manager borrowed from its owning pass manager.
pub struct OpPassManager<'pm> {
    raw: n::MlirOpPassManager,
    context: n::MlirContext,
    anchor: Option<Box<str>>,
    _parent: PhantomData<&'pm mut ()>,
}

/// A Rust-authored external MLIR pass.
///
/// `clone_box` must return an independent clone with equivalent pass behavior.
/// Each native clone owns its returned value, and each instance is dropped once
/// by MLIR's external-pass destructor callback.
pub trait ExternalPass: Send + 'static {
    /// Clone this pass instance for MLIR's pass-manager scheduling.
    fn clone_box(&self) -> Box<dyn ExternalPass>;

    /// Initialize this pass before the first run on a context.
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Run on the current operation. Returning an error fails the pass.
    fn run(&mut self, operation: &mut Operation<'_, Borrowed>) -> Result<()>;
}

/// Per-instance external pass state; `None` after a failed clone.
type ExternalState = Mutex<Option<Box<dyn ExternalPass>>>;

/// IR printing configuration for [`PassManager::enable_ir_printing`].
pub struct IrPrinting<'a> {
    /// Print before every pass.
    pub before_all: bool,
    /// Print after every pass.
    pub after_all: bool,
    /// Print the whole module rather than only the pass's anchor operation.
    pub module_scope: bool,
    /// With `after_all`, print only when the pass changed the IR.
    pub after_only_on_change: bool,
    /// With `after_all`, print only when the pass failed.
    pub after_only_on_failure: bool,
    /// Flags controlling the printed form.
    pub flags: &'a OpPrintingFlags,
    /// Directory for per-pass tree dumps instead of standard error.
    pub tree_printing_path: Option<&'a str>,
}

/// How pass statistics are displayed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatisticsDisplay {
    /// One flat list of statistics.
    #[default]
    List,
    /// Statistics nested under their pipeline structure.
    Pipeline,
}

impl<'ctx> Pass<'ctx> {
    /// Create an external pass implemented by Rust.
    ///
    /// The pass is anchored on `operation_name`; use an empty name for a
    /// generic operation pass. Names and descriptions must not contain NUL.
    /// Each Rust pass type gets its own MLIR TypeID.
    pub fn external<Ctx, P: ExternalPass>(
        context: &'ctx Context<'_, Ctx>,
        name: &str,
        argument: &str,
        description: &str,
        operation_name: &str,
        pass: P,
    ) -> Result<Self> {
        if [name, argument, description, operation_name]
            .iter()
            .any(|value| value.contains('\0'))
            || name.is_empty()
        {
            return Err(Error::Invalid(
                "external pass names must be valid C strings".into(),
            ));
        }
        let state: Box<ExternalState> = Box::new(Mutex::new(Some(Box::new(pass))));
        let data = Box::into_raw(state).cast::<c_void>();
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe {
            n::mlirCreateExternalPass(
                n::mlirTypeIDCreate(pass_identity::<P>()),
                string_ref(name),
                string_ref(argument),
                string_ref(description),
                string_ref(operation_name),
                0,
                ptr::null_mut(),
                n::MlirExternalPassCallbacks {
                    construct: Some(external_construct),
                    destruct: Some(external_destruct),
                    initialize: Some(external_initialize),
                    clone: Some(external_clone),
                    run: Some(external_run),
                },
                data,
            )
        };
        if raw.ptr.is_null() {
            // Ownership transfers only when mlirCreateExternalPass succeeds.
            // SAFETY: this deleter reconstructs the unique Box pointer transferred at registration, and the native API invokes it once.
            drop(unsafe { Box::from_raw(data.cast::<ExternalState>()) });
            return Err(Error::Null("external MLIR pass"));
        }
        Ok(Self {
            raw,
            context: context.raw,
            anchor: (!operation_name.is_empty()).then(|| operation_name.into()),
            _context: PhantomData,
        })
    }

    /// Take ownership of a native pass created for `context`.
    ///
    /// `anchor` is the operation name the pass is restricted to, or `None`
    /// for an operation-agnostic pass.
    ///
    /// # Safety
    ///
    /// `raw` must be a non-null pass handle owned by the caller and created
    /// for `context`, and `anchor` must match the pass's real anchor: MLIR
    /// aborts the process when an anchored pass joins a mismatched manager.
    pub unsafe fn from_raw<Ctx>(
        context: &'ctx Context<'_, Ctx>,
        raw: n::MlirPass,
        anchor: Option<&str>,
    ) -> Result<Self> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR pass"));
        }
        if let Some(anchor) = anchor {
            validate_operation_name(anchor)?;
        }
        Ok(Self {
            raw,
            context: context.raw,
            anchor: anchor.map(Into::into),
            _context: PhantomData,
        })
    }

    /// The operation name this pass is restricted to, if any.
    pub fn anchor(&self) -> Option<&str> {
        self.anchor.as_deref()
    }

    /// Borrow the native pass handle.
    pub fn as_raw(&self) -> n::MlirPass {
        self.raw
    }

    /// Transfer the pass handle to the caller.
    ///
    /// # Safety
    ///
    /// The caller must destroy or transfer the handle exactly once and keep its
    /// context alive for any context-dependent behavior.
    pub unsafe fn into_raw(mut self) -> n::MlirPass {
        let raw = self.raw;
        self.raw = n::MlirPass {
            ptr: ptr::null_mut(),
        };
        raw
    }
}

impl Drop for Pass<'_> {
    fn drop(&mut self) {
        if self.raw.ptr.is_null() {
            return;
        }
        // MLIR exposes pass destruction only through manager ownership. A
        // temporary manager on the pass's own anchor ("any" when unanchored)
        // releases it without tripping MLIR's fatal anchor-mismatch check.
        let anchor = self.anchor.as_deref().unwrap_or(ANY_ANCHOR);
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        unsafe {
            let manager = n::mlirPassManagerCreateOnOperation(self.context, string_ref(anchor));
            if !manager.ptr.is_null() {
                n::mlirPassManagerAddOwnedPass(manager, self.raw);
                n::mlirPassManagerDestroy(manager);
            }
        }
        self.raw = n::MlirPass {
            ptr: ptr::null_mut(),
        };
    }
}

impl<'ctx> PassManager<'ctx> {
    /// Create an operation-agnostic manager (anchored on `any`).
    pub fn new<Ctx>(context: &'ctx Context<'_, Ctx>) -> Result<Self> {
        Self::create(context, None)
    }

    /// Create a manager anchored on operations with the given name.
    pub fn new_on_operation<Ctx>(context: &'ctx Context<'_, Ctx>, name: &str) -> Result<Self> {
        if name.is_empty() || name.contains('\0') {
            return Err(Error::Invalid(
                "pass-manager operation name is invalid".into(),
            ));
        }
        Self::create(context, Some(name))
    }

    fn create<Ctx>(context: &'ctx Context<'_, Ctx>, name: Option<&str>) -> Result<Self> {
        let anchor = name.unwrap_or(ANY_ANCHOR);
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        let raw = unsafe { n::mlirPassManagerCreateOnOperation(context.raw, string_ref(anchor)) };
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR pass manager"));
        }
        Ok(Self {
            raw,
            context: context.raw,
            anchor: name.map(Into::into),
            _context: PhantomData,
        })
    }

    /// Borrow the native pass-manager handle.
    pub fn as_raw(&self) -> n::MlirPassManager {
        self.raw
    }

    /// Run this pipeline on a mutable operation from the manager's context.
    pub fn run<Ownership>(&mut self, operation: &mut Operation<'_, Ownership>) -> Result<()> {
        // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationGetContext`.
        if unsafe { n::mlirOperationGetContext(operation.raw) }.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "operation belongs to another MLIR context".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerRunOnOp`.
        let result = unsafe { n::mlirPassManagerRunOnOp(self.raw, operation.raw) };
        if n::compiler::logical_result::is_success(result) {
            Ok(())
        } else {
            Err(Error::Invalid("MLIR pass pipeline failed".into()))
        }
    }

    /// Enable or disable verification after each pass.
    pub fn enable_verifier(&mut self, enable: bool) {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerEnableVerifier`.
        unsafe { n::mlirPassManagerEnableVerifier(self.raw, enable) };
    }

    /// Enable IR printing around passes as configured by `options`.
    pub fn enable_ir_printing(&mut self, options: &IrPrinting<'_>) {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        unsafe {
            n::mlirPassManagerEnableIRPrinting(
                self.raw,
                options.before_all,
                options.after_all,
                options.module_scope,
                options.after_only_on_change,
                options.after_only_on_failure,
                options.flags.raw,
                string_ref(options.tree_printing_path.unwrap_or("")),
            )
        };
    }

    /// Enable pass timing output.
    pub fn enable_timing(&mut self) {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerEnableTiming`.
        unsafe { n::mlirPassManagerEnableTiming(self.raw) };
    }

    /// Enable pass statistics with the given display form.
    pub fn enable_statistics(&mut self, display: StatisticsDisplay) {
        let mode = match display {
            StatisticsDisplay::List => n::MLIR_PASS_DISPLAY_MODE_LIST,
            StatisticsDisplay::Pipeline => n::MLIR_PASS_DISPLAY_MODE_PIPELINE,
        };
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerEnableStatistics`.
        unsafe { n::mlirPassManagerEnableStatistics(self.raw, mode) };
    }

    /// Borrow the root operation pass manager.
    pub fn as_op_pass_manager(&mut self) -> OpPassManager<'_> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerGetAsOpPassManager`.
        let raw = unsafe { n::mlirPassManagerGetAsOpPassManager(self.raw) };
        OpPassManager {
            raw,
            context: self.context,
            anchor: self.anchor.clone(),
            _parent: PhantomData,
        }
    }

    /// Borrow a manager nested under operations with this name.
    pub fn nested_under(&mut self, operation_name: &str) -> Result<OpPassManager<'_>> {
        validate_operation_name(operation_name)?;
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let raw = unsafe { n::mlirPassManagerGetNestedUnder(self.raw, string_ref(operation_name)) };
        if raw.ptr.is_null() {
            return Err(Error::Null("nested MLIR operation pass manager"));
        }
        Ok(OpPassManager {
            raw,
            context: self.context,
            anchor: Some(operation_name.into()),
            _parent: PhantomData,
        })
    }

    /// Parse and replace the manager's pipeline, capturing native diagnostics.
    pub fn parse_pipeline(&mut self, pipeline: &str) -> Result<()> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerGetAsOpPassManager`.
        let manager = unsafe { n::mlirPassManagerGetAsOpPassManager(self.raw) };
        pipeline_result(capture(|callback, data| {
            // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
            unsafe { n::mlirParsePassPipeline(manager, string_ref(pipeline), callback, data) }
        }))
    }

    /// Transfer an owned pass into this manager.
    pub fn add_pass(&mut self, mut pass: Pass<'ctx>) -> Result<()> {
        check_pass(self.context, self.anchor.as_deref(), &pass)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPassManagerAddOwnedPass`.
        unsafe { n::mlirPassManagerAddOwnedPass(self.raw, pass.raw) };
        pass.raw = n::MlirPass {
            ptr: ptr::null_mut(),
        };
        Ok(())
    }
}

impl Drop for PassManager<'_> {
    fn drop(&mut self) {
        // SAFETY: `self` uniquely owns the native handle released by `mlirPassManagerDestroy`.
        unsafe { n::mlirPassManagerDestroy(self.raw) };
    }
}

impl OpPassManager<'_> {
    /// Borrow the native nested pass-manager handle.
    pub fn as_raw(&self) -> n::MlirOpPassManager {
        self.raw
    }

    /// Borrow a manager nested under operations with this name.
    pub fn nested_under(&mut self, operation_name: &str) -> Result<OpPassManager<'_>> {
        validate_operation_name(operation_name)?;
        let raw =
// SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
            unsafe { n::mlirOpPassManagerGetNestedUnder(self.raw, string_ref(operation_name)) };
        if raw.ptr.is_null() {
            return Err(Error::Null("nested MLIR operation pass manager"));
        }
        Ok(OpPassManager {
            raw,
            context: self.context,
            anchor: Some(operation_name.into()),
            _parent: PhantomData,
        })
    }

    /// Parse and append a textual pass pipeline, preserving parser diagnostics.
    pub fn add_pipeline(&mut self, pipeline: &str) -> Result<()> {
        pipeline_result(capture(|callback, data| {
            // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
            unsafe {
                n::mlirOpPassManagerAddPipeline(self.raw, string_ref(pipeline), callback, data)
            }
        }))
    }

    /// Add an owned pass to this manager.
    pub fn add_pass(&mut self, mut pass: Pass<'_>) -> Result<()> {
        check_pass(self.context, self.anchor.as_deref(), &pass)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOpPassManagerAddOwnedPass`.
        unsafe { n::mlirOpPassManagerAddOwnedPass(self.raw, pass.raw) };
        pass.raw = n::MlirPass {
            ptr: ptr::null_mut(),
        };
        Ok(())
    }

    /// Print the current pipeline representation.
    pub fn pipeline(&self) -> String {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirPrintPassPipeline`.
        print_with(|callback, data| unsafe { n::mlirPrintPassPipeline(self.raw, callback, data) })
    }
}

fn pipeline_result((result, diagnostics): (n::MlirLogicalResult, String)) -> Result<()> {
    if n::compiler::logical_result::is_success(result) {
        return Ok(());
    }
    let diagnostics = diagnostics.trim();
    Err(Error::Invalid(if diagnostics.is_empty() {
        "failed to parse MLIR pass pipeline".into()
    } else {
        format!("failed to parse MLIR pass pipeline: {diagnostics}")
    }))
}

/// MLIR's operation-agnostic pass-manager anchor.
const ANY_ANCHOR: &str = "any";

/// Rejects passes MLIR would otherwise abort on: a foreign context, or an
/// anchored pass added to a manager anchored on a different operation.
fn check_pass(context: n::MlirContext, manager: Option<&str>, pass: &Pass<'_>) -> Result<()> {
    if pass.context.ptr != context.ptr {
        return Err(Error::Invalid(
            "pass belongs to another MLIR context".into(),
        ));
    }
    if let (Some(manager), Some(anchor)) = (manager, pass.anchor())
        && manager != anchor
    {
        return Err(Error::Invalid(format!(
            "pass anchored on '{anchor}' cannot run in a manager on '{manager}'; \
             add it through nested_under(\"{anchor}\")"
        )));
    }
    Ok(())
}

/// A unique, 8-byte-aligned address per Rust pass type, as `mlirTypeIDCreate`
/// requires. Leaked once per type because TypeIDs must outlive every pass.
fn pass_identity<P: 'static>() -> *const c_void {
    use std::{any::TypeId, collections::BTreeMap};
    static IDENTITIES: Mutex<BTreeMap<TypeId, usize>> = Mutex::new(BTreeMap::new());
    let mut identities = IDENTITIES.lock().unwrap_or_else(|error| error.into_inner());
    let address = *identities
        .entry(TypeId::of::<P>())
        .or_insert_with(|| ptr::from_ref(Box::leak(Box::new(0u64))) as usize);
    address as *const c_void
}

fn validate_operation_name(name: &str) -> Result<()> {
    if name.is_empty() || name.contains('\0') {
        Err(Error::Invalid(
            "pass-manager operation name is invalid".into(),
        ))
    } else {
        Ok(())
    }
}

impl ExternalPass for Box<dyn ExternalPass> {
    fn clone_box(&self) -> Box<dyn ExternalPass> {
        (**self).clone_box()
    }

    fn initialize(&mut self) -> Result<()> {
        (**self).initialize()
    }

    fn run(&mut self, operation: &mut Operation<'_, Borrowed>) -> Result<()> {
        (**self).run(operation)
    }
}

unsafe extern "C" fn external_construct(_: *mut c_void) {}

unsafe extern "C" fn external_destruct(data: *mut c_void) {
    // SAFETY: this deleter reconstructs the unique Box pointer transferred at registration, and the native API invokes it once.
    contain(|| drop(unsafe { Box::from_raw(data.cast::<ExternalState>()) }));
}

/// Runs `f` on the pass instance; a failed clone, an error, or a panic is a failure.
fn with_pass(data: *mut c_void, f: impl FnOnce(&mut dyn ExternalPass) -> Result<()>) -> bool {
    // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
    let state = unsafe { &*data.cast::<ExternalState>() };
    contain(|| {
        let mut pass = state.lock().unwrap_or_else(PoisonError::into_inner);
        pass.as_mut().is_some_and(|pass| f(pass.as_mut()).is_ok())
    })
    .unwrap_or(false)
}

unsafe extern "C" fn external_initialize(
    _: n::MlirContext,
    data: *mut c_void,
) -> n::MlirLogicalResult {
    let succeeded = with_pass(data, |pass| pass.initialize());
    if succeeded {
        n::compiler::logical_result::success()
    } else {
        n::compiler::logical_result::failure()
    }
}

unsafe extern "C" fn external_clone(data: *mut c_void) -> *mut c_void {
    // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
    let state = unsafe { &*data.cast::<ExternalState>() };
    let pass = contain(|| {
        let pass = state.lock().unwrap_or_else(PoisonError::into_inner);
        pass.as_ref().map(|pass| pass.clone_box())
    })
    .flatten();
    let clone: Box<ExternalState> = Box::new(Mutex::new(pass));
    Box::into_raw(clone).cast()
}

unsafe extern "C" fn external_run(
    raw: n::MlirOperation,
    pass: n::MlirExternalPass,
    data: *mut c_void,
) {
    let succeeded = with_pass(data, |external| {
        let mut operation = Operation {
            raw,
            owns: false,
            _ownership: PhantomData::<Borrowed>,
            _context: PhantomData,
        };
        external.run(&mut operation)
    });
    if !succeeded {
        // SAFETY: `pass` is a live native handle; their wrappers keep them valid throughout `mlirExternalPassSignalFailure`.
        unsafe { n::mlirExternalPassSignalFailure(pass) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    proptest! {
        #[test]
        fn pipeline_parse_print_is_stable(space_count in 0usize..8) {
            let context = Context::new().unwrap();
            let mut manager = PassManager::new(&context).unwrap();
            let pipeline = format!("builtin.module(canonicalize{})", " ".repeat(space_count));
            manager.parse_pipeline(&pipeline).unwrap();
            let printed = manager.as_op_pass_manager().pipeline();
            manager.parse_pipeline(&printed).unwrap();
            prop_assert_eq!(manager.as_op_pass_manager().pipeline(), printed);
        }
    }

    #[test]
    fn pass_identities_are_aligned_and_unique_per_type() {
        struct A;
        struct B;
        let a = pass_identity::<A>();
        assert_eq!(a as usize % 8, 0);
        assert_eq!(a, pass_identity::<A>());
        assert_ne!(a, pass_identity::<B>());
    }

    #[test]
    fn anchored_pass_is_rejected_by_mismatched_manager() -> Result<()> {
        struct Noop;
        impl ExternalPass for Noop {
            fn clone_box(&self) -> Box<dyn ExternalPass> {
                Box::new(Noop)
            }

            fn run(&mut self, _: &mut Operation<'_, Borrowed>) -> Result<()> {
                Ok(())
            }
        }
        let context = Context::new()?;
        let pass = Pass::external(&context, "rust-noop", "rust-noop", "", "func.func", Noop)?;
        let mut manager = PassManager::new_on_operation(&context, "builtin.module")?;
        // The rejected pass is dropped through a manager on its own anchor.
        assert!(manager.add_pass(pass).is_err());
        let pass = Pass::external(&context, "rust-noop", "rust-noop", "", "func.func", Noop)?;
        manager.nested_under("func.func")?.add_pass(pass)?;
        Ok(())
    }

    #[test]
    fn external_pass_panic_is_converted_to_mlir_failure() -> Result<()> {
        struct Panics;
        impl ExternalPass for Panics {
            fn clone_box(&self) -> Box<dyn ExternalPass> {
                Box::new(Panics)
            }

            fn run(&mut self, _: &mut Operation<'_, Borrowed>) -> Result<()> {
                panic!("external pass panic");
            }
        }

        let context = Context::new()?;
        let module = crate::Module::parse(&context, "module {}")?;
        let pass = Pass::external(
            &context,
            "rust-panic",
            "rust-panic",
            "",
            "builtin.module",
            Panics,
        )?;
        let mut manager = PassManager::new(&context)?;
        manager.add_pass(pass)?;
        let mut operation = module.operation();
        assert!(manager.run(&mut operation).is_err());
        Ok(())
    }

    #[test]
    fn external_pass_runs_on_its_anchor() -> Result<()> {
        struct Counter(Arc<AtomicUsize>);
        impl ExternalPass for Counter {
            fn clone_box(&self) -> Box<dyn ExternalPass> {
                Box::new(Self(self.0.clone()))
            }

            fn run(&mut self, _: &mut Operation<'_, Borrowed>) -> Result<()> {
                self.0.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }
        let context = Context::new()?;
        let module = crate::Module::parse(&context, "module {}")?;
        let count = Arc::new(AtomicUsize::new(0));
        let pass = Pass::external(
            &context,
            "rust-anchor-count",
            "rust-anchor-count",
            "Counts the anchored operation",
            "builtin.module",
            Counter(count.clone()),
        )?;
        let mut manager = PassManager::new(&context)?;
        manager.add_pass(pass)?;
        let mut operation = module.operation();
        manager.run(&mut operation)?;
        assert_eq!(count.load(Ordering::SeqCst), 1);
        Ok(())
    }
}
