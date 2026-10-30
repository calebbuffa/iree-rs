//! Interception of IREE's Vulkan function table.
//!
//! IREE resolves every device function through the `vkGetInstanceProcAddr`
//! its function table is created from (`iree_hal_vulkan_syms_create`). The
//! context hands it [`get_instance_proc_addr`], which forwards to the real
//! loader except for a few device functions on registered devices:
//!
//! - **Queue access.** `vkQueueSubmit2`, `vkQueueBindSparse` (and, should IREE
//!   ever call them, `vkQueueSubmit`, `vkQueueWaitIdle` and
//!   `vkDeviceWaitIdle`) take a per-`VkQueue` lock that the context also
//!   holds around every WGPU queue call. Vulkan requires queue access to be
//!   externally synchronized; this makes it so across both APIs.
//! - **Object lifetimes.** Every object IREE creates on the device is counted.
//!   IREE buffers do not retain the IREE device and the IREE device does not
//!   own a wrapped `VkDevice`, so this count, not any reference, says when the
//!   `VkDevice` may be destroyed.
//! - **Queue-family sharing.** When IREE's queue is in a different family from
//!   WGPU's, IREE buffers are created with concurrent sharing between the two
//!   families, so neither API needs queue-family ownership transfers.
//!
//! Hooks run on IREE's threads inside C calls: they never panic and never
//! block on anything but the short queue and bookkeeping locks.

use ash::vk::{self, Handle};
use std::{
    collections::HashMap,
    ffi::{CStr, c_char, c_void},
    sync::{Arc, LazyLock, Mutex, MutexGuard, OnceLock, PoisonError, RwLock},
};

/// Work that destroys the `VkDevice` once nothing uses it any more.
pub(crate) type Teardown = Box<dyn FnOnce() + Send>;

/// The loader's `vkGetInstanceProcAddr`, shared with WGPU.
static LOADER: OnceLock<vk::PFN_vkGetInstanceProcAddr> = OnceLock::new();
/// The loader's `vkGetDeviceProcAddr`, for devices that are not registered.
static DEVICE_LOADER: OnceLock<vk::PFN_vkGetDeviceProcAddr> = OnceLock::new();
static DEVICES: LazyLock<RwLock<HashMap<u64, Arc<DeviceHook>>>> = LazyLock::new(Default::default);
static QUEUES: LazyLock<RwLock<HashMap<u64, Arc<QueueHook>>>> = LazyLock::new(Default::default);

/// Bookkeeping for one `VkDevice` shared by WGPU and IREE.
pub(crate) struct DeviceHook {
    /// The device's real (unhooked) function table.
    raw: ash::Device,
    get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    /// Queue families IREE buffers are shared between, when they differ.
    concurrent: Option<[u32; 2]>,
    life: Mutex<Life>,
}

#[derive(Default)]
struct Life {
    /// Live objects IREE created on the device.
    live: usize,
    /// Set once every Rust owner of the device is gone.
    teardown: Option<Teardown>,
    queues: Vec<Arc<QueueHook>>,
}

/// A `VkQueue` and the lock serializing access to it.
pub(crate) struct QueueHook {
    handle: vk::Queue,
    raw: ash::Device,
    lock: Mutex<()>,
}

impl QueueHook {
    /// Hold the queue for the duration of a queue call.
    pub(crate) fn lock(&self) -> MutexGuard<'_, ()> {
        self.lock.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn handle(&self) -> vk::Queue {
        self.handle
    }
}

fn read<T>(lock: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(PoisonError::into_inner)
}

fn write<T>(lock: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    lock.write().unwrap_or_else(PoisonError::into_inner)
}

/// The `vkGetInstanceProcAddr` IREE's function table must be created from.
///
/// Fails if WGPU's loader differs from one registered earlier in the process.
pub(crate) fn proc_address(
    loader: vk::PFN_vkGetInstanceProcAddr,
) -> Result<*mut c_void, &'static str> {
    let installed = *LOADER.get_or_init(|| loader);
    if installed as usize != loader as usize {
        return Err("WGPU instances in this process use different Vulkan loaders");
    }
    Ok(get_instance_proc_addr as *const () as *mut c_void)
}

/// Register a device before IREE wraps it.
///
/// `raw` must be the device's real function table, and `concurrent` the WGPU
/// and IREE queue families when they differ.
pub(crate) fn register(
    raw: ash::Device,
    get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    concurrent: Option<[u32; 2]>,
) -> Arc<DeviceHook> {
    let hook = Arc::new(DeviceHook {
        raw,
        get_device_proc_addr,
        concurrent,
        life: Mutex::default(),
    });
    write(&DEVICES).insert(hook.raw.handle().as_raw(), hook.clone());
    hook
}

impl DeviceHook {
    fn life(&self) -> MutexGuard<'_, Life> {
        self.life.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The lock for `queue`, registering it on first use.
    pub(crate) fn queue(&self, queue: vk::Queue) -> Arc<QueueHook> {
        let mut queues = write(&QUEUES);
        let hook = queues
            .entry(queue.as_raw())
            .or_insert_with(|| {
                Arc::new(QueueHook {
                    handle: queue,
                    raw: self.raw.clone(),
                    lock: Mutex::new(()),
                })
            })
            .clone();
        drop(queues);
        let mut life = self.life();
        if !life.queues.iter().any(|known| Arc::ptr_eq(known, &hook)) {
            life.queues.push(hook.clone());
        }
        hook
    }

    /// Run `f` while holding every queue of the device, as
    /// `vkDeviceWaitIdle` and swapchain teardown require.
    pub(crate) fn with_all_queues<R>(&self, f: impl FnOnce() -> R) -> R {
        let mut queues = self.life().queues.clone();
        // A global order keeps two such callers from deadlocking.
        queues.sort_by_key(|queue| queue.handle.as_raw());
        with_locked(&queues, f)
    }

    fn created(&self, count: usize) {
        if count != 0 {
            self.life().live += count;
        }
    }

    fn destroyed(&self) {
        let teardown = {
            let mut life = self.life();
            life.live = life.live.saturating_sub(1);
            if life.live == 0 {
                life.teardown.take()
            } else {
                None
            }
        };
        if let Some(teardown) = teardown {
            self.finish(teardown);
        }
    }

    /// Every Rust owner of the device is gone: destroy it once IREE's last
    /// object is, which may be now or on whichever thread destroys it.
    pub(crate) fn release(&self, teardown: Teardown) {
        let now = {
            let mut life = self.life();
            if life.live == 0 {
                Some(teardown)
            } else {
                life.teardown = Some(teardown);
                None
            }
        };
        if let Some(teardown) = now {
            self.finish(teardown);
        }
    }

    fn finish(&self, teardown: Teardown) {
        let queues = std::mem::take(&mut self.life().queues);
        {
            let mut registered = write(&QUEUES);
            for queue in &queues {
                registered.remove(&queue.handle.as_raw());
            }
        }
        write(&DEVICES).remove(&self.raw.handle().as_raw());
        teardown();
    }
}

fn with_locked<R>(queues: &[Arc<QueueHook>], f: impl FnOnce() -> R) -> R {
    match queues.split_first() {
        None => f(),
        Some((first, rest)) => {
            let _held = first.lock();
            with_locked(rest, f)
        }
    }
}

fn device(handle: vk::Device) -> Option<Arc<DeviceHook>> {
    read(&DEVICES).get(&handle.as_raw()).cloned()
}

fn queue(handle: vk::Queue) -> Option<Arc<QueueHook>> {
    read(&QUEUES).get(&handle.as_raw()).cloned()
}

fn name<'a>(name: *const c_char) -> &'a [u8] {
    if name.is_null() {
        return &[];
    }
    // SAFETY: Vulkan passes a NUL-terminated function name.
    unsafe { CStr::from_ptr(name) }.to_bytes()
}

macro_rules! pfn {
    ($function:expr) => {
        // SAFETY: Vulkan casts every function to `PFN_vkVoidFunction`; callers
        // cast it back to the signature of the name they looked up, which is
        // the signature of `$function`.
        Some(unsafe {
            std::mem::transmute::<*const (), unsafe extern "system" fn()>($function as *const ())
        })
    };
}

unsafe extern "system" fn get_instance_proc_addr(
    instance: vk::Instance,
    symbol: *const c_char,
) -> vk::PFN_vkVoidFunction {
    let loader = LOADER.get()?;
    if name(symbol) == b"vkGetDeviceProcAddr" {
        // SAFETY: forwarded unchanged to the loader.
        let real = unsafe { loader(instance, symbol) };
        if let Some(real) = real {
            // SAFETY: the loader returned `vkGetDeviceProcAddr`.
            DEVICE_LOADER.get_or_init(|| unsafe {
                std::mem::transmute::<unsafe extern "system" fn(), vk::PFN_vkGetDeviceProcAddr>(
                    real,
                )
            });
            return pfn!(get_device_proc_addr);
        }
        return None;
    }
    // SAFETY: forwarded unchanged to the loader.
    unsafe { loader(instance, symbol) }
}

unsafe extern "system" fn get_device_proc_addr(
    handle: vk::Device,
    symbol: *const c_char,
) -> vk::PFN_vkVoidFunction {
    let Some(hook) = device(handle) else {
        // SAFETY: forwarded unchanged to the loader.
        return DEVICE_LOADER
            .get()
            .and_then(|real| unsafe { real(handle, symbol) });
    };
    let hooked = match name(symbol) {
        b"vkGetDeviceQueue" => pfn!(get_device_queue),
        b"vkGetDeviceQueue2" => pfn!(get_device_queue2),
        b"vkQueueSubmit" => pfn!(queue_submit),
        b"vkQueueSubmit2" | b"vkQueueSubmit2KHR" => pfn!(queue_submit2),
        b"vkQueueBindSparse" => pfn!(queue_bind_sparse),
        b"vkQueueWaitIdle" => pfn!(queue_wait_idle),
        b"vkDeviceWaitIdle" => pfn!(device_wait_idle),
        b"vkDestroyDevice" => pfn!(destroy_device),
        b"vkCreateBuffer" => pfn!(create_buffer),
        b"vkDestroyBuffer" => pfn!(destroy_buffer),
        b"vkAllocateMemory" => pfn!(allocate_memory),
        b"vkFreeMemory" => pfn!(free_memory),
        b"vkCreateFence" => pfn!(create_fence),
        b"vkDestroyFence" => pfn!(destroy_fence),
        b"vkCreateSemaphore" => pfn!(create_semaphore),
        b"vkDestroySemaphore" => pfn!(destroy_semaphore),
        b"vkCreateCommandPool" => pfn!(create_command_pool),
        b"vkDestroyCommandPool" => pfn!(destroy_command_pool),
        b"vkCreateQueryPool" => pfn!(create_query_pool),
        b"vkDestroyQueryPool" => pfn!(destroy_query_pool),
        b"vkCreatePipelineCache" => pfn!(create_pipeline_cache),
        b"vkDestroyPipelineCache" => pfn!(destroy_pipeline_cache),
        b"vkCreateDescriptorSetLayout" => pfn!(create_descriptor_set_layout),
        b"vkDestroyDescriptorSetLayout" => pfn!(destroy_descriptor_set_layout),
        b"vkCreateDescriptorPool" => pfn!(create_descriptor_pool),
        b"vkDestroyDescriptorPool" => pfn!(destroy_descriptor_pool),
        b"vkCreatePipelineLayout" => pfn!(create_pipeline_layout),
        b"vkDestroyPipelineLayout" => pfn!(destroy_pipeline_layout),
        b"vkCreateShaderModule" => pfn!(create_shader_module),
        b"vkDestroyShaderModule" => pfn!(destroy_shader_module),
        b"vkCreateComputePipelines" => pfn!(create_compute_pipelines),
        b"vkDestroyPipeline" => pfn!(destroy_pipeline),
        _ => None,
    };
    // SAFETY: forwarded unchanged to the device's resolver.
    hooked.or_else(|| unsafe { (hook.get_device_proc_addr)(handle, symbol) })
}

unsafe extern "system" fn get_device_queue(
    handle: vk::Device,
    family: u32,
    index: u32,
    out: *mut vk::Queue,
) {
    let Some(hook) = device(handle) else { return };
    // SAFETY: arguments forwarded unchanged.
    unsafe { (hook.raw.fp_v1_0().get_device_queue)(handle, family, index, out) };
    // SAFETY: Vulkan wrote the queue handle.
    let queue = unsafe { *out };
    if !queue.is_null() {
        hook.queue(queue);
    }
}

unsafe extern "system" fn get_device_queue2(
    handle: vk::Device,
    info: *const vk::DeviceQueueInfo2<'_>,
    out: *mut vk::Queue,
) {
    let Some(hook) = device(handle) else { return };
    // SAFETY: arguments forwarded unchanged.
    unsafe { (hook.raw.fp_v1_1().get_device_queue2)(handle, info, out) };
    // SAFETY: Vulkan wrote the queue handle.
    let queue = unsafe { *out };
    if !queue.is_null() {
        hook.queue(queue);
    }
}

unsafe extern "system" fn queue_submit(
    handle: vk::Queue,
    count: u32,
    submits: *const vk::SubmitInfo<'_>,
    fence: vk::Fence,
) -> vk::Result {
    let Some(queue) = queue(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    let _held = queue.lock();
    // SAFETY: arguments forwarded unchanged, with the queue held.
    unsafe { (queue.raw.fp_v1_0().queue_submit)(handle, count, submits, fence) }
}

unsafe extern "system" fn queue_submit2(
    handle: vk::Queue,
    count: u32,
    submits: *const vk::SubmitInfo2<'_>,
    fence: vk::Fence,
) -> vk::Result {
    let Some(queue) = queue(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    let _held = queue.lock();
    // SAFETY: arguments forwarded unchanged, with the queue held.
    unsafe { (queue.raw.fp_v1_3().queue_submit2)(handle, count, submits, fence) }
}

unsafe extern "system" fn queue_bind_sparse(
    handle: vk::Queue,
    count: u32,
    binds: *const vk::BindSparseInfo<'_>,
    fence: vk::Fence,
) -> vk::Result {
    let Some(queue) = queue(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    let _held = queue.lock();
    // SAFETY: arguments forwarded unchanged, with the queue held.
    unsafe { (queue.raw.fp_v1_0().queue_bind_sparse)(handle, count, binds, fence) }
}

unsafe extern "system" fn queue_wait_idle(handle: vk::Queue) -> vk::Result {
    let Some(queue) = queue(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    let _held = queue.lock();
    // SAFETY: forwarded unchanged, with the queue held.
    unsafe { (queue.raw.fp_v1_0().queue_wait_idle)(handle) }
}

unsafe extern "system" fn device_wait_idle(handle: vk::Device) -> vk::Result {
    let Some(hook) = device(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    // SAFETY: forwarded unchanged, with every queue held.
    hook.with_all_queues(|| unsafe { (hook.raw.fp_v1_0().device_wait_idle)(handle) })
}

/// The context owns the device; IREE never destroys a wrapped device, and
/// must not if that ever changes.
unsafe extern "system" fn destroy_device(_: vk::Device, _: *const vk::AllocationCallbacks<'_>) {}

unsafe extern "system" fn create_buffer(
    handle: vk::Device,
    info: *const vk::BufferCreateInfo<'_>,
    allocator: *const vk::AllocationCallbacks<'_>,
    out: *mut vk::Buffer,
) -> vk::Result {
    let Some(hook) = device(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    let mut shared;
    let mut info = info;
    let exclusive = if info.is_null() {
        false
    } else {
        // SAFETY: Vulkan passes a valid create info.
        (unsafe { (*info).sharing_mode }) == vk::SharingMode::EXCLUSIVE
    };
    // SAFETY: Vulkan passes a valid create info.
    if let Some(families) = &hook.concurrent
        && exclusive
    {
        // SAFETY: as above; the copy keeps the caller's extension chain, and
        // `families` outlives the call.
        shared = unsafe { *info };
        shared.sharing_mode = vk::SharingMode::CONCURRENT;
        shared.queue_family_index_count = families.len() as u32;
        shared.p_queue_family_indices = families.as_ptr();
        info = &shared;
    }
    // SAFETY: arguments forwarded, the create info possibly widened to
    // concurrent sharing between the device's two queue families.
    let result = unsafe { (hook.raw.fp_v1_0().create_buffer)(handle, info, allocator, out) };
    // SAFETY: Vulkan wrote the handle on success.
    if result == vk::Result::SUCCESS && !unsafe { *out }.is_null() {
        hook.created(1);
    }
    result
}

unsafe extern "system" fn destroy_buffer(
    handle: vk::Device,
    buffer: vk::Buffer,
    allocator: *const vk::AllocationCallbacks<'_>,
) {
    let Some(hook) = device(handle) else { return };
    // SAFETY: arguments forwarded unchanged.
    unsafe { (hook.raw.fp_v1_0().destroy_buffer)(handle, buffer, allocator) };
    if !buffer.is_null() {
        hook.destroyed();
    }
}

/// Counted `vkCreate*`/`vkDestroy*` pairs that need no other treatment.
macro_rules! counted {
    ($($create:ident / $destroy:ident: $info:ident => $object:ident;)*) => {$(
        unsafe extern "system" fn $create(
            handle: vk::Device,
            info: *const vk::$info<'_>,
            allocator: *const vk::AllocationCallbacks<'_>,
            out: *mut vk::$object,
        ) -> vk::Result {
            let Some(hook) = device(handle) else {
                return vk::Result::ERROR_DEVICE_LOST;
            };
            // SAFETY: arguments forwarded unchanged.
            let result = unsafe { (hook.raw.fp_v1_0().$create)(handle, info, allocator, out) };
            // SAFETY: Vulkan wrote the handle on success.
            if result == vk::Result::SUCCESS && !unsafe { *out }.is_null() {
                hook.created(1);
            }
            result
        }

        unsafe extern "system" fn $destroy(
            handle: vk::Device,
            object: vk::$object,
            allocator: *const vk::AllocationCallbacks<'_>,
        ) {
            let Some(hook) = device(handle) else { return };
            // SAFETY: arguments forwarded unchanged.
            unsafe { (hook.raw.fp_v1_0().$destroy)(handle, object, allocator) };
            if !object.is_null() {
                hook.destroyed();
            }
        }
    )*};
}

counted! {
    allocate_memory / free_memory: MemoryAllocateInfo => DeviceMemory;
    create_fence / destroy_fence: FenceCreateInfo => Fence;
    create_semaphore / destroy_semaphore: SemaphoreCreateInfo => Semaphore;
    create_command_pool / destroy_command_pool: CommandPoolCreateInfo => CommandPool;
    create_query_pool / destroy_query_pool: QueryPoolCreateInfo => QueryPool;
    create_pipeline_cache / destroy_pipeline_cache: PipelineCacheCreateInfo => PipelineCache;
    create_descriptor_set_layout / destroy_descriptor_set_layout: DescriptorSetLayoutCreateInfo => DescriptorSetLayout;
    create_descriptor_pool / destroy_descriptor_pool: DescriptorPoolCreateInfo => DescriptorPool;
    create_pipeline_layout / destroy_pipeline_layout: PipelineLayoutCreateInfo => PipelineLayout;
    create_shader_module / destroy_shader_module: ShaderModuleCreateInfo => ShaderModule;
}

unsafe extern "system" fn create_compute_pipelines(
    handle: vk::Device,
    cache: vk::PipelineCache,
    count: u32,
    infos: *const vk::ComputePipelineCreateInfo<'_>,
    allocator: *const vk::AllocationCallbacks<'_>,
    out: *mut vk::Pipeline,
) -> vk::Result {
    let Some(hook) = device(handle) else {
        return vk::Result::ERROR_DEVICE_LOST;
    };
    // SAFETY: arguments forwarded unchanged.
    let result = unsafe {
        (hook.raw.fp_v1_0().create_compute_pipelines)(handle, cache, count, infos, allocator, out)
    };
    // Even on failure Vulkan writes every entry: a pipeline, or null for
    // those that failed. Count exactly the pipelines IREE must destroy.
    if !out.is_null() {
        // SAFETY: `out` holds `count` entries.
        let pipelines = unsafe { std::slice::from_raw_parts(out, count as usize) };
        hook.created(
            pipelines
                .iter()
                .filter(|pipeline| !pipeline.is_null())
                .count(),
        );
    }
    result
}

unsafe extern "system" fn destroy_pipeline(
    handle: vk::Device,
    pipeline: vk::Pipeline,
    allocator: *const vk::AllocationCallbacks<'_>,
) {
    let Some(hook) = device(handle) else { return };
    // SAFETY: arguments forwarded unchanged.
    unsafe { (hook.raw.fp_v1_0().destroy_pipeline)(handle, pipeline, allocator) };
    if !pipeline.is_null() {
        hook.destroyed();
    }
}
