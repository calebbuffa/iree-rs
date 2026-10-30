//! Compiled IREE programs bound to a context.

use crate::context::Context;
use crate::context::QueueTopology;
use crate::error::invalid;
use crate::ordering::next_value;
use ireert::Code;
use ireert::Result;
use ireert::hal::BufferView;
use ireert::hal::Fence;
use ireert::hal::WaitFlags;
use ireert::vm;

/// IREE modules loaded on a [`Context`]'s device.
///
/// Create one with [`Context::load`] or `Context::compile`. A program is not
/// `Send`; load one per thread to call from several.
pub struct Program {
    pub(crate) context: Context,
    _instance: vm::Instance,
    pub(crate) vm: vm::Context,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program")
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}

impl Program {
    /// Call `function` (for example `module.main`) and return its results.
    ///
    /// The call observes all WGPU work submitted before it, and WGPU work
    /// submitted after it observes the call's results.
    ///
    /// With [`QueueTopology::Separate`] and a function compiled with IREE's
    /// default asynchronous ABI (`function$async` exists), this returns once
    /// the work is enqueued; results are ordered on the logical queue, so pass
    /// them to further calls, [`Context::copy_to_shared`] or
    /// [`Context::read_view`] without waiting. Otherwise the call completes
    /// before returning.
    ///
    /// Do not host-map returned buffers directly; use [`Context::read_view`].
    ///
    /// # Errors
    ///
    /// Returns an error when `function` does not exist, the arguments do not
    /// match its signature, a buffer-view argument belongs to another device,
    /// or the invocation fails. A failed call leaves the buffers it writes
    /// with unspecified contents; the context stays usable.
    pub fn call(&self, function: &str, inputs: &vm::List) -> Result<vm::List> {
        self.check_input_devices(inputs)?;
        let inner = &self.context.inner;
        if inner.topology == QueueTopology::Separate
            && let Ok(asynchronous) = self.vm.resolve_function(&format!("{function}$async"))
        {
            return self.call_async(&asynchronous, inputs);
        }
        let function = self.vm.resolve_function(function)?;
        let (_, results) = function.argument_result_counts()?;
        let mut outputs = vm::List::new(vm::TypeDef::Any, results)?;
        inner.blocking(|_| {
            self.vm
                .invoke(&function, Default::default(), inputs, &mut outputs)
        })?;
        Ok(outputs)
    }

    /// Invoke a coarse-fenced `function$async` ordered on the logical queue.
    pub(crate) fn call_async(
        &self,
        function: &vm::Function,
        inputs: &vm::List,
    ) -> Result<vm::List> {
        let inner = &self.context.inner;
        let (arguments, results) = function.argument_result_counts()?;
        if arguments != inputs.len() + 2 {
            return Err(invalid(format!(
                "{} expects {} arguments, got {}",
                function.name(),
                arguments.saturating_sub(2),
                inputs.len()
            )));
        }
        let mut state = inner.lock()?;
        let waits = inner.iree_waits(&mut state)?;
        let next = next_value(state.launched)?;
        let wait = Fence::new(waits.len().max(1))?;
        for (semaphore, value) in &waits {
            wait.insert(semaphore, *value)?;
        }
        let signal = Fence::at(&state.work, next)?;
        let mut arguments = inputs.try_clone()?;
        arguments.push(wait.to_ref())?;
        arguments.push(signal.to_ref())?;
        let mut outputs = vm::List::new(vm::TypeDef::Any, results)?;
        if let Err(error) = self
            .vm
            .invoke(function, Default::default(), &arguments, &mut outputs)
        {
            // Release anything waiting on this call, then start a fresh
            // timeline: separate queues cannot race, so the context survives.
            if state.launched > 0 {
                let _ = state.work.wait(state.launched, None, WaitFlags::default());
            }
            signal.fail(Code::Aborted);
            inner.reset_work(&mut state);
            return Err(error);
        }
        state.launched = next;
        Ok(outputs)
    }

    /// Call `function` with tensor arguments and return its tensor results.
    ///
    /// A convenience for [`Self::call`] when every argument and result is a
    /// tensor.
    ///
    /// # Errors
    ///
    /// As [`Self::call`], and when an input belongs to another device or a
    /// result is not a tensor.
    pub fn call_views(&self, function: &str, inputs: &[&BufferView]) -> Result<Vec<BufferView>> {
        let mut list = vm::List::new(vm::TypeDef::Any, inputs.len())?;
        list.resize(inputs.len())?;
        for (index, view) in inputs.iter().enumerate() {
            list.set_buffer_view(index, view)?;
        }
        let outputs = self.call(function, &list)?;
        (0..outputs.len())
            .map(|index| {
                outputs
                    .get_buffer_view(index)
                    .ok_or_else(|| invalid(format!("result {index} is not a buffer view")))
            })
            .collect()
    }

    /// The context this program runs on.
    pub fn context(&self) -> &Context {
        &self.context
    }

    fn check_input_devices(&self, inputs: &vm::List) -> Result<()> {
        for index in 0..inputs.len() {
            if let Some(view) = inputs.get_buffer_view(index) {
                let buffer = view.buffer();
                self.context.check_buffer_device(&buffer)?;
            }
        }
        Ok(())
    }
}

impl Context {
    /// Load VM modules (for example bytecode) into a program on this device.
    ///
    /// The HAL module for this context's device is registered first, so the
    /// modules must import `hal` only from it.
    ///
    /// # Errors
    ///
    /// Returns an error when the modules' imports cannot be resolved.
    pub fn load(&self, modules: &[vm::Module]) -> Result<Program> {
        let instance = vm::Instance::new()?;
        let hal = vm::Module::hal(
            &instance,
            Default::default(),
            self.inner.iree.device_group(),
            Default::default(),
            Default::default(),
        )?;
        let mut all = Vec::with_capacity(modules.len() + 1);
        all.push(hal);
        all.extend(modules.iter().cloned());
        let vm = vm::Context::with_modules(&instance, Default::default(), &all)?;
        Ok(Program {
            context: self.clone(),
            _instance: instance,
            vm,
        })
    }

    /// Compile MLIR `source` for this device and load it.
    ///
    /// The compiler produces trusted bytecode, so no `unsafe` is needed.
    /// Requires the `compiler` feature.
    ///
    /// # Errors
    ///
    /// Returns an error with [`Code::InvalidArgument`] when compilation fails,
    /// and as [`Self::load`].
    #[cfg(feature = "compiler")]
    pub fn compile(&self, source: &str, input: ireec::InputType) -> Result<Program> {
        use ireec::{Compiler, Options, Output, Pipeline, Target};
        fn compile_error(error: impl std::fmt::Display) -> ireert::Error {
            invalid(format!("compilation failed: {error}"))
        }
        let compiler = Compiler::new().map_err(compile_error)?;
        let mut session = compiler.session().map_err(compile_error)?;
        session
            .configure(
                &Options::default()
                    .with_target(Target::VulkanSpirv {
                        target: None,
                        features: None,
                    })
                    .with_input_type(input),
            )
            .map_err(compile_error)?;
        let source = session
            .source_buffer("iree-wgpu.mlir", source.as_bytes())
            .map_err(compile_error)?;
        let mut invocation = session.invocation().map_err(compile_error)?;
        invocation.parse_source(source).map_err(compile_error)?;
        invocation.pipeline(Pipeline::Std).map_err(compile_error)?;
        let mut output = Output::memory(&compiler).map_err(compile_error)?;
        invocation
            .output_vm_bytecode(&mut output)
            .map_err(compile_error)?;
        let bytecode = output.bytes().map_err(compile_error)?.to_vec();
        let instance = vm::Instance::new()?;
        // SAFETY: the archive was just produced by the IREE compiler.
        let module = unsafe { vm::BytecodeModule::new(&instance, Default::default(), bytecode)? }
            .into_module();
        self.load(&[module])
    }
}
