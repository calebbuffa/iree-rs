//! The shared WGPU/IREE [`Context`] and its queue topology.

use crate::device::NativeDevice;
use crate::hooks;
use crate::marker::Marker;
use crate::ordering::State;
use ireert::Result;
use ireert::asynchronous::ProactorPool;
use ireert::hal::GroupedDevice;
use ireert::hal::WaitFlags;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

/// How WGPU and IREE submit to the shared Vulkan device.
///
/// The topology changes performance, not the API or its guarantees.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueTopology {
    /// WGPU and IREE own distinct queues, of WGPU's graphics/compute family
    /// when it has two, or else of another compute-capable family.
    ///
    /// Cross-API ordering uses timeline semaphores; IREE calls return once
    /// their work is enqueued, and WGPU submissions wait for it on the GPU.
    Separate,
    /// The device exposes one compute-capable queue, which both APIs submit to.
    ///
    /// The context completes each IREE operation before it releases the queue,
    /// so IREE calls block until their work finishes.
    Shared,
}

/// Portable options for selecting the Vulkan adapter used by a context.
///
/// The default prefers a high-performance adapter and accepts hardware or
/// software implementations.
#[derive(Clone, Copy, Debug)]
pub struct ContextOptions {
    /// The adapter power/performance tradeoff to request.
    pub power_preference: wgpu::PowerPreference,
    /// Whether to require a software/fallback adapter.
    pub force_fallback_adapter: bool,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }
    }
}

/// A Vulkan device shared by WGPU and IREE, and the only way to submit to it.
///
/// Cloning is cheap and shares the same device and ordering. The context is
/// `Send + Sync`; submissions from several threads are serialized.
///
/// Create one with [`Context::new`] for headless use, or with
/// [`Context::with_window`] to also present to a window.
#[derive(Clone)]
pub struct Context {
    pub(crate) inner: Arc<Inner>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("adapter", &self.inner.name)
            .field("topology", &self.inner.topology)
            .finish_non_exhaustive()
    }
}

pub(crate) struct Inner {
    // Field order is drop order; `Drop for Inner` first drains both APIs and
    // drops `queue` while holding its lock.
    pub(crate) marker: Marker,
    pub(crate) state: Mutex<State>,
    pub(crate) iree: GroupedDevice,
    pub(crate) queue: ManuallyDrop<wgpu::Queue>,
    /// The lock IREE also takes for WGPU's `VkQueue`.
    pub(crate) wgpu_queue: Arc<hooks::QueueHook>,
    pub(crate) device: wgpu::Device,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) instance: wgpu::Instance,
    pub(crate) name: String,
    pub(crate) topology: QueueTopology,
    pub(crate) family: u32,
    pub(crate) native: Arc<NativeDevice>,
    pub(crate) _pool: ProactorPool,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let state = self.state.get_mut().unwrap_or_else(PoisonError::into_inner);
        if state.launched > 0 {
            let _ = state.work.wait(state.launched, None, WaitFlags::default());
        }
        if let Some(watcher) = state.watcher.take() {
            watcher.stop();
        }
        self.marker.retire = self.marker.wait(state.marker).is_ok();
        // Retire WGPU's own submissions (and the semaphores they wait on).
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
        // Dropping the queue waits for it to idle, a queue access IREE's
        // submissions must not overlap.
        let _held = self.wgpu_queue.lock();
        // SAFETY: `queue` is dropped exactly once, here, and never used after.
        unsafe { ManuallyDrop::drop(&mut self.queue) };
    }
}

impl Context {
    /// The selected Vulkan adapter's diagnostic name.
    pub fn adapter_name(&self) -> &str {
        &self.inner.name
    }

    /// Whether WGPU and IREE own separate queues or share one.
    pub fn queue_topology(&self) -> QueueTopology {
        self.inner.topology
    }

    /// The selected adapter.
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.inner.adapter
    }

    /// The WGPU device, for creating resources, pipelines and command encoders.
    ///
    /// Submit through the context: [`Self::submit`], [`Self::write_buffer`],
    /// [`Self::write_texture`] and [`Self::present`].
    pub fn device(&self) -> &wgpu::Device {
        &self.inner.device
    }

    /// Submit WGPU command buffers after all previously submitted IREE work.
    ///
    /// Never blocks on the host with [`QueueTopology::Separate`]: the
    /// submission waits for outstanding IREE work on the GPU.
    ///
    /// WGPU runs some callbacks (such as buffer-map callbacks) during a
    /// submission; calling back into this context from one returns an error
    /// rather than deadlocking.
    ///
    /// # Errors
    ///
    /// Returns [`Code::FailedPrecondition`](ireert::Code::FailedPrecondition) for a re-entrant call (for example
    /// from a WGPU callback run during a submission), and [`Code::Aborted`](ireert::Code::Aborted)
    /// when the context can no longer order work.
    pub fn submit<I>(&self, commands: I) -> Result<wgpu::SubmissionIndex>
    where
        I: IntoIterator<Item = wgpu::CommandBuffer>,
    {
        // Collected first: the iterator is user code and must not run under
        // the queue lock.
        let commands = commands.into_iter().collect();
        let mut state = self.inner.lock()?;
        self.inner.wgpu_submit(&mut state, commands)
    }

    /// Schedule a buffer write, as [`wgpu::Queue::write_buffer`].
    ///
    /// Like WGPU, the write executes at the next submission, which this
    /// context orders after previously submitted IREE work. An IREE call
    /// observes it.
    ///
    /// # Errors
    ///
    /// Returns [`Code::FailedPrecondition`](ireert::Code::FailedPrecondition) for a re-entrant call (for example
    /// from a WGPU callback run during a submission), and [`Code::Aborted`](ireert::Code::Aborted)
    /// when the context can no longer order work.
    pub fn write_buffer(
        &self,
        buffer: &wgpu::Buffer,
        offset: wgpu::BufferAddress,
        data: &[u8],
    ) -> Result<()> {
        let mut state = self.inner.lock()?;
        self.inner.queue.write_buffer(buffer, offset, data);
        state.wgpu_dirty = true;
        Ok(())
    }

    /// Schedule a texture write, as [`wgpu::Queue::write_texture`].
    ///
    /// Ordered like [`Self::write_buffer`].
    ///
    /// # Errors
    ///
    /// Returns [`Code::FailedPrecondition`](ireert::Code::FailedPrecondition) for a re-entrant call (for example
    /// from a WGPU callback run during a submission), and [`Code::Aborted`](ireert::Code::Aborted)
    /// when the context can no longer order work.
    pub fn write_texture(
        &self,
        texture: wgpu::TexelCopyTextureInfo<'_>,
        data: &[u8],
        layout: wgpu::TexelCopyBufferLayout,
        size: wgpu::Extent3d,
    ) -> Result<()> {
        let mut state = self.inner.lock()?;
        self.inner.queue.write_texture(texture, data, layout, size);
        state.wgpu_dirty = true;
        Ok(())
    }

    /// Run `callback` once all WGPU work submitted so far completes.
    ///
    /// As with WGPU, callbacks run during [`wgpu::Device::poll`].
    pub fn on_submitted_work_done(&self, callback: impl FnOnce() + Send + 'static) {
        self.inner.queue.on_submitted_work_done(callback);
    }

    /// Complete all submitted work of both APIs, blocking the calling thread.
    ///
    /// # Errors
    ///
    /// Returns the error of failed IREE work, and as [`Self::submit`].
    pub fn finish(&self) -> Result<()> {
        let mut state = self.inner.lock()?;
        self.inner.quiesce(&mut state)
    }

    /// The WGPU queue, bypassing the context's ordering.
    ///
    /// # Safety
    ///
    /// Work submitted directly is not ordered against IREE work, and bypasses
    /// the lock that keeps IREE's submissions off the Vulkan queue while WGPU
    /// submits. Order it yourself, for example with [`Self::finish`] before
    /// and after, never touch buffers IREE may be using, and do not submit
    /// while any IREE work may still be enqueued.
    pub unsafe fn wgpu_queue(&self) -> &wgpu::Queue {
        &self.inner.queue
    }

    /// The IREE device, bypassing the context's ordering.
    ///
    /// # Safety
    ///
    /// Same contract as [`Self::wgpu_queue`]; with [`QueueTopology::Shared`],
    /// IREE work (including its deferred submissions) must also complete
    /// before the context submits again.
    pub unsafe fn iree_device(&self) -> &GroupedDevice {
        &self.inner.iree
    }
}
