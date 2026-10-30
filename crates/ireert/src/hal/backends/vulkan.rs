//! Vulkan driver configuration, external-device wrapping and handle interop.
//!
//! Unset options keep the native defaults. Vulkan handles use the native
//! opaque handle types and stay owned by the application or the HAL object
//! they were queried from.
use super::super::bitfield::bitfield;
use crate::hal::{Buffer, Device, DeviceCreateParams, Driver, Semaphore};
use crate::{Code, Error, Result, check, system, view};
use iree_native as n;
use std::ffi::{CStr, CString, c_void};

bitfield! {
    /// Vulkan features requested from or enabled on a logical device.
    FeatureFlags: n::iree_hal_vulkan_features_t {
        /// Robust buffer access.
        ROBUST_BUFFER_ACCESS = n::IREE_HAL_VULKAN_FEATURE_ENABLE_ROBUST_BUFFER_ACCESS,
        /// Sparse binding.
        SPARSE_BINDING = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SPARSE_BINDING,
        /// Sparse residency with aliased mappings.
        SPARSE_RESIDENCY_ALIASED = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SPARSE_RESIDENCY_ALIASED,
        /// Buffer device addresses.
        BUFFER_DEVICE_ADDRESSES = n::IREE_HAL_VULKAN_FEATURE_ENABLE_BUFFER_DEVICE_ADDRESSES,
        /// Timeline semaphores.
        TIMELINE_SEMAPHORES = n::IREE_HAL_VULKAN_FEATURE_ENABLE_TIMELINE_SEMAPHORES,
        /// Synchronization2.
        SYNCHRONIZATION2 = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SYNCHRONIZATION2,
        /// Scalar block layout.
        SCALAR_BLOCK_LAYOUT = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SCALAR_BLOCK_LAYOUT,
        /// Subgroup size control.
        SUBGROUP_SIZE_CONTROL = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SUBGROUP_SIZE_CONTROL,
        /// Cooperative matrices.
        COOPERATIVE_MATRIX = n::IREE_HAL_VULKAN_FEATURE_ENABLE_COOPERATIVE_MATRIX,
        /// 8-bit storage-buffer access.
        STORAGE_BUFFER_8BIT_ACCESS = n::IREE_HAL_VULKAN_FEATURE_ENABLE_STORAGE_BUFFER_8BIT_ACCESS,
        /// Shader float16.
        SHADER_FLOAT16 = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_FLOAT16,
        /// Shader float64.
        SHADER_FLOAT64 = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_FLOAT64,
        /// Shader int8.
        SHADER_INT8 = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INT8,
        /// Shader int16.
        SHADER_INT16 = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INT16,
        /// Shader int64.
        SHADER_INT64 = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INT64,
        /// Shader integer dot product.
        SHADER_INTEGER_DOT_PRODUCT = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INTEGER_DOT_PRODUCT,
        /// Vulkan memory model.
        VULKAN_MEMORY_MODEL = n::IREE_HAL_VULKAN_FEATURE_ENABLE_VULKAN_MEMORY_MODEL,
        /// Vulkan memory model device scope.
        VULKAN_MEMORY_MODEL_DEVICE_SCOPE = n::IREE_HAL_VULKAN_FEATURE_ENABLE_VULKAN_MEMORY_MODEL_DEVICE_SCOPE,
        /// 16-bit storage-buffer access.
        STORAGE_BUFFER_16BIT_ACCESS = n::IREE_HAL_VULKAN_FEATURE_ENABLE_STORAGE_BUFFER_16BIT_ACCESS,
        /// Shader bfloat16 type.
        SHADER_BFLOAT16_TYPE = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_BFLOAT16_TYPE,
        /// Shader bfloat16 dot product.
        SHADER_BFLOAT16_DOT_PRODUCT = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_BFLOAT16_DOT_PRODUCT,
        /// Shader bfloat16 cooperative matrices.
        SHADER_BFLOAT16_COOPERATIVE_MATRIX = n::IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_BFLOAT16_COOPERATIVE_MATRIX,
        /// Features every IREE Vulkan device requires.
        REQUIRED_BASELINE = n::IREE_HAL_VULKAN_FEATURE_REQUIRED_BASELINE,
    }
}

bitfield! {
    /// Non-feature behavior requested from Vulkan.
    RequestFlags: n::iree_hal_vulkan_request_flags_t {
        /// Enable validation layers.
        VALIDATION_LAYERS = n::IREE_HAL_VULKAN_REQUEST_FLAG_VALIDATION_LAYERS,
        /// Enable debug-utils labels and names.
        DEBUG_UTILS = n::IREE_HAL_VULKAN_REQUEST_FLAG_DEBUG_UTILS,
        /// Include Vulkan events in tracing.
        TRACING = n::IREE_HAL_VULKAN_REQUEST_FLAG_TRACING,
    }
}

bitfield! {
    /// Executable dispatch ABIs a device may use.
    DispatchAbis: n::iree_hal_vulkan_dispatch_abis_t {
        /// Descriptor-set dispatch.
        DESCRIPTOR = n::IREE_HAL_VULKAN_DISPATCH_ABI_DESCRIPTOR,
        /// Buffer-device-address dispatch.
        BDA = n::IREE_HAL_VULKAN_DISPATCH_ABI_BDA,
        /// Every recognized ABI.
        ALL = n::IREE_HAL_VULKAN_DISPATCH_ABI_ALL_RECOGNIZED,
    }
}

bitfield! {
    /// Device extensions IREE recognizes.
    DeviceExtensions: n::iree_hal_vulkan_device_extensions_t {
        /// `VK_KHR_portability_subset`.
        PORTABILITY_SUBSET = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_PORTABILITY_SUBSET,
        /// `VK_KHR_external_memory`.
        EXTERNAL_MEMORY = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_EXTERNAL_MEMORY,
        /// `VK_KHR_external_memory_fd`.
        EXTERNAL_MEMORY_FD = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_EXTERNAL_MEMORY_FD,
        /// `VK_KHR_external_memory_win32`.
        EXTERNAL_MEMORY_WIN32 = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_EXTERNAL_MEMORY_WIN32,
        /// `VK_EXT_external_memory_host`.
        EXTERNAL_MEMORY_HOST = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_EXT_EXTERNAL_MEMORY_HOST,
        /// `VK_EXT_calibrated_timestamps`.
        CALIBRATED_TIMESTAMPS = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_EXT_CALIBRATED_TIMESTAMPS,
        /// `VK_KHR_push_descriptor`.
        PUSH_DESCRIPTOR = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_PUSH_DESCRIPTOR,
        /// `VK_KHR_cooperative_matrix`.
        COOPERATIVE_MATRIX = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_COOPERATIVE_MATRIX,
        /// `VK_KHR_shader_bfloat16`.
        SHADER_BFLOAT16 = n::IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_SHADER_BFLOAT16,
    }
}

impl DeviceExtensions {
    /// Collects the recognized bits from enabled extension names; unknown names are ignored.
    pub fn from_names(names: &[&str]) -> Result<Self> {
        let names = names
            .iter()
            .map(|name| {
                CString::new(*name)
                    .map_err(|_| Error::new(Code::InvalidArgument, "extension name contains NUL"))
            })
            .collect::<Result<Vec<_>>>()?;
        let pointers: Vec<_> = names.iter().map(|name| name.as_ptr()).collect();
        let mut bits = 0;
        // SAFETY: every name is NUL-terminated and outlives the call.
        check(unsafe {
            n::iree_hal_vulkan_device_extensions_from_names(
                pointers.len(),
                pointers.as_ptr(),
                &mut bits,
            )
        })?;
        Ok(Self::from_bits_retain(bits))
    }
}

/// A layer or extension name set IREE needs for a request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExtensibilitySet {
    /// Required instance layers.
    RequiredInstanceLayers,
    /// Optional instance layers.
    OptionalInstanceLayers,
    /// Required instance extensions.
    RequiredInstanceExtensions,
    /// Optional instance extensions.
    OptionalInstanceExtensions,
    /// Required device extensions.
    RequiredDeviceExtensions,
    /// Optional device extensions.
    OptionalDeviceExtensions,
}

/// Returns the layer or extension names applications must (or may) enable
/// on an external instance or device for `request`.
pub fn extensibility_set(
    request: RequestFlags,
    set: ExtensibilitySet,
) -> Result<Vec<&'static CStr>> {
    let set = match set {
        ExtensibilitySet::RequiredInstanceLayers => {
            n::IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_LAYERS_REQUIRED
        }
        ExtensibilitySet::OptionalInstanceLayers => {
            n::IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_LAYERS_OPTIONAL
        }
        ExtensibilitySet::RequiredInstanceExtensions => {
            n::IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_EXTENSIONS_REQUIRED
        }
        ExtensibilitySet::OptionalInstanceExtensions => {
            n::IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_EXTENSIONS_OPTIONAL
        }
        ExtensibilitySet::RequiredDeviceExtensions => {
            n::IREE_HAL_VULKAN_EXTENSIBILITY_DEVICE_EXTENSIONS_REQUIRED
        }
        ExtensibilitySet::OptionalDeviceExtensions => {
            n::IREE_HAL_VULKAN_EXTENSIBILITY_DEVICE_EXTENSIONS_OPTIONAL
        }
    };
    let mut count = 0;
    // SAFETY: zero capacity with null storage is the count-only query.
    check(unsafe {
        n::iree_hal_vulkan_query_extensibility_set(
            request.bits(),
            set,
            0,
            &mut count,
            core::ptr::null_mut(),
        )
    })?;
    let mut values = vec![core::ptr::null(); count];
    // SAFETY: `values` has `count` writable slots.
    check(unsafe {
        n::iree_hal_vulkan_query_extensibility_set(
            request.bits(),
            set,
            values.len(),
            &mut count,
            values.as_mut_ptr(),
        )
    })?;
    // SAFETY: upstream returns static NUL-terminated names.
    Ok(values
        .into_iter()
        .take(count)
        .map(|value| unsafe { CStr::from_ptr(value) })
        .collect())
}

/// Vulkan logical-device options; unset fields keep native defaults.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceOptions {
    /// Prefer a dedicated compute queue.
    pub dedicated_compute_queue: bool,
    /// Dispatch ABIs to enable when the device supports them.
    pub dispatch_abis: Option<DispatchAbis>,
    /// Cached BDA replay instances per queue lane; zero disables the cache.
    pub max_cached_bda_replay_instances: Option<u32>,
    /// Host-visible BDA publication bytes cached per queue lane.
    pub max_cached_bda_replay_publication_bytes: Option<u64>,
    /// Idle cached BDA replay instances kept per command buffer by trim.
    pub retained_cached_bda_replay_instances: Option<u32>,
}
impl DeviceOptions {
    fn raw(self) -> n::iree_hal_vulkan_device_options_t {
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw: n::iree_hal_vulkan_device_options_t = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_vulkan_device_options_initialize(&mut raw) };
        if self.dedicated_compute_queue {
            raw.flags |= n::IREE_HAL_VULKAN_DEVICE_FLAG_DEDICATED_COMPUTE_QUEUE as u32;
        }
        if let Some(value) = self.dispatch_abis {
            raw.dispatch_abis = value.bits();
        }
        if let Some(value) = self.max_cached_bda_replay_instances {
            raw.max_cached_bda_replay_instances = value;
        }
        if let Some(value) = self.max_cached_bda_replay_publication_bytes {
            raw.max_cached_bda_replay_publication_bytes = value;
        }
        if let Some(value) = self.retained_cached_bda_replay_instances {
            raw.retained_cached_bda_replay_instances = value;
        }
        raw
    }
}

/// Vulkan driver options; unset fields keep native defaults.
#[derive(Clone, Debug, Default)]
pub struct DriverOptions {
    /// Directories or files searched for the Vulkan loader library.
    pub library_search_paths: Vec<String>,
    /// Vulkan API version requested for driver-created instances.
    pub api_version: Option<u32>,
    /// Behavior requested for driver-created instances.
    pub request_flags: Option<RequestFlags>,
    /// Features requested for driver-created logical devices.
    pub requested_features: Option<FeatureFlags>,
    /// Debug output cutoff: 0 none, 1 errors, 2 warnings, 3 info, 4 debug.
    pub debug_verbosity: Option<i32>,
    /// Options for driver-created devices.
    pub device: DeviceOptions,
}
impl DriverOptions {
    fn with_raw<T>(&self, f: impl FnOnce(&n::iree_hal_vulkan_driver_options_t) -> T) -> T {
        let paths: Vec<_> = self
            .library_search_paths
            .iter()
            .map(|path| view(path))
            .collect();
        // SAFETY: plain-data storage that the native initializer fills.
        let mut raw: n::iree_hal_vulkan_driver_options_t = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_vulkan_driver_options_initialize(&mut raw) };
        raw.libvulkan_search_paths = n::iree_string_view_list_t {
            count: paths.len(),
            values: paths.as_ptr(),
        };
        if let Some(value) = self.api_version {
            raw.api_version = value;
        }
        if let Some(value) = self.request_flags {
            raw.request_flags = value.bits();
        }
        if let Some(value) = self.requested_features {
            raw.requested_features = value.bits();
        }
        if let Some(value) = self.debug_verbosity {
            raw.debug_verbosity = value;
        }
        raw.device_options = self.device.raw();
        f(&raw)
    }
}

/// Creates a Vulkan driver that owns its instance, using the system loader.
pub fn create_driver(identifier: &str, options: &DriverOptions) -> Result<Driver> {
    let mut raw = core::ptr::null_mut();
    options.with_raw(|options| {
        // SAFETY: options and search paths outlive the call; upstream clones them.
        check(unsafe {
            n::iree_hal_vulkan_driver_create(
                view(identifier),
                options,
                core::ptr::null_mut(),
                system(),
                &mut raw,
            )
        })
    })?;
    // SAFETY: successful creation returns one owned driver reference.
    Ok(unsafe { Driver::from_raw(raw) })
}

/// Creates a Vulkan driver over an application-owned instance.
///
/// # Safety
/// `instance` must stay live while the driver uses it, and `syms` must
/// resolve functions for that instance.
pub unsafe fn create_driver_using_instance(
    identifier: &str,
    options: &DriverOptions,
    syms: &Syms,
    instance: n::VkInstance,
) -> Result<Driver> {
    let mut raw = core::ptr::null_mut();
    options.with_raw(|options| {
        // SAFETY: the caller guarantees `instance`; options outlive the call.
        check(unsafe {
            n::iree_hal_vulkan_driver_create_using_instance(
                view(identifier),
                options,
                syms.0,
                instance,
                system(),
                &mut raw,
            )
        })
    })?;
    // SAFETY: successful creation returns one owned driver reference.
    Ok(unsafe { Driver::from_raw(raw) })
}

/// A retained Vulkan function table.
pub struct Syms(*mut n::iree_hal_vulkan_syms_t);
impl Syms {
    /// Creates a table from an application-resolved `vkGetInstanceProcAddr`.
    ///
    /// # Safety
    /// `get_instance_proc_addr` must be a valid `vkGetInstanceProcAddr` that
    /// outlives this table and everything created from it.
    pub unsafe fn from_proc_address(get_instance_proc_addr: *mut c_void) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the caller guarantees the function pointer.
        check(unsafe {
            n::iree_hal_vulkan_syms_create(get_instance_proc_addr, system(), &mut raw)
        })?;
        Ok(Self(raw))
    }

    /// Loads the system Vulkan loader.
    pub fn from_system_loader() -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: `raw` is writable.
        check(unsafe { n::iree_hal_vulkan_syms_create_from_system_loader(system(), &mut raw) })?;
        Ok(Self(raw))
    }

    /// Borrows the native table pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_vulkan_syms_t {
        self.0
    }
}
impl Clone for Syms {
    fn clone(&self) -> Self {
        // SAFETY: the live table is retained before creating another owner.
        unsafe { n::iree_hal_vulkan_syms_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for Syms {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one table reference.
        unsafe { n::iree_hal_vulkan_syms_release(self.0) };
    }
}

/// A queue family and the queue indices within it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueueSet {
    /// Queue family index.
    pub family_index: u32,
    /// Bitset of queue indices within the family.
    pub queue_indices: u64,
}
impl QueueSet {
    fn raw(self) -> n::iree_hal_vulkan_queue_set_t {
        n::iree_hal_vulkan_queue_set_t {
            queue_family_index: self.family_index,
            queue_indices: self.queue_indices,
        }
    }
}

/// What an application-created `VkDevice` offers IREE.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExternalDevice {
    /// Features enabled on the device; must include [`FeatureFlags::REQUIRED_BASELINE`].
    pub enabled_features: FeatureFlags,
    /// Recognized extensions enabled on the device.
    pub enabled_extensions: DeviceExtensions,
    /// Behavior IREE may use with the instance and device.
    pub request_flags: RequestFlags,
    /// Compute queues available to IREE.
    pub compute_queues: QueueSet,
    /// Transfer queues; `None` reuses the compute queue.
    pub transfer_queues: Option<QueueSet>,
    /// Sparse-binding queues; `None` reuses a compute or transfer queue that supports it.
    pub sparse_binding_queues: Option<QueueSet>,
}

/// Inputs for wrapping an application-created Vulkan device as a HAL device.
pub struct WrapDeviceParams<'args, 'pool> {
    /// Driver identifier.
    pub identifier: &'args str,
    /// HAL-specific device options.
    pub options: DeviceOptions,
    /// Retained Vulkan function table for the instance.
    pub syms: &'args Syms,
    /// Application-owned Vulkan instance.
    pub instance: n::VkInstance,
    /// Physical device selected from `instance`.
    pub physical_device: n::VkPhysicalDevice,
    /// Application-owned logical device.
    pub device: n::VkDevice,
    /// Features, extensions, and queues enabled on `device`.
    pub external: &'args ExternalDevice,
    /// Shared HAL device-creation parameters.
    pub create_params: DeviceCreateParams<'pool>,
}

/// Wraps an application-created `VkDevice` as a HAL device.
///
/// # Safety
/// The handles in `params` must be related live handles that outlive the
/// returned device and everything created from it. `params.external` must
/// accurately describe the logical device.
pub unsafe fn wrap_device(params: WrapDeviceParams<'_, '_>) -> Result<Device> {
    let options = params.options.raw();
    let external = n::iree_hal_vulkan_external_device_params_t {
        enabled_features: params.external.enabled_features.bits(),
        enabled_extensions: params.external.enabled_extensions.bits(),
        request_flags: params.external.request_flags.bits(),
        compute_queue_set: params.external.compute_queues.raw(),
        transfer_queue_set: params.external.transfer_queues.unwrap_or_default().raw(),
        sparse_binding_queue_set: params
            .external
            .sparse_binding_queues
            .unwrap_or_default()
            .raw(),
    };
    let mut raw = core::ptr::null_mut();
    // SAFETY: the caller guarantees the handles; all parameters outlive the call.
    check(unsafe {
        n::iree_hal_vulkan_wrap_device(
            view(params.identifier),
            &options,
            params.create_params.as_raw(),
            params.syms.0,
            params.instance,
            params.physical_device,
            params.device,
            &external,
            system(),
            &mut raw,
        )
    })?;
    // SAFETY: successful creation returns one owned device reference.
    Ok(unsafe { Device::from_raw(raw) })
}

/// Vulkan memory and buffer handles kept valid by a retained HAL buffer.
pub struct BufferHandles {
    /// Device memory backing the buffer.
    pub memory: n::VkDeviceMemory,
    /// The Vulkan buffer.
    pub buffer: n::VkBuffer,
    _owner: Buffer,
}

/// Returns the Vulkan handles behind an allocated HAL buffer.
pub fn buffer_handles(buffer: &Buffer) -> Result<BufferHandles> {
    let mut memory = core::ptr::null_mut();
    let mut handle = core::ptr::null_mut();
    // SAFETY: `buffer` is live and both outputs are writable.
    check(unsafe {
        n::iree_hal_vulkan_allocated_buffer_handle(buffer.as_raw(), &mut memory, &mut handle)
    })?;
    Ok(BufferHandles {
        memory,
        buffer: handle,
        _owner: buffer.clone(),
    })
}

/// A Vulkan semaphore handle kept valid by a retained HAL semaphore.
pub struct SemaphoreHandle {
    /// The Vulkan semaphore.
    pub handle: n::VkSemaphore,
    _owner: Semaphore,
}

/// Returns the Vulkan semaphore behind a HAL semaphore.
///
/// # Safety
/// `semaphore` must have been created by the Vulkan HAL backend. Native
/// handle extraction does not validate the backend of a generic semaphore.
pub unsafe fn semaphore_handle(semaphore: &Semaphore) -> Result<SemaphoreHandle> {
    let mut handle = core::ptr::null_mut();
    // SAFETY: the caller guarantees Vulkan provenance; the output is writable.
    check(unsafe { n::iree_hal_vulkan_semaphore_handle(semaphore.as_raw(), &mut handle) })?;
    Ok(SemaphoreHandle {
        handle,
        _owner: semaphore.clone(),
    })
}

/// One cooperative-matrix shape and type combination a device supports.
///
/// Type fields are `VkComponentTypeKHR` values; `scope` is a `VkScopeKHR`.
pub type CooperativeMatrixProperty = n::iree_hal_vulkan_cooperative_matrix_property_t;

/// Returns the cooperative-matrix properties of a Vulkan HAL device; empty
/// when the device has no cooperative-matrix support.
pub fn cooperative_matrix_properties(device: &Device) -> Result<Vec<CooperativeMatrixProperty>> {
    let mut count = 0;
    // SAFETY: zero capacity with null storage is the count-only query.
    let query = check(unsafe {
        n::iree_hal_vulkan_device_query_cooperative_matrix_properties(
            device.as_raw(),
            0,
            &mut count,
            core::ptr::null_mut(),
        )
    });
    match query {
        Ok(()) => {}
        Err(error) if error.code() == Code::OutOfRange => {}
        Err(error) => return Err(error),
    }
    let mut rows = vec![CooperativeMatrixProperty::default(); count];
    // SAFETY: `rows` has `count` writable slots.
    check(unsafe {
        n::iree_hal_vulkan_device_query_cooperative_matrix_properties(
            device.as_raw(),
            rows.len(),
            &mut count,
            rows.as_mut_ptr(),
        )
    })?;
    rows.truncate(count);
    Ok(rows)
}
