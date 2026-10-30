//! Window surfaces presented through the context's WGPU queue.

use crate::context::Context;
use crate::context::ContextOptions;
use crate::device::instance;
use crate::error::invalid;
use ireert::Result;
use std::mem::ManuallyDrop;

impl Context {
    /// Create a context that can present to `window`, and its surface.
    ///
    /// Selects an adapter according to `options` that can present to the
    /// window. `window` is typically an `Arc` of a windowing-library window,
    /// such as `Arc<winit::window::Window>`.
    ///
    /// # Errors
    ///
    /// As [`Context::new`], and also when the surface cannot be created or
    /// no adapter can present to it. WGPU 30 queries presentation support on
    /// queue family zero only, so the adapter's combined graphics/compute
    /// family must be family zero.
    pub async fn with_window(
        window: impl Into<wgpu::SurfaceTarget<'static>>,
        options: ContextOptions,
    ) -> Result<(Self, Surface)> {
        let instance = instance();
        let surface = instance.create_surface(window).map_err(invalid)?;
        let context = Self::create(instance, Some(&surface), options).await?;
        let surface = Surface {
            surface: ManuallyDrop::new(surface),
            context: context.clone(),
        };
        Ok((context, surface))
    }

    /// Create another presentation surface on this context's device, for
    /// example to replace a lost surface or to render to a second window.
    ///
    /// # Errors
    ///
    /// Returns an error when the surface cannot be created or this context's
    /// adapter and queue cannot present to it.
    pub fn create_surface(
        &self,
        window: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> Result<Surface> {
        let inner = &self.inner;
        let surface = inner.instance.create_surface(window).map_err(invalid)?;
        if inner.family != 0 || !inner.adapter.is_surface_supported(&surface) {
            return Err(invalid(
                "this context's adapter and queue family cannot present to the window",
            ));
        }
        Ok(Surface {
            surface: ManuallyDrop::new(surface),
            context: self.clone(),
        })
    }

    /// Present a frame acquired with [`Surface::current_texture`].
    ///
    /// Like [`Context::submit`], presentation is ordered after all previously
    /// submitted work of both APIs.
    ///
    /// # Errors
    ///
    /// Returns an error only for a re-entrant call or a poisoned context; see
    /// [`Context::submit`].
    pub fn present(&self, frame: wgpu::SurfaceTexture) -> Result<()> {
        let _state = self.inner.lock()?;
        let _held = self.inner.wgpu_queue.lock();
        self.inner.queue.present(frame);
        Ok(())
    }
}

/// A presentation surface on a [`Context`]'s device.
///
/// Acquire frames with [`Self::current_texture`] and present them with
/// [`Context::present`]. Reconfiguring and dropping a surface wait for the
/// device to idle, so the context first completes outstanding IREE work.
pub struct Surface {
    pub(crate) surface: ManuallyDrop<wgpu::Surface<'static>>,
    pub(crate) context: Context,
}

impl std::fmt::Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Surface").finish_non_exhaustive()
    }
}

impl Surface {
    fn raw(&self) -> &wgpu::Surface<'static> {
        &self.surface
    }

    /// The surface's capabilities on this context's adapter.
    pub fn capabilities(&self) -> wgpu::SurfaceCapabilities {
        self.raw().get_capabilities(&self.context.inner.adapter)
    }

    /// A default configuration for a `width` by `height` surface, if supported.
    pub fn default_config(&self, width: u32, height: u32) -> Option<wgpu::SurfaceConfiguration> {
        self.raw()
            .get_default_config(&self.context.inner.adapter, width, height)
    }

    /// Configure (or reconfigure) the surface, for example after a resize.
    ///
    /// Reconfiguring waits for the device to idle: it completes all work
    /// submitted to either API first.
    ///
    /// # Errors
    ///
    /// Returns an error when outstanding work fails, for a re-entrant call or
    /// for a poisoned context. Invalid configurations are reported by WGPU's
    /// error handling, as for [`wgpu::Surface::configure`].
    pub fn configure(&self, config: &wgpu::SurfaceConfiguration) -> Result<()> {
        let inner = &self.context.inner;
        let mut state = inner.lock()?;
        // Reconfiguration calls vkDeviceWaitIdle. Completing both APIs' work
        // first means no queued submission can be waiting on later work, and
        // holding every queue's lock keeps IREE's deferred submissions out.
        inner.quiesce(&mut state)?;
        inner
            .native
            .hook
            .with_all_queues(|| self.raw().configure(&inner.device, config));
        Ok(())
    }

    /// Acquire the next frame, as [`wgpu::Surface::get_current_texture`].
    ///
    /// Present it with [`Context::present`].
    pub fn current_texture(&self) -> wgpu::CurrentSurfaceTexture {
        self.raw().get_current_texture()
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        // SAFETY: taken exactly once, here; `self.surface` is not used again.
        let surface = unsafe { ManuallyDrop::take(&mut self.surface) };
        let inner = &self.context.inner;
        let Some(mut state) = inner.lock_for_drop() else {
            eprintln!(
                "iree-wgpu: leaking a surface dropped from a WGPU callback during a submission"
            );
            std::mem::forget(surface);
            return;
        };
        // Unconfiguring also waits for the device to idle.
        match inner.quiesce(&mut state) {
            Ok(()) => inner.native.hook.with_all_queues(|| drop(surface)),
            Err(error) => {
                eprintln!("iree-wgpu: leaking a surface that cannot be safely released: {error}");
                std::mem::forget(surface);
            }
        }
    }
}
