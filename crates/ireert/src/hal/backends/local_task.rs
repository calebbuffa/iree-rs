//! Local-task backend: multithreaded CPU execution on task executors.
//!
//! Unset options keep the native defaults.
use crate::hal::backends::local_loader::{ExecutableLoader, ensure_shared_type_registry};
use crate::hal::{Allocator, Device, DeviceCreateParams, Driver};
use crate::task::Executor;
use crate::{Result, check, system, view};
use iree_native as n;

/// Task device tuning; unset fields keep native defaults.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceOptions {
    /// Command arena block size in bytes.
    pub arena_block_size: Option<usize>,
    /// Abort a queue scope as soon as one of its tasks fails.
    pub abort_on_failure: bool,
    /// Transfers at or below this many bytes run inline on the submitting thread.
    pub inline_transfer_threshold: Option<usize>,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_task_device_params_t {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw: n::iree_hal_task_device_params_t = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_task_device_params_initialize(&mut raw) };
        if let Some(value) = self.arena_block_size {
            raw.arena_block_size = value;
        }
        if self.abort_on_failure {
            raw.queue_scope_flags |= n::IREE_TASK_SCOPE_FLAG_ABORT_ON_FAILURE as u32;
        }
        if let Some(value) = self.inline_transfer_threshold {
            raw.inline_transfer_threshold = value;
        }
        raw
    }
}

/// Creates a local-task driver whose devices run on `executors`.
///
/// Loaders created with an isolated VM instance are rejected; bind VMVX
/// loaders to the process [`Instance`](crate::vm::Instance) instead.
pub fn create_driver(
    identifier: &str,
    options: DeviceOptions,
    executors: &[Executor],
    loaders: &[ExecutableLoader],
    device_allocator: Option<&Allocator>,
) -> Result<Driver> {
    ensure_shared_type_registry(loaders)?;
    let params = options.raw();
    let executors: Vec<_> = executors.iter().map(Executor::as_raw).collect();
    let mut loaders: Vec<_> = loaders.iter().map(ExecutableLoader::as_raw).collect();
    let mut raw = core::ptr::null_mut();
    // SAFETY: all pointer arrays and their referents outlive the call; the
    // driver retains what it keeps.
    check(unsafe {
        n::iree_hal_task_driver_create(
            view(identifier),
            &params,
            executors.len(),
            executors.as_ptr(),
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

/// Creates a local-task device directly, bypassing the driver registry.
///
/// Loaders created with an isolated VM instance are rejected; bind VMVX
/// loaders to the process [`Instance`](crate::vm::Instance) instead.
pub fn create_device(
    identifier: &str,
    options: DeviceOptions,
    executors: &[Executor],
    loaders: &[ExecutableLoader],
    device_allocator: Option<&Allocator>,
    create_params: &DeviceCreateParams<'_>,
) -> Result<Device> {
    ensure_shared_type_registry(loaders)?;
    let params = options.raw();
    let executors: Vec<_> = executors.iter().map(Executor::as_raw).collect();
    let mut loaders: Vec<_> = loaders.iter().map(ExecutableLoader::as_raw).collect();
    let mut raw = core::ptr::null_mut();
    // SAFETY: all pointer arrays and their referents outlive the call; the
    // device retains what it keeps.
    check(unsafe {
        n::iree_hal_task_device_create(
            view(identifier),
            &params,
            executors.len(),
            executors.as_ptr(),
            loaders.len(),
            loaders.as_mut_ptr(),
            device_allocator.map_or(core::ptr::null_mut(), Allocator::as_raw),
            create_params.as_raw(),
            system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns one owned device reference.
    Ok(unsafe { Device::from_raw(raw) })
}
