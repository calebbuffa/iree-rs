use crate::{Device, Instance, Result, runtime::DeviceInner, sys};
use std::{ptr::NonNull, rc::Rc, sync::Arc};

struct SharedDeviceInner {
    device: NonNull<sys::iree_hal_device_t>,
    hal_module: NonNull<sys::iree_vm_module_t>,
    instance: Instance,
}

// HAL devices provide synchronized allocation/submission and native atomic
// resource ownership. VM module implementations must be thread-safe. This
// constructor accepts only native registered devices, never external owners or
// arbitrary raw handles; each thread creates its own VM context and Rc graph.
unsafe impl Send for SharedDeviceInner {}
unsafe impl Sync for SharedDeviceInner {}

impl Drop for SharedDeviceInner {
    fn drop(&mut self) {
        unsafe {
            sys::iree_vm_module_release(self.hal_module.as_ptr());
            sys::iree_hal_device_release(self.device.as_ptr());
        }
    }
}

/// A native IREE device shared across threads with independent local sessions.
///
/// Cloning this handle shares native hardware, allocators, task threads, and one
/// HAL module/device group. [`Self::local`] creates a fresh thread-local Rust
/// ownership graph. Load an artifact on that local device to obtain an isolated
/// VM context; do not share mutable session state across threads.
///
/// External graphics devices and unsafe raw-device adoption are intentionally
/// excluded. Their external owners may impose additional thread constraints.
#[derive(Clone)]
pub struct SharedDevice(Arc<SharedDeviceInner>);

impl Instance {
    /// Create a registered native device for reuse across worker threads.
    ///
    /// The `task` feature requires a native build with synchronization enabled.
    /// All workers share this device's native resources rather than initializing
    /// a separate hardware device for every session.
    pub fn shared_device(&self, driver: &str) -> Result<SharedDevice> {
        let local = self.device(driver)?;
        unsafe {
            sys::iree_hal_device_retain(local.as_raw());
            sys::iree_vm_module_retain(local.0.hal_module.as_ptr());
        }
        Ok(SharedDevice(Arc::new(SharedDeviceInner {
            device: local.0.raw,
            hal_module: local.0.hal_module,
            instance: self.clone(),
        })))
    }
}

impl SharedDevice {
    /// Obtain a local handle for tensor allocation and independent VM sessions.
    ///
    /// This retains the existing HAL module; it never reassigns device topology
    /// or registers a second device group for the same device.
    pub fn local(&self) -> Device {
        unsafe {
            sys::iree_hal_device_retain(self.0.device.as_ptr());
            sys::iree_vm_module_retain(self.0.hal_module.as_ptr());
        }
        Device(Rc::new(DeviceInner {
            raw: self.0.device,
            hal_module: self.0.hal_module,
            instance: self.0.instance.clone(),
            allocation_access: Default::default(),
            _owner: Some(Box::new(self.clone())),
        }))
    }
}
