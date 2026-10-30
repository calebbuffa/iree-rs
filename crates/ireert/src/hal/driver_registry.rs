//! HAL driver registry ownership and driver discovery.
use super::{
    device::Device,
    driver::{DeviceCreateParams, Driver},
};
use crate::{Error, Result, check, system, view};
use core::{marker::PhantomData, mem::ManuallyDrop, ptr};
use iree_native as n;
use n::{slice, view_string};

/// One registered HAL driver name and its human-readable description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverInfo {
    /// Canonical identifier accepted by `DriverRegistry::try_create`.
    pub name: String,
    /// Human-readable driver name.
    pub full_name: String,
}

/// An IREE HAL registry used to enumerate and create drivers.
pub struct DriverRegistry {
    raw: *mut n::iree_hal_driver_registry_t,
    owned: bool,
    _not_send: PhantomData<*mut ()>,
}
impl DriverRegistry {
    /// Returns the process-wide default registry, which is never freed.
    pub fn default_registry() -> Self {
        Self {
            // SAFETY: IREE returns its process-lifetime singleton.
            raw: unsafe { n::iree_hal_driver_registry_default() },
            owned: false,
            _not_send: PhantomData,
        }
    }
    /// Creates a caller-owned registry.
    pub fn new() -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: the output pointer is writable.
        check(unsafe { n::iree_hal_driver_registry_allocate(system(), &mut raw) })?;
        // SAFETY: a successful allocation returns an owned registry.
        Ok(unsafe { Self::from_raw(raw) })
    }
    /// Adopts an owned registry.
    ///
    /// # Safety
    /// `raw` must be a live registry whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_driver_registry_t) -> Self {
        Self {
            raw,
            owned: true,
            _not_send: PhantomData,
        }
    }
    /// Borrows the native registry pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_driver_registry_t {
        self.raw
    }
    /// Transfers registry ownership to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_driver_registry_t {
        ManuallyDrop::new(self).raw
    }
    /// Returns the registered drivers.
    pub fn enumerate(&self) -> Result<Vec<DriverInfo>> {
        let mut count = 0;
        let mut infos = ptr::null_mut();
        // SAFETY: output pointers are writable; the array is freed with the same allocator below.
        check(unsafe {
            n::iree_hal_driver_registry_enumerate(self.raw, system(), &mut count, &mut infos)
        })?;
        // SAFETY: enumeration returned `count` records whose strings live in the same allocation.
        let drivers = unsafe { slice(infos, count) }
            .iter()
            .map(|info| DriverInfo {
                // SAFETY: as above.
                name: unsafe { view_string(info.driver_name) },
                // SAFETY: as above.
                full_name: unsafe { view_string(info.full_name) },
            })
            .collect();
        // SAFETY: the array was allocated by enumeration with `system()`.
        unsafe { n::iree_allocator_free(system(), infos.cast()) };
        Ok(drivers)
    }
    /// Registers all driver factories compiled into the native runtime.
    pub fn register_all_available(&self) -> Result<()> {
        // SAFETY: the registry is live for the duration of this call.
        check(unsafe { n::iree_hal_register_all_available_drivers(self.raw) })
    }
    /// Creates a device from a non-local IREE device URI.
    ///
    /// `local-task` and `local-sync` create isolated VMVX instances in the
    /// native registry path, which conflict with the process-wide VM type
    /// registry. Construct those devices through the direct constructors in
    /// `hal::backends::local_task` or `hal::backends::local_sync`, using
    /// loaders created from the shared [`crate::vm::Instance`].
    pub fn create_device(&self, uri: &str, params: &DeviceCreateParams<'_>) -> Result<Device> {
        if uses_isolated_vmvx(uri) {
            return Err(Error::invalid(
                "local HAL drivers require explicit VMVX loaders from the shared VM instance",
            ));
        }
        // SAFETY: handles and strings are borrowed for the synchronous call.
        super::driver::create(params, |params, out| unsafe {
            n::iree_hal_create_device(self.raw, view(uri), params, system(), out)
        })
    }
    /// Registers `factory` to serve future driver queries and creation requests.
    ///
    /// The factory is unregistered when the returned registration is dropped.
    pub fn register<F: DriverFactory>(&self, factory: F) -> Result<FactoryRegistration<'_>> {
        let drivers = factory.drivers();
        let infos = drivers
            .iter()
            .map(|info| n::iree_hal_driver_info_t {
                driver_name: view(&info.name),
                full_name: view(&info.full_name),
            })
            .collect();
        let mut state = Box::new(FactoryState {
            native: n::iree_hal_driver_factory_t::default(),
            infos,
            _drivers: drivers,
            factory: Box::new(factory),
        });
        let native = n::iree_hal_driver_factory_t {
            self_: (&raw mut *state).cast(),
            enumerate: Some(enumerate),
            try_create: Some(try_create),
        };
        state.native = native;
        // SAFETY: the boxed factory stays at a stable address until unregistered on drop.
        check(unsafe { n::iree_hal_driver_registry_register_factory(self.raw, &state.native) })?;
        Ok(FactoryRegistration {
            registry: self,
            state: Some(state),
        })
    }
    /// Creates a non-local driver by its canonical name.
    pub fn try_create(&self, name: &str) -> Result<Driver> {
        if uses_isolated_vmvx(name) {
            return Err(Error::invalid(
                "local HAL drivers require explicit VMVX loaders from the shared VM instance",
            ));
        }
        let mut raw = ptr::null_mut();
        // SAFETY: the name is borrowed for the call; the output is writable.
        check(unsafe {
            n::iree_hal_driver_registry_try_create(self.raw, view(name), system(), &mut raw)
        })?;
        // SAFETY: successful creation returns an owned driver reference.
        Ok(unsafe { Driver::from_raw(raw) })
    }
}
impl Drop for DriverRegistry {
    fn drop(&mut self) {
        if self.owned {
            // SAFETY: the wrapper owns the registry and frees it once.
            unsafe { n::iree_hal_driver_registry_free(self.raw) }
        }
    }
}

fn uses_isolated_vmvx(uri_or_name: &str) -> bool {
    let driver = uri_or_name
        .split_once("://")
        .map_or(uri_or_name, |(driver, _)| driver);
    matches!(driver, "local-task" | "local-sync")
}

/// A Rust source of HAL drivers registered with a [`DriverRegistry`].
///
/// Methods may be called from any thread while the registry lock is held, so
/// implementations must not re-enter the registry.
pub trait DriverFactory: Send + Sync + 'static {
    /// The drivers this factory provides; fixed for the registration's lifetime.
    fn drivers(&self) -> Vec<DriverInfo>;
    /// Creates the driver named `name`, one of [`drivers`](Self::drivers).
    fn create(&self, name: &str) -> Result<Driver>;
}

struct FactoryState {
    native: n::iree_hal_driver_factory_t,
    infos: Vec<n::iree_hal_driver_info_t>,
    // Owns the strings viewed by `infos`.
    _drivers: Vec<DriverInfo>,
    factory: Box<dyn DriverFactory>,
}

unsafe extern "C" fn enumerate(
    state: *mut core::ffi::c_void,
    out_count: *mut usize,
    out_infos: *mut *const n::iree_hal_driver_info_t,
) -> n::iree_status_t {
    // SAFETY: `state` is the registered factory state and outputs are writable.
    unsafe {
        let state = &*state.cast::<FactoryState>();
        *out_count = state.infos.len();
        *out_infos = state.infos.as_ptr();
    }
    ptr::null_mut()
}

unsafe extern "C" fn try_create(
    state: *mut core::ffi::c_void,
    name: n::iree_string_view_t,
    _host_allocator: n::iree_allocator_t,
    out_driver: *mut *mut n::iree_hal_driver_t,
) -> n::iree_status_t {
    n::callback_status(|| {
        // SAFETY: `state` is the registered factory state for this call.
        let state = unsafe { &*state.cast::<FactoryState>() };
        // SAFETY: the name is borrowed for this call.
        let driver = state.factory.create(&unsafe { view_string(name) })?;
        // SAFETY: the output is writable; the caller takes the reference.
        unsafe { *out_driver = driver.into_raw() };
        Ok(())
    })
}

/// A [`DriverFactory`] registration; unregistered on drop.
pub struct FactoryRegistration<'r> {
    registry: &'r DriverRegistry,
    state: Option<Box<FactoryState>>,
}
impl FactoryRegistration<'_> {
    /// Unregisters the factory; drivers it already created stay live.
    pub fn unregister(mut self) -> Result<()> {
        self.release()
    }
    fn release(&mut self) -> Result<()> {
        let Some(state) = self.state.take() else {
            return Ok(());
        };
        // SAFETY: the factory was registered with this registry at this address.
        let status = check(unsafe {
            n::iree_hal_driver_registry_unregister_factory(self.registry.raw, &state.native)
        });
        if status.is_err() {
            // The registry may still reference the factory, so it must stay live.
            core::mem::forget(state);
        }
        status
    }
}
impl Drop for FactoryRegistration<'_> {
    fn drop(&mut self) {
        let _status = self.release();
    }
}

#[cfg(test)]
mod tests {
    use super::uses_isolated_vmvx;

    #[test]
    fn identifies_local_driver_names_and_uris() {
        for value in [
            "local-task",
            "local-task://default",
            "local-sync",
            "local-sync://default",
        ] {
            assert!(uses_isolated_vmvx(value), "{value}");
        }
        for value in ["vulkan", "vulkan://0", "cuda", "custom://device"] {
            assert!(!uses_isolated_vmvx(value), "{value}");
        }
    }
}
