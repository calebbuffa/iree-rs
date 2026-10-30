//! Local-sync backend: inline CPU execution on the submitting thread.
//!
//! Unset options keep the native defaults.
use crate::hal::backends::local_loader::{ExecutableLoader, ensure_shared_type_registry};
use crate::hal::{Allocator, Device, DeviceCreateParams, Driver};
use crate::{Result, check, system, view};
use iree_native as n;

/// Sync device tuning; unset fields keep native defaults.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceOptions {
    /// Command arena block size in bytes.
    pub arena_block_size: Option<usize>,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_sync_device_params_t {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw: n::iree_hal_sync_device_params_t = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_sync_device_params_initialize(&mut raw) };
        if let Some(size) = self.arena_block_size {
            raw.arena_block_size = size;
        }
        raw
    }
}

/// Creates a local-sync driver.
///
/// Loaders created with an isolated VM instance are rejected; bind VMVX
/// loaders to the process [`Instance`](crate::vm::Instance) instead.
pub fn create_driver(
    identifier: &str,
    options: DeviceOptions,
    loaders: &[ExecutableLoader],
    device_allocator: Option<&Allocator>,
) -> Result<Driver> {
    ensure_shared_type_registry(loaders)?;
    let params = options.raw();
    let mut loaders: Vec<_> = loaders.iter().map(ExecutableLoader::as_raw).collect();
    let mut raw = core::ptr::null_mut();
    // SAFETY: loaders and allocator outlive the call; the driver retains them.
    check(unsafe {
        n::iree_hal_sync_driver_create(
            view(identifier),
            &params,
            loaders.len(),
            loaders.as_mut_ptr(),
            device_allocator.map_or(core::ptr::null_mut(), Allocator::as_raw),
            system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns one owned driver reference.
    Ok(unsafe { Driver::from_raw(raw) })
}

/// Creates a local-sync device directly, bypassing the driver registry.
///
/// Loaders created with an isolated VM instance are rejected; bind VMVX
/// loaders to the process [`Instance`](crate::vm::Instance) instead.
pub fn create_device(
    identifier: &str,
    options: DeviceOptions,
    loaders: &[ExecutableLoader],
    device_allocator: Option<&Allocator>,
    create_params: &DeviceCreateParams<'_>,
) -> Result<Device> {
    ensure_shared_type_registry(loaders)?;
    let params = options.raw();
    let mut loaders: Vec<_> = loaders.iter().map(ExecutableLoader::as_raw).collect();
    let mut raw = core::ptr::null_mut();
    // SAFETY: loaders and allocator outlive the call; the device retains them.
    check(unsafe {
        n::iree_hal_sync_device_create(
            view(identifier),
            &params,
            create_params.as_raw(),
            loaders.len(),
            loaders.as_mut_ptr(),
            device_allocator.map_or(core::ptr::null_mut(), Allocator::as_raw),
            system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns one owned device reference.
    Ok(unsafe { Device::from_raw(raw) })
}
