//! Backend discovery shared by runtime-only and end-to-end HAL CTS tests.

#[cfg(feature = "local-task")]
use ireert::task::{Executor, Topology};
use ireert::{
    Code,
    asynchronous::{FrontierTracker, ProactorPool},
    hal::{DeviceCreateParams, DeviceGroup, Driver, DriverRegistry, GroupedDevice},
};
#[cfg(any(feature = "local-task", feature = "local-sync"))]
use ireert::{
    hal::{Allocator, backends::local_loader::ExecutableLoader},
    vm::Instance,
};
use std::sync::{Mutex, MutexGuard, OnceLock};

static REGISTER_DRIVERS: OnceLock<Mutex<bool>> = OnceLock::new();
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

// GroupedDevice owns both its native device reference and the topology-owning group.
pub struct Devices {
    devices: Vec<(String, Driver, GroupedDevice)>,
    _drivers: Vec<Driver>,
    _guard: MutexGuard<'static, ()>,
}

// Preserve the same topology and driver lifetimes while devices are consumed.
pub struct DevicesIntoIter {
    devices: std::vec::IntoIter<(String, Driver, GroupedDevice)>,
    _drivers: Vec<Driver>,
    _guard: MutexGuard<'static, ()>,
}

impl IntoIterator for Devices {
    type Item = (String, Driver, GroupedDevice);
    type IntoIter = DevicesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DevicesIntoIter {
            devices: self.devices.into_iter(),
            _drivers: self._drivers,
            _guard: self._guard,
        }
    }
}

impl Iterator for DevicesIntoIter {
    type Item = (String, Driver, GroupedDevice);

    fn next(&mut self) -> Option<Self::Item> {
        self.devices.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.devices.size_hint()
    }
}

impl ExactSizeIterator for DevicesIntoIter {}

fn register_drivers(registry: &DriverRegistry) -> ireert::Result<()> {
    let mut registered = REGISTER_DRIVERS
        .get_or_init(|| Mutex::new(false))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if !*registered {
        registry.register_all_available()?;
        *registered = true;
    }
    Ok(())
}

pub fn registry() -> ireert::Result<DriverRegistry> {
    let registry = DriverRegistry::default_registry();
    register_drivers(&registry)?;
    Ok(registry)
}

pub fn selected_driver(name: &str) -> bool {
    std::env::var("IREE_TEST_DEVICE")
        .ok()
        .is_none_or(|selected| selected == name)
}

pub fn create_test_driver(registry: &DriverRegistry, name: &str) -> ireert::Result<Driver> {
    match name {
        #[cfg(feature = "local-task")]
        "local-task" => {
            let instance = Instance::new()?;
            let loaders = test_local_loaders(&instance)?;
            let allocator = Allocator::heap("local-task-cts")?;
            let topology = Topology::from_group_count(1);
            let executor = Executor::new(&topology, Default::default())?;
            ireert::hal::backends::local_task::create_driver(
                name,
                Default::default(),
                &[executor],
                &loaders,
                Some(&allocator),
            )
        }
        #[cfg(feature = "local-sync")]
        "local-sync" => {
            let instance = Instance::new()?;
            let loaders = test_local_loaders(&instance)?;
            let allocator = Allocator::heap("local-sync-cts")?;
            ireert::hal::backends::local_sync::create_driver(
                name,
                Default::default(),
                &loaders,
                Some(&allocator),
            )
        }
        #[cfg(not(feature = "local-task"))]
        "local-task" => Err(ireert::Error::new(
            Code::Unavailable,
            "enable the matching local backend feature to test this driver safely",
        )),
        #[cfg(not(feature = "local-sync"))]
        "local-sync" => Err(ireert::Error::new(
            Code::Unavailable,
            "enable the matching local backend feature to test this driver safely",
        )),
        _ => registry.try_create(name),
    }
}

#[cfg(any(feature = "local-task", feature = "local-sync"))]
fn test_local_loaders(instance: &Instance) -> ireert::Result<Vec<ExecutableLoader>> {
    Ok(vec![
        ExecutableLoader::system_library(None)?,
        ExecutableLoader::embedded_elf(None)?,
        ExecutableLoader::vmvx(instance, &[])?,
    ])
}

pub fn devices() -> ireert::Result<Devices> {
    let guard = TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let registry = registry()?;
    let pool = ProactorPool::shared()?;
    let mut devices = Vec::new();
    let mut drivers = Vec::new();
    for info in registry.enumerate()? {
        if !selected_driver(&info.name) {
            continue;
        }
        let driver = match create_test_driver(&registry, &info.name) {
            Ok(driver) => driver,
            Err(error)
                if matches!(
                    error.code(),
                    Code::Unavailable | Code::Unimplemented | Code::NotFound
                ) =>
            {
                eprintln!(
                    "skipping {}: driver creation unavailable: {error}",
                    info.name
                );
                continue;
            }
            Err(error) => return Err(error),
        };
        let params = DeviceCreateParams::new(&pool);
        match driver.create_default_device(&params) {
            Ok(device) => {
                // Upstream HAL CTS gives each backend its own causal frontier.
                let frontier_tracker = FrontierTracker::new(256, 1, 0)?;
                let group = DeviceGroup::from_device(device, &frontier_tracker)?;
                let grouped_device = group.device(0).ok_or_else(|| {
                    ireert::Error::new(
                        Code::NotFound,
                        "HAL device group did not contain its device",
                    )
                })?;
                drivers.push(driver.clone());
                devices.push((info.name, driver, grouped_device));
            }
            Err(error)
                if matches!(
                    error.code(),
                    Code::Unavailable | Code::Unimplemented | Code::NotFound
                ) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(Devices {
        devices,
        _drivers: drivers,
        _guard: guard,
    })
}
