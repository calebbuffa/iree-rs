//! Command-buffer recording, dispatch configuration, and debug labels.

use super::bitfield::{bitfield, scalar_value};
use super::debug_label::{LabelColor, LabelLocation};
use super::{
    CollectiveOp, ExecutableFunction, QueueAffinity, buffer::Buffer, channel::Channel,
    device::Device, event::Event, executable::Executable,
};
use crate::hal::DeviceSize;
use crate::{Code, Error, Result, check};
use core::marker::PhantomData;
use iree_native as n;
use iree_native::const_span;

/// A direct or binding-table reference used by dispatch commands.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BufferRef<'a> {
    raw: n::iree_hal_buffer_ref_t,
    _buffer: PhantomData<&'a Buffer>,
}
impl<'a> BufferRef<'a> {
    /// References a byte range of a buffer.
    pub fn new(buffer: &'a Buffer, offset: DeviceSize, length: DeviceSize) -> Self {
        Self {
            raw: buffer_ref(buffer, offset, length),
            _buffer: PhantomData,
        }
    }

    /// Returns an indirect reference to a queue binding-table slot.
    pub fn indirect(slot: u32, offset: DeviceSize, length: DeviceSize) -> BufferRef<'static> {
        BufferRef {
            // SAFETY: constructs a plain value.
            raw: unsafe { n::iree_hal_make_indirect_buffer_ref(slot, offset, length) },
            _buffer: PhantomData,
        }
    }

    pub(crate) fn is_direct(&self) -> bool {
        !self.raw.buffer.is_null()
    }
}

fn buffer_ref(buffer: &Buffer, offset: DeviceSize, length: DeviceSize) -> n::iree_hal_buffer_ref_t {
    // SAFETY: constructs a plain value borrowing the live buffer pointer.
    unsafe { n::iree_hal_make_buffer_ref(buffer.as_raw(), offset, length) }
}

/// A buffer-range dependency attached to a command-buffer barrier.
#[derive(Clone, Copy)]
pub struct BufferBarrier<'a> {
    /// Buffer whose accesses are synchronized.
    pub buffer: &'a Buffer,
    /// Byte offset in the buffer.
    pub offset: DeviceSize,
    /// Byte length in the buffer.
    pub length: DeviceSize,
    /// Accesses that must complete before the barrier.
    pub source: AccessScope,
    /// Accesses that may proceed after the barrier.
    pub target: AccessScope,
}
impl BufferBarrier<'_> {
    fn raw(self) -> n::iree_hal_buffer_barrier_t {
        n::iree_hal_buffer_barrier_t {
            source_scope: self.source.bits(),
            target_scope: self.target.bits(),
            buffer_ref: buffer_ref(self.buffer, self.offset, self.length),
        }
    }
}

/// One collective operation recorded on a command buffer.
pub struct CollectiveCommand<'a> {
    /// Channel participating in the operation.
    pub channel: &'a Channel,
    /// Operation kind, reduction, and element type.
    pub op: CollectiveOp,
    /// Operation-specific parameter.
    pub param: u32,
    /// Source buffer range.
    pub send: BufferRef<'a>,
    /// Destination buffer range.
    pub receive: BufferRef<'a>,
    /// Number of elements in the operation.
    pub element_count: DeviceSize,
}

/// One host-described command consumed while creating a transfer command buffer.
pub enum TransferCommand<'a> {
    /// Fills a device-buffer range with a 1-, 2-, or 4-byte pattern.
    Fill {
        /// Buffer to fill.
        target: &'a Buffer,
        /// Byte offset in the target.
        target_offset: DeviceSize,
        /// Byte length to fill.
        length: DeviceSize,
        /// Pattern bytes.
        pattern: &'a [u8],
    },
    /// Copies host bytes into a device-buffer range.
    Update {
        /// Host source bytes.
        source: &'a [u8],
        /// Byte offset in the source.
        source_offset: usize,
        /// Buffer to update.
        target: &'a Buffer,
        /// Byte offset in the target.
        target_offset: DeviceSize,
        /// Byte length to copy.
        length: DeviceSize,
    },
    /// Copies a device-buffer range to another device-buffer range.
    Copy {
        /// Buffer to read.
        source: &'a Buffer,
        /// Byte offset in the source.
        source_offset: DeviceSize,
        /// Buffer to write.
        target: &'a Buffer,
        /// Byte offset in the target.
        target_offset: DeviceSize,
        /// Byte length to copy.
        length: DeviceSize,
    },
}
impl TransferCommand<'_> {
    fn raw(&self) -> Result<n::iree_hal_transfer_command_t> {
        let raw = match self {
            Self::Fill {
                target,
                target_offset,
                length,
                pattern,
            } => n::iree_hal_transfer_command_t {
                type_: n::IREE_HAL_TRANSFER_COMMAND_TYPE_FILL,
                __bindgen_anon_1: n::iree_hal_transfer_command_t__bindgen_ty_1 {
                    fill: n::iree_hal_transfer_command_t__bindgen_ty_1__bindgen_ty_1 {
                        target_buffer: target.as_raw(),
                        target_offset: *target_offset,
                        length: *length,
                        pattern: pattern.as_ptr().cast(),
                        pattern_length: pattern.len(),
                    },
                },
            },
            Self::Update {
                source,
                source_offset,
                target,
                target_offset,
                length,
            } => {
                check_host_range(source, *source_offset, *length)?;
                n::iree_hal_transfer_command_t {
                    type_: n::IREE_HAL_TRANSFER_COMMAND_TYPE_UPDATE,
                    __bindgen_anon_1: n::iree_hal_transfer_command_t__bindgen_ty_1 {
                        update: n::iree_hal_transfer_command_t__bindgen_ty_1__bindgen_ty_2 {
                            source_buffer: source.as_ptr().cast(),
                            source_offset: *source_offset,
                            target_buffer: target.as_raw(),
                            target_offset: *target_offset,
                            length: *length,
                        },
                    },
                }
            }
            Self::Copy {
                source,
                source_offset,
                target,
                target_offset,
                length,
            } => n::iree_hal_transfer_command_t {
                type_: n::IREE_HAL_TRANSFER_COMMAND_TYPE_COPY,
                __bindgen_anon_1: n::iree_hal_transfer_command_t__bindgen_ty_1 {
                    copy: n::iree_hal_transfer_command_t__bindgen_ty_1__bindgen_ty_3 {
                        source_buffer: source.as_raw(),
                        source_offset: *source_offset,
                        target_buffer: target.as_raw(),
                        target_offset: *target_offset,
                        length: *length,
                    },
                },
            },
        };
        Ok(raw)
    }
}

/// Rejects host source ranges outside `source`; C cannot see the slice length.
pub(super) fn check_host_range(source: &[u8], offset: usize, length: DeviceSize) -> Result<()> {
    let end = offset.checked_add(length);
    if end.is_none_or(|end| end > source.len()) {
        return Err(Error::new(
            Code::OutOfRange,
            "update range exceeds source slice",
        ));
    }
    Ok(())
}

/// An owned `iree_hal_command_buffer_t` reference.
pub struct CommandBuffer {
    raw: *mut n::iree_hal_command_buffer_t,
    _not_thread_safe: PhantomData<*mut ()>,
}
impl CommandBuffer {
    /// Records a batch of fill, update, and copy transfer commands.
    pub fn create_transfer(
        device: &Device,
        mode: CommandBufferMode,
        queue_affinity: QueueAffinity,
        commands: &[TransferCommand<'_>],
    ) -> Result<Self> {
        let unchecked = CommandBufferMode::UNRETAINED | CommandBufferMode::UNVALIDATED;
        if !(mode & unchecked).is_empty() {
            return Err(Error::new(
                Code::InvalidArgument,
                "transfer command buffers require resource retention and validation",
            ));
        }
        let raw_commands = commands
            .iter()
            .map(TransferCommand::raw)
            .collect::<Result<Vec<_>>>()?;
        let mut raw = core::ptr::null_mut();
        // SAFETY: command records and referenced source data remain live during recording; the mode retains referenced buffers.
        check(unsafe {
            n::iree_hal_create_transfer_command_buffer(
                device.as_raw(),
                mode.bits(),
                queue_affinity.bits(),
                raw_commands.len(),
                raw_commands.as_ptr(),
                &mut raw,
            )
        })?;
        // SAFETY: successful creation returns one owned command-buffer reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Creates a command buffer with the requested recording capabilities.
    ///
    /// Rejects [`CommandBufferMode::UNRETAINED`] and
    /// [`CommandBufferMode::UNVALIDATED`], which move resource-lifetime and
    /// bounds guarantees onto the caller; use [`Self::create_unchecked`].
    pub fn create(
        device: &Device,
        mode: CommandBufferMode,
        categories: CommandCategory,
        queue_affinity: QueueAffinity,
        binding_capacity: usize,
    ) -> Result<Self> {
        let unchecked = CommandBufferMode::UNRETAINED | CommandBufferMode::UNVALIDATED;
        if !(mode & unchecked).is_empty() {
            return Err(Error::new(
                Code::InvalidArgument,
                "unretained or unvalidated command buffers require create_unchecked",
            ));
        }
        // SAFETY: the mode keeps native resource retention and validation.
        unsafe {
            Self::create_unchecked(device, mode, categories, queue_affinity, binding_capacity)
        }
    }

    /// Creates a command buffer with any recording mode.
    ///
    /// # Safety
    /// With [`CommandBufferMode::UNRETAINED`], every recorded resource must
    /// outlive all submissions of this command buffer. With
    /// [`CommandBufferMode::UNVALIDATED`], every recorded command must be valid
    /// (in-bounds ranges, compatible usage), as the device will not check it.
    pub unsafe fn create_unchecked(
        device: &Device,
        mode: CommandBufferMode,
        categories: CommandCategory,
        queue_affinity: QueueAffinity,
        binding_capacity: usize,
    ) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the device is live and `raw` is writable; the caller upholds the mode contract.
        check(unsafe {
            n::iree_hal_command_buffer_create(
                device.as_raw(),
                mode.bits(),
                categories.bits(),
                queue_affinity.bits(),
                binding_capacity,
                &mut raw,
            )
        })?;
        // SAFETY: successful create returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned command-buffer reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_command_buffer_t) -> Self {
        Self {
            raw,
            _not_thread_safe: PhantomData,
        }
    }

    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_command_buffer_t {
        self.raw
    }

    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_command_buffer_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Returns the command buffer recording mode.
    pub fn mode(&self) -> CommandBufferMode {
        // SAFETY: the wrapper owns a live reference.
        CommandBufferMode::from_bits_retain(unsafe { n::iree_hal_command_buffer_mode(self.raw) })
    }

    /// Returns the allowed command categories.
    pub fn allowed_categories(&self) -> CommandCategory {
        // SAFETY: the wrapper owns a live reference.
        CommandCategory::from_bits_retain(unsafe {
            n::iree_hal_command_buffer_allowed_categories(self.raw)
        })
    }

    /// Returns queue affinity.
    pub fn queue_affinity(&self) -> QueueAffinity {
        // SAFETY: the wrapper owns a live reference.
        QueueAffinity::from_bits_retain(unsafe {
            n::iree_hal_command_buffer_queue_affinity(self.raw)
        })
    }

    /// Returns the associated profiling identifier.
    pub fn profile_id(&self) -> u64 {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_command_buffer_profile_id(self.raw) }
    }

    /// Begins recording commands.
    pub fn begin(&mut self) -> Result<()> {
        // SAFETY: the wrapper owns a live reference and `&mut self` serializes recording.
        check(unsafe { n::iree_hal_command_buffer_begin(self.raw) })
    }

    /// Ends recording commands.
    pub fn end(&mut self) -> Result<()> {
        // SAFETY: the wrapper owns a live reference and `&mut self` serializes recording.
        check(unsafe { n::iree_hal_command_buffer_end(self.raw) })
    }

    /// Begins a labeled debug group.
    pub fn begin_debug_group(
        &mut self,
        label: &str,
        color: LabelColor,
        location: Option<LabelLocation<'_>>,
    ) -> Result<()> {
        let native_location = location.map(LabelLocation::raw);
        let location = native_location
            .as_ref()
            .map_or(core::ptr::null(), |value| value as *const _);
        // SAFETY: label and location are borrowed for the call; the native copies them.
        check(unsafe {
            n::iree_hal_command_buffer_begin_debug_group(
                self.raw,
                crate::view(label),
                color.raw(),
                location,
            )
        })
    }

    /// Ends the current debug group.
    pub fn end_debug_group(&mut self) -> Result<()> {
        // SAFETY: the wrapper owns a live reference and `&mut self` serializes recording.
        check(unsafe { n::iree_hal_command_buffer_end_debug_group(self.raw) })
    }

    /// Records an execution barrier with the supplied memory and buffer dependencies.
    pub fn execution_barrier(
        &mut self,
        source: ExecutionStage,
        target: ExecutionStage,
        memory: &[MemoryBarrier],
        buffers: &[BufferBarrier],
    ) -> Result<()> {
        let raw_memory: Vec<_> = memory.iter().copied().map(MemoryBarrier::raw).collect();
        let raw_buffers: Vec<_> = buffers.iter().copied().map(BufferBarrier::raw).collect();
        // SAFETY: barrier arrays borrow live buffers for the call.
        check(unsafe {
            n::iree_hal_command_buffer_execution_barrier(
                self.raw,
                source.bits(),
                target.bits(),
                n::IREE_HAL_EXECUTION_BARRIER_FLAG_NONE as n::iree_hal_execution_barrier_flags_t,
                raw_memory.len(),
                raw_memory.as_ptr(),
                raw_buffers.len(),
                raw_buffers.as_ptr(),
            )
        })
    }

    /// Records an event signal.
    pub fn signal_event(&mut self, event: &Event, source: ExecutionStage) -> Result<()> {
        // SAFETY: the event is borrowed for the call and retained by the recording.
        check(unsafe {
            n::iree_hal_command_buffer_signal_event(self.raw, event.as_raw(), source.bits())
        })
    }

    /// Records an event reset.
    pub fn reset_event(&mut self, event: &Event, source: ExecutionStage) -> Result<()> {
        // SAFETY: the event is borrowed for the call and retained by the recording.
        check(unsafe {
            n::iree_hal_command_buffer_reset_event(self.raw, event.as_raw(), source.bits())
        })
    }

    /// Records a wait for events and applies the supplied dependencies.
    pub fn wait_events(
        &mut self,
        events: &[&Event],
        source: ExecutionStage,
        target: ExecutionStage,
        memory: &[MemoryBarrier],
        buffers: &[BufferBarrier],
    ) -> Result<()> {
        let mut raw_events: Vec<*const n::iree_hal_event_t> = events
            .iter()
            .map(|event| event.as_raw() as *const _)
            .collect();
        let raw_memory: Vec<_> = memory.iter().copied().map(MemoryBarrier::raw).collect();
        let raw_buffers: Vec<_> = buffers.iter().copied().map(BufferBarrier::raw).collect();
        // SAFETY: event and barrier arrays borrow live resources for the call.
        check(unsafe {
            n::iree_hal_command_buffer_wait_events(
                self.raw,
                raw_events.len(),
                raw_events.as_mut_ptr(),
                source.bits(),
                target.bits(),
                raw_memory.len(),
                raw_memory.as_ptr(),
                raw_buffers.len(),
                raw_buffers.as_ptr(),
            )
        })
    }

    /// Records a buffer memory advice operation.
    pub fn advise_buffer(
        &mut self,
        buffer: &Buffer,
        offset: DeviceSize,
        length: DeviceSize,
        advice: MemoryAdvice,
        arg0: u64,
        arg1: u64,
    ) -> Result<()> {
        let reference = buffer_ref(buffer, offset, length);
        // SAFETY: the buffer is borrowed for the call and retained by the recording.
        check(unsafe {
            n::iree_hal_command_buffer_advise_buffer(self.raw, reference, advice.bits(), arg0, arg1)
        })
    }

    /// Records a host cache flush for a buffer range.
    pub fn flush_buffer(
        &mut self,
        buffer: &Buffer,
        offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        let reference = buffer_ref(buffer, offset, length);
        // SAFETY: the buffer is borrowed for the call and retained by the recording.
        check(unsafe { n::iree_hal_command_buffer_flush_buffer(self.raw, reference) })
    }

    /// Records a host cache invalidation for a buffer range.
    pub fn invalidate_buffer(
        &mut self,
        buffer: &Buffer,
        offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        let reference = buffer_ref(buffer, offset, length);
        // SAFETY: the buffer is borrowed for the call and retained by the recording.
        check(unsafe { n::iree_hal_command_buffer_invalidate_buffer(self.raw, reference) })
    }

    /// Records filling a direct buffer range with a repeated native pattern.
    pub fn fill_buffer(
        &mut self,
        buffer: &Buffer,
        offset: DeviceSize,
        length: DeviceSize,
        pattern: &[u8],
    ) -> Result<()> {
        self.fill_buffer_ref(BufferRef::new(buffer, offset, length), pattern)
    }

    /// Records filling a direct or binding-table buffer reference.
    ///
    /// Indirect references are resolved from the binding table supplied to
    /// [`GroupedDevice::queue_execute`](super::device::GroupedDevice::queue_execute) when the
    /// command buffer is submitted.
    pub fn fill_buffer_ref(&mut self, target: BufferRef<'_>, pattern: &[u8]) -> Result<()> {
        // SAFETY: the pattern is borrowed for the call and copied by the native.
        check(unsafe {
            n::iree_hal_command_buffer_fill_buffer(
                self.raw,
                target.raw,
                pattern.as_ptr().cast(),
                pattern.len(),
                n::IREE_HAL_FILL_FLAG_NONE as n::iree_hal_fill_flags_t,
            )
        })
    }

    /// Records updating a direct buffer range from host memory.
    pub fn update_buffer(
        &mut self,
        source: &[u8],
        source_offset: usize,
        target: &Buffer,
        target_offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        self.update_buffer_ref(
            source,
            source_offset,
            BufferRef::new(target, target_offset, length),
        )
    }

    /// Records updating a direct or binding-table buffer reference from host memory.
    ///
    /// The source bytes are copied into the command buffer during recording.
    /// Indirect references are resolved from the binding table supplied to
    /// [`GroupedDevice::queue_execute`](super::device::GroupedDevice::queue_execute) at submission.
    pub fn update_buffer_ref(
        &mut self,
        source: &[u8],
        source_offset: usize,
        target: BufferRef<'_>,
    ) -> Result<()> {
        check_host_range(source, source_offset, target.raw.length)?;
        // SAFETY: the source range was bounds-checked and is copied during recording.
        check(unsafe {
            n::iree_hal_command_buffer_update_buffer(
                self.raw,
                source.as_ptr().cast(),
                source_offset,
                target.raw,
                n::IREE_HAL_UPDATE_FLAG_NONE as n::iree_hal_update_flags_t,
            )
        })
    }

    /// Records a device-to-device copy between direct buffer ranges.
    pub fn copy_buffer(
        &mut self,
        source: &Buffer,
        source_offset: DeviceSize,
        target: &Buffer,
        target_offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        self.copy_buffer_refs(
            BufferRef::new(source, source_offset, length),
            BufferRef::new(target, target_offset, length),
        )
    }

    /// Records a device-to-device copy between direct or binding-table references.
    ///
    /// Indirect references are resolved from the binding table supplied to
    /// [`GroupedDevice::queue_execute`](super::device::GroupedDevice::queue_execute) at submission.
    pub fn copy_buffer_refs(&mut self, source: BufferRef<'_>, target: BufferRef<'_>) -> Result<()> {
        // SAFETY: direct buffers are retained by the recording; indirect references are
        // resolved against the submission's retained binding table.
        check(unsafe {
            n::iree_hal_command_buffer_copy_buffer(
                self.raw,
                source.raw,
                target.raw,
                n::IREE_HAL_COPY_FLAG_NONE as n::iree_hal_copy_flags_t,
            )
        })
    }

    /// Records a collective operation on a channel.
    pub fn collective(&mut self, command: CollectiveCommand<'_>) -> Result<()> {
        // SAFETY: the channel and buffers are borrowed for the call and retained by the recording.
        check(unsafe {
            n::iree_hal_command_buffer_collective(
                self.raw,
                command.channel.as_raw(),
                command.op.raw(),
                command.param,
                command.send.raw,
                command.receive.raw,
                command.element_count,
            )
        })
    }

    /// Records an executable dispatch using the supplied binding references.
    pub fn dispatch(
        &mut self,
        executable: &Executable,
        function: ExecutableFunction,
        config: DispatchConfig,
        constants: &[u8],
        bindings: &[BufferRef<'_>],
        flags: DispatchFlags,
    ) -> Result<()> {
        // `BufferRef` is `repr(transparent)` over the native reference.
        let list = n::iree_hal_buffer_ref_list_t {
            count: bindings.len(),
            values: bindings.as_ptr().cast(),
        };
        // SAFETY: executable, constants, and bindings are borrowed for the call.
        check(unsafe {
            n::iree_hal_command_buffer_dispatch(
                self.raw,
                executable.as_raw(),
                function.raw(),
                config.raw(),
                const_span(constants),
                list,
                flags.bits(),
            )
        })
    }
}
impl Clone for CommandBuffer {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_command_buffer_retain(self.raw) };
        Self {
            raw: self.raw,
            _not_thread_safe: PhantomData,
        }
    }
}
impl Drop for CommandBuffer {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_command_buffer_release(self.raw) }
    }
}

bitfield! {
    /// Command-buffer recording mode bits (`iree_hal_command_buffer_mode_t`).
    CommandBufferMode: n::iree_hal_command_buffer_mode_t {
        /// A command buffer that can be submitted only once.
        ONE_SHOT = n::IREE_HAL_COMMAND_BUFFER_MODE_ONE_SHOT,
        /// Allows commands to execute inline when submitted.
        ALLOW_INLINE_EXECUTION = n::IREE_HAL_COMMAND_BUFFER_MODE_ALLOW_INLINE_EXECUTION,
        /// Disables command validation.
        UNVALIDATED = n::IREE_HAL_COMMAND_BUFFER_MODE_UNVALIDATED,
        /// Does not retain recorded resources.
        UNRETAINED = n::IREE_HAL_COMMAND_BUFFER_MODE_UNRETAINED,
        /// Retains profiling metadata.
        RETAIN_PROFILE_METADATA = n::IREE_HAL_COMMAND_BUFFER_MODE_RETAIN_PROFILE_METADATA,
        /// Retains dispatch metadata.
        RETAIN_DISPATCH_METADATA = n::IREE_HAL_COMMAND_BUFFER_MODE_RETAIN_DISPATCH_METADATA,
    }
}

bitfield! {
    /// Allowed command categories for a command buffer (`iree_hal_command_category_t`).
    CommandCategory: n::iree_hal_command_category_t {
        /// Transfer commands.
        TRANSFER = n::IREE_HAL_COMMAND_CATEGORY_TRANSFER,
        /// Dispatch commands.
        DISPATCH = n::IREE_HAL_COMMAND_CATEGORY_DISPATCH,
        /// All command categories.
        ANY = n::IREE_HAL_COMMAND_CATEGORY_ANY,
    }
}

bitfield! {
    /// Command-buffer execution stages (`iree_hal_execution_stage_t`).
    ExecutionStage: n::iree_hal_execution_stage_t {
        /// Command issue stage.
        COMMAND_ISSUE = n::IREE_HAL_EXECUTION_STAGE_COMMAND_ISSUE,
        /// Command processing stage.
        COMMAND_PROCESS = n::IREE_HAL_EXECUTION_STAGE_COMMAND_PROCESS,
        /// Dispatch stage.
        DISPATCH = n::IREE_HAL_EXECUTION_STAGE_DISPATCH,
        /// Transfer stage.
        TRANSFER = n::IREE_HAL_EXECUTION_STAGE_TRANSFER,
        /// Command retirement stage.
        COMMAND_RETIRE = n::IREE_HAL_EXECUTION_STAGE_COMMAND_RETIRE,
        /// Host stage.
        HOST = n::IREE_HAL_EXECUTION_STAGE_HOST,
    }
}

bitfield! {
    /// Dispatch command behavior flags (`iree_hal_dispatch_flags_t`).
    DispatchFlags: n::iree_hal_dispatch_flags_t {
        /// The workgroup count is read from an indirect reference at execution time.
        DYNAMIC_INDIRECT_PARAMETERS = n::IREE_HAL_DISPATCH_FLAG_DYNAMIC_INDIRECT_PARAMETERS,
        /// Indirect parameters are known at recording time.
        STATIC_INDIRECT_PARAMETERS = n::IREE_HAL_DISPATCH_FLAG_STATIC_INDIRECT_PARAMETERS,
        /// Uses custom direct arguments.
        CUSTOM_DIRECT_ARGUMENTS = n::IREE_HAL_DISPATCH_FLAG_CUSTOM_DIRECT_ARGUMENTS,
        /// Dispatch arguments are read indirectly at execution time.
        DYNAMIC_INDIRECT_ARGUMENTS = n::IREE_HAL_DISPATCH_FLAG_DYNAMIC_INDIRECT_ARGUMENTS,
        /// Dispatch arguments are fixed at recording time.
        STATIC_INDIRECT_ARGUMENTS = n::IREE_HAL_DISPATCH_FLAG_STATIC_INDIRECT_ARGUMENTS,
        /// Allows inline execution.
        ALLOW_INLINE_EXECUTION = n::IREE_HAL_DISPATCH_FLAG_ALLOW_INLINE_EXECUTION,
        /// Borrows resource lifetimes through dispatch completion.
        BORROW_RESOURCE_LIFETIMES = n::IREE_HAL_DISPATCH_FLAG_BORROW_RESOURCE_LIFETIMES,
    }
}

scalar_value! {
    /// Buffer memory advice (`iree_hal_memory_advice_t`).
    MemoryAdvice: n::iree_hal_memory_advice_t {
        /// Normal memory use.
        NORMAL = n::IREE_HAL_MEMORY_ADVICE_NORMAL,
        /// The buffer is expected to be used soon.
        WILL_NEED = n::IREE_HAL_MEMORY_ADVICE_WILL_NEED,
        /// The buffer is not expected to be used soon.
        DONT_NEED = n::IREE_HAL_MEMORY_ADVICE_DONT_NEED,
    }
}

bitfield! {
    /// Memory access scopes for command barriers (`iree_hal_access_scope_t`).
    AccessScope: n::iree_hal_access_scope_t {
        /// Indirect-command reads.
        INDIRECT_COMMAND_READ = n::IREE_HAL_ACCESS_SCOPE_INDIRECT_COMMAND_READ,
        /// Constant reads.
        CONSTANT_READ = n::IREE_HAL_ACCESS_SCOPE_CONSTANT_READ,
        /// Dispatch reads.
        DISPATCH_READ = n::IREE_HAL_ACCESS_SCOPE_DISPATCH_READ,
        /// Dispatch writes.
        DISPATCH_WRITE = n::IREE_HAL_ACCESS_SCOPE_DISPATCH_WRITE,
        /// Transfer reads.
        TRANSFER_READ = n::IREE_HAL_ACCESS_SCOPE_TRANSFER_READ,
        /// Transfer writes.
        TRANSFER_WRITE = n::IREE_HAL_ACCESS_SCOPE_TRANSFER_WRITE,
        /// Host reads.
        HOST_READ = n::IREE_HAL_ACCESS_SCOPE_HOST_READ,
        /// Host writes.
        HOST_WRITE = n::IREE_HAL_ACCESS_SCOPE_HOST_WRITE,
        /// All memory reads.
        MEMORY_READ = n::IREE_HAL_ACCESS_SCOPE_MEMORY_READ,
        /// All memory writes.
        MEMORY_WRITE = n::IREE_HAL_ACCESS_SCOPE_MEMORY_WRITE,
    }
}

/// Memory dependency between two access scopes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MemoryBarrier {
    /// Accesses that must complete before the barrier.
    pub source: AccessScope,
    /// Accesses that may proceed after the barrier.
    pub target: AccessScope,
}
impl MemoryBarrier {
    pub(crate) fn raw(self) -> n::iree_hal_memory_barrier_t {
        n::iree_hal_memory_barrier_t {
            source_scope: self.source.bits(),
            target_scope: self.target.bits(),
        }
    }
}

/// Workgroup and indirect-parameter configuration for an executable dispatch.
#[derive(Clone, Copy, Debug, Default)]
pub struct DispatchConfig {
    /// Workgroup size in each dimension, or zeros for the executable default.
    pub workgroup_size: [u32; 3],
    /// Workgroup count in each dimension for direct dispatch.
    pub workgroup_count: [u32; 3],
    /// Dynamic local-memory allocation size in bytes.
    pub dynamic_workgroup_local_memory: u32,
    /// Optional indirect workgroup-count binding `(slot, offset, length)`.
    pub indirect_workgroup_count: Option<(u32, DeviceSize, DeviceSize)>,
}
impl DispatchConfig {
    pub(crate) fn raw(self) -> n::iree_hal_dispatch_config_t {
        self.raw_with_workgroup_count_ref(None)
    }

    pub(crate) fn raw_with_workgroup_count_ref(
        self,
        workgroup_count_ref: Option<BufferRef<'_>>,
    ) -> n::iree_hal_dispatch_config_t {
        let workgroup_count_ref = workgroup_count_ref.map_or_else(
            || {
                self.indirect_workgroup_count.map_or_else(
                    n::iree_hal_buffer_ref_t::default,
                    // SAFETY: constructs a plain value.
                    |(slot, offset, length)| unsafe {
                        n::iree_hal_make_indirect_buffer_ref(slot, offset, length)
                    },
                )
            },
            |reference| reference.raw,
        );
        n::iree_hal_dispatch_config_t {
            workgroup_size: self.workgroup_size,
            workgroup_count: self.workgroup_count,
            workgroup_count_ref,
            dynamic_workgroup_local_memory: self.dynamic_workgroup_local_memory,
        }
    }
}
