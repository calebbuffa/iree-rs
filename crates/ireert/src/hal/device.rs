//! HAL devices, device groups, synchronous transfers and capture controls.
use crate::{Code, Error, Result, check};
use iree_native as n;
use std::ops::Deref;

/// A retained HAL device.
///
/// Queue submission and topology access require a [`GroupedDevice`], obtained
/// from [`DeviceGroup::device`], so the topology-owning group cannot be dropped
/// while those operations or views are in use.
///
/// ```compile_fail
/// use ireert::hal::{Device, QueueAffinity};
///
/// fn submit_without_group(device: &Device) {
///     let _ = device.queue_flush(QueueAffinity::ANY);
/// }
/// ```
pub struct Device(*mut n::iree_hal_device_t);
impl Device {
    /// Adopts one owned device reference.
    ///
    /// # Safety
    /// `raw` must be a live, owned device reference. If that reference is later
    /// passed to [`DeviceGroup::from_device`], it must be an ungrouped device;
    /// assigning one native device to multiple groups can invalidate topology.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_device_t) -> Self {
        Self(raw)
    }
    /// Borrows the native device pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_device_t {
        self.0
    }
    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_device_t {
        core::mem::ManuallyDrop::new(self).0
    }
    /// Returns the device's canonical identifier.
    pub fn id(&self) -> String {
        // SAFETY: the returned view is borrowed from the live device.
        unsafe { n::view_string(n::iree_hal_device_id(self.0)) }
    }
    /// Returns the device's retained allocator.
    pub fn allocator(&self) -> super::allocator::Allocator {
        // SAFETY: device owns the allocator; retain it before creating an owned wrapper.
        let raw = unsafe { n::iree_hal_device_allocator(self.0) };
        // SAFETY: the device keeps the allocator live; take our own reference.
        unsafe {
            n::iree_hal_allocator_retain(raw);
            super::allocator::Allocator::from_raw(raw)
        }
    }
    /// Replaces the device's default allocator; the device retains `allocator`.
    ///
    /// # Safety
    /// Upstream does not synchronize replacement. The device must be idle, no other
    /// thread may use it concurrently, and no buffer from the previous allocator may
    /// be used afterwards.
    pub unsafe fn replace_allocator(&self, allocator: &super::allocator::Allocator) {
        // SAFETY: the caller guarantees exclusive, idle access to the device.
        unsafe { n::iree_hal_device_replace_allocator(self.0, allocator.as_raw()) }
    }
    /// Queries the capabilities used for topology construction.
    pub fn capabilities(&self) -> Result<super::topology::DeviceCapabilities> {
        let mut raw = n::iree_hal_device_capabilities_t::default();
        // SAFETY: the device is live and the output is writable.
        check(unsafe { n::iree_hal_device_query_capabilities(self.0, &mut raw) })?;
        Ok(super::topology::DeviceCapabilities::from_raw(&raw))
    }
    /// Trims unused device resources.
    pub fn trim(&self) -> Result<()> {
        // SAFETY: the device is live.
        check(unsafe { n::iree_hal_device_trim(self.0) })
    }
    /// Queries an integer capability by category and key.
    ///
    /// Returns `Ok(None)` when the category/key pair is unsupported.
    pub fn query(&self, category: &str, key: &str) -> Result<Option<i64>> {
        let mut value = 0i64;
        // SAFETY: both strings are borrowed for the query and the output pointer is valid.
        let status = unsafe {
            n::iree_hal_device_query_i64(
                self.0,
                crate::view(category),
                crate::view(key),
                &mut value,
            )
        };
        match check(status) {
            Ok(()) => Ok(Some(value)),
            Err(error)
                if matches!(
                    error.code(),
                    crate::Code::NotFound | crate::Code::Unimplemented
                ) =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
    /// Flushes pending profiling data.
    pub fn profiling_flush(&self) -> Result<()> {
        // SAFETY: the device is live.
        check(unsafe { n::iree_hal_device_profiling_flush(self.0) })
    }
    /// Ends the current profiling session.
    pub fn profiling_end(&self) -> Result<()> {
        // SAFETY: the device is live.
        check(unsafe { n::iree_hal_device_profiling_end(self.0) })
    }
    /// Starts an external profiler capture range.
    pub fn external_capture_begin(&self, options: &ExternalCaptureOptions<'_>) -> Result<()> {
        let raw = options.raw();
        // SAFETY: the device is live and the borrowed option strings outlive the call.
        check(unsafe { n::iree_hal_device_external_capture_begin(self.0, &raw) })
    }
    /// Ends an external profiler capture range.
    pub fn external_capture_end(&self) -> Result<()> {
        // SAFETY: the device is live.
        check(unsafe { n::iree_hal_device_external_capture_end(self.0) })
    }
    /// Synchronously copies host bytes into a device buffer.
    ///
    /// Slice `source` to transfer a sub-range.
    pub fn transfer_h2d(
        &self,
        source: &[u8],
        target: &super::buffer::Buffer,
        target_offset: super::DeviceSize,
        timeout: Option<core::time::Duration>,
    ) -> Result<()> {
        // SAFETY: source remains live for the duration of this synchronous transfer.
        check(unsafe {
            n::iree_hal_device_transfer_h2d(
                self.0,
                source.as_ptr().cast(),
                target.as_raw(),
                target_offset,
                source.len() as super::DeviceSize,
                n::IREE_HAL_TRANSFER_BUFFER_FLAG_DEFAULT as n::iree_hal_transfer_buffer_flags_t,
                n::timeout(timeout)?,
            )
        })
    }
    /// Synchronously copies initialized device bytes into `target`.
    ///
    /// # Safety
    /// Every byte in the source range must have been initialized by the host or
    /// device before this transfer.
    pub unsafe fn transfer_d2h(
        &self,
        source: &super::buffer::Buffer,
        source_offset: super::DeviceSize,
        target: &mut [u8],
        timeout: Option<core::time::Duration>,
    ) -> Result<()> {
        let length = super::DeviceSize::try_from(target.len())
            .map_err(|_| Error::new(Code::OutOfRange, "transfer length exceeds native size"))?;
        // SAFETY: target remains mutable and live for the duration of this synchronous transfer.
        check(unsafe {
            n::iree_hal_device_transfer_d2h(
                self.0,
                source.as_raw(),
                source_offset,
                target.as_mut_ptr().cast(),
                length,
                n::IREE_HAL_TRANSFER_BUFFER_FLAG_DEFAULT as n::iree_hal_transfer_buffer_flags_t,
                n::timeout(timeout)?,
            )
        })
    }
    /// Synchronously copies initialized device bytes into a typed host slice.
    ///
    /// # Safety
    /// Every byte in the source range must have been initialized by the host or
    /// device before this transfer.
    pub unsafe fn transfer_d2h_typed<T: bytemuck::Pod>(
        &self,
        source: &super::buffer::Buffer,
        source_offset: super::DeviceSize,
        target: &mut [T],
        timeout: Option<core::time::Duration>,
    ) -> Result<()> {
        let target = bytemuck::try_cast_slice_mut(target).map_err(super::pod_cast_error)?;
        // SAFETY: the caller guarantees every source byte is initialized.
        unsafe { self.transfer_d2h(source, source_offset, target, timeout) }
    }
    /// Synchronously copies between device buffers; the ranges must not overlap.
    pub fn transfer_d2d(
        &self,
        source: &super::buffer::Buffer,
        source_offset: super::DeviceSize,
        target: &super::buffer::Buffer,
        target_offset: super::DeviceSize,
        length: super::DeviceSize,
        timeout: Option<core::time::Duration>,
    ) -> Result<()> {
        // SAFETY: both buffers remain live for the duration of this synchronous transfer.
        check(unsafe {
            n::iree_hal_device_transfer_d2d(
                self.0,
                source.as_raw(),
                source_offset,
                target.as_raw(),
                target_offset,
                length,
                n::IREE_HAL_TRANSFER_BUFFER_FLAG_DEFAULT as n::iree_hal_transfer_buffer_flags_t,
                n::timeout(timeout)?,
            )
        })
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one reference.
        unsafe { n::iree_hal_device_release(self.0) }
    }
}
// SAFETY: HAL device operations and reference counting are designed for concurrent use; mutating allocator/provider hooks remain explicitly unsafe to share through this wrapper.
unsafe impl Send for Device {}
// SAFETY: same device concurrency guarantee as `Send`; the wrapper exposes only synchronized operations.
unsafe impl Sync for Device {}

/// A device handle that keeps its topology-owning group alive.
///
/// Queue submission and topology views are only available through this type.
pub struct GroupedDevice {
    device: Device,
    group: DeviceGroup,
}

impl GroupedDevice {
    /// Returns the group that owns this device's topology metadata.
    pub fn device_group(&self) -> &DeviceGroup {
        &self.group
    }

    /// Returns this device's topology placement.
    pub fn topology_info(&self) -> Option<super::topology::DeviceTopologyInfo<'_>> {
        // SAFETY: this handle retains the group that owns the topology storage.
        unsafe { n::iree_hal_device_topology_info(self.device.as_raw()).as_ref() }
            .map(super::topology::DeviceTopologyInfo::new)
    }
}

impl Deref for GroupedDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl Clone for GroupedDevice {
    fn clone(&self) -> Self {
        let raw = self.device.as_raw();
        // SAFETY: this handle owns a live device reference.
        unsafe { n::iree_hal_device_retain(raw) };
        Self {
            // SAFETY: the retained reference is transferred into this wrapper.
            device: unsafe { Device::from_raw(raw) },
            group: self.group.clone(),
        }
    }
}

/// An owned `iree_hal_device_group_t` reference.
pub struct DeviceGroup(*mut n::iree_hal_device_group_t);
// SAFETY: finalized groups are immutable, their reference count is atomic, and
// their retained devices and frontier tracker support concurrent use.
unsafe impl Send for DeviceGroup {}
// SAFETY: immutable topology data and atomic reference counting are thread-safe.
unsafe impl Sync for DeviceGroup {}

impl DeviceGroup {
    /// Consumes and groups `devices`, in topology order, under one causal-progress tracker.
    ///
    /// The group retains the tracker and every device. Consuming ungrouped
    /// handles prevents a device from being assigned to multiple groups.
    pub fn new(
        frontier_tracker: &crate::asynchronous::frontier_tracker::FrontierTracker,
        devices: Vec<Device>,
    ) -> Result<Self> {
        let mut builder = n::iree_hal_device_group_builder_t::default();
        // SAFETY: the builder is writable and the tracker is live; the builder retains it.
        unsafe {
            n::iree_hal_device_group_builder_initialize(&mut builder, frontier_tracker.as_raw())
        };
        let mut raw = core::ptr::null_mut();
        let result = devices
            .iter()
            // SAFETY: the builder is initialized and each device outlives finalization.
            .try_for_each(|device| {
                // SAFETY: The builder and each borrowed device are live throughout this synchronous add call.
                check(unsafe {
                    n::iree_hal_device_group_builder_add_device(&mut builder, device.0)
                })
            })
            // SAFETY: the builder is initialized; finalization retains the devices.
            .and_then(|()| {
                // SAFETY: The initialized builder retains the listed devices before the local borrow ends.
                check(unsafe {
                    n::iree_hal_device_group_builder_finalize(
                        &mut builder,
                        crate::system(),
                        &mut raw,
                    )
                })
            });
        // SAFETY: deinitialization accepts both live and finalized builders.
        unsafe { n::iree_hal_device_group_builder_deinitialize(&mut builder) };
        result.map(|()| Self(raw))
    }
    /// Consumes an ungrouped device and assigns it to a single-device group.
    ///
    /// Retrieve the group-bound handle with [`DeviceGroup::device`].
    pub fn from_device(
        device: Device,
        frontier_tracker: &crate::asynchronous::frontier_tracker::FrontierTracker,
    ) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the device and tracker are live; the group retains both.
        check(unsafe {
            n::iree_hal_device_group_create_from_device(
                device.as_raw(),
                frontier_tracker.as_raw(),
                crate::system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }
    /// Creates a group with the same topology and tracker but replaced devices.
    ///
    /// `replace` runs once per device in topology order, receiving its index and
    /// the source device, and returns the device to use in its place.
    pub fn with_replacements<F>(&self, mut replace: F) -> Result<Self>
    where
        F: FnMut(usize, &Device) -> Result<Device>,
    {
        unsafe extern "C" fn trampoline<F>(
            _group: *mut n::iree_hal_device_group_t,
            index: usize,
            source: *mut n::iree_hal_device_t,
            user_data: *mut core::ffi::c_void,
            out: *mut *mut n::iree_hal_device_t,
        ) -> n::iree_status_t
        where
            F: FnMut(usize, &Device) -> Result<Device>,
        {
            n::callback_status(|| {
                // SAFETY: user_data is the closure borrowed for the synchronous call.
                let replace = unsafe { &mut *user_data.cast::<F>() };
                // SAFETY: the source device is borrowed from the live group for this call.
                let source = core::mem::ManuallyDrop::new(unsafe { Device::from_raw(source) });
                let replacement = replace(index, &source)?;
                // SAFETY: `out` is writable; upstream consumes the returned reference.
                unsafe { *out = replacement.into_raw() };
                Ok(())
            })
        }
        let callback = n::iree_hal_device_group_replacement_callback_t {
            fn_: Some(trampoline::<F>),
            user_data: (&mut replace as *mut F).cast(),
        };
        let mut raw = core::ptr::null_mut();
        // SAFETY: the closure outlives the synchronous call and the group is live.
        check(unsafe {
            n::iree_hal_device_group_create_with_replacements(
                self.0,
                callback,
                crate::system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }
    /// Adopts one owned device-group reference.
    ///
    /// # Safety
    /// `raw` must be a live owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_device_group_t) -> Self {
        Self(raw)
    }
    /// Borrows the native device-group handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_device_group_t {
        self.0
    }
    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_device_group_t {
        core::mem::ManuallyDrop::new(self).0
    }
    /// Number of devices in the group.
    pub fn device_count(&self) -> usize {
        // SAFETY: the group is live.
        unsafe { n::iree_hal_device_group_device_count(self.0) }
    }
    /// Returns a group-bound handle to the device at `index`, or `None` when out of range.
    ///
    /// The returned handle retains this group, so topology-backed device state
    /// remains valid even after the original `DeviceGroup` is dropped.
    pub fn device(&self, index: usize) -> Option<GroupedDevice> {
        if index >= self.device_count() {
            return None;
        }
        // SAFETY: the index is in range and the group keeps the device alive.
        let raw = unsafe { n::iree_hal_device_group_device_at(self.0, index) };
        // SAFETY: a borrowed live device is retained into a new owned reference.
        (!raw.is_null()).then(|| unsafe {
            n::iree_hal_device_retain(raw);
            GroupedDevice {
                device: Device::from_raw(raw),
                group: self.clone(),
            }
        })
    }
    /// The group's immutable device topology.
    pub fn topology(&self) -> Option<super::topology::Topology<'_>> {
        // SAFETY: the group is live; the topology is valid for the group's lifetime.
        unsafe { n::iree_hal_device_group_topology(self.0).as_ref() }
            .map(super::topology::Topology::new)
    }
}

/// An owned device reference that cannot be used to assign group topology.
///
/// Returned when an API exposes a device already owned by another native
/// object. Use [`DeviceGroup::device`] to obtain queue and topology access.
pub struct RetainedDevice(Device);

impl RetainedDevice {
    /// Retains a borrowed native device pointer.
    ///
    /// # Safety
    /// `raw` must be a live device pointer for the duration of this call.
    pub(crate) unsafe fn from_borrowed(raw: *mut n::iree_hal_device_t) -> Self {
        // SAFETY: guaranteed live by the caller; this wrapper owns the retained reference.
        unsafe { n::iree_hal_device_retain(raw) };
        // SAFETY: the retained reference is transferred into Device.
        Self(unsafe { Device::from_raw(raw) })
    }
}

impl Deref for RetainedDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for RetainedDevice {
    fn clone(&self) -> Self {
        let raw = self.0.as_raw();
        // SAFETY: this wrapper owns a live reference.
        unsafe { n::iree_hal_device_retain(raw) };
        // SAFETY: the retained reference is transferred into Device.
        Self(unsafe { Device::from_raw(raw) })
    }
}

impl Clone for DeviceGroup {
    fn clone(&self) -> Self {
        // SAFETY: this wrapper owns a live reference.
        unsafe { n::iree_hal_device_group_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for DeviceGroup {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one reference.
        unsafe { n::iree_hal_device_group_release(self.0) }
    }
}

/// External profiler capture settings borrowed for `external_capture_begin`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExternalCaptureOptions<'a> {
    /// Capture provider such as `renderdoc` or a driver identifier.
    pub provider: &'a str,
    /// Optional output file path or template.
    pub file_path: Option<&'a str>,
    /// Optional named capture label.
    pub label: Option<&'a str>,
}
impl ExternalCaptureOptions<'_> {
    fn raw(&self) -> n::iree_hal_device_external_capture_options_t {
        n::iree_hal_device_external_capture_options_t {
            provider: crate::view(self.provider),
            file_path: crate::view(self.file_path.unwrap_or_default()),
            label: crate::view(self.label.unwrap_or_default()),
            flags: n::IREE_HAL_DEVICE_EXTERNAL_CAPTURE_FLAG_NONE
                as n::iree_hal_device_external_capture_flags_t,
        }
    }
}
