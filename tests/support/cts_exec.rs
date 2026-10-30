//! Backend discovery and buffers shared by runtime-only and end-to-end CTS tests.

use ireert::asynchronous::{FrontierTracker, ProactorPool};
use ireert::hal::{
    Buffer, BufferParams, BufferUsage, Device, DeviceCreateParams, DeviceGroup, Driver,
    DriverRegistry, GroupedDevice, MemoryAccess, MemoryType,
};
#[cfg(feature = "local-task")]
use ireert::task::{Executor, Topology};
#[cfg(any(feature = "local-task", feature = "local-sync"))]
use ireert::{
    hal::{Allocator, backends::local_loader::ExecutableLoader},
    vm::Instance,
};
use std::ops::Deref;
use std::sync::{Mutex, MutexGuard, OnceLock};

static REGISTER_DRIVERS: OnceLock<Mutex<bool>> = OnceLock::new();
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

// GroupedDevice owns both its native device reference and the topology-owning group.
pub struct Devices {
    devices: Vec<(String, GroupedDevice)>,
    _drivers: Vec<Driver>,
    _guard: MutexGuard<'static, ()>,
}

// Preserve the same topology and driver lifetimes while devices are consumed.
pub struct DevicesIntoIter {
    devices: std::vec::IntoIter<(String, GroupedDevice)>,
    _drivers: Vec<Driver>,
    _guard: MutexGuard<'static, ()>,
}

impl Deref for Devices {
    type Target = [(String, GroupedDevice)];

    fn deref(&self) -> &Self::Target {
        &self.devices
    }
}

impl IntoIterator for Devices {
    type Item = (String, GroupedDevice);
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
    type Item = (String, GroupedDevice);

    fn next(&mut self) -> Option<Self::Item> {
        self.devices.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.devices.size_hint()
    }
}

impl ExactSizeIterator for DevicesIntoIter {}

fn create_test_driver(registry: &DriverRegistry, name: &str) -> ireert::Result<Driver> {
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
            ireert::Code::Unavailable,
            "enable the matching local backend feature to test this driver safely",
        )),
        #[cfg(not(feature = "local-sync"))]
        "local-sync" => Err(ireert::Error::new(
            ireert::Code::Unavailable,
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

/// Creates every compiled driver device that is available on this host.
pub fn available_devices() -> ireert::Result<Devices> {
    let guard = TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let registry = DriverRegistry::default_registry();
    {
        let mut registered = REGISTER_DRIVERS
            .get_or_init(|| Mutex::new(false))
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !*registered {
            registry.register_all_available()?;
            *registered = true;
        }
    }
    let pool = ProactorPool::shared()?;
    let params = DeviceCreateParams::new(&pool);
    let drivers = registry.enumerate()?;
    let mut devices = Vec::new();
    let mut driver_handles = Vec::new();
    for driver in drivers {
        if std::env::var("IREE_TEST_DEVICE")
            .ok()
            .is_some_and(|selected| selected != driver.name)
        {
            continue;
        }
        let driver_handle = match create_test_driver(&registry, &driver.name) {
            Ok(driver) => driver,
            Err(error)
                if matches!(
                    error.code(),
                    ireert::Code::Unavailable
                        | ireert::Code::NotFound
                        | ireert::Code::Unimplemented
                ) =>
            {
                continue;
            }
            Err(error) => return Err(error),
        };
        match driver_handle.create_default_device(&params) {
            Ok(device) => {
                // Upstream HAL CTS gives each backend its own causal frontier.
                let frontier_tracker = FrontierTracker::new(256, 1, 0)?;
                let group = DeviceGroup::from_device(device, &frontier_tracker)?;
                let grouped_device = group.device(0).ok_or_else(|| {
                    ireert::Error::new(
                        ireert::Code::NotFound,
                        "HAL device group did not contain its device",
                    )
                })?;
                driver_handles.push(driver_handle);
                devices.push((driver.name, grouped_device));
            }
            Err(error)
                if matches!(
                    error.code(),
                    ireert::Code::Unavailable
                        | ireert::Code::NotFound
                        | ireert::Code::Unimplemented
                ) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(Devices {
        devices,
        _drivers: driver_handles,
        _guard: guard,
    })
}

pub fn allocate_transfer_buffer(device: &Device, size: usize) -> ireert::Result<Buffer> {
    let buffer = device.allocator().allocate_buffer(
        BufferParams {
            usage: BufferUsage::TRANSFER,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
            ..BufferParams::default()
        },
        size,
    )?;
    device.transfer_h2d(&vec![0; size], &buffer, 0, None)?;
    Ok(buffer)
}
