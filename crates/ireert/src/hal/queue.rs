//! Queue submission operations on devices.

use super::bitfield::bitfield;
use super::command_buffer::check_host_range;
use super::{
    DispatchConfig, DispatchFlags, ExecutableFunction,
    buffer::Buffer,
    command_buffer::{BufferRef, CommandBuffer},
    device::GroupedDevice,
    executable::Executable,
    file::File,
    flags::BufferParams,
    pool::Pool,
    semaphore::{Semaphore, SemaphoreList},
};
use crate::hal::DeviceSize;
use crate::{Code, Error, Result, check};
use core::ffi::c_void;
use iree_native as n;
use n::const_span;
use std::sync::{Arc, Mutex};

/// One semaphore and payload value used in queue dependencies.
#[derive(Clone, Copy)]
pub struct SemaphorePoint<'a> {
    /// Semaphore participating in the dependency.
    pub semaphore: &'a Semaphore,
    /// Required or signaled payload value.
    pub value: u64,
}

/// Queue selection and semaphore dependencies shared by one queue operation.
///
/// The submission owns its small, copyable timepoint lists, so it can be
/// reused across operations without tying the operation to a temporary slice.
#[derive(Clone, Default)]
pub struct QueueSubmission<'a> {
    affinity: QueueAffinity,
    waits: Vec<SemaphorePoint<'a>>,
    signals: Vec<SemaphorePoint<'a>>,
}

impl<'a> QueueSubmission<'a> {
    /// Creates a submission with no dependencies.
    pub fn new(affinity: QueueAffinity) -> Self {
        Self {
            affinity,
            waits: Vec::new(),
            signals: Vec::new(),
        }
    }

    /// Adds semaphore timepoints that must be reached before execution.
    pub fn with_waits(mut self, waits: impl IntoIterator<Item = SemaphorePoint<'a>>) -> Self {
        self.waits.extend(waits);
        self
    }

    /// Adds semaphore timepoints signaled after execution.
    pub fn with_signals(mut self, signals: impl IntoIterator<Item = SemaphorePoint<'a>>) -> Self {
        self.signals.extend(signals);
        self
    }

    /// Returns the selected queue affinity.
    pub const fn affinity(&self) -> QueueAffinity {
        self.affinity
    }

    /// Returns the submission's wait timepoints.
    pub fn waits(&self) -> &[SemaphorePoint<'a>] {
        &self.waits
    }

    /// Returns the submission's signal timepoints.
    pub fn signals(&self) -> &[SemaphorePoint<'a>] {
        &self.signals
    }
}

/// A borrowed queue buffer binding.
#[derive(Clone, Copy)]
pub struct BufferBinding<'a> {
    /// Buffer used by the command submission.
    pub buffer: &'a Buffer,
    /// Byte offset in the buffer.
    pub offset: DeviceSize,
    /// Byte length, or the native whole-buffer sentinel.
    pub length: DeviceSize,
}
impl<'a> BufferBinding<'a> {
    /// Binds a byte range of `buffer`.
    pub const fn new(buffer: &'a Buffer, offset: DeviceSize, length: DeviceSize) -> Self {
        Self {
            buffer,
            offset,
            length,
        }
    }

    /// Binds all of `buffer`.
    pub fn whole(buffer: &'a Buffer) -> Self {
        Self::new(buffer, 0, buffer.byte_length())
    }
}
impl BufferBinding<'_> {
    fn raw(&self) -> n::iree_hal_buffer_binding_t {
        n::iree_hal_buffer_binding_t {
            buffer: self.buffer.as_raw(),
            offset: self.offset,
            length: self.length,
        }
    }
}

/// A byte range in a HAL file.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileRange {
    /// Byte offset in the file.
    pub offset: u64,
    /// Number of bytes in the range.
    pub length: DeviceSize,
}

impl FileRange {
    /// Creates a file range.
    pub const fn new(offset: u64, length: DeviceSize) -> Self {
        Self { offset, length }
    }
}

/// Inputs for one queue dispatch, whether workgroup counts are direct or
/// supplied by a device buffer.
pub struct QueueDispatch<'a> {
    executable: &'a Executable,
    function: ExecutableFunction,
    config: DispatchConfig,
    workgroup_count_buffer: Option<BufferRef<'a>>,
    constants: &'a [u8],
    bindings: &'a [BufferRef<'a>],
    flags: DispatchFlags,
}

impl<'a> QueueDispatch<'a> {
    /// Creates a dispatch with no constants, bindings, or custom flags.
    pub const fn new(
        executable: &'a Executable,
        function: ExecutableFunction,
        config: DispatchConfig,
    ) -> Self {
        Self {
            executable,
            function,
            config,
            workgroup_count_buffer: None,
            constants: &[],
            bindings: &[],
            flags: DispatchFlags::from_bits_retain(0),
        }
    }

    /// Uses a device buffer containing three `u32` workgroup counts.
    pub fn with_workgroup_count_buffer(mut self, buffer: BufferRef<'a>) -> Self {
        self.workgroup_count_buffer = Some(buffer);
        self
    }

    /// Supplies push-constant bytes for this dispatch.
    pub fn with_constants(mut self, constants: &'a [u8]) -> Self {
        self.constants = constants;
        self
    }

    /// Supplies direct buffer references used by the executable.
    pub fn with_bindings(mut self, bindings: &'a [BufferRef<'a>]) -> Self {
        self.bindings = bindings;
        self
    }

    /// Sets native dispatch flags.
    pub fn with_flags(mut self, flags: DispatchFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl GroupedDevice {
    /// Allocates a buffer asynchronously on the device queue.
    pub fn queue_alloca(
        &self,
        submission: &QueueSubmission<'_>,
        pool: Option<&Pool>,
        params: BufferParams,
        size: DeviceSize,
        flags: AllocaFlags,
    ) -> Result<Buffer> {
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        let mut out = core::ptr::null_mut();
        // SAFETY: lists, pool, and output are valid for the call.
        check(unsafe {
            n::iree_hal_device_queue_alloca(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                pool.map_or(core::ptr::null_mut(), Pool::as_raw),
                params.raw(),
                size,
                flags.bits(),
                &mut out,
            )
        })?;
        // SAFETY: successful allocation returns one owned buffer reference.
        Ok(unsafe { Buffer::from_raw(out) })
    }
    /// Deallocates a buffer asynchronously on the device queue.
    pub fn queue_dealloca(
        &self,
        submission: &QueueSubmission<'_>,
        buffer: &Buffer,
        flags: DeallocaFlags,
    ) -> Result<()> {
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: lists and buffer are valid for the call; the queue retains the buffer.
        check(unsafe {
            n::iree_hal_device_queue_dealloca(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                buffer.as_raw(),
                flags.bits(),
            )
        })
    }
    /// Fills a buffer range with a 1-, 2- or 4-byte pattern on the device queue.
    pub fn queue_fill(
        &self,
        submission: &QueueSubmission<'_>,
        target: BufferBinding<'_>,
        pattern: &[u8],
    ) -> Result<()> {
        // `iree_hal_device_queue_fill` forwards unvalidated to the driver.
        if !matches!(pattern.len(), 1 | 2 | 4) {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue fill pattern must be 1, 2, or 4 bytes",
            ));
        }
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: lists, buffer and pattern are valid for the call; the pattern is copied.
        check(unsafe {
            n::iree_hal_device_queue_fill(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                target.buffer.as_raw(),
                target.offset,
                target.length,
                pattern.as_ptr().cast(),
                pattern.len(),
                n::IREE_HAL_FILL_FLAG_NONE as n::iree_hal_fill_flags_t,
            )
        })
    }
    /// Copies a byte range between buffers on the device queue.
    pub fn queue_copy(
        &self,
        submission: &QueueSubmission<'_>,
        source: BufferBinding<'_>,
        target: BufferBinding<'_>,
    ) -> Result<()> {
        // `iree_hal_device_queue_copy` forwards unvalidated to the driver.
        if source.length != target.length {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue copy source and target ranges must have equal lengths",
            ));
        }
        super::buffer::ensure_disjoint(
            source.buffer,
            source.offset,
            target.buffer,
            target.offset,
            source.length,
        )?;
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: lists and buffers are valid for the call; the queue retains the buffers.
        check(unsafe {
            n::iree_hal_device_queue_copy(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.buffer.as_raw(),
                source.offset,
                target.buffer.as_raw(),
                target.offset,
                source.length,
                n::IREE_HAL_COPY_FLAG_NONE as n::iree_hal_copy_flags_t,
            )
        })
    }
    /// Signals `signals` once `waits` are reached, without other work.
    pub fn queue_barrier(
        &self,
        submission: &QueueSubmission<'_>,
        flags: ExecuteFlags,
    ) -> Result<()> {
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: lists are valid for the call.
        check(unsafe {
            n::iree_native_hal_device_queue_barrier(
                self.as_raw(),
                submission.affinity().bits(),
                &waits.as_raw(),
                &signals.as_raw(),
                flags.bits(),
            )
        })
    }
    /// Flushes queued work for the selected queues.
    pub fn queue_flush(&self, affinity: QueueAffinity) -> Result<()> {
        // SAFETY: the device is live.
        check(unsafe { n::iree_hal_device_queue_flush(self.as_raw(), affinity.bits()) })
    }
    /// Blocks until all or any of `points` are reached, or `timeout` (`None` waits forever).
    pub fn wait_semaphores(
        &self,
        mode: WaitMode,
        points: &[SemaphorePoint<'_>],
        timeout: Option<core::time::Duration>,
        flags: WaitFlags,
    ) -> Result<()> {
        let timeout = n::timeout(timeout)?;
        let points = SemaphoreList::from_points(points);
        // SAFETY: the list is valid for the call.
        check(unsafe {
            n::iree_hal_device_wait_semaphores(
                self.as_raw(),
                mode.raw(),
                points.as_raw(),
                timeout,
                flags.bits(),
            )
        })
    }
    /// Updates a buffer range from host memory asynchronously.
    pub fn queue_update(
        &self,
        submission: &QueueSubmission<'_>,
        source: &[u8],
        source_offset: usize,
        target: BufferBinding<'_>,
    ) -> Result<()> {
        check_host_range(source, source_offset, target.length)?;
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: the source range was bounds-checked and is copied before the call returns.
        check(unsafe {
            n::iree_hal_device_queue_update(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.as_ptr().cast(),
                source_offset,
                target.buffer.as_raw(),
                target.offset,
                target.length,
                n::IREE_HAL_UPDATE_FLAG_NONE as n::iree_hal_update_flags_t,
            )
        })
    }
    /// Reads an owned file range into a buffer asynchronously on the device queue.
    ///
    /// Borrowed host storage is not accepted: forgetting the returned guard
    /// must not permit the queue to access expired memory.
    ///
    /// ```compile_fail
    /// use ireert::hal::{BufferBinding, FileRange, GroupedDevice, QueueSubmission, file::File};
    /// fn submit(device: &GroupedDevice, file: &File<'_>, target: BufferBinding<'_>,
    ///           at: &QueueSubmission<'_>) {
    ///     let _ = device.queue_read(at, file, FileRange::new(0, target.length), target);
    /// }
    /// ```
    pub fn queue_read(
        &self,
        submission: &QueueSubmission<'_>,
        source: &File<'static>,
        source_range: FileRange,
        target: BufferBinding<'_>,
    ) -> Result<FileTransfer<'static>> {
        if source_range.length != target.length {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue read file and buffer ranges must have equal lengths",
            ));
        }
        let completion = Semaphore::new(self, submission.affinity(), 0)?;
        let signal_points = append_completion_signal(submission.signals(), &completion);
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(&signal_points);
        // SAFETY: lists, file, and buffers remain valid through submission; the returned guard retains the file.
        check(unsafe {
            n::iree_hal_device_queue_read(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.as_raw(),
                source_range.offset,
                target.buffer.as_raw(),
                target.offset,
                source_range.length,
                n::IREE_HAL_READ_FLAG_NONE as n::iree_hal_read_flags_t,
            )
        })?;
        Ok(FileTransfer {
            _file: source.clone(),
            completion,
        })
    }
    /// Writes a buffer range to an owned file asynchronously on the device queue.
    ///
    /// Borrowed host storage is only supported by synchronous transfers.
    pub fn queue_write(
        &self,
        submission: &QueueSubmission<'_>,
        source: BufferBinding<'_>,
        target: &File<'static>,
        target_range: FileRange,
    ) -> Result<FileTransfer<'static>> {
        if source.length != target_range.length {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue write buffer and file ranges must have equal lengths",
            ));
        }
        let completion = Semaphore::new(self, submission.affinity(), 0)?;
        let signal_points = append_completion_signal(submission.signals(), &completion);
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(&signal_points);
        // SAFETY: lists, file, and buffers remain valid through submission; the returned guard retains the file.
        check(unsafe {
            n::iree_hal_device_queue_write(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.buffer.as_raw(),
                source.offset,
                target.as_raw(),
                target_range.offset,
                source.length,
                n::IREE_HAL_WRITE_FLAG_NONE as n::iree_hal_write_flags_t,
            )
        })?;
        Ok(FileTransfer {
            _file: target.clone(),
            completion,
        })
    }
    /// Performs a synchronous file-to-buffer transfer using host staging.
    pub fn queue_read_streaming(
        &self,
        submission: &QueueSubmission<'_>,
        source: &File<'_>,
        source_range: FileRange,
        target: BufferBinding<'_>,
        options: FileTransferOptions,
    ) -> Result<()> {
        if source_range.length != target.length {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue read file and buffer ranges must have equal lengths",
            ));
        }
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: this transfer function is synchronous; handles and semaphore lists remain live through completion.
        check(unsafe {
            n::iree_hal_device_queue_read_streaming(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.as_raw(),
                source_range.offset,
                target.buffer.as_raw(),
                target.offset,
                source_range.length,
                n::IREE_HAL_READ_FLAG_NONE as n::iree_hal_read_flags_t,
                options.raw(),
            )
        })
    }
    /// Performs a synchronous buffer-to-file transfer using host staging.
    pub fn queue_write_streaming(
        &self,
        submission: &QueueSubmission<'_>,
        source: BufferBinding<'_>,
        target: &File<'_>,
        target_range: FileRange,
        options: FileTransferOptions,
    ) -> Result<()> {
        if source.length != target_range.length {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue write buffer and file ranges must have equal lengths",
            ));
        }
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: this transfer function is synchronous; handles and semaphore lists remain live through completion.
        check(unsafe {
            n::iree_hal_device_queue_write_streaming(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.buffer.as_raw(),
                source.offset,
                target.as_raw(),
                target_range.offset,
                source.length,
                n::IREE_HAL_WRITE_FLAG_NONE as n::iree_hal_write_flags_t,
                options.raw(),
            )
        })
    }
    /// Submits a command buffer and buffer binding table asynchronously.
    ///
    /// This safe method does not allow [`ExecuteFlags::BORROW_BINDING_TABLE_LIFETIME`],
    /// which requires the caller to keep every bound buffer alive until execution completes.
    pub fn queue_execute(
        &self,
        submission: &QueueSubmission<'_>,
        command_buffer: &CommandBuffer,
        bindings: &[BufferBinding<'_>],
        flags: ExecuteFlags,
    ) -> Result<()> {
        if flags.contains(ExecuteFlags::BORROW_BINDING_TABLE_LIFETIME) {
            return Err(Error::new(
                Code::InvalidArgument,
                "borrowed binding lifetimes require queue_execute_unchecked",
            ));
        }
        self.queue_execute_impl(submission, command_buffer, bindings, flags)
    }

    /// Submits a command buffer with the native execution flags.
    ///
    /// # Safety
    /// If `flags` contains [`ExecuteFlags::BORROW_BINDING_TABLE_LIFETIME`],
    /// every buffer referenced by `bindings` must remain live until the submitted
    /// work completes. Keep them alive until a signal semaphore reaches its
    /// submitted value, or until another device synchronization establishes completion.
    pub unsafe fn queue_execute_unchecked(
        &self,
        submission: &QueueSubmission<'_>,
        command_buffer: &CommandBuffer,
        bindings: &[BufferBinding<'_>],
        flags: ExecuteFlags,
    ) -> Result<()> {
        self.queue_execute_impl(submission, command_buffer, bindings, flags)
    }

    fn queue_execute_impl(
        &self,
        submission: &QueueSubmission<'_>,
        command_buffer: &CommandBuffer,
        bindings: &[BufferBinding<'_>],
        flags: ExecuteFlags,
    ) -> Result<()> {
        let raw: Vec<n::iree_hal_buffer_binding_t> =
            bindings.iter().map(BufferBinding::raw).collect();
        let table = n::iree_hal_buffer_binding_table_t {
            count: raw.len(),
            bindings: if raw.is_empty() {
                core::ptr::null()
            } else {
                raw.as_ptr()
            },
        };
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: upstream captures binding-table entries during this call; any borrowed buffer lifetimes are the caller's obligation.
        check(unsafe {
            n::iree_native_hal_device_queue_execute(
                self.as_raw(),
                submission.affinity().bits(),
                &waits.as_raw(),
                &signals.as_raw(),
                command_buffer.as_raw(),
                &table,
                flags.bits(),
            )
        })
    }

    /// Enqueues a dispatch over a workgroup grid.
    ///
    /// Constants and bindings are copied, and the queue retains the executable
    /// and buffers until the dispatch completes. Bindings must reference
    /// buffers directly; slot-based indirect workgroup counts exist only in
    /// command buffers. A device-buffer workgroup count can be supplied on the
    /// [`QueueDispatch`] value.
    pub fn queue_dispatch(
        &self,
        submission: &QueueSubmission<'_>,
        dispatch: &QueueDispatch<'_>,
    ) -> Result<()> {
        self.queue_dispatch_impl(submission, dispatch, false)
    }

    /// Enqueues a dispatch without retaining resources whose lifetimes are
    /// marked borrowed.
    ///
    /// # Safety
    /// Every executable and buffer referenced by the dispatch must remain
    /// valid until the submitted work has completed. The caller must also
    /// ensure the completion signal is observed before releasing them.
    pub unsafe fn queue_dispatch_unchecked(
        &self,
        submission: &QueueSubmission<'_>,
        dispatch: &QueueDispatch<'_>,
    ) -> Result<()> {
        self.queue_dispatch_impl(submission, dispatch, true)
    }

    fn queue_dispatch_impl(
        &self,
        submission: &QueueSubmission<'_>,
        dispatch: &QueueDispatch<'_>,
        allow_borrowed_lifetimes: bool,
    ) -> Result<()> {
        if !allow_borrowed_lifetimes
            && dispatch
                .flags
                .contains(DispatchFlags::BORROW_RESOURCE_LIFETIMES)
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "borrowed resource lifetimes cannot be guaranteed by a safe dispatch",
            ));
        }
        if dispatch.config.indirect_workgroup_count.is_some()
            && dispatch.workgroup_count_buffer.is_some()
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue dispatch cannot combine a binding-table slot with a direct parameter buffer",
            ));
        }
        if dispatch.config.indirect_workgroup_count.is_some()
            && dispatch.workgroup_count_buffer.is_none()
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue dispatches cannot read workgroup counts from binding-table slots",
            ));
        }
        if dispatch
            .workgroup_count_buffer
            .is_some_and(|reference| !reference.is_direct())
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue dispatch workgroup-count references must be direct",
            ));
        }
        if !dispatch.bindings.iter().all(BufferRef::is_direct) {
            return Err(Error::new(
                Code::InvalidArgument,
                "queue dispatch bindings must reference buffers directly",
            ));
        }
        // `BufferRef` is `repr(transparent)` over the native reference.
        let binding_list = n::iree_hal_buffer_ref_list_t {
            count: dispatch.bindings.len(),
            values: dispatch.bindings.as_ptr().cast(),
        };
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        let raw_config = dispatch
            .config
            .raw_with_workgroup_count_ref(dispatch.workgroup_count_buffer);
        // SAFETY: every handle is borrowed live for the call, the copied
        // inputs outlive it, and safe calls use native resource retention;
        // unchecked calls transfer the documented lifetime obligation to
        // the caller.
        check(unsafe {
            n::iree_hal_device_queue_dispatch(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                dispatch.executable.as_raw(),
                dispatch.function.raw(),
                raw_config,
                const_span(dispatch.constants),
                binding_list,
                dispatch.flags.bits(),
            )
        })
    }

    /// Enqueues `call` to run on a host thread once every wait is satisfied.
    ///
    /// By default the signals fire when `call` returns `Ok` and fail with its
    /// error otherwise. With [`HostCallFlags::NON_BLOCKING`] the signals fire
    /// as soon as the call is issued and its result is ignored.
    ///
    /// `call` must not enqueue queue work or block waiting on the device: IREE
    /// does not support re-entrant queue use from host calls and may hang. If
    /// the queue drops an accepted call without running it (for example
    /// because a wait failed), its captured state is leaked rather than
    /// dropped.
    pub fn queue_host_call<F>(
        &self,
        submission: &QueueSubmission<'_>,
        flags: HostCallFlags,
        call: F,
    ) -> Result<()>
    where
        F: FnOnce(&HostCallContext) -> Result<()> + Send + 'static,
    {
        // Both the queue and this function hold the slot: some devices run the
        // call inline and still fail the enqueue afterwards, so ownership of the
        // closure is settled by whichever side takes it first.
        let slot: HostCallSlot<F> = Arc::new(Mutex::new(Some(call)));
        let user_data = Arc::into_raw(Arc::clone(&slot));
        let native_call = n::iree_hal_host_call_t {
            fn_: Some(host_call_trampoline::<F>),
            user_data: user_data.cast_mut().cast(),
        };
        let args = [0u64; 4];
        let waits = SemaphoreList::from_points(submission.waits());
        let signals = SemaphoreList::from_points(submission.signals());
        // SAFETY: the device and semaphores are borrowed live for the call, and
        // the queue's slot reference stays valid until the trampoline takes it.
        let result = check(unsafe {
            n::iree_hal_device_queue_host_call(
                self.as_raw(),
                submission.affinity().bits(),
                waits.as_raw(),
                signals.as_raw(),
                native_call,
                args.as_ptr(),
                flags.bits(),
            )
        });
        if result.is_err() {
            // The queue may still hold its reference and run the trampoline
            // later, so that reference is leaked; an untaken closure is dropped
            // here and the trampoline then finds the slot empty.
            drop(lock_slot(&slot).take());
        }
        result
    }
}

/// Retains a HAL file until its asynchronous queue transfer completes.
///
/// Dropping the guard blocks until the transfer finishes; use [`FileTransfer::wait`]
/// to observe a transfer failure.
pub struct FileTransfer<'a> {
    _file: File<'a>,
    completion: Semaphore,
}
impl FileTransfer<'_> {
    /// The completion timepoint, for chaining another queued operation.
    pub fn completion_point(&self) -> SemaphorePoint<'_> {
        SemaphorePoint {
            semaphore: &self.completion,
            value: 1,
        }
    }

    /// Blocks until the transfer has completed, returning its failure if any.
    pub fn wait(self) -> Result<()> {
        self.completion.wait(1, None, WaitFlags::default())
    }
}
impl Drop for FileTransfer<'_> {
    fn drop(&mut self) {
        // The file must outlive the transfer; a failure is reported by `wait`.
        let _ = self.completion.wait(1, None, WaitFlags::default());
    }
}

/// Chunking settings for synchronous streaming file transfers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileTransferOptions {
    /// Number of staging chunks; zero selects the native default.
    pub chunk_count: DeviceSize,
    /// Maximum chunk size in bytes; zero selects the native default.
    pub chunk_size: DeviceSize,
}
impl FileTransferOptions {
    fn raw(self) -> n::iree_hal_file_transfer_options_t {
        n::iree_hal_file_transfer_options_t {
            chunk_count: self.chunk_count,
            chunk_size: self.chunk_size,
        }
    }
}

fn append_completion_signal<'a>(
    signals: &[SemaphorePoint<'a>],
    completion: &'a Semaphore,
) -> Vec<SemaphorePoint<'a>> {
    signals
        .iter()
        .copied()
        .chain([SemaphorePoint {
            semaphore: completion,
            value: 1,
        }])
        .collect()
}

type HostCallSlot<F> = Arc<Mutex<Option<F>>>;

fn lock_slot<F>(slot: &Mutex<Option<F>>) -> std::sync::MutexGuard<'_, Option<F>> {
    slot.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Where a queued host call is running.
#[derive(Clone, Copy, Debug)]
pub struct HostCallContext {
    queue_affinity: QueueAffinity,
}

impl HostCallContext {
    /// The queues blocked on this call; a subset of the enqueued affinity.
    pub fn queue_affinity(&self) -> QueueAffinity {
        self.queue_affinity
    }
}

unsafe extern "C" fn host_call_trampoline<F>(
    user_data: *mut c_void,
    _args: *const u64,
    context: *mut n::iree_hal_host_call_context_t,
) -> n::iree_status_t
where
    F: FnOnce(&HostCallContext) -> Result<()> + Send + 'static,
{
    // SAFETY: `user_data` is the slot reference handed to the queue by
    // `queue_host_call`, and the queue invokes a host call at most once.
    let slot = unsafe { Arc::from_raw(user_data.cast_const().cast::<Mutex<Option<F>>>()) };
    let Some(call) = lock_slot(&slot).take() else {
        return Error::new(Code::Cancelled, "host call enqueue failed").into_status();
    };
    drop(slot);
    let queue_affinity = if context.is_null() {
        QueueAffinity::default()
    } else {
        // SAFETY: a non-null context is valid for the duration of the call.
        QueueAffinity::from_bits_retain(unsafe { (*context).queue_affinity })
    };
    let host_context = HostCallContext { queue_affinity };
    n::callback_status(move || match call(&host_context) {
        Err(error) if error.code() == Code::Deferred => Err(Error::new(
            Code::Internal,
            "host calls cannot defer: their signal list is not retained",
        )),
        result => result,
    })
}

bitfield! {
    /// Queue-affinity bitfield (`iree_hal_queue_affinity_t`); zero lets the device choose.
    QueueAffinity: n::iree_hal_queue_affinity_t {
        /// Any queue may be selected (`IREE_HAL_QUEUE_AFFINITY_ANY`).
        ANY = n::iree_hal_queue_affinity_t::MAX,
    }
}

/// A device identifier returned by a specific driver instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeviceId(usize);
impl DeviceId {
    /// Creates an identifier from an enumerated raw value.
    pub const fn from_raw(value: usize) -> Self {
        Self(value)
    }
    /// Returns the identifier value.
    pub const fn value(self) -> usize {
        self.0
    }
}

/// Mode used when waiting for a collection of semaphore timepoints.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WaitMode {
    /// Wait until every timepoint is satisfied.
    All,
    /// Return when any timepoint is satisfied.
    Any,
}
impl WaitMode {
    pub(crate) const fn raw(self) -> n::iree_async_wait_mode_t {
        match self {
            Self::All => n::IREE_ASYNC_WAIT_MODE_ALL as n::iree_async_wait_mode_t,
            Self::Any => n::IREE_ASYNC_WAIT_MODE_ANY as n::iree_async_wait_mode_t,
        }
    }
}

bitfield! {
    /// Options modifying host wait behavior (`iree_async_wait_flags_t`).
    WaitFlags: n::iree_async_wait_flags_t {
        /// Yield while waiting instead of busy-spinning.
        YIELD = n::IREE_ASYNC_WAIT_FLAG_YIELD,
        /// Actively spin while waiting.
        ACTIVE = n::IREE_ASYNC_WAIT_FLAG_ACTIVE,
    }
}

bitfield! {
    /// Queue allocation flags (`iree_hal_alloca_flags_t`).
    AllocaFlags: n::iree_hal_alloca_flags_t {
        /// The allocation has an indeterminate lifetime.
        INDETERMINATE_LIFETIME = n::IREE_HAL_ALLOCA_FLAG_INDETERMINATE_LIFETIME,
        /// The allocator may wait for a pool frontier.
        ALLOW_POOL_WAIT_FRONTIER = n::IREE_HAL_ALLOCA_FLAG_ALLOW_POOL_WAIT_FRONTIER,
    }
}

bitfield! {
    /// Queue deallocation flags (`iree_hal_dealloca_flags_t`).
    DeallocaFlags: n::iree_hal_dealloca_flags_t {
        /// Prefer returning the allocation to its origin.
        PREFER_ORIGIN = n::IREE_HAL_DEALLOCA_FLAG_PREFER_ORIGIN,
    }
}

bitfield! {
    /// Queue execution flags (`iree_hal_execute_flags_t`).
    ExecuteFlags: n::iree_hal_execute_flags_t {
        /// Borrow the binding-table lifetime through execution completion.
        BORROW_BINDING_TABLE_LIFETIME = n::IREE_HAL_EXECUTE_FLAG_BORROW_BINDING_TABLE_LIFETIME,
    }
}

bitfield! {
    /// Queue host-call behavior flags (`iree_hal_host_call_flags_t`).
    HostCallFlags: n::iree_hal_host_call_flags_t {
        /// Signals dependencies when issued; the call runs detached.
        NON_BLOCKING = n::IREE_HAL_HOST_CALL_FLAG_NON_BLOCKING,
        /// Hints that the call is short and the queue may spin while waiting.
        WAIT_ACTIVE = n::IREE_HAL_HOST_CALL_FLAG_WAIT_ACTIVE,
        /// Hints that no device cache flush or invalidation is needed.
        RELAXED = n::IREE_HAL_HOST_CALL_FLAG_RELAXED,
    }
}
