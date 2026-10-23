//! Native Vulkan interoperability with WGPU 30.
//!
//! The bridge creates a device with the union of WGPU and IREE requirements.
//! Raw resource sharing is unsafe because WGPU cannot track external accesses.
//! Explicit idle/barrier handoffs are provided for correctness-first integration;
//! this is not a nonblocking semaphore integration or browser WebGPU support.

use ::wgpu as graphics;
use ash::vk::{self, Handle};
use iree::{Device, Element, Error, Instance, Result, StatusCode, Tensor, sys};
use std::{ptr, sync::Arc};

/// A Vulkan device shared by WGPU and IREE.
pub struct Context {
    device: graphics::Device,
    queue: graphics::Queue,
    iree: Device,
    name: String,
    native: Arc<NativeDevice>,
}

struct NativeDevice {
    raw: ash::Device,
    _instance: graphics::Instance,
}

impl Drop for NativeDevice {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_device(None) };
    }
}

fn vk_error(error: impl std::fmt::Display) -> Error {
    Error::new(
        StatusCode::INVALID_ARGUMENT,
        format!("Vulkan interoperability: {error}"),
    )
}

fn check(status: sys::iree_status_t) -> Result<()> {
    unsafe { Error::from_raw(status) }.map_or(Ok(()), Err)
}

fn tensor_buffer_params() -> sys::iree_hal_buffer_params_t {
    sys::iree_hal_buffer_params_t {
        type_: sys::IREE_HAL_MEMORY_TYPE_DEVICE_LOCAL as _,
        usage: sys::IREE_HAL_BUFFER_USAGE_DEFAULT as _,
        access: sys::IREE_HAL_MEMORY_ACCESS_ALL as _,
        ..Default::default()
    }
}

fn tensor_byte_length<T: Element>(shape: &[usize], data: &[T]) -> Result<usize> {
    let elements = if shape.contains(&0) {
        0
    } else {
        shape.iter().try_fold(1usize, |count, &dimension| {
            count
                .checked_mul(dimension)
                .ok_or_else(|| vk_error("tensor element count overflows"))
        })?
    };
    if elements != data.len() {
        return Err(vk_error("tensor shape does not match the supplied data"));
    }
    elements
        .checked_mul(std::mem::size_of::<T>())
        .filter(|&length| length <= isize::MAX as usize)
        .ok_or_else(|| vk_error("tensor byte size exceeds address space"))
}

impl Context {
    /// Create a native Vulkan 1.3 device with IREE's required features enabled.
    pub async fn new(instance: &Instance) -> Result<Self> {
        let mut descriptor = graphics::InstanceDescriptor::new_without_display_handle();
        descriptor.backends = graphics::Backends::VULKAN;
        let graphics_instance = graphics::Instance::new(descriptor);
        let adapter = graphics_instance
            .request_adapter(&graphics::RequestAdapterOptions {
                power_preference: graphics::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .map_err(vk_error)?;
        let name = adapter.get_info().name;
        let features = graphics::Features::empty();
        let limits = graphics::Limits::default();
        let memory_hints = graphics::MemoryHints::MemoryUsage;

        let (device, queue, native_owner) = unsafe {
            let hal = adapter
                .as_hal::<graphics::hal::api::Vulkan>()
                .ok_or_else(|| vk_error("adapter is not Vulkan"))?;
            let shared = hal.shared_instance();
            let native_instance = shared.raw_instance();
            let physical = hal.raw_physical_device();
            let properties = native_instance.get_physical_device_properties(physical);
            if properties.api_version < vk::API_VERSION_1_3
                || shared.instance_api_version() < vk::API_VERSION_1_3
            {
                return Err(vk_error("this bridge requires Vulkan 1.3"));
            }

            let mut timeline = vk::PhysicalDeviceTimelineSemaphoreFeatures::default();
            let mut synchronization = vk::PhysicalDeviceSynchronization2Features::default();
            let mut scalar = vk::PhysicalDeviceScalarBlockLayoutFeatures::default();
            let mut available = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut timeline)
                .push_next(&mut synchronization)
                .push_next(&mut scalar);
            native_instance.get_physical_device_features2(physical, &mut available);
            if timeline.timeline_semaphore == 0
                || synchronization.synchronization2 == 0
                || scalar.scalar_block_layout == 0
            {
                return Err(vk_error(
                    "timeline semaphores, synchronization2, and scalar block layout are required",
                ));
            }
            let families = native_instance.get_physical_device_queue_family_properties(physical);
            let family = families
                .iter()
                .position(|family| {
                    family.queue_count != 0
                        && family
                            .queue_flags
                            .contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE)
                })
                .ok_or_else(|| vk_error("no combined graphics/compute queue family"))?
                as u32;
            let priorities = [1.0];
            let queues = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(family)
                .queue_priorities(&priorities)];
            let extensions = hal.required_device_extensions(features);
            let extension_names = extensions
                .iter()
                .map(|name| name.as_ptr())
                .collect::<Vec<_>>();
            let mut required = hal.physical_device_features(&extensions, features);
            let mut scalar =
                vk::PhysicalDeviceScalarBlockLayoutFeatures::default().scalar_block_layout(true);
            let mut synchronization =
                vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true);
            let base = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names);
            let mut create = required
                .add_to_device_create(base)
                .push_next(&mut scalar)
                .push_next(&mut synchronization);

            // WGPU normally includes this struct; do not put duplicate feature
            // structures in Vulkan's pNext chain.
            let mut node =
                create.p_next.cast::<vk::BaseOutStructure<'_>>() as *mut vk::BaseOutStructure<'_>;
            let mut found_timeline = false;
            while !node.is_null() {
                if (*node).s_type == vk::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES
                {
                    (*node.cast::<vk::PhysicalDeviceTimelineSemaphoreFeatures<'_>>())
                        .timeline_semaphore = vk::TRUE;
                    found_timeline = true;
                    break;
                }
                node = (*node).p_next;
            }
            let mut timeline =
                vk::PhysicalDeviceTimelineSemaphoreFeatures::default().timeline_semaphore(true);
            if !found_timeline {
                create = create.push_next(&mut timeline);
            }
            let native = native_instance
                .create_device(physical, &create, None)
                .map_err(vk_error)?;
            let native_owner = Arc::new(NativeDevice {
                raw: native.clone(),
                _instance: graphics_instance.clone(),
            });
            let lease = native_owner.clone();
            let open = hal
                .device_from_raw(
                    native,
                    Some(Box::new(move || drop(lease))),
                    &extensions,
                    features,
                    &limits,
                    &memory_hints,
                    family,
                    0,
                )
                .map_err(vk_error)?;
            let descriptor = graphics::DeviceDescriptor {
                label: Some("IREE/WGPU shared Vulkan device"),
                required_features: features,
                required_limits: limits,
                memory_hints,
                ..Default::default()
            };
            let (device, queue) = adapter
                .create_device_from_hal(open, &descriptor)
                .map_err(vk_error)?;
            (device, queue, native_owner)
        };
        let iree = unsafe { wrap_device(instance, &device, native_owner.clone())? };
        Ok(Self {
            device,
            queue,
            iree,
            name,
            native: native_owner,
        })
    }

    /// Return the selected Vulkan adapter's diagnostic name.
    pub fn adapter_name(&self) -> &str {
        &self.name
    }

    /// Borrow the IREE device which shares this context's Vulkan queue.
    pub fn iree_device(&self) -> &Device {
        &self.iree
    }

    /// Upload a dense tensor with a dedicated, zero-offset Vulkan allocation.
    ///
    /// Unlike the runtime's pooled allocations, these buffers can be exported
    /// without exposing unrelated or uninitialized slab storage to WGPU.
    pub fn tensor<T: Element>(&self, shape: &[usize], data: &[T]) -> Result<Tensor> {
        let length = tensor_byte_length(shape, data)?;
        let buffer = self.allocate_shared(length)?;
        check(unsafe {
            sys::iree_hal_device_transfer_h2d(
                self.iree.as_raw(),
                data.as_ptr().cast(),
                buffer.0,
                0,
                length,
                0,
                sys::iree_rs_infinite_timeout(),
            )
        })?;
        let mut view = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_buffer_view_create(
                buffer.0,
                shape.len(),
                shape.as_ptr(),
                T::TYPE,
                sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as sys::iree_hal_encoding_type_t,
                sys::iree_rs_allocator_system(),
                &mut view,
            )
        })?;
        let tensor = unsafe { Tensor::from_raw(self.iree.clone(), view)? };
        Ok(tensor)
    }

    /// Copy a tensor into dedicated exportable storage, entirely on the GPU.
    ///
    /// Use this for ordinary pooled IREE outputs. To avoid this copy, compile
    /// with IREE's destination-passing ABI and pass a tensor from `tensor` as
    /// the output argument.
    pub fn copy_to_shared(&self, tensor: &Tensor) -> Result<Tensor> {
        if tensor.device().as_raw() != self.iree.as_raw() {
            return Err(vk_error("tensor was allocated on a different device"));
        }
        if unsafe { sys::iree_hal_buffer_view_encoding_type(tensor.as_raw()) }
            != sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as sys::iree_hal_encoding_type_t
        {
            return Err(vk_error("sharing requires a dense row-major tensor"));
        }
        let buffer = self.allocate_shared(tensor.byte_len())?;
        tensor.wait()?;
        check(unsafe {
            sys::iree_hal_device_transfer_d2d(
                self.iree.as_raw(),
                sys::iree_hal_buffer_view_buffer(tensor.as_raw()),
                0,
                buffer.0,
                0,
                tensor.byte_len(),
                0,
                sys::iree_rs_infinite_timeout(),
            )
        })?;
        let mut view = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_buffer_view_create_like(
                buffer.0,
                tensor.as_raw(),
                sys::iree_rs_allocator_system(),
                &mut view,
            )
        })?;
        unsafe { Tensor::from_raw(self.iree.clone(), view) }
    }

    fn validate_shared_length(&self, length: usize) -> Result<()> {
        let limits = self.device.limits();
        if length == 0
            || !length.is_multiple_of(4)
            || length as u64 > limits.max_buffer_size
            || length as u64 > limits.max_storage_buffer_binding_size
        {
            return Err(vk_error(
                "shared tensor length must be nonzero, four-byte aligned, and within WGPU buffer limits",
            ));
        }
        Ok(())
    }

    fn allocate_shared(&self, length: usize) -> Result<Buffer> {
        self.validate_shared_length(length)?;
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_rs_vulkan_allocate_dedicated_buffer(
                self.iree.as_raw(),
                tensor_buffer_params(),
                length,
                &mut raw,
            )
        })?;
        if raw.is_null() {
            return Err(vk_error("IREE returned a null shared buffer"));
        }
        Ok(Buffer(raw))
    }

    /// Access graphics handles for externally synchronized use.
    ///
    /// # Safety
    ///
    /// Do not submit WGPU work concurrently with IREE work on this device.
    /// Before switching users, complete work and call `handoff`. All cloned
    /// handles and shared buffers inherit that obligation.
    pub unsafe fn graphics(&self) -> (&graphics::Device, &graphics::Queue) {
        (&self.device, &self.queue)
    }

    /// Complete prior work and establish a device-memory dependency for the next user.
    ///
    /// # Safety
    ///
    /// The caller must exclude concurrent WGPU/IREE submissions and ensure any
    /// host-side IREE calls have returned. This is a blocking correctness path.
    /// An error does not transfer ownership to the other API.
    pub unsafe fn handoff(&self) -> Result<()> {
        check(unsafe { sys::iree_hal_device_queue_flush(self.iree.as_raw(), u64::MAX) })?;
        let hal = unsafe { self.device.as_hal::<graphics::hal::api::Vulkan>() }
            .ok_or_else(|| vk_error("device is not Vulkan"))?;
        let native = hal.raw_device();
        unsafe { native.device_wait_idle() }.map_err(vk_error)?;
        let create = vk::CommandPoolCreateInfo::default()
            .queue_family_index(hal.queue_family_index())
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);
        let pool = unsafe { native.create_command_pool(&create, None) }.map_err(vk_error)?;
        let mut pending = false;
        let result = (|| {
            let allocation = vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let command =
                unsafe { native.allocate_command_buffers(&allocation) }.map_err(vk_error)?[0];
            unsafe {
                native
                    .begin_command_buffer(command, &vk::CommandBufferBeginInfo::default())
                    .map_err(vk_error)?;
                let barriers = [vk::MemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
                    .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)];
                native.cmd_pipeline_barrier(
                    command,
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::DependencyFlags::empty(),
                    &barriers,
                    &[],
                    &[],
                );
                native.end_command_buffer(command).map_err(vk_error)?;
                let commands = [command];
                let submits = [vk::SubmitInfo::default().command_buffers(&commands)];
                native
                    .queue_submit(hal.raw_queue(), &submits, vk::Fence::null())
                    .map_err(vk_error)?;
                pending = true;
                native.queue_wait_idle(hal.raw_queue()).map_err(vk_error)?;
                pending = false;
            }
            Ok(())
        })();
        // A failed wait may leave commands pending. In that error case retain
        // the pool until VkDevice teardown rather than freeing in-use storage.
        if !pending {
            unsafe { native.destroy_command_pool(pool, None) };
        }
        result
    }

    /// Expose an IREE tensor as a WGPU storage/copy buffer without a data copy.
    ///
    /// # Safety
    ///
    /// Exclude concurrent access through the two APIs, and call `handoff` at
    /// ownership transitions. Do not destroy the returned WGPU buffer while
    /// shared accesses are in flight. The lease retains native IREE ownership.
    /// Multiple exports of the same allocation alias without WGPU tracking:
    /// exclude conflicting accesses through those handles as well.
    pub unsafe fn export_tensor(&self, tensor: &Tensor) -> Result<SharedBuffer> {
        if tensor.device().as_raw() != self.iree.as_raw() {
            return Err(vk_error("tensor was allocated on a different device"));
        }
        let length = tensor.byte_len();
        self.validate_shared_length(length)?;
        if unsafe { sys::iree_hal_buffer_view_encoding_type(tensor.as_raw()) }
            != sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as sys::iree_hal_encoding_type_t
        {
            return Err(vk_error("sharing requires a dense row-major tensor"));
        }
        let buffer = unsafe { sys::iree_hal_buffer_view_buffer(tensor.as_raw()) };
        let mut backing = ptr::null_mut();
        check(unsafe { sys::iree_hal_vulkan_buffer_resolve_backing(buffer, &mut backing) })?;
        let mut offset = 0;
        check(unsafe {
            sys::iree_hal_vulkan_buffer_resolve_backing_offset(buffer, backing, 0, &mut offset)
        })?;
        if offset != 0 {
            return Err(vk_error(
                "pooled/nonzero-offset tensor: use copy_to_shared or a destination-passing output",
            ));
        }
        let size = unsafe { sys::iree_rs_buffer_allocation_size(backing) };
        if length > size {
            return Err(vk_error("tensor exceeds its backing allocation"));
        }
        let mut memory: sys::VkDeviceMemory = unsafe { std::mem::zeroed() };
        let mut raw_buffer: sys::VkBuffer = unsafe { std::mem::zeroed() };
        check(unsafe {
            sys::iree_hal_vulkan_buffer_handle(backing, &mut memory, &mut raw_buffer)
        })?;
        if raw_buffer as u64 == 0 {
            return Err(vk_error("tensor has no Vulkan buffer handle"));
        }
        // The callback can run on a WGPU cleanup thread. Only native atomic
        // retain/release operations cross threads, not the Rust Rc wrappers.
        let lease = NativeLease::new(tensor, self.native.clone());
        let hal_buffer = unsafe {
            graphics::hal::vulkan::Buffer::from_raw_externally_owned(
                vk::Buffer::from_raw(raw_buffer as u64),
                Box::new(move || drop(lease)),
            )
        };
        let descriptor = graphics::BufferDescriptor {
            label: Some("IREE shared allocation"),
            size: length as u64,
            usage: graphics::BufferUsages::STORAGE
                | graphics::BufferUsages::COPY_SRC
                | graphics::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        let buffer = unsafe {
            self.device
                .create_buffer_from_hal::<graphics::hal::api::Vulkan>(hal_buffer, &descriptor)
        };
        Ok(SharedBuffer {
            buffer,
            offset: offset as u64,
            length: length as u64,
        })
    }
}

struct Buffer(*mut sys::iree_hal_buffer_t);
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { sys::iree_hal_buffer_release(self.0) };
    }
}

/// A bounded tensor range within an externally owned Vulkan buffer.
pub struct SharedBuffer {
    /// Externally owned WGPU buffer whose cleanup lease retains native storage.
    pub buffer: graphics::Buffer,
    /// Byte offset of the exported tensor within the WGPU buffer.
    pub offset: u64,
    /// Initialized tensor bytes exposed by this export.
    pub length: u64,
}

impl SharedBuffer {
    /// Create a storage-buffer binding restricted to the exported tensor range.
    pub fn binding(&self) -> graphics::BufferBinding<'_> {
        graphics::BufferBinding {
            buffer: &self.buffer,
            offset: self.offset,
            size: std::num::NonZeroU64::new(self.length),
        }
    }
}

struct NativeLease {
    view: usize,
    device: usize,
    hal_module: usize,
    _native: Arc<NativeDevice>,
}

impl NativeLease {
    fn new(tensor: &Tensor, native: Arc<NativeDevice>) -> Self {
        unsafe {
            sys::iree_hal_buffer_view_retain(tensor.as_raw());
            sys::iree_hal_device_retain(tensor.device().as_raw());
            sys::iree_vm_module_retain(tensor.device().as_raw_hal_module());
        }
        Self {
            view: tensor.as_raw() as usize,
            device: tensor.device().as_raw() as usize,
            hal_module: tensor.device().as_raw_hal_module() as usize,
            _native: native,
        }
    }
}

impl Drop for NativeLease {
    fn drop(&mut self) {
        unsafe {
            sys::iree_hal_buffer_view_release(self.view as *mut _);
            sys::iree_vm_module_release(self.hal_module as *mut _);
            sys::iree_hal_device_release(self.device as *mut _);
        }
    }
}

unsafe fn wrap_device(
    instance: &Instance,
    device: &graphics::Device,
    native: Arc<NativeDevice>,
) -> Result<Device> {
    let hal = unsafe { device.as_hal::<graphics::hal::api::Vulkan>() }
        .ok_or_else(|| vk_error("device is not Vulkan"))?;
    let shared = hal.shared_instance();
    let mut symbols = ptr::null_mut();
    check(unsafe {
        sys::iree_hal_vulkan_syms_create_from_system_loader(
            sys::iree_rs_allocator_system(),
            &mut symbols,
        )
    })?;
    let mut options = unsafe { std::mem::zeroed() };
    unsafe { sys::iree_hal_vulkan_device_options_initialize(&mut options) };
    options.dispatch_abis =
        sys::IREE_HAL_VULKAN_DISPATCH_ABI_DESCRIPTOR as sys::iree_hal_vulkan_dispatch_abis_t;
    let names = hal
        .enabled_device_extensions()
        .iter()
        .map(|name| name.as_ptr())
        .collect::<Vec<_>>();
    let mut enabled_extensions = 0;
    let extension_status = unsafe {
        sys::iree_hal_vulkan_device_extensions_from_names(
            names.len(),
            names.as_ptr(),
            &mut enabled_extensions,
        )
    };
    if let Err(error) = check(extension_status) {
        unsafe { sys::iree_hal_vulkan_syms_release(symbols) };
        return Err(error);
    }
    let mut external: sys::iree_hal_vulkan_external_device_params_t = unsafe { std::mem::zeroed() };
    external.enabled_features =
        sys::IREE_HAL_VULKAN_FEATURE_REQUIRED_BASELINE as sys::iree_hal_vulkan_features_t;
    external.enabled_extensions = enabled_extensions;
    external.compute_queue_set.queue_family_index = hal.queue_family_index();
    external.compute_queue_set.queue_indices = 1u64 << hal.queue_index();
    let mut pool = ptr::null_mut();
    let pool_status = unsafe {
        sys::iree_rs_proactor_pool_create_default(sys::iree_rs_allocator_system(), &mut pool)
    };
    if let Err(error) = check(pool_status) {
        unsafe { sys::iree_hal_vulkan_syms_release(symbols) };
        return Err(error);
    }
    let mut create = unsafe { sys::iree_rs_device_create_params_default() };
    create.proactor_pool = pool;
    let mut raw = ptr::null_mut();
    let status = unsafe {
        sys::iree_hal_vulkan_wrap_device(
            sys::iree_string_view_t {
                data: c"vulkan".as_ptr(),
                size: 6,
            },
            &options,
            &create,
            symbols,
            shared.raw_instance().handle().as_raw() as *mut _,
            hal.raw_physical_device().as_raw() as *mut _,
            hal.raw_device().handle().as_raw() as *mut _,
            &external,
            sys::iree_rs_allocator_system(),
            &mut raw,
        )
    };
    unsafe {
        sys::iree_async_proactor_pool_release(pool);
        sys::iree_hal_vulkan_syms_release(symbols);
    }
    check(status)?;
    unsafe { Device::from_raw_with_owner(instance.clone(), raw, native) }
}
