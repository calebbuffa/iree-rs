//! Host-defined VM modules.
//!
//! [`NativeModule`] describes exports, imports and per-context state;
//! [`NativeModule::build`] turns it into an ordinary [`Module`]. Callbacks
//! never unwind into C and may defer execution for resumption. Arguments
//! borrow references; results own them.

use super::ContextSignal;
use super::{Function, Instance, Module, Ref};
use crate::{Code, Error, Result, check};
use bytemuck::{AnyBitPattern, NoUninit};
use iree_native as n;
use std::any::Any;
use std::ptr;
use std::sync::{Arc, Mutex};

mod descriptor;
#[cfg(test)]
mod tests;
mod trampolines;
mod typed;

use descriptor::{Dependency, Export, Fragment, Import, Layout, ModuleData, validate_names};
use trampolines::{allocate_state, destroy, fork_state, free_state, notify, resolve_import};
pub use typed::{NativeArguments, NativeResults, NativeValue};

type StateValue = Arc<dyn Any + Send + Sync>;
type StateFactory = dyn Fn() -> Result<StateValue> + Send + Sync;
type StateFork = dyn Fn(&(dyn Any + Send + Sync)) -> Result<StateValue> + Send + Sync;
type Notify = dyn Fn(&(dyn Any + Send + Sync), ContextSignal) -> Result<()> + Send + Sync;
type Callback = dyn for<'a> Fn(&mut NativeCall<'a>) -> Result<()> + Send + Sync;

/// A host-defined VM module under construction.
///
/// Callbacks and state must be `Send + Sync`: IREE may call or drop them on any
/// thread. An export may return [`Code::Deferred`] to yield and will be called
/// again with [`NativeCallPhase::Resume`] when the VM resumes it.
///
/// ```no_run
/// # use ireert::{Result, vm::{Instance, NativeModule}};
/// # fn example() -> Result<()> {
/// let instance = Instance::new()?;
/// let module = NativeModule::builder("host")
///     .export("increment", "0i_i", |call| {
///         let value: i32 = call.argument(0)?;
///         call.set_result(0, value.wrapping_add(1))
///     })
///     .build(&instance)?;
/// # Ok(())
/// # }
/// ```
pub struct NativeModule {
    name: String,
    version: u32,
    attributes: Vec<(String, String)>,
    dependencies: Vec<Dependency>,
    imports: Vec<Import>,
    exports: Vec<Export>,
    state_factory: Box<StateFactory>,
    state_fork: Box<StateFork>,
    notify: Option<Box<Notify>>,
}

impl NativeModule {
    /// Starts a module named `name`.
    pub fn builder(name: &str) -> Self {
        Self {
            name: name.into(),
            version: 0,
            attributes: Vec::new(),
            dependencies: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            state_factory: Box::new(|| Ok(Arc::new(()))),
            state_fork: Box::new(|_| Ok(Arc::new(()))),
            notify: None,
        }
    }

    /// Sets the version that dependents check against.
    pub fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Adds a module attribute.
    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push((key.into(), value.into()));
        self
    }

    /// Declares a dependency on module `name` of at least `minimum_version`.
    pub fn dependency(mut self, name: &str, minimum_version: u32, optional: bool) -> Self {
        self.dependencies.push(Dependency {
            name: name.into(),
            version: minimum_version,
            optional,
        });
        self
    }

    /// Adds an export with a VM calling convention such as `0ir_i`.
    ///
    /// `i`, `I`, `f`, `F` and `r` slots are supported. As upstream lays them
    /// out, each slot is naturally aligned in native byte order and the
    /// storage is padded to its widest slot; use
    /// [`NativeCall::argument_offset`] to address them.
    pub fn export(
        mut self,
        name: &str,
        cconv: &str,
        callback: impl Fn(&mut NativeCall<'_>) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.exports.push(Export {
            name: name.into(),
            cconv: cconv.into(),
            attributes: Vec::new(),
            callback: Box::new(callback),
            layout: None,
        });
        self
    }

    /// Adds an export whose VM calling convention is derived from Rust types.
    ///
    /// Supported argument and result values are `bool`, `i32`, `u32`, `i64`,
    /// `u64`, `f32`, `f64`, [`Ref`], and `Option<Ref>`. Tuples up to six
    /// elements describe multiple arguments or results; `()` means no
    /// arguments or results. Implement [`NativeValue`] for domain newtypes to
    /// use them in typed exports.
    ///
    /// ```no_run
    /// # use ireert::{Result, vm::{Instance, NativeModule}};
    /// # fn example() -> Result<()> {
    /// let instance = Instance::new()?;
    /// let module = NativeModule::builder("math")
    ///     .export_typed("add", |(lhs, rhs): (i32, i32)| Ok(lhs + rhs))
    ///     .build(&instance)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_typed<A, R, F>(mut self, name: &str, callback: F) -> Self
    where
        A: NativeArguments,
        R: NativeResults,
        F: Fn(A) -> Result<R> + Send + Sync + 'static,
    {
        let cconv = format!("0{}_{}", A::abi_fragment(), R::abi_fragment());
        self.exports.push(Export {
            name: name.into(),
            cconv,
            attributes: Vec::new(),
            callback: Box::new(move |call| callback(A::decode(call)?)?.encode(call)),
            layout: None,
        });
        self
    }

    /// Adds a typed export with access to this context's module state.
    ///
    /// The callback receives the state value created by [`Self::state`] (or
    /// its forked value) and decoded arguments. Its argument and result ABI is
    /// derived in the same way as [`Self::export_typed`].
    ///
    /// ```no_run
    /// # use ireert::{Result, vm::{Instance, NativeModule}};
    /// # fn example() -> Result<()> {
    /// let instance = Instance::new()?;
    /// let module = NativeModule::builder("counter")
    ///     .state(|| Ok(0_i32), |value| Ok(*value))
    ///     .export_typed_with_state(
    ///         "get",
    ///         |value: &i32, (): ()| -> Result<i32> { Ok(*value) },
    ///     )
    ///     .build(&instance)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_typed_with_state<T, A, R, F>(mut self, name: &str, callback: F) -> Self
    where
        T: Any + Send + Sync + 'static,
        A: NativeArguments,
        R: NativeResults,
        F: Fn(&T, A) -> Result<R> + Send + Sync + 'static,
    {
        let cconv = format!("0{}_{}", A::abi_fragment(), R::abi_fragment());
        self.exports.push(Export {
            name: name.into(),
            cconv,
            attributes: Vec::new(),
            callback: Box::new(move |call| {
                let state = call.state::<T>().ok_or_else(|| {
                    Error::new(
                        Code::FailedPrecondition,
                        "native module state type mismatch",
                    )
                })?;
                callback(state, A::decode(call)?)?.encode(call)
            }),
            layout: None,
        });
        self
    }

    /// Adds an attribute to the export `export`.
    pub fn export_attribute(mut self, export: &str, key: &str, value: &str) -> Result<Self> {
        self.exports
            .iter_mut()
            .find(|entry| entry.name == export)
            .ok_or_else(|| Error::new(Code::NotFound, "export not found"))?
            .attributes
            .push((key.into(), value.into()));
        Ok(self)
    }

    /// Imports the function `full_name`, such as `module.function`.
    ///
    /// Callbacks reach resolved imports through [`NativeCall::import`].
    pub fn import(mut self, full_name: &str, optional: bool) -> Self {
        self.imports.push(Import {
            name: full_name.into(),
            optional,
        });
        self
    }

    /// Gives each context its own `T`, created by `allocate`; forked contexts
    /// get `fork(&parent)`.
    pub fn state<T: Any + Send + Sync>(
        mut self,
        allocate: impl Fn() -> Result<T> + Send + Sync + 'static,
        fork: impl Fn(&T) -> Result<T> + Send + Sync + 'static,
    ) -> Self {
        self.state_factory = Box::new(move || Ok(Arc::new(allocate()?)));
        self.state_fork = Box::new(move |value| Ok(Arc::new(fork(downcast(value)?)?)));
        self
    }

    /// Calls `callback` with the context's state for each context signal.
    pub fn notify<T: Any + Send + Sync>(
        mut self,
        callback: impl Fn(&T, ContextSignal) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.notify = Some(Box::new(move |value, signal| {
            callback(downcast(value)?, signal)
        }));
        self
    }

    /// Creates the module in `instance`.
    pub fn build(mut self, instance: &Instance) -> Result<Module> {
        // Upstream verifies descriptors only in debug builds, and function
        // ordinals are 16-bit.
        if self.name.is_empty() {
            return Err(Error::new(Code::InvalidArgument, "module name is empty"));
        }
        let ordinals = usize::from(u16::MAX) + 1;
        if self.exports.len() > ordinals || self.imports.len() > ordinals {
            return Err(Error::new(
                Code::OutOfRange,
                "too many functions for 16-bit ordinals",
            ));
        }
        self.exports.sort_by(|a, b| a.name.cmp(&b.name));
        self.imports.sort_by(|a, b| a.name.cmp(&b.name));
        validate_names(self.exports.iter().map(|export| export.name.as_str()))?;
        validate_names(self.imports.iter().map(|import| import.name.as_str()))?;
        for export in &mut self.exports {
            export.layout = Some(Layout::parse(&export.cconv)?);
        }
        let data = Arc::into_raw(Arc::new(ModuleData::new(self, instance.clone())));
        let interface = n::iree_vm_module_t {
            self_: data.cast_mut().cast(),
            destroy: Some(destroy),
            alloc_state: Some(allocate_state),
            free_state: Some(free_state),
            fork_state: Some(fork_state),
            resolve_import: Some(resolve_import),
            notify: Some(notify),
            ..Default::default()
        };
        let mut raw = ptr::null_mut();
        // SAFETY: the tables live in `data`; on success native owns that Arc
        // and `destroy` releases it once.
        let result = check(unsafe {
            n::iree_vm_native_module_create(
                &interface,
                &(*data).descriptor,
                instance.as_raw(),
                crate::system(),
                &mut raw,
            )
        });
        if let Err(error) = result {
            // SAFETY: native did not take the Arc.
            drop(unsafe { Arc::from_raw(data) });
            return Err(error);
        }
        // SAFETY: success returns one owned module reference.
        Ok(unsafe { Module::from_parts(raw, instance.clone()) })
    }
}

fn downcast<T: Any>(value: &(dyn Any + Send + Sync)) -> Result<&T> {
    value.downcast_ref().ok_or_else(|| {
        Error::new(
            Code::FailedPrecondition,
            "native module state type mismatch",
        )
    })
}

struct CallState {
    value: StateValue,
    imports: Mutex<Vec<Option<Function>>>,
}

/// Native export callback phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeCallPhase {
    /// First invocation of the export.
    Begin,
    /// Resumption after the callback returned [`Code::Deferred`].
    Resume,
}

/// One call of a native export.
///
/// Arguments and results are addressed by byte offset into the aligned calling
/// convention. Results start zeroed; on error, written result references are
/// released.
pub struct NativeCall<'a> {
    module: &'a ModuleData,
    state: &'a CallState,
    layout: &'a Layout,
    arguments: &'a [u8],
    results: n::iree_byte_span_t,
    complete: bool,
    phase: NativeCallPhase,
}

impl NativeCall<'_> {
    /// Whether this callback starts an export or resumes a deferred export.
    pub fn phase(&self) -> NativeCallPhase {
        self.phase
    }

    /// Returns the byte offset of argument slot `index`.
    pub fn argument_offset(&self, index: usize) -> Option<usize> {
        self.layout.arguments.slots.get(index).copied()
    }

    /// Returns the byte offset of result slot `index`.
    pub fn result_offset(&self, index: usize) -> Option<usize> {
        self.layout.results.slots.get(index).copied()
    }

    /// Reads the scalar argument at byte `offset`.
    pub fn argument<T: AnyBitPattern>(&self, offset: usize) -> Result<T> {
        let range = scalar(&self.layout.arguments, offset, size_of::<T>())?;
        let bytes = self.arguments.get(range).ok_or_else(|| {
            Error::new(
                Code::OutOfRange,
                "argument storage is unavailable on resume",
            )
        })?;
        Ok(bytemuck::pod_read_unaligned(bytes))
    }

    /// Writes the scalar result at byte `offset`.
    pub fn set_result<T: NoUninit>(&mut self, offset: usize, value: T) -> Result<()> {
        let range = scalar(&self.layout.results, offset, size_of::<T>())?;
        let bytes = bytemuck::bytes_of(&value);
        // SAFETY: the range lies in the result storage and holds no reference.
        unsafe {
            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.results.data.add(range.start),
                bytes.len(),
            )
        };
        Ok(())
    }

    /// Returns the reference argument at byte `offset`, or `None` when null.
    pub fn argument_ref(&self, offset: usize) -> Result<Option<Ref>> {
        if !self.layout.arguments.refs.contains(&offset) {
            return Err(Error::new(
                Code::InvalidArgument,
                "offset is not a reference argument",
            ));
        }
        let bytes = self
            .arguments
            .get(offset..offset + size_of::<n::iree_vm_ref_t>())
            .ok_or_else(|| {
                Error::new(
                    Code::OutOfRange,
                    "argument storage is unavailable on resume",
                )
            })?;
        // SAFETY: the layout puts a reference at `offset`; caller storage may be
        // unaligned, and the complete reference lies within the argument span.
        let mut source = unsafe { ptr::read_unaligned(bytes.as_ptr().cast::<n::iree_vm_ref_t>()) };
        if source.ptr.is_null() {
            return Ok(None);
        }
        let mut retained = n::iree_vm_ref_t::default();
        // SAFETY: the argument is borrowed for the call; retaining gives us our own reference.
        unsafe { n::iree_vm_ref_retain(&mut source, &mut retained) };
        // SAFETY: `retained` is an owned reference.
        Ok(Some(unsafe { Ref::from_raw(retained) }))
    }

    /// Sets the reference result at byte `offset`, replacing any earlier one.
    pub fn set_result_ref(&mut self, offset: usize, value: Option<Ref>) -> Result<()> {
        if !self.layout.results.refs.contains(&offset) {
            return Err(Error::new(
                Code::InvalidArgument,
                "offset is not a reference result",
            ));
        }
        let raw = value.map(Ref::into_raw).unwrap_or_default();
        // SAFETY: the slot holds null or a reference written here; caller storage
        // may be unaligned.
        unsafe {
            let slot = self.results.data.add(offset).cast::<n::iree_vm_ref_t>();
            let mut previous = ptr::read_unaligned(slot);
            ptr::write_unaligned(slot, raw);
            n::iree_vm_ref_release(&mut previous);
        }
        Ok(())
    }

    /// Returns this context's state if it is a `T`.
    pub fn state<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.state.value.downcast_ref()
    }

    /// Returns the import `full_name`, or `None` if it was not declared or an
    /// optional import did not resolve.
    pub fn import(&self, full_name: &str) -> Option<Function> {
        let ordinal = self
            .module
            .definition
            .imports
            .binary_search_by(|import| import.name.as_str().cmp(full_name))
            .ok()?;
        let imports = self
            .state
            .imports
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        imports.get(ordinal)?.clone()
    }
}

/// Returns the byte range of a scalar slot, rejecting overlap with references.
fn scalar(fragment: &Fragment, offset: usize, length: usize) -> Result<std::ops::Range<usize>> {
    let end = offset
        .checked_add(length)
        .filter(|end| *end <= fragment.size)
        .ok_or_else(|| Error::new(Code::OutOfRange, "scalar is out of bounds"))?;
    let reference = size_of::<n::iree_vm_ref_t>();
    if fragment
        .refs
        .iter()
        .any(|slot| offset < slot + reference && *slot < end)
    {
        return Err(Error::new(
            Code::InvalidArgument,
            "scalar overlaps a reference",
        ));
    }
    Ok(offset..end)
}

impl Drop for NativeCall<'_> {
    fn drop(&mut self) {
        if self.complete {
            return;
        }
        for offset in &self.layout.results.refs {
            // SAFETY: each slot holds null or a reference written by `set_result_ref`.
            unsafe {
                let slot = self.results.data.add(*offset).cast::<n::iree_vm_ref_t>();
                let mut raw = ptr::read_unaligned(slot);
                ptr::write_unaligned(slot, n::iree_vm_ref_t::default());
                n::iree_vm_ref_release(&mut raw);
            }
        }
    }
}
