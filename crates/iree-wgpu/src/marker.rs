//! The timeline semaphore shared by WGPU and IREE, and its completion watcher.

use crate::device::NativeDevice;
use crate::error::invalid;
use crate::hooks;
use ash::vk;
use ireert::Code;
use ireert::Result;
use ireert::hal::Semaphore;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread::JoinHandle;

/// The private Vulkan timeline marking WGPU queue progress.
///
/// With [`QueueTopology::Separate`](crate::QueueTopology::Separate) the WGPU
/// queue signals it and a watcher thread forwards values to `handoff`, an IREE
/// semaphore IREE work can wait on. With
/// [`QueueTopology::Shared`](crate::QueueTopology::Shared) a raw submission of
/// a full memory barrier signals it.
pub(crate) struct Marker {
    pub(crate) handoff: Semaphore,
    pub(crate) native: Arc<NativeDevice>,
    pub(crate) queue: Arc<hooks::QueueHook>,
    pub(crate) pool: vk::CommandPool,
    pub(crate) command: vk::CommandBuffer,
    pub(crate) semaphore: vk::Semaphore,
    /// Cleared when teardown could not prove the native objects idle.
    pub(crate) retire: bool,
}

/// Forwards the private marker timeline to the IREE `handoff` semaphore.
///
/// IREE's Vulkan queue resolves a wait natively only for values its own queues
/// produced; other waits are deferred until the host signals the semaphore.
/// The thread blocks on the native timeline so application threads never do.
pub(crate) struct Watcher {
    pub(crate) values: mpsc::Sender<u64>,
    pub(crate) thread: JoinHandle<()>,
}

impl Watcher {
    pub(crate) fn spawn(
        native: Arc<NativeDevice>,
        semaphore: vk::Semaphore,
        handoff: Semaphore,
    ) -> Result<Self> {
        let (values, pending) = mpsc::channel::<u64>();
        let thread = std::thread::Builder::new()
            .name("iree-wgpu-handoff".into())
            .spawn(move || {
                while let Ok(mut value) = pending.recv() {
                    // Timeline values are monotonic; only the newest matters.
                    while let Ok(newer) = pending.try_recv() {
                        value = newer;
                    }
                    let waited = native.wait(semaphore, value);
                    if let Err(error) = waited.and_then(|()| handoff.signal(value)) {
                        eprintln!("iree-wgpu: WGPU to IREE forwarding failed: {error}");
                        // Failing the semaphore releases IREE waiters with an error.
                        handoff.fail(Code::Internal);
                        return;
                    }
                }
            })
            .map_err(invalid)?;
        Ok(Self { values, thread })
    }

    /// Forward every published value, then stop the thread.
    pub(crate) fn stop(self) {
        drop(self.values);
        if self.thread.join().is_err() {
            eprintln!("iree-wgpu: handoff watcher panicked");
        }
    }
}

impl Marker {
    /// Create the marker's command pool, barrier command and timeline.
    ///
    /// # Safety
    ///
    /// `family` must be the family of `queue`, a queue of `native`'s device.
    pub(crate) unsafe fn new(
        native: Arc<NativeDevice>,
        queue: Arc<hooks::QueueHook>,
        family: u32,
        handoff: Semaphore,
    ) -> Result<Self> {
        let raw = &native.raw;
        // SAFETY: `raw` is a live device and the create info is complete.
        let pool = unsafe {
            raw.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(family)
                    .flags(vk::CommandPoolCreateFlags::TRANSIENT),
                None,
            )
        }
        .map_err(invalid)?;
        let result = (|| {
            // SAFETY: `pool` was created above on this device.
            let command = unsafe {
                raw.allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::default()
                        .command_pool(pool)
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )
            }
            .map_err(invalid)?[0];
            // SAFETY: `command` is a fresh primary buffer, recorded once here
            // and never reset; it is externally synchronized by construction.
            unsafe {
                raw.begin_command_buffer(
                    command,
                    &vk::CommandBufferBeginInfo::default()
                        .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE),
                )
                .map_err(invalid)?;
                let barriers = [vk::MemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
                    .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)];
                raw.cmd_pipeline_barrier(
                    command,
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::DependencyFlags::empty(),
                    &barriers,
                    &[],
                    &[],
                );
                raw.end_command_buffer(command).map_err(invalid)?;
            }
            let mut timeline =
                vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE);
            // SAFETY: the chained create info outlives the call.
            let semaphore = unsafe {
                raw.create_semaphore(
                    &vk::SemaphoreCreateInfo::default().push_next(&mut timeline),
                    None,
                )
            }
            .map_err(invalid)?;
            Ok((command, semaphore))
        })();
        match result {
            Ok((command, semaphore)) => Ok(Self {
                handoff,
                native,
                queue,
                pool,
                command,
                semaphore,
                retire: true,
            }),
            Err(error) => {
                // SAFETY: nothing from `pool` was submitted; destroying it frees
                // its command buffer.
                unsafe { raw.destroy_command_pool(pool, None) };
                Err(error)
            }
        }
    }

    pub(crate) fn wait(&self, value: u64) -> Result<()> {
        self.native.wait(self.semaphore, value)
    }

    /// Submit the full barrier on the shared queue, signalling `value`.
    ///
    /// Only called with [`QueueTopology::Shared`](crate::QueueTopology::Shared),
    /// under the context lock.
    pub(crate) fn barrier(&self, value: u64) -> Result<()> {
        let commands = [self.command];
        let semaphores = [self.semaphore];
        let values = [value];
        let mut timeline =
            vk::TimelineSemaphoreSubmitInfo::default().signal_semaphore_values(&values);
        let submissions = [vk::SubmitInfo::default()
            .command_buffers(&commands)
            .signal_semaphores(&semaphores)
            .push_next(&mut timeline)];
        let _held = self.queue.lock();
        // SAFETY: the queue is externally synchronized by `_held`; the
        // command is SIMULTANEOUS_USE and the submit info outlives the call.
        unsafe {
            self.native
                .raw
                .queue_submit(self.queue.handle(), &submissions, vk::Fence::null())
        }
        .map_err(invalid)
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        if !self.retire {
            eprintln!("iree-wgpu: teardown could not drain the device; leaking it");
            // Unretired work may still reference the device and these objects.
            self.native.leak.store(true, Ordering::Relaxed);
            return;
        }
        // SAFETY: `retire` proves every submission using these objects completed.
        unsafe {
            self.native.raw.destroy_command_pool(self.pool, None);
            self.native.raw.destroy_semaphore(self.semaphore, None);
        }
    }
}
