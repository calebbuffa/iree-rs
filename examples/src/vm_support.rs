//! Helpers used by the VM-centric sample ports.

use ireec::{Compiler, Output, Pipeline, Session};
#[cfg(any(feature = "local-task", feature = "local-sync"))]
use ireert::hal::backends::local_loader::ExecutableLoader;
#[cfg(feature = "local-task")]
use ireert::task::{Executor, Topology};
use ireert::{
    asynchronous::{FrontierTracker, ProactorPool},
    hal::{self, Device, DeviceCreateParams, DeviceGroup, DriverRegistry, GroupedDevice},
    vm::{DebugSink, HalModuleDevicePolicy, HalModuleFlags, Instance, Module},
};
use std::{error::Error, sync::OnceLock};

/// Compiles MLIR source to an in-memory VM bytecode archive.
pub fn compile_mlir(name: &str, source: &str, extra_flags: &[&str]) -> ireec::Result<Vec<u8>> {
    compile_mlir_for_backend(name, source, LocalBackend::Vmvx, extra_flags)
}

/// A backend selected through IREE's local HAL target-device options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalBackend {
    /// Portable VMVX backend.
    Vmvx,
    /// Host CPU LLVM backend.
    LlvmCpu,
}

impl LocalBackend {
    fn flag_value(self) -> &'static str {
        match self {
            Self::Vmvx => "vmvx",
            Self::LlvmCpu => "llvm-cpu",
        }
    }
}

/// Compiles a host-only VM module without creating a HAL target.
pub fn compile_mlir_host_only(
    name: &str,
    source: &str,
    extra_flags: &[&str],
) -> ireec::Result<Vec<u8>> {
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    let mut flags = vec!["--iree-execution-model=host-only"];
    flags.extend_from_slice(extra_flags);
    session.set_flags(&flags)?;
    compile_session(&compiler, &session, name, source)
}

/// Compiles MLIR source to bytecode for the requested local HAL backend.
pub fn compile_mlir_for_backend(
    name: &str,
    source: &str,
    backend: LocalBackend,
    extra_flags: &[&str],
) -> ireec::Result<Vec<u8>> {
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    let mut flags = vec![
        "--iree-hal-target-device=local".to_owned(),
        format!(
            "--iree-hal-local-target-device-backends={}",
            backend.flag_value()
        ),
    ];
    flags.extend(extra_flags.iter().map(|flag| (*flag).to_owned()));
    session.set_flags(&flags)?;
    compile_session(&compiler, &session, name, source)
}

fn compile_session(
    compiler: &Compiler,
    session: &Session,
    name: &str,
    source: &str,
) -> ireec::Result<Vec<u8>> {
    let source = session.source_buffer(name, source.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    Ok(output.bytes()?.to_vec())
}

/// A device and its VM-facing HAL module group.
pub struct SampleDevice {
    /// Device selected by `IREE_SAMPLE_DEVICE` (default `local-task`).
    pub device: GroupedDevice,
    _pool: ProactorPool,
    _tracker: FrontierTracker,
}

impl SampleDevice {
    /// Creates the device named by `IREE_SAMPLE_DEVICE` or defaults to `local-task`.
    pub fn from_environment() -> Result<Self, Box<dyn Error>> {
        let uri = match std::env::var("IREE_SAMPLE_DEVICE") {
            Ok(uri) => uri,
            Err(std::env::VarError::NotPresent) => "local-task".to_owned(),
            Err(error) => return Err(error.into()),
        };
        Self::from_uri(&uri)
    }

    /// Creates the device named by a HAL driver URI.
    pub fn from_uri(uri: &str) -> Result<Self, Box<dyn Error>> {
        let pool = ProactorPool::shared()?;
        let driver = uri.split_once("://").map_or(uri, |(driver, _)| driver);
        if matches!(driver, "local-task" | "local-sync") {
            if uri != driver {
                return Err("local CPU sample devices do not accept URI parameters".into());
            }
            let ungrouped_device = create_local_device(driver, &pool)?;
            return Self::from_device(ungrouped_device, pool);
        }

        let registry = DriverRegistry::default_registry();
        static REGISTER_DRIVERS: OnceLock<Result<(), String>> = OnceLock::new();
        if let Err(error) = REGISTER_DRIVERS
            .get_or_init(|| registry.register_all_available().map_err(|e| e.to_string()))
        {
            return Err(std::io::Error::other(error.clone()).into());
        }
        let ungrouped_device = registry.create_device(uri, &DeviceCreateParams::new(&pool))?;
        Self::from_device(ungrouped_device, pool)
    }

    /// Creates a local-sync device with the specified executable loaders.
    #[cfg(feature = "local-sync")]
    pub fn local_sync_with_loaders(
        loaders: &[ireert::hal::backends::local_loader::ExecutableLoader],
    ) -> Result<Self, Box<dyn Error>> {
        let pool = ProactorPool::shared()?;
        let ungrouped_device = ireert::hal::backends::local_sync::create_device(
            "local-sync",
            Default::default(),
            loaders,
            None,
            &DeviceCreateParams::new(&pool),
        )?;
        Self::from_device(ungrouped_device, pool)
    }

    fn from_device(device: Device, pool: ProactorPool) -> Result<Self, Box<dyn Error>> {
        let tracker = FrontierTracker::new(16, 0, 0)?;
        let group = DeviceGroup::from_device(device, &tracker)?;
        let device = group
            .device(0)
            .ok_or("new device group unexpectedly contains no device")?;
        Ok(Self {
            device,
            _pool: pool,
            _tracker: tracker,
        })
    }

    /// Creates the `hal` VM module that binds bytecode execution to this device.
    pub fn hal_module(&self, instance: &Instance) -> ireert::Result<Module> {
        Module::hal(
            instance,
            HalModuleDevicePolicy::default(),
            self.device.device_group(),
            HalModuleFlags::default(),
            DebugSink::default(),
        )
    }

    /// Returns the device's allocator for tensor input/output buffers.
    pub fn allocator(&self) -> hal::Allocator {
        self.device.allocator()
    }
}

#[cfg(any(feature = "local-task", feature = "local-sync"))]
fn shared_local_loaders(instance: &Instance) -> ireert::Result<Vec<ExecutableLoader>> {
    Ok(vec![
        ExecutableLoader::system_library(None)?,
        ExecutableLoader::embedded_elf(None)?,
        ExecutableLoader::vmvx(instance, &[])?,
    ])
}

fn create_local_device(driver: &str, pool: &ProactorPool) -> Result<Device, Box<dyn Error>> {
    #[cfg(any(feature = "local-task", feature = "local-sync"))]
    {
        let instance = Instance::new()?;
        let loaders = shared_local_loaders(&instance)?;
        let params = DeviceCreateParams::new(pool);
        let allocator = hal::Allocator::heap("local")?;
        match driver {
            "local-task" => {
                #[cfg(feature = "local-task")]
                {
                    let worker_count = std::thread::available_parallelism().map_or(1, usize::from);
                    let topology = Topology::from_group_count(worker_count);
                    let executor = Executor::new(&topology, Default::default())?;
                    Ok(ireert::hal::backends::local_task::create_device(
                        driver,
                        Default::default(),
                        &[executor],
                        &loaders,
                        Some(&allocator),
                        &params,
                    )?)
                }
                #[cfg(not(feature = "local-task"))]
                {
                    Err("enable the `local-task` feature to use this device".into())
                }
            }
            "local-sync" => {
                #[cfg(feature = "local-sync")]
                {
                    Ok(ireert::hal::backends::local_sync::create_device(
                        driver,
                        Default::default(),
                        &loaders,
                        Some(&allocator),
                        &params,
                    )?)
                }
                #[cfg(not(feature = "local-sync"))]
                {
                    Err("enable the `local-sync` feature to use this device".into())
                }
            }
            _ => Err("unknown local CPU backend".into()),
        }
    }
    #[cfg(not(any(feature = "local-task", feature = "local-sync")))]
    {
        let _ = (driver, pool);
        Err("enable a local CPU backend feature to use this device".into())
    }
}
