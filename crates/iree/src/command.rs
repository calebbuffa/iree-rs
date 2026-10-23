//! Initialized byte buffers and ordered native transfer command submission.

use crate::{
    Device, Element, Error, Fence, FenceWait, Result, Tensor,
    runtime::{AllocationAccess, check, element_count, tensor_buffer_params},
    sys,
};
use std::{
    future::Future,
    ops::Range,
    pin::Pin,
    ptr::{self, NonNull},
    rc::Rc,
    task::{Context, Poll},
};

struct BufferInner {
    raw: NonNull<sys::iree_hal_buffer_t>,
    device: Device,
    access: Rc<AllocationAccess>,
}

impl Drop for BufferInner {
    fn drop(&mut self) {
        if let Err(error) = self.access.wait() {
            eprintln!("IREE buffer work failed during destruction: {error}");
        }
        unsafe { sys::iree_hal_buffer_release(self.raw.as_ptr()) };
    }
}

/// Initialized, device-owned bytes with shared subrange hazard tracking.
///
/// Clones and slices share storage. Safe reads/writes wait for recorded work
/// touching any slice of that allocation. All bytes are initialized before a
/// buffer is exposed. Dropping the last owner drains pending transfers.
#[derive(Clone)]
pub struct Buffer {
    inner: Rc<BufferInner>,
    offset: usize,
    length: usize,
}

impl Device {
    /// Allocate byte storage and initialize it from a host slice.
    pub fn buffer(&self, bytes: &[u8]) -> Result<Buffer> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_allocator_allocate_buffer(
                sys::iree_hal_device_allocator(self.as_raw()),
                tensor_buffer_params(),
                bytes.len(),
                &mut raw,
            )
        })?;
        let buffer = Buffer {
            inner: Rc::new(BufferInner {
                raw: NonNull::new(raw)
                    .ok_or_else(|| Error::invalid("IREE returned a null buffer"))?,
                device: self.clone(),
                access: unsafe { self.allocation_access(raw) },
            }),
            offset: 0,
            length: bytes.len(),
        };
        if !bytes.is_empty() {
            check(unsafe {
                sys::iree_hal_device_transfer_h2d(
                    self.as_raw(),
                    bytes.as_ptr().cast(),
                    raw,
                    0,
                    bytes.len(),
                    0,
                    sys::iree_infinite_timeout(),
                )
            })?;
        }
        Ok(buffer)
    }

    /// Begin recording a one-shot native transfer command buffer.
    ///
    /// The safe recorder retains resources, validates bounds/device identity,
    /// and inserts dependencies between consecutive transfer operations.
    pub fn transfer_commands(&self) -> Result<TransferEncoder> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_command_buffer_create(
                self.as_raw(),
                sys::IREE_HAL_COMMAND_BUFFER_MODE_ONE_SHOT as _,
                sys::IREE_HAL_COMMAND_CATEGORY_TRANSFER as _,
                u64::MAX,
                0,
                &mut raw,
            )
        })?;
        let encoder = TransferEncoder {
            raw,
            device: self.clone(),
            buffers: Vec::new(),
            failed: false,
        };
        check(unsafe { sys::iree_hal_command_buffer_begin(raw) })?;
        Ok(encoder)
    }
}

impl Buffer {
    /// Length of this view in bytes.
    pub fn len(&self) -> usize {
        self.length
    }
    /// Whether this view contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Interpret this initialized byte view as a dense row-major tensor.
    ///
    /// No bytes are copied or transferred. The shape's byte size must exactly
    /// match this view, and its allocation-relative offset must be aligned to
    /// the element size. The tensor and all byte views share pending transfer
    /// and invocation dependencies.
    pub fn as_tensor<T: Element>(&self, shape: &[usize]) -> Result<Tensor> {
        let size = std::mem::size_of::<T>();
        let bytes = element_count(shape)?
            .checked_mul(size)
            .filter(|&bytes| bytes <= isize::MAX as usize)
            .ok_or_else(|| Error::invalid("tensor byte size exceeds address space"))?;
        if bytes != self.length {
            return Err(Error::invalid(
                "tensor shape does not match byte view length",
            ));
        }
        let offset = unsafe { sys::iree_hal_buffer_byte_offset(self.inner.raw.as_ptr()) }
            .checked_add(self.offset)
            .ok_or_else(|| Error::invalid("tensor byte offset overflows address space"))?;
        if !offset.is_multiple_of(size) {
            return Err(Error::invalid("tensor byte offset is not element-aligned"));
        }
        let mut span = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_buffer_subspan(
                self.inner.raw.as_ptr(),
                self.offset,
                self.length,
                sys::iree_allocator_system(),
                &mut span,
            )
        })?;
        let mut raw = ptr::null_mut();
        let status = unsafe {
            sys::iree_hal_buffer_view_create(
                span,
                shape.len(),
                shape.as_ptr(),
                T::TYPE,
                sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as _,
                sys::iree_allocator_system(),
                &mut raw,
            )
        };
        unsafe { sys::iree_hal_buffer_release(span) };
        check(status)?;
        unsafe { Tensor::from_owned(self.inner.device.clone(), raw) }
    }
    /// Create a checked subrange sharing the allocation and hazard state.
    pub fn slice(&self, range: Range<usize>) -> Result<Self> {
        if range.start > range.end || range.end > self.length {
            return Err(Error::invalid("buffer slice is out of bounds"));
        }
        Ok(Self {
            inner: self.inner.clone(),
            offset: self.offset + range.start,
            length: range.end - range.start,
        })
    }
    /// Copy this view to host memory after outstanding commands complete.
    pub fn read(&self) -> Result<Vec<u8>> {
        self.inner.access.wait()?;
        let mut bytes = vec![0; self.length];
        if self.length != 0 {
            check(unsafe {
                sys::iree_hal_device_transfer_d2h(
                    self.inner.device.as_raw(),
                    self.inner.raw.as_ptr(),
                    self.offset,
                    bytes.as_mut_ptr().cast(),
                    self.length,
                    0,
                    sys::iree_infinite_timeout(),
                )
            })?;
        }
        Ok(bytes)
    }
    /// Replace a subrange, waiting for outstanding readers and writers first.
    pub fn write(&self, offset: usize, bytes: &[u8]) -> Result<()> {
        if offset
            .checked_add(bytes.len())
            .is_none_or(|end| end > self.length)
        {
            return Err(Error::invalid("buffer write is out of bounds"));
        }
        self.inner.access.wait()?;
        if !bytes.is_empty() {
            check(unsafe {
                sys::iree_hal_device_transfer_h2d(
                    self.inner.device.as_raw(),
                    bytes.as_ptr().cast(),
                    self.inner.raw.as_ptr(),
                    self.offset + offset,
                    bytes.len(),
                    0,
                    sys::iree_infinite_timeout(),
                )
            })?;
        }
        Ok(())
    }
    fn reference(&self) -> sys::iree_hal_buffer_ref_t {
        sys::iree_hal_buffer_ref_t {
            buffer: self.inner.raw.as_ptr(),
            offset: self.offset,
            length: self.length,
            ..Default::default()
        }
    }
}

impl Tensor {
    /// Create a zero-copy dense view with a different shape.
    ///
    /// The element count and element type are preserved. This creates separate
    /// shape metadata, so existing clones keep their original dimensions.
    /// Storage and pending transfer/invocation dependencies remain shared.
    pub fn reshape(&self, shape: &[usize]) -> Result<Tensor> {
        if element_count(shape)? != element_count(&self.shape())? {
            return Err(Error::invalid("reshape must preserve tensor element count"));
        }
        // Validate the dense encoding before constructing another dense view.
        let _bytes = self.as_buffer()?;
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_buffer_view_create(
                self.buffer(),
                shape.len(),
                shape.as_ptr(),
                self.element_type(),
                sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as _,
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        unsafe { Tensor::from_owned(self.device.clone(), raw) }
    }

    /// Replace all tensor elements with host data of the same type.
    ///
    /// Waits for outstanding safe API readers and writers before transferring.
    /// Every aliased tensor and byte view observes the update; the shape is
    /// unchanged. The input length must match the complete tensor extent.
    pub fn write<T: Element>(&self, values: &[T]) -> Result<()> {
        if self.element_type() != T::TYPE {
            return Err(Error::invalid(
                "tensor element type does not match requested Rust type",
            ));
        }
        let length = crate::runtime::tensor_byte_length(&self.shape(), values)?;
        let buffer = self.as_buffer()?;
        if buffer.len() != length {
            return Err(Error::invalid(
                "tensor byte size does not match its element type",
            ));
        }
        let bytes = unsafe { std::slice::from_raw_parts(values.as_ptr().cast::<u8>(), length) };
        buffer.write(0, bytes)
    }

    /// Borrow the tensor's initialized dense bytes as an owned shared buffer.
    ///
    /// The buffer retains native storage independently of this tensor and
    /// exposes only the logical tensor extent, not unused pool allocation space.
    /// Mutating its bytes changes every aliased tensor. Pending native accesses
    /// are tracked across all tensor and byte views of the same allocation.
    pub fn as_buffer(&self) -> Result<Buffer> {
        if unsafe { sys::iree_hal_buffer_view_encoding_type(self.as_raw()) }
            != sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as sys::iree_hal_encoding_type_t
        {
            return Err(Error::invalid(
                "byte view requires a dense row-major tensor",
            ));
        }
        let raw = self.buffer();
        unsafe { sys::iree_hal_buffer_retain(raw) };
        Ok(Buffer {
            inner: Rc::new(BufferInner {
                raw: NonNull::new(raw)
                    .ok_or_else(|| Error::invalid("IREE returned a null tensor buffer"))?,
                device: self.device.clone(),
                access: unsafe { self.device.allocation_access(raw) },
            }),
            offset: 0,
            length: self.byte_len(),
        })
    }
}

/// A native one-shot transfer recorder with automatic ordering and retention.
///
/// Commands execute only after [`Self::submit`], never during recording. A
/// native recording failure poisons the encoder and prevents submission.
pub struct TransferEncoder {
    raw: *mut sys::iree_hal_command_buffer_t,
    device: Device,
    buffers: Vec<Buffer>,
    failed: bool,
}

impl TransferEncoder {
    fn validate(&self, buffer: &Buffer) -> Result<()> {
        if self.failed {
            return Err(Error::invalid(
                "transfer recorder was poisoned by a native failure",
            ));
        }
        if self.device.as_raw() != buffer.inner.device.as_raw() {
            return Err(Error::invalid("transfer buffer belongs to another device"));
        }
        Ok(())
    }

    fn recorded(&mut self, status: sys::iree_status_t) -> Result<()> {
        if let Err(error) = check(status) {
            self.failed = true;
            return Err(error);
        }
        let scope = (sys::IREE_HAL_ACCESS_SCOPE_TRANSFER_READ
            | sys::IREE_HAL_ACCESS_SCOPE_TRANSFER_WRITE) as _;
        let barrier = sys::iree_hal_memory_barrier_t {
            source_scope: scope,
            target_scope: scope,
        };
        if let Err(error) = check(unsafe {
            sys::iree_hal_command_buffer_execution_barrier(
                self.raw,
                sys::IREE_HAL_EXECUTION_STAGE_TRANSFER as _,
                sys::IREE_HAL_EXECUTION_STAGE_TRANSFER as _,
                0,
                1,
                &barrier,
                0,
                ptr::null(),
            )
        }) {
            self.failed = true;
            return Err(error);
        }
        Ok(())
    }

    /// Fill a complete view with a repeating 1-, 2-, or 4-byte pattern.
    pub fn fill(&mut self, target: &Buffer, pattern: &[u8]) -> Result<&mut Self> {
        self.validate(target)?;
        if !matches!(pattern.len(), 1 | 2 | 4)
            || !target.offset.is_multiple_of(pattern.len())
            || !target.length.is_multiple_of(pattern.len())
        {
            return Err(Error::invalid(
                "fill pattern or target alignment is invalid",
            ));
        }
        if target.is_empty() {
            return Ok(self);
        }
        self.recorded(unsafe {
            sys::iree_hal_command_buffer_fill_buffer(
                self.raw,
                target.reference(),
                pattern.as_ptr().cast(),
                pattern.len(),
                0,
            )
        })?;
        self.buffers.push(target.clone());
        Ok(self)
    }

    /// Record a host update. Native recording copies the source immediately.
    pub fn update(&mut self, target: &Buffer, bytes: &[u8]) -> Result<&mut Self> {
        self.validate(target)?;
        if target.len() != bytes.len() {
            return Err(Error::invalid("update size must equal the target view"));
        }
        if bytes.is_empty() {
            return Ok(self);
        }
        self.recorded(unsafe {
            sys::iree_hal_command_buffer_update_buffer(
                self.raw,
                bytes.as_ptr().cast(),
                0,
                target.reference(),
                0,
            )
        })?;
        self.buffers.push(target.clone());
        Ok(self)
    }

    /// Copy between equally sized non-overlapping views on this device.
    pub fn copy(&mut self, source: &Buffer, target: &Buffer) -> Result<&mut Self> {
        self.validate(source)?;
        self.validate(target)?;
        if source.len() != target.len() {
            return Err(Error::invalid("copy view lengths differ"));
        }
        if unsafe {
            sys::iree_hal_buffer_test_overlap(
                source.inner.raw.as_ptr(),
                source.offset,
                source.length,
                target.inner.raw.as_ptr(),
                target.offset,
                target.length,
            )
        } != sys::IREE_HAL_BUFFER_OVERLAP_DISJOINT
        {
            return Err(Error::invalid("copy source and destination overlap"));
        }
        if source.is_empty() {
            return Ok(self);
        }
        self.recorded(unsafe {
            sys::iree_hal_command_buffer_copy_buffer(
                self.raw,
                source.reference(),
                target.reference(),
                0,
            )
        })?;
        self.buffers.extend([source.clone(), target.clone()]);
        Ok(self)
    }

    /// End recording and submit native work with automatic buffer dependencies.
    ///
    /// Subsequent safe access to any referenced allocation waits on completion.
    /// Dropping the future does not cancel device work. If all buffer owners
    /// are then dropped, releasing the final allocation drains its native work.
    pub fn submit(self) -> Result<Submission> {
        if self.failed {
            return Err(Error::invalid("cannot submit a poisoned transfer recorder"));
        }
        check(unsafe { sys::iree_hal_command_buffer_end(self.raw) })?;
        let dependencies: Vec<_> = self
            .buffers
            .iter()
            .flat_map(|buffer| buffer.inner.access.0.borrow().clone())
            .collect();
        let waits = Fence::join(&dependencies)?;
        let completion = self.device.semaphore(0)?.at(1)?;
        let wait = completion.wait()?;
        check(unsafe {
            sys::iree_hal_device_queue_execute(
                self.device.as_raw(),
                u64::MAX,
                sys::iree_hal_fence_semaphore_list(waits.as_raw()),
                sys::iree_hal_fence_semaphore_list(completion.as_raw()),
                self.raw,
                sys::iree_hal_buffer_binding_table_t::default(),
                0,
            )
        })?;
        for buffer in &self.buffers {
            *buffer.inner.access.0.borrow_mut() = vec![completion.clone()];
        }
        Ok(Submission {
            wait,
            completion,
            _buffers: self.buffers.clone(),
        })
    }
}

impl Drop for TransferEncoder {
    fn drop(&mut self) {
        unsafe { sys::iree_hal_command_buffer_release(self.raw) };
    }
}

/// Native completion of a submitted transfer command buffer.
#[must_use = "await the submission to observe native completion failures"]
pub struct Submission {
    wait: FenceWait,
    completion: Fence,
    _buffers: Vec<Buffer>,
}

impl Submission {
    /// Borrow the immutable completion fence for chaining native submissions.
    pub fn completion(&self) -> &Fence {
        &self.completion
    }
}

impl Future for Submission {
    type Output = Result<()>;
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.wait).poll(context)
    }
}
