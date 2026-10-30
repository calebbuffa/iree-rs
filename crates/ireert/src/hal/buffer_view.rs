//! HAL views pairing a buffer with shape and element metadata.
use super::flags::{BufferParams, ElementType, EncodingType};
use crate::hal::{DeviceSize, Dim};
use crate::{Code, Error, Result, check, system};
use core::{mem::ManuallyDrop, ptr};
use iree_native as n;

/// A retained shaped view over a HAL buffer.
pub struct BufferView(*mut n::iree_hal_buffer_view_t);
impl BufferView {
    /// Creates a view over a buffer.
    pub fn create(
        buffer: &super::buffer::Buffer,
        shape: &[Dim],
        element_type: ElementType,
        encoding: EncodingType,
    ) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: shape is valid for this call and the buffer is borrowed.
        check(unsafe {
            n::iree_hal_buffer_view_create(
                buffer.as_raw(),
                shape.len(),
                shape.as_ptr(),
                element_type.bits(),
                encoding.bits(),
                system(),
                &mut out,
            )
        })?;
        // SAFETY: a successful call returns a new owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }
    /// Creates a view over `buffer` with this view's shape, element type and encoding.
    pub fn create_like(&self, buffer: &super::buffer::Buffer) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: both objects are live; the new view retains the buffer.
        check(unsafe {
            n::iree_hal_buffer_view_create_like(buffer.as_raw(), self.0, system(), &mut out)
        })?;
        // SAFETY: a successful call returns a new owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }
    /// Returns the total number of elements in the view.
    pub fn element_count(&self) -> usize {
        // SAFETY: the view is live.
        unsafe { n::iree_hal_buffer_view_element_count(self.0) }
    }
    /// Returns the byte size of one element.
    pub fn element_size(&self) -> usize {
        // SAFETY: the view is live.
        unsafe { n::iree_hal_buffer_view_element_size(self.0) }
    }
    /// Allocates a buffer and initializes it from `data`, returning its view.
    pub fn allocate_buffer_copy(
        device: &super::device::Device,
        allocator: &super::allocator::Allocator,
        shape: &[Dim],
        element_type: ElementType,
        encoding: EncodingType,
        params: BufferParams,
        data: &[u8],
    ) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: input slices remain live for the call; output pointer is writable.
        check(unsafe {
            n::iree_hal_buffer_view_allocate_buffer_copy(
                device.as_raw(),
                allocator.as_raw(),
                shape.len(),
                shape.as_ptr(),
                element_type.bits(),
                encoding.bits(),
                params.raw(),
                n::const_span(data),
                &mut out,
            )
        })?;
        // SAFETY: a successful call returns a new owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }
    /// Allocates a shaped view initialized from plain-data elements.
    pub fn allocate_buffer_copy_typed<T: bytemuck::Pod>(
        device: &super::device::Device,
        allocator: &super::allocator::Allocator,
        shape: &[Dim],
        element_type: ElementType,
        encoding: EncodingType,
        params: BufferParams,
        data: &[T],
    ) -> Result<Self> {
        super::pod_element_size::<T>()?;
        let bytes = bytemuck::try_cast_slice(data).map_err(super::pod_cast_error)?;
        Self::allocate_buffer_copy(
            device,
            allocator,
            shape,
            element_type,
            encoding,
            params,
            bytes,
        )
    }

    /// Parses a textual buffer view representation using the supplied device.
    pub fn parse(
        text: &str,
        device: &super::device::Device,
        allocator: &super::allocator::Allocator,
    ) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: the text and handles are borrowed for the synchronous parse.
        check(unsafe {
            n::iree_hal_buffer_view_parse(
                crate::view(text),
                device.as_raw(),
                allocator.as_raw(),
                &mut out,
            )
        })?;
        // SAFETY: a successful call returns a new owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }
    /// Returns the buffer view held by `reference`.
    pub fn from_ref(reference: &crate::vm::Ref) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: the reference is live and the output pointer is writable.
        check(unsafe { n::iree_hal_buffer_view_check_deref(*reference.as_raw(), &mut raw) })?;
        // SAFETY: the reference keeps the view alive; retain gives this wrapper its own reference.
        unsafe { n::iree_hal_buffer_view_retain(raw) };
        // SAFETY: the retain above produced one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Returns a new VM reference to this buffer view.
    pub fn to_ref(&self) -> crate::vm::Ref {
        // SAFETY: this live view is retained into the returned VM reference.
        let raw = unsafe { n::iree_hal_buffer_view_retain_ref(self.0) };
        // SAFETY: retain_ref returns an owned VM reference.
        unsafe { crate::vm::Ref::from_raw(raw) }
    }
    /// Adopts one owned buffer-view reference.
    ///
    /// # Safety
    /// `raw` must be a live view pointer whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_buffer_view_t) -> Self {
        Self(raw)
    }
    /// Borrows the native view pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_buffer_view_t {
        self.0
    }
    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_buffer_view_t {
        ManuallyDrop::new(self).0
    }
    /// Returns the shape dimensions.
    pub fn shape(&self) -> Result<Vec<Dim>> {
        // SAFETY: the view is retained for this query.
        let rank = unsafe { n::iree_hal_buffer_view_shape_rank(self.0) };
        let mut shape = vec![0; rank];
        let mut out_rank = 0;
        // SAFETY: shape storage and out-rank are writable for the supplied capacity.
        check(unsafe {
            n::iree_hal_buffer_view_shape(self.0, shape.len(), shape.as_mut_ptr(), &mut out_rank)
        })?;
        shape.truncate(out_rank);
        Ok(shape)
    }

    /// Returns a single shape dimension, or an out-of-range error.
    pub fn shape_dim(&self, index: usize) -> Result<Dim> {
        // SAFETY: the wrapper owns a live reference.
        let rank = unsafe { n::iree_hal_buffer_view_shape_rank(self.0) };
        if index >= rank {
            return Err(Error::new(
                Code::OutOfRange,
                "buffer view shape dimension is out of range",
            ));
        }
        // SAFETY: index was checked against the live view's rank.
        Ok(unsafe { n::iree_hal_buffer_view_shape_dim(self.0, index) })
    }

    /// Replaces the shape while preserving the number of elements.
    pub fn reshape(&mut self, shape: &[Dim]) -> Result<()> {
        // SAFETY: shape is borrowed through the synchronous mutation and `&mut self` is exclusive.
        check(unsafe { n::iree_hal_buffer_view_reshape(self.0, shape.as_ptr(), shape.len()) })
    }

    /// Computes the byte offset of an element at `indices`.
    pub fn compute_offset(&self, indices: &[Dim]) -> Result<DeviceSize> {
        let mut offset = 0;
        // SAFETY: indices and output storage remain valid through the query.
        check(unsafe {
            n::iree_hal_buffer_view_compute_offset(
                self.0,
                indices.len(),
                indices.as_ptr(),
                &mut offset,
            )
        })?;
        Ok(offset)
    }

    /// Computes the byte offset and length of a shaped subrange.
    pub fn compute_range(
        &self,
        start_indices: &[Dim],
        lengths: &[Dim],
    ) -> Result<(DeviceSize, DeviceSize)> {
        let mut offset = 0;
        let mut length = 0;
        // SAFETY: index slices and output storage remain valid through the query.
        check(unsafe {
            n::iree_hal_buffer_view_compute_range(
                self.0,
                start_indices.len(),
                start_indices.as_ptr(),
                lengths.len(),
                lengths.as_ptr(),
                &mut offset,
                &mut length,
            )
        })?;
        Ok((offset, length))
    }
    /// Returns the element type.
    pub fn element_type(&self) -> ElementType {
        // SAFETY: the wrapper owns a live reference.
        ElementType::from_bits_retain(unsafe { n::iree_hal_buffer_view_element_type(self.0) })
    }
    /// Returns the encoding type.
    pub fn encoding_type(&self) -> EncodingType {
        // SAFETY: the wrapper owns a live reference.
        EncodingType::from_bits_retain(unsafe { n::iree_hal_buffer_view_encoding_type(self.0) })
    }
    /// Reads initialized view elements into an owned vector after checking
    /// element type and size metadata.
    ///
    /// # Safety
    /// Every byte in the underlying buffer range must have been initialized by
    /// the caller or by the device before this read. `T` must also represent
    /// the values encoded by `element_type`; this method checks byte width but
    /// does not convert values or validate signedness.
    pub unsafe fn read_elements_unchecked<T: bytemuck::Pod>(
        &self,
        element_type: ElementType,
    ) -> Result<Vec<T>> {
        self.validate_element_type::<T>(element_type)?;
        let byte_length = self.byte_length();
        let element_size = super::pod_element_size::<T>()?;
        if !byte_length.is_multiple_of(element_size) {
            return Err(Error::new(
                Code::InvalidArgument,
                "buffer view length is not a whole number of typed elements",
            ));
        }
        let mut elements = vec![T::zeroed(); byte_length / element_size];
        // SAFETY: the caller guarantees the buffer contents are initialized.
        unsafe { self.buffer().read_typed_unchecked(0, &mut elements)? };
        Ok(elements)
    }

    /// Writes all view elements after checking element type, size metadata,
    /// and byte length. `T` must represent the values encoded by
    /// `element_type`; this method copies their bytes without conversion.
    pub fn write_elements<T: bytemuck::Pod>(
        &self,
        element_type: ElementType,
        elements: &[T],
    ) -> Result<()> {
        self.validate_element_type::<T>(element_type)?;
        if DeviceSize::try_from(core::mem::size_of_val(elements)).ok() != Some(self.byte_length()) {
            return Err(Error::new(
                Code::InvalidArgument,
                "typed element bytes do not match the buffer view length",
            ));
        }
        self.buffer().write_typed(0, elements)
    }

    fn validate_element_type<T: bytemuck::Pod>(&self, element_type: ElementType) -> Result<()> {
        if self.element_type() != element_type {
            return Err(Error::new(
                Code::InvalidArgument,
                "requested element type does not match the buffer view metadata",
            ));
        }
        if super::pod_element_size::<T>()? != self.element_size() {
            return Err(Error::new(
                Code::InvalidArgument,
                "Rust element size does not match the buffer view element size",
            ));
        }
        Ok(())
    }

    /// Returns the storage byte length.
    pub fn byte_length(&self) -> DeviceSize {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_view_byte_length(self.0) }
    }
    /// Retains and returns the underlying buffer.
    pub fn buffer(&self) -> super::buffer::Buffer {
        // SAFETY: the view keeps its buffer live; take our own reference.
        unsafe { super::buffer::Buffer::from_borrowed(n::iree_hal_buffer_view_buffer(self.0)) }
    }
    /// Formats initialized view contents like `2x4xi16=[[1 2][3 4]]`, eliding
    /// elements past `max_elements` with `...`.
    ///
    /// # Safety
    /// Every byte in the underlying buffer range must have been initialized by
    /// the host or device before formatting.
    pub unsafe fn format_unchecked(&self, max_elements: usize) -> Result<String> {
        let mut length = 0;
        // SAFETY: a zero-capacity query writes only the required length.
        let query = check(unsafe {
            n::iree_hal_buffer_view_format(self.0, max_elements, 0, ptr::null_mut(), &mut length)
        });
        match query {
            Ok(()) => return Ok(String::new()),
            Err(error) if error.code() == Code::OutOfRange => {}
            Err(error) => return Err(error),
        }
        // One extra byte for the NUL terminator written by IREE.
        let mut text = vec![0u8; length + 1];
        // SAFETY: `text` is writable for its full capacity; initialization is guaranteed by the caller.
        check(unsafe {
            n::iree_hal_buffer_view_format(
                self.0,
                max_elements,
                text.len(),
                text.as_mut_ptr().cast(),
                &mut length,
            )
        })?;
        text.truncate(length);
        String::from_utf8(text)
            .map_err(|_| Error::new(Code::DataLoss, "buffer view text is not UTF-8"))
    }
}
impl Clone for BufferView {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_view_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for BufferView {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_buffer_view_release(self.0) }
    }
}

// SAFETY: buffer-view metadata is immutable and the retained HAL object can be moved between threads.
unsafe impl Send for BufferView {}
