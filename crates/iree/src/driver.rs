use crate::{
    Device, Error, Instance, Result,
    runtime::{check, string_view},
    sys,
};
use std::{
    ptr::{self, NonNull},
    rc::Rc,
};

/// A driver registered in this native runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverInfo {
    /// Canonical driver name, such as `local-task` or `vulkan`.
    pub name: String,
    /// Human-readable driver description.
    pub description: String,
}

pub(crate) unsafe fn owned_string(view: sys::iree_string_view_t) -> String {
    if view.size == 0 {
        return String::new();
    }
    String::from_utf8_lossy(unsafe {
        std::slice::from_raw_parts(view.data.cast::<u8>(), view.size)
    })
    .into_owned()
}

pub(crate) struct HostAllocation<T> {
    pub(crate) pointer: *mut T,
    pub(crate) count: usize,
}

impl<T> HostAllocation<T> {
    pub(crate) fn new() -> Self {
        Self {
            pointer: ptr::null_mut(),
            count: 0,
        }
    }
    pub(crate) fn slice(&self) -> &[T] {
        if self.count == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.pointer, self.count) }
        }
    }
}

impl<T> Drop for HostAllocation<T> {
    fn drop(&mut self) {
        unsafe { sys::iree_allocator_free(sys::iree_allocator_system(), self.pointer.cast()) };
    }
}

struct DriverInner {
    raw: NonNull<sys::iree_hal_driver_t>,
    instance: Instance,
    name: String,
}

impl Drop for DriverInner {
    fn drop(&mut self) {
        unsafe { sys::iree_hal_driver_release(self.raw.as_ptr()) };
    }
}

/// A reusable native driver, retained by its discovered devices and builders.
///
/// Reuse a driver when creating multiple devices: native driver initialization
/// may load process-wide libraries or reserve hardware resources.
#[derive(Clone)]
pub struct Driver(Rc<DriverInner>);

/// An enumerated physical device associated with its originating driver.
///
/// The opaque native ID cannot accidentally be reused with a different driver.
#[derive(Clone)]
pub struct DeviceInfo {
    driver: Driver,
    id: sys::iree_hal_device_id_t,
    name: String,
    path: String,
}

impl DeviceInfo {
    /// Human-readable device name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Stable driver-specific device path.
    pub fn path(&self) -> &str {
        &self.path
    }
    /// Configure creation of this exact device using its originating driver.
    pub fn builder(&self) -> DeviceBuilder<'_> {
        DeviceBuilder {
            driver: &self.driver,
            selection: Selection::Id(self.id),
            parameters: Vec::new(),
        }
    }
}

impl Instance {
    fn registry(&self) -> Result<*mut sys::iree_hal_driver_registry_t> {
        let registry = unsafe { sys::iree_runtime_instance_driver_registry(self.as_raw()) };
        if registry.is_null() {
            return Err(Error::invalid("instance has no driver registry"));
        }
        Ok(registry)
    }

    /// Enumerate registered drivers without opening their hardware devices.
    ///
    /// A registered driver may still fail to initialize if its platform SDK,
    /// hardware, or runtime library is unavailable.
    pub fn drivers(&self) -> Result<Vec<DriverInfo>> {
        let mut allocation = HostAllocation::new();
        check(unsafe {
            sys::iree_hal_driver_registry_enumerate(
                self.registry()?,
                sys::iree_allocator_system(),
                &mut allocation.count,
                &mut allocation.pointer,
            )
        })?;
        Ok(allocation
            .slice()
            .iter()
            .map(|info| DriverInfo {
                name: unsafe { owned_string(info.driver_name) },
                description: unsafe { owned_string(info.full_name) },
            })
            .collect())
    }

    /// Open one reusable registered driver.
    pub fn driver(&self, name: &str) -> Result<Driver> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_driver_registry_try_create(
                self.registry()?,
                string_view(name),
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Driver(Rc::new(DriverInner {
            raw: NonNull::new(raw).ok_or_else(|| Error::invalid("IREE returned a null driver"))?,
            instance: self.clone(),
            name: name.into(),
        })))
    }
}

impl Driver {
    /// Canonical driver name used when this driver was opened.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Discover physical devices and retain the driver underlying their IDs.
    pub fn devices(&self) -> Result<Vec<DeviceInfo>> {
        let mut allocation = HostAllocation::new();
        check(unsafe {
            sys::iree_hal_driver_query_available_devices(
                self.as_raw(),
                sys::iree_allocator_system(),
                &mut allocation.count,
                &mut allocation.pointer,
            )
        })?;
        Ok(allocation
            .slice()
            .iter()
            .map(|info| DeviceInfo {
                driver: self.clone(),
                id: info.device_id,
                name: unsafe { owned_string(info.name) },
                path: unsafe { owned_string(info.path) },
            })
            .collect())
    }

    /// Configure the driver's default device with native key-value options.
    pub fn device_builder(&self) -> DeviceBuilder<'_> {
        DeviceBuilder {
            driver: self,
            selection: Selection::Path(String::new()),
            parameters: Vec::new(),
        }
    }

    /// Borrow the native driver without transferring its reference.
    pub fn as_raw(&self) -> *mut sys::iree_hal_driver_t {
        self.0.raw.as_ptr()
    }
}

enum Selection {
    Id(sys::iree_hal_device_id_t),
    Ordinal(usize),
    Path(String),
}

/// Device selection and driver-specific creation parameters.
///
/// Unknown or unsupported options are reported by the native driver, not
/// silently discarded. The resulting device retains this driver's lifetime.
pub struct DeviceBuilder<'a> {
    driver: &'a Driver,
    selection: Selection,
    parameters: Vec<(String, String)>,
}

impl DeviceBuilder<'_> {
    /// Select a device by the driver's enumeration order.
    pub fn ordinal(mut self, ordinal: usize) -> Self {
        self.selection = Selection::Ordinal(ordinal);
        self
    }
    /// Select a stable driver-specific path.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.selection = Selection::Path(path.into());
        self
    }
    /// Add a native driver-specific creation parameter.
    pub fn parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.push((key.into(), value.into()));
        self
    }
    /// Create the selected device with a retained native proactor pool.
    pub fn build(self) -> Result<Device> {
        let pool = ProactorPool::new()?;
        let create_params = sys::iree_hal_device_create_params_t {
            next: ptr::null(),
            proactor_pool: pool.0,
        };
        let parameters: Vec<_> = self
            .parameters
            .iter()
            .map(|(key, value)| sys::iree_string_pair_t {
                __bindgen_anon_1: sys::iree_string_pair_t__bindgen_ty_1 {
                    key: string_view(key),
                },
                __bindgen_anon_2: sys::iree_string_pair_t__bindgen_ty_2 {
                    value: string_view(value),
                },
            })
            .collect();
        let mut raw = ptr::null_mut();
        check(unsafe {
            match &self.selection {
                Selection::Id(id) => sys::iree_hal_driver_create_device_by_id(
                    self.driver.as_raw(),
                    *id,
                    parameters.len(),
                    parameters.as_ptr(),
                    &create_params,
                    sys::iree_allocator_system(),
                    &mut raw,
                ),
                Selection::Ordinal(ordinal) => sys::iree_hal_driver_create_device_by_ordinal(
                    self.driver.as_raw(),
                    *ordinal,
                    parameters.len(),
                    parameters.as_ptr(),
                    &create_params,
                    sys::iree_allocator_system(),
                    &mut raw,
                ),
                Selection::Path(path) => sys::iree_hal_driver_create_device_by_path(
                    self.driver.as_raw(),
                    string_view(self.driver.name()),
                    string_view(path),
                    parameters.len(),
                    parameters.as_ptr(),
                    &create_params,
                    sys::iree_allocator_system(),
                    &mut raw,
                ),
            }
        })?;
        unsafe {
            Device::from_owned(
                self.driver.0.instance.clone(),
                raw,
                Some(Box::new(self.driver.clone())),
            )
        }
    }
}

pub(crate) struct ProactorPool(pub(crate) *mut sys::iree_async_proactor_pool_t);

impl ProactorPool {
    pub(crate) fn new() -> Result<Self> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_rs_proactor_pool_create_default(sys::iree_allocator_system(), &mut raw)
        })?;
        Ok(Self(raw))
    }
}

impl Drop for ProactorPool {
    fn drop(&mut self) {
        unsafe { sys::iree_async_proactor_pool_release(self.0) };
    }
}
