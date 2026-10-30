//! HAL driver ownership, device enumeration, and device creation.
use super::device::Device;
use crate::asynchronous::PoolRef;
use crate::{Result, check, system, view};
use core::{ffi::c_void, marker::PhantomData, mem::ManuallyDrop, ptr};
use iree_native as n;
use n::{slice, view_string};

/// One driver-reported physical device.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceInfo {
    /// Opaque identifier accepted by `Driver::create_device_by_id`.
    pub id: super::DeviceId,
    /// Driver-defined path for this device.
    pub path: String,
    /// Driver-provided display name.
    pub name: String,
}

/// A retained HAL driver.
pub struct Driver(*mut n::iree_hal_driver_t);
impl Driver {
    /// Adopts one owned driver reference.
    ///
    /// # Safety
    /// `raw` must be a live driver pointer whose ownership is transferred here.
    /// If it was created by a native path that uses isolated VMVX loaders, the
    /// caller must ensure it cannot coexist with or reset the process VM type
    /// registry used by [`crate::vm::Instance`].
    pub unsafe fn from_raw(raw: *mut n::iree_hal_driver_t) -> Self {
        Self(raw)
    }
    /// Borrows the native driver pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_driver_t {
        self.0
    }
    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_driver_t {
        ManuallyDrop::new(self).0
    }
    /// Enumerates physical devices with their opaque identifiers and names.
    pub fn devices(&self) -> Result<Vec<DeviceInfo>> {
        let mut count = 0;
        let mut infos = ptr::null_mut();
        // SAFETY: output pointers are writable; the array is freed with the same allocator below.
        check(unsafe {
            n::iree_hal_driver_query_available_devices(self.0, system(), &mut count, &mut infos)
        })?;
        // SAFETY: the query returned `count` records whose strings live in the same allocation.
        let devices = unsafe { slice(infos, count) }
            .iter()
            .map(|info| DeviceInfo {
                id: super::DeviceId::from_raw(info.device_id),
                // SAFETY: as above.
                path: unsafe { view_string(info.path) },
                // SAFETY: as above.
                name: unsafe { view_string(info.name) },
            })
            .collect();
        // SAFETY: the array was allocated by the query with `system()`.
        unsafe { n::iree_allocator_free(system(), infos.cast()) };
        Ok(devices)
    }
    /// Formats driver-specific details about the device `id` as text.
    pub fn dump_device_info(&self, id: super::DeviceId) -> Result<String> {
        crate::base::string_builder::format(|builder| {
            // SAFETY: the driver is live and the builder is initialized.
            unsafe { n::iree_hal_driver_dump_device_info(self.0, id.value(), builder) }
        })
    }
    /// Creates a device by its driver-defined ordinal, with driver-specific
    /// key/value `parameters`.
    pub fn create_device_by_ordinal(
        &self,
        ordinal: usize,
        parameters: &[(&str, &str)],
        params: &DeviceCreateParams<'_>,
    ) -> Result<Device> {
        let parameters = string_pairs(parameters);
        // SAFETY: handles and strings are borrowed for the synchronous call.
        create(params, |params, out| unsafe {
            n::iree_hal_driver_create_device_by_ordinal(
                self.0,
                ordinal,
                parameters.len(),
                parameters.as_ptr(),
                params,
                system(),
                out,
            )
        })
    }
    /// Creates a device by a previously enumerated opaque identifier, with
    /// driver-specific key/value `parameters`.
    pub fn create_device_by_id(
        &self,
        device_id: super::DeviceId,
        parameters: &[(&str, &str)],
        params: &DeviceCreateParams<'_>,
    ) -> Result<Device> {
        let parameters = string_pairs(parameters);
        // SAFETY: handles and strings are borrowed for the synchronous call.
        create(params, |params, out| unsafe {
            n::iree_hal_driver_create_device_by_id(
                self.0,
                device_id.value(),
                parameters.len(),
                parameters.as_ptr(),
                params,
                system(),
                out,
            )
        })
    }
    /// Creates a device from a driver-specific path, with driver-specific
    /// key/value `parameters`.
    pub fn create_device_by_path(
        &self,
        driver_name: &str,
        path: &str,
        parameters: &[(&str, &str)],
        params: &DeviceCreateParams<'_>,
    ) -> Result<Device> {
        let parameters = string_pairs(parameters);
        // SAFETY: handles and strings are borrowed for the synchronous call.
        create(params, |params, out| unsafe {
            n::iree_hal_driver_create_device_by_path(
                self.0,
                view(driver_name),
                view(path),
                parameters.len(),
                parameters.as_ptr(),
                params,
                system(),
                out,
            )
        })
    }
    /// Creates a device using a `driver://path?parameters` URI.
    pub fn create_device_by_uri(
        &self,
        uri: &str,
        params: &DeviceCreateParams<'_>,
    ) -> Result<Device> {
        // SAFETY: handles and strings are borrowed for the synchronous call.
        create(params, |params, out| unsafe {
            n::iree_hal_driver_create_device_by_uri(self.0, view(uri), params, system(), out)
        })
    }
    /// Creates the driver's heuristic default device.
    pub fn create_default_device(&self, params: &DeviceCreateParams<'_>) -> Result<Device> {
        // SAFETY: handles and strings are borrowed for the synchronous call.
        create(params, |params, out| unsafe {
            n::iree_hal_driver_create_default_device(self.0, params, system(), out)
        })
    }
}

/// Parameters shared by every device-creation path
/// (`iree_hal_device_create_params_t`).
///
/// IREE requires a proactor pool, which the device retains; pass
/// [`ProactorPool::shared`](crate::asynchronous::ProactorPool::shared) unless the host drives I/O itself.
#[derive(Clone, Copy)]
pub struct DeviceCreateParams<'a> {
    raw: n::iree_hal_device_create_params_t,
    _borrows: PhantomData<&'a ()>,
}

impl<'a> DeviceCreateParams<'a> {
    /// Parameters that create devices on `pool`.
    pub fn new(pool: impl Into<PoolRef<'a>>) -> Self {
        // SAFETY: returns plain default parameters.
        let mut raw = unsafe { n::iree_hal_device_create_params_default() };
        raw.proactor_pool = pool.into().as_raw();
        Self {
            raw,
            _borrows: PhantomData,
        }
    }
    /// Chains extension parameters for drivers that accept them.
    ///
    /// # Safety
    /// `next` must be null or a valid IREE parameter chain that outlives `'a`.
    pub unsafe fn with_next(mut self, next: *const c_void) -> Self {
        self.raw.next = next;
        self
    }
    /// Borrows the native parameters.
    pub fn as_raw(&self) -> &n::iree_hal_device_create_params_t {
        &self.raw
    }
}

/// Borrows key/value pairs as native string pairs.
fn string_pairs(pairs: &[(&str, &str)]) -> Vec<n::iree_string_pair_t> {
    pairs
        .iter()
        .map(|&(key, value)| n::iree_string_pair_t {
            __bindgen_anon_1: n::iree_string_pair_t__bindgen_ty_1 { key: view(key) },
            __bindgen_anon_2: n::iree_string_pair_t__bindgen_ty_2 { value: view(value) },
        })
        .collect()
}

/// Runs a device-creation call with `params` and adopts its result.
///
/// Every caller's closure borrows only live handles and strings for the
/// duration of the synchronous call.
pub(super) fn create(
    params: &DeviceCreateParams<'_>,
    call: impl FnOnce(
        *const n::iree_hal_device_create_params_t,
        *mut *mut n::iree_hal_device_t,
    ) -> n::iree_status_t,
) -> Result<Device> {
    let mut out = ptr::null_mut();
    check(call(params.as_raw(), &mut out))?;
    // SAFETY: a successful creation call returns an owned device reference.
    Ok(unsafe { Device::from_raw(out) })
}
impl Clone for Driver {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_driver_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for Driver {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_driver_release(self.0) }
    }
}
