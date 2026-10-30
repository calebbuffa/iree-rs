//! Vulkan device creation and the IREE HAL device wrapping it.

use crate::context::Context;
use crate::context::ContextOptions;
use crate::context::Inner;
use crate::context::QueueTopology;
use crate::error::invalid;
use crate::hooks;
use crate::marker::Marker;
use crate::marker::Watcher;
use crate::ordering::State;
use crate::ordering::device_waitable;
use ash::vk::{self, Handle};
use ireert::Result;
use ireert::asynchronous::FrontierTracker;
use ireert::asynchronous::ProactorPool;
use ireert::hal;
use ireert::hal::DeviceCreateParams;
use ireert::hal::DeviceGroup;
use ireert::hal::QueueAffinity;
use ireert::hal::Semaphore;
use ireert::hal::backends::vulkan;
use ireert::hal::backends::vulkan::DeviceExtensions;
use ireert::hal::backends::vulkan::DeviceOptions;
use ireert::hal::backends::vulkan::DispatchAbis;
use ireert::hal::backends::vulkan::ExternalDevice;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

/// The native device and the instance it was created from.
///
/// IREE objects such as [`BufferView`](ireert::hal::BufferView)s and
/// [`List`](ireert::vm::List)s returned by [`Program::call`](crate::Program::call)
/// may outlive the context, and IREE buffers do not retain the IREE device, so no reference count covers every user of the
/// `VkDevice`. Instead the IREE function table counts IREE's live Vulkan
/// objects ([`hooks`]): once this handle (held by the context, WGPU's device
/// and shared-buffer leases) is released and IREE's last object is
/// destroyed, the `VkDevice` is destroyed, then the instance released.
pub(crate) struct NativeDevice {
    pub(crate) raw: ash::Device,
    pub(crate) hook: Arc<hooks::DeviceHook>,
    pub(crate) instance: Option<wgpu::Instance>,
    /// Set when teardown could not prove the device idle: leak it instead.
    pub(crate) leak: AtomicBool,
}

impl Drop for NativeDevice {
    fn drop(&mut self) {
        let instance = self.instance.take();
        if *self.leak.get_mut() {
            std::mem::forget(instance);
            return;
        }
        let raw = self.raw.clone();
        self.hook.release(Box::new(move || {
            // SAFETY: the hooks run this once, after the last WGPU and IREE
            // user of the device released it.
            unsafe {
                // Every queue user is gone; this only retires completed work.
                let _ = raw.device_wait_idle();
                raw.destroy_device(None);
            }
            drop(instance);
        }));
    }
}

impl NativeDevice {
    pub(crate) fn wait(&self, semaphore: vk::Semaphore, value: u64) -> Result<()> {
        let semaphores = [semaphore];
        let values = [value];
        // SAFETY: `semaphore` belongs to this live device; the wait info
        // outlives the call.
        unsafe {
            self.raw.wait_semaphores(
                &vk::SemaphoreWaitInfo::default()
                    .semaphores(&semaphores)
                    .values(&values),
                u64::MAX,
            )
        }
        .map_err(invalid)
    }
}

impl Context {
    /// Create a headless context with the requested adapter-selection options.
    ///
    /// Use [`Self::with_window`] to also present to a window.
    ///
    /// # Errors
    ///
    /// Returns an error when no matching Vulkan adapter is available, the
    /// adapter lacks Vulkan 1.3, timeline semaphores, synchronization2 or
    /// scalar block layout, or device creation fails.
    pub async fn new(options: ContextOptions) -> Result<Self> {
        Self::create(instance(), None, options).await
    }

    pub(crate) async fn create(
        graphics_instance: wgpu::Instance,
        surface: Option<&wgpu::Surface<'_>>,
        options: ContextOptions,
    ) -> Result<Self> {
        let adapter = graphics_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preference,
                force_fallback_adapter: options.force_fallback_adapter,
                compatible_surface: surface,
                ..Default::default()
            })
            .await
            .map_err(invalid)?;
        let name = adapter.get_info().name;
        let features = wgpu::Features::empty();
        let limits = wgpu::Limits::default();
        let memory_hints = wgpu::MemoryHints::MemoryUsage;

        // SAFETY: every Vulkan call below uses handles WGPU's live adapter
        // provides, and every pNext chain outlives the call using it. The new
        // device is handed to WGPU with a drop callback releasing
        // `native_owner`, so neither API destroys it while the other uses it.
        let (device, queue, native_owner, topology, family, iree_family, iree_index) = unsafe {
            let hal = adapter
                .as_hal::<wgpu::hal::api::Vulkan>()
                .ok_or_else(|| invalid("adapter is not Vulkan"))?;
            let shared = hal.shared_instance();
            let native_instance = shared.raw_instance();
            let physical = hal.raw_physical_device();
            let properties = native_instance.get_physical_device_properties(physical);
            if properties.api_version < vk::API_VERSION_1_3
                || shared.instance_api_version() < vk::API_VERSION_1_3
            {
                return Err(invalid("this bridge requires Vulkan 1.3"));
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
                return Err(invalid(
                    "timeline semaphores, synchronization2, and scalar block layout are required",
                ));
            }
            let families = native_instance.get_physical_device_queue_family_properties(physical);
            let combined = |family: &vk::QueueFamilyProperties| {
                family.queue_count != 0
                    && family
                        .queue_flags
                        .contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE)
            };
            // WGPU 30's native surface capability query uses family zero, so
            // WGPU takes family zero whenever it can.
            let family = if families.first().is_some_and(combined) {
                0
            } else if surface.is_some() {
                return Err(invalid(
                    "WGPU 30 presentation requires the combined queue in family zero",
                ));
            } else {
                families
                    .iter()
                    .position(combined)
                    .ok_or_else(|| invalid("no combined graphics/compute queue family"))?
                    as u32
            };
            // IREE takes a second queue of WGPU's family, or else a queue of
            // another compute-capable family (preferring a compute-only one);
            // the hooks give IREE buffers concurrent sharing between the two
            // families, so neither API transfers queue-family ownership.
            let other = families
                .iter()
                .enumerate()
                .filter(|(index, candidate)| {
                    *index as u32 != family
                        && candidate.queue_count != 0
                        && candidate.queue_flags.contains(vk::QueueFlags::COMPUTE)
                })
                .min_by_key(|(_, candidate)| {
                    candidate.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                })
                .map(|(index, _)| index as u32);
            let (topology, iree_family, iree_index) = if families[family as usize].queue_count >= 2
            {
                (QueueTopology::Separate, family, 1)
            } else if let Some(other) = other {
                (QueueTopology::Separate, other, 0)
            } else {
                (QueueTopology::Shared, family, 0)
            };
            let one: &[f32] = &[1.0];
            let two: &[f32] = &[1.0, 1.0];
            let mut queues = vec![
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(family)
                    .queue_priorities(if iree_family == family && iree_index == 1 {
                        two
                    } else {
                        one
                    }),
            ];
            if iree_family != family {
                queues.push(
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(iree_family)
                        .queue_priorities(one),
                );
            }
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
                .map_err(invalid)?;
            let concurrent = (iree_family != family).then_some([family, iree_family]);
            let hook = hooks::register(
                native.clone(),
                native_instance.fp_v1_0().get_device_proc_addr,
                concurrent,
            );
            let native_owner = Arc::new(NativeDevice {
                raw: native.clone(),
                hook,
                instance: Some(graphics_instance.clone()),
                leak: AtomicBool::new(false),
            });
            let lease = native_owner.clone();
            // With a drop callback WGPU does not destroy the device itself.
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
                .map_err(invalid)?;
            let descriptor = wgpu::DeviceDescriptor {
                label: Some("IREE/WGPU shared Vulkan device"),
                required_features: features,
                required_limits: limits,
                memory_hints,
                ..Default::default()
            };
            let (device, queue) = adapter
                .create_device_from_hal(open, &descriptor)
                .map_err(invalid)?;
            (
                device,
                queue,
                native_owner,
                topology,
                family,
                iree_family,
                iree_index,
            )
        };
        let pool = ProactorPool::shared()?;
        let wgpu_queue = {
            // SAFETY: only the raw queue handle is read.
            let hal = unsafe { device.as_hal::<wgpu::hal::api::Vulkan>() }
                .ok_or_else(|| invalid("device is not Vulkan"))?;
            native_owner.hook.queue(hal.raw_queue())
        };
        let iree = {
            // The group retains the native tracker; no bridges are registered,
            // so the Rust handle (which is `!Send`) is not kept.
            let tracker = FrontierTracker::new(16, 0, 0)?;
            DeviceGroup::from_device(
                // SAFETY: `device` was created above with `iree_family`'s queue
                // `iree_index`, and `native_owner` keeps it alive.
                unsafe { wrap_device(&device, &pool, iree_family, iree_index)? },
                &tracker,
            )?
            .device(0)
            .ok_or_else(|| invalid("new device group contains no device"))?
        };
        let work = Semaphore::new(&iree, QueueAffinity::ANY, 0)?;
        let work_vk = match topology {
            // SAFETY: `iree` wraps the Vulkan device created above, and `work`
            // was just created on it. Vulkan's create_semaphore callback
            // constructs a native Vulkan semaphore, not a generic host semaphore.
            QueueTopology::Separate => unsafe { device_waitable(&iree, &work)? },
            QueueTopology::Shared => None,
        };
        let handoff = Semaphore::new(&iree, QueueAffinity::ANY, 0)?;
        // SAFETY: `wgpu_queue` is queue 0 of `family` on `native_owner`.
        let marker = unsafe {
            Marker::new(
                native_owner.clone(),
                wgpu_queue.clone(),
                family,
                handoff.clone(),
            )?
        };
        let watcher = match topology {
            QueueTopology::Separate => Some(Watcher::spawn(
                native_owner.clone(),
                marker.semaphore,
                handoff,
            )?),
            QueueTopology::Shared => None,
        };
        let state = State {
            wgpu_dirty: false,
            iree_dirty: false,
            marker: 0,
            work,
            work_vk,
            launched: 0,
            poisoned: None,
            watcher,
        };
        Ok(Self {
            inner: Arc::new(Inner {
                marker,
                state: Mutex::new(state),
                iree,
                queue: ManuallyDrop::new(queue),
                wgpu_queue,
                device,
                adapter,
                instance: graphics_instance,
                name,
                topology,
                family,
                native: native_owner,
                _pool: pool,
            }),
        })
    }
}

pub(crate) fn instance() -> wgpu::Instance {
    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    descriptor.backends = wgpu::Backends::VULKAN;
    wgpu::Instance::new(descriptor)
}

/// Wrap WGPU's `VkDevice` as an IREE HAL device using queue `queue_index` of
/// `family`, with IREE's Vulkan calls routed through [`hooks`].
///
/// # Safety
///
/// `device` must have been created with that queue, and its `VkDevice`
/// registered with [`hooks::register`].
pub(crate) unsafe fn wrap_device(
    device: &wgpu::Device,
    pool: &ProactorPool,
    family: u32,
    queue_index: u32,
) -> Result<hal::Device> {
    // SAFETY: only handles and the extension list are read.
    let hal = unsafe { device.as_hal::<wgpu::hal::api::Vulkan>() }
        .ok_or_else(|| invalid("device is not Vulkan"))?;
    let shared = hal.shared_instance();
    // IREE resolves every device function through this loader, so its queue
    // calls, device destruction and object lifetimes go through the hooks.
    let loader =
        hooks::proc_address(shared.entry().static_fn().get_instance_proc_addr).map_err(invalid)?;
    // SAFETY: `loader` resolves through the live instance's loader.
    let syms = unsafe { vulkan::Syms::from_proc_address(loader)? };
    let names = hal
        .enabled_device_extensions()
        .iter()
        .map(|name| name.to_str().map_err(invalid))
        .collect::<Result<Vec<_>>>()?;
    let external = ExternalDevice {
        enabled_features: vulkan::FeatureFlags::REQUIRED_BASELINE,
        enabled_extensions: DeviceExtensions::from_names(&names)?,
        request_flags: Default::default(),
        compute_queues: vulkan::QueueSet {
            family_index: family,
            queue_indices: 1u64 << queue_index,
        },
        transfer_queues: None,
        sparse_binding_queues: None,
    };
    let options = DeviceOptions {
        dispatch_abis: Some(DispatchAbis::DESCRIPTOR),
        ..Default::default()
    };
    // SAFETY: the handles are WGPU's live instance, physical device and
    // device, which `NativeDevice` keeps alive while IREE objects exist; the
    // caller guarantees the queue selection.
    unsafe {
        vulkan::wrap_device(vulkan::WrapDeviceParams {
            identifier: "vulkan",
            options,
            syms: &syms,
            instance: shared.raw_instance().handle().as_raw() as _,
            physical_device: hal.raw_physical_device().as_raw() as _,
            device: hal.raw_device().handle().as_raw() as _,
            external: &external,
            create_params: DeviceCreateParams::new(pool),
        })
    }
}
