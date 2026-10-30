//! Buffers shared zero-copy between WGPU and IREE.

use crate::context::Context;
use crate::context::Inner;
use crate::context::QueueTopology;
use crate::device::NativeDevice;
use crate::element::Element;
use crate::error::invalid;
use ash::vk::{self, Handle};
use ireert::Result;
use ireert::hal;
use ireert::hal::BufferParams;
use ireert::hal::BufferUsage;
use ireert::hal::BufferView;
use ireert::hal::ElementType;
use ireert::hal::EncodingType;
use ireert::hal::MemoryAccess;
use ireert::hal::MemoryType;
use ireert::hal::QueueAffinity;
use ireert::hal::backends::vulkan;
use ireert::hal::{BufferBinding, QueueSubmission};
use std::sync::Arc;

/// One allocation visible to both WGPU and IREE, without copies.
///
/// Use [`Self::wgpu`] in bind groups and copies submitted with
/// [`Context::submit`], and [`Self::view`] as an IREE argument or
/// destination-passing output. The context orders all accesses; read the
/// contents with [`Context::read`]. The WGPU buffer has `STORAGE`, `COPY_SRC`
/// and `COPY_DST` usages and cannot be mapped.
pub struct SharedBuffer {
    pub(crate) buffer: SendBuffer,
    pub(crate) wgpu: wgpu::Buffer,
    pub(crate) len: usize,
    pub(crate) owner: Arc<Inner>,
}

impl std::fmt::Debug for SharedBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedBuffer")
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

impl SharedBuffer {
    /// The WGPU buffer, for bind groups and copies.
    ///
    /// Submit work using it with [`Context::submit`].
    pub fn wgpu(&self) -> &wgpu::Buffer {
        &self.wgpu
    }

    /// A whole-buffer binding for bind groups.
    pub fn binding(&self) -> wgpu::BindingResource<'_> {
        self.wgpu.as_entire_binding()
    }

    /// A dense row-major IREE tensor view of the allocation, for program
    /// arguments and destination-passing outputs.
    ///
    /// # Errors
    ///
    /// Returns an error when `shape` and `element_type` cover more bytes than
    /// [`Self::len`].
    pub fn view(&self, shape: &[hal::Dim], element_type: ElementType) -> Result<BufferView> {
        BufferView::create(
            &self.buffer.0,
            shape,
            element_type,
            EncodingType::DENSE_ROW_MAJOR,
        )
    }

    /// The length in bytes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Always `false`: shared buffers are never empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    pub(crate) fn check_context(&self, context: &Context) -> Result<()> {
        if !Arc::ptr_eq(&self.owner, &context.inner) {
            return Err(invalid("shared buffer belongs to a different context"));
        }
        Ok(())
    }
}

/// A HAL buffer reference that may be used from any thread.
pub(crate) struct SendBuffer(hal::Buffer);
// SAFETY: HAL buffer retain/release use atomic reference counts. Shared
// buffers cannot be host-mapped, and all device access is submitted through
// the context, which serializes it.
unsafe impl Send for SendBuffer {}
// SAFETY: see `Send`.
unsafe impl Sync for SendBuffer {}

/// Keeps an exported allocation and its device alive while WGPU uses it.
pub(crate) struct NativeLease {
    pub(crate) _buffer: SendBuffer,
    pub(crate) _native: Arc<NativeDevice>,
}

pub(crate) fn shared_buffer_params() -> BufferParams {
    // `BufferUsage::DEFAULT` is transfer and dispatch storage: no host mapping,
    // so shared memory is only reachable through ordered device work.
    BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    }
}

pub(crate) fn validate_export_permissions(access: MemoryAccess, usage: BufferUsage) -> Result<()> {
    let required_access = MemoryAccess::READ | MemoryAccess::WRITE;
    let required_usage = BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE;
    if !access.contains(required_access) {
        return Err(invalid(
            "WGPU storage/copy export requires readable and writable IREE memory",
        ));
    }
    if !usage.contains(required_usage) || usage.contains(BufferUsage::SHARING_IMMUTABLE) {
        return Err(invalid(
            "WGPU storage/copy export requires mutable IREE transfer and storage usages",
        ));
    }
    Ok(())
}

pub(crate) fn validate_length(
    length: usize,
    max_buffer_size: u64,
    max_binding_size: u64,
) -> Result<()> {
    if length == 0
        || !length.is_multiple_of(4)
        || length as u64 > max_buffer_size
        || length as u64 > max_binding_size
    {
        return Err(invalid(
            "shared buffer length must be nonzero, four-byte aligned, and within WGPU buffer limits",
        ));
    }
    Ok(())
}

pub(crate) fn element_count(shape: &[hal::Dim]) -> Option<usize> {
    shape
        .iter()
        .try_fold(1usize, |count, dim| count.checked_mul(*dim))
}

impl Context {
    /// Allocate a zero-filled buffer of `length` bytes shared by WGPU and IREE.
    ///
    /// The zero fill is ordered like any other IREE work, so it does not block.
    ///
    /// # Errors
    ///
    /// Returns an error when `length` is zero, not a multiple of four, or
    /// beyond WGPU's buffer or storage-binding limits, and when allocation
    /// fails.
    pub fn allocate_shared(&self, length: usize) -> Result<SharedBuffer> {
        let shared = self.allocate_shared_uninit(length)?;
        self.inner.enqueue(|device, waits, signals| {
            device.queue_fill(
                &QueueSubmission::new(QueueAffinity::ANY)
                    .with_waits(waits.iter().copied())
                    .with_signals(signals.iter().copied()),
                BufferBinding::new(&shared.buffer.0, 0, length as hal::DeviceSize),
                &[0],
            )
        })?;
        Ok(shared)
    }

    /// Allocate a shared buffer holding `data`.
    ///
    /// # Errors
    ///
    /// As [`Self::allocate_shared`] for the byte length of `data`, and when the
    /// upload fails.
    pub fn shared_buffer<T: Element>(&self, data: &[T]) -> Result<SharedBuffer> {
        let bytes = bytemuck::cast_slice::<T, u8>(data);
        let shared = self.allocate_shared_uninit(bytes.len())?;
        self.transfer_h2d(bytes, &shared.buffer.0)?;
        Ok(shared)
    }

    /// Upload `data` as a new IREE tensor of `shape`, for program arguments.
    ///
    /// # Errors
    ///
    /// Returns an error when `shape` does not hold exactly `data.len()`
    /// elements, or when allocation or the upload fails.
    pub fn upload<T: Element>(&self, data: &[T], shape: &[hal::Dim]) -> Result<BufferView> {
        if element_count(shape) != Some(data.len()) {
            return Err(invalid("upload shape does not match the data length"));
        }
        let bytes = bytemuck::cast_slice::<T, u8>(data);
        let buffer = self.inner.iree.allocator().allocate_buffer(
            shared_buffer_params(),
            bytes.len().max(1) as hal::DeviceSize,
        )?;
        self.transfer_h2d(bytes, &buffer)?;
        BufferView::create(&buffer, shape, T::TYPE, EncodingType::DENSE_ROW_MAJOR)
    }

    pub(crate) fn transfer_h2d(&self, bytes: &[u8], buffer: &hal::Buffer) -> Result<()> {
        if bytes.is_empty() {
            return Ok(());
        }
        match self.inner.topology {
            // A fresh allocation: only IREE's own queue is involved.
            QueueTopology::Separate => self.inner.iree.transfer_h2d(bytes, buffer, 0, None),
            QueueTopology::Shared => self
                .inner
                .blocking(|device| device.transfer_h2d(bytes, buffer, 0, None)),
        }
    }

    /// Copy an IREE tensor into a new shared buffer, entirely on the GPU.
    ///
    /// Use this for IREE outputs WGPU should consume. To avoid the copy,
    /// pass a [`SharedBuffer::view`] as a destination-passing output. The copy
    /// is ordered like any other IREE work, so it does not block.
    ///
    /// # Errors
    ///
    /// Returns an error when `view` is not dense row-major, belongs to another
    /// device, or its length is not a valid [`Self::allocate_shared`] length.
    pub fn copy_to_shared(&self, view: &BufferView) -> Result<SharedBuffer> {
        if view.encoding_type() != EncodingType::DENSE_ROW_MAJOR {
            return Err(invalid("sharing requires a dense row-major buffer view"));
        }
        let source = view.buffer();
        self.check_buffer_device(&source)?;
        let length = view.byte_length();
        let shared = self.allocate_shared_uninit(length)?;
        self.inner.enqueue(|device, waits, signals| {
            device.queue_copy(
                &QueueSubmission::new(QueueAffinity::ANY)
                    .with_waits(waits.iter().copied())
                    .with_signals(signals.iter().copied()),
                BufferBinding::new(&source, 0, length as hal::DeviceSize),
                BufferBinding::new(&shared.buffer.0, 0, length as hal::DeviceSize),
            )
        })?;
        Ok(shared)
    }

    /// Read a shared buffer after all previously submitted work of both APIs.
    ///
    /// Blocks until that work completes.
    ///
    /// # Safety
    /// Every byte in the shared buffer must have been initialized by the host
    /// or device before this read.
    ///
    /// # Errors
    ///
    /// Returns an error when `shared` belongs to another context, its length
    /// is not a whole number of `T`s, or outstanding work fails.
    pub unsafe fn read<T: Element>(&self, shared: &SharedBuffer) -> Result<Vec<T>> {
        shared.check_context(self)?;
        // SAFETY: initialization is guaranteed by the caller.
        unsafe { self.read_buffer(&shared.buffer.0, shared.len) }
    }

    /// Read an IREE tensor after all previously submitted work of both APIs.
    ///
    /// Blocks until that work completes.
    ///
    /// # Safety
    /// Every byte in the view's buffer range must have been initialized by the
    /// host or device before this read.
    ///
    /// # Errors
    ///
    /// Returns an error when the view's element type is not `T`, it is not
    /// dense row-major, it belongs to another device, or outstanding work fails.
    pub unsafe fn read_view<T: Element>(&self, view: &BufferView) -> Result<Vec<T>> {
        if view.element_type() != T::TYPE {
            return Err(invalid(
                "buffer view element type does not match the requested type",
            ));
        }
        if view.encoding_type() != EncodingType::DENSE_ROW_MAJOR {
            return Err(invalid("reading requires a dense row-major buffer view"));
        }
        let buffer = view.buffer();
        self.check_buffer_device(&buffer)?;
        let length = view.byte_length();
        // SAFETY: initialization is guaranteed by the caller.
        unsafe { self.read_buffer(&buffer, length) }
    }

    pub(crate) fn check_buffer_device(&self, buffer: &hal::Buffer) -> Result<()> {
        if buffer
            .allocation_placement()
            .device
            .is_some_and(|device| device.as_raw() != self.inner.iree.as_raw())
        {
            return Err(invalid("buffer belongs to a different IREE device"));
        }
        Ok(())
    }

    pub(crate) unsafe fn read_buffer<T: Element>(
        &self,
        buffer: &hal::Buffer,
        length: usize,
    ) -> Result<Vec<T>> {
        let size = std::mem::size_of::<T>();
        if !length.is_multiple_of(size) {
            return Err(invalid("buffer is not a whole number of elements"));
        }
        let mut values = vec![T::zeroed(); length / size];
        if length == 0 {
            return Ok(values);
        }
        self.inner.blocking(|device| {
            // SAFETY: the caller guarantees that the full source range is initialized.
            unsafe { device.transfer_d2h(buffer, 0, bytemuck::cast_slice_mut(&mut values), None) }
        })?;
        Ok(values)
    }

    pub(crate) fn allocate_shared_uninit(&self, length: usize) -> Result<SharedBuffer> {
        self.validate_shared_length(length)?;
        let buffer = self
            .inner
            .iree
            .allocator()
            .allocate_buffer(shared_buffer_params(), length as hal::DeviceSize)?;
        let wgpu = self.export_buffer(&buffer)?;
        Ok(SharedBuffer {
            buffer: SendBuffer(buffer),
            wgpu,
            len: length,
            owner: self.inner.clone(),
        })
    }

    pub(crate) fn validate_shared_length(&self, length: usize) -> Result<()> {
        let limits = self.inner.device.limits();
        validate_length(
            length,
            limits.max_buffer_size,
            limits.max_storage_buffer_binding_size,
        )
    }

    /// Adopt a dedicated, zero-offset IREE allocation as a WGPU buffer.
    ///
    /// Restricted-usage, read-only, immutable, sub-allocated and sparse
    /// buffers are rejected rather than granting WGPU broader permissions.
    /// The WGPU buffer retains the native allocation until WGPU releases it.
    pub(crate) fn export_buffer(&self, buffer: &hal::Buffer) -> Result<wgpu::Buffer> {
        let length = buffer.byte_length();
        self.validate_shared_length(length)?;
        validate_export_permissions(buffer.allowed_access(), buffer.allowed_usage())?;
        if buffer.byte_offset() != 0 {
            return Err(invalid(
                "pooled/nonzero-offset buffer: use copy_to_shared or allocate_shared",
            ));
        }
        let backing = buffer.allocated_buffer().unwrap_or_else(|| buffer.clone());
        validate_export_permissions(backing.allowed_access(), backing.allowed_usage())?;
        if backing.byte_length() < length {
            return Err(invalid("buffer exceeds its backing allocation"));
        }
        let handles = vulkan::buffer_handles(&backing)?;
        if handles.buffer as u64 == 0 {
            return Err(invalid("buffer has no Vulkan buffer handle"));
        }
        if handles.memory as u64 == 0 {
            return Err(invalid("sparse buffers cannot be exported"));
        }
        // The callback can run on a WGPU cleanup thread; the lease only
        // releases an atomically reference-counted native buffer.
        let lease = NativeLease {
            _buffer: SendBuffer(backing),
            _native: self.inner.native.clone(),
        };
        // SAFETY: the handle is the live backing `VkBuffer` of this context's
        // device, and the lease keeps it and the device alive until WGPU
        // releases the buffer.
        let hal_buffer = unsafe {
            wgpu::hal::vulkan::Buffer::from_raw_externally_owned(
                vk::Buffer::from_raw(handles.buffer as u64),
                Box::new(move || drop(lease)),
            )
        };
        let descriptor = wgpu::BufferDescriptor {
            label: Some("IREE shared allocation"),
            size: length as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        // SAFETY: `hal_buffer` was created on this device, covers `length`
        // bytes (checked against the backing allocation) and supports these
        // usages (checked by `validate_export_permissions`).
        Ok(unsafe {
            self.inner
                .device
                .create_buffer_from_hal::<wgpu::hal::api::Vulkan>(hal_buffer, &descriptor)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BufferUsage, Context, MemoryAccess, element_count, shared_buffer_params,
        validate_export_permissions, validate_length,
    };
    use crate::context::ContextOptions;

    #[test]
    fn exports_do_not_escalate_native_permissions() {
        let usage = BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE;
        assert!(validate_export_permissions(MemoryAccess::ALL, usage).is_ok());
        for denied in [MemoryAccess::READ, MemoryAccess::WRITE] {
            assert!(validate_export_permissions(denied, usage).is_err());
        }
        for denied in [
            BufferUsage::TRANSFER,
            BufferUsage::DISPATCH_STORAGE,
            BufferUsage::TRANSFER_SOURCE | BufferUsage::DISPATCH_STORAGE,
            BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE_READ,
            usage | BufferUsage::SHARING_IMMUTABLE,
        ] {
            assert!(validate_export_permissions(MemoryAccess::ALL, denied).is_err());
        }
    }

    #[test]
    fn validates_shared_length() {
        assert!(validate_length(16, 1 << 20, 1 << 20).is_ok());
        assert!(validate_length(0, 1 << 20, 1 << 20).is_err());
        assert!(validate_length(6, 1 << 20, 1 << 20).is_err());
        assert!(validate_length(16, 8, 1 << 20).is_err());
        assert!(validate_length(16, 1 << 20, 8).is_err());
    }

    #[test]
    fn counts_shape_elements() {
        assert_eq!(element_count(&[]), Some(1));
        assert_eq!(element_count(&[2, 3]), Some(6));
        assert_eq!(element_count(&[0, 3]), Some(0));
    }

    #[test]
    #[ignore = "requires a Vulkan 1.3 graphics/compute device"]
    fn restricted_native_buffers_are_rejected_before_wgpu_adoption() -> ireert::Result<()> {
        let context = pollster::block_on(Context::new(ContextOptions::default()))?;
        for usage in [
            BufferUsage::TRANSFER,
            BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE_READ,
            BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE | BufferUsage::SHARING_IMMUTABLE,
        ] {
            let params = ireert::hal::BufferParams {
                usage,
                ..shared_buffer_params()
            };
            let buffer = context.inner.iree.allocator().allocate_buffer(params, 16)?;
            let error = context.export_buffer(&buffer).err().ok_or_else(|| {
                ireert::Error::new(
                    ireert::Code::Internal,
                    "restricted native usage became writable WGPU storage",
                )
            })?;
            assert!(
                error
                    .to_string()
                    .contains("mutable IREE transfer and storage")
            );
        }
        Ok(())
    }
}
