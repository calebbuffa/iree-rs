//! HAL VM modules and debug sinks.
use super::{Context, Instance, Module, ModuleState};
use crate::hal::bitfield::bitfield;
use crate::hal::{Allocator, BufferView, DeviceGroup, GroupedDevice, RetainedDevice};
use crate::{Code, Error, Result, check};
use iree_native as n;
use n::slice;
use std::ffi::c_void;

bitfield! {
    /// HAL VM module behavior flags.
    HalModuleFlags: n::iree_hal_module_flags_t {
        /// Forces synchronous queue behavior.
        SYNCHRONOUS = n::IREE_HAL_MODULE_FLAG_SYNCHRONOUS,
        /// Retains command-buffer profiling metadata.
        RETAIN_COMMAND_BUFFER_PROFILE_METADATA =
            n::IREE_HAL_MODULE_FLAG_RETAIN_COMMAND_BUFFER_PROFILE_METADATA,
    }
}

/// How a HAL module selects among its devices.
///
/// The default always selects the first eligible device.
#[derive(Clone, Copy)]
pub struct HalModuleDevicePolicy(n::iree_hal_module_device_policy_t);

impl Default for HalModuleDevicePolicy {
    fn default() -> Self {
        // SAFETY: pure constructor.
        Self(unsafe { n::iree_hal_module_device_policy_default() })
    }
}

impl HalModuleDevicePolicy {
    /// Adopts a native policy.
    ///
    /// # Safety
    /// The callback data must outlive every module using the policy, and the
    /// callback must be thread-safe and must not unwind.
    pub unsafe fn from_raw(raw: n::iree_hal_module_device_policy_t) -> Self {
        Self(raw)
    }
}

type TraceCallback = dyn Fn(&[u8], &[BufferView]) -> Result<()> + Send + Sync;

/// Receives buffer views traced by `hal.buffer_view.trace`.
///
/// A module that accepts the sink releases it; otherwise dropping releases it.
pub struct DebugSink(n::iree_hal_module_debug_sink_t);

impl DebugSink {
    /// Creates a sink calling `trace` with each trace key and its buffer views.
    ///
    /// Keys are bytes and need not be UTF-8. Errors propagate to the VM;
    /// panics become [`Code::Internal`].
    pub fn new<F>(trace: F) -> Self
    where
        F: Fn(&[u8], &[BufferView]) -> Result<()> + Send + Sync + 'static,
    {
        let state: *mut c_void =
            Box::into_raw(Box::new(Box::new(trace) as Box<TraceCallback>)).cast();
        Self(n::iree_hal_module_debug_sink_t {
            release: n::iree_hal_module_debug_sink_release_callback_t {
                fn_: Some(release_trace),
                user_data: state,
            },
            buffer_view_trace: n::iree_hal_module_buffer_view_trace_callback_t {
                fn_: Some(trace_buffer_views),
                user_data: state,
            },
        })
    }

    /// Adopts a native sink.
    ///
    /// # Safety
    /// The callback data must stay live until release, the callbacks must be
    /// thread-safe and must not unwind, and release must accept one call.
    pub unsafe fn from_raw(raw: n::iree_hal_module_debug_sink_t) -> Self {
        Self(raw)
    }

    /// Borrows the native sink.
    pub fn as_raw(&self) -> &n::iree_hal_module_debug_sink_t {
        &self.0
    }

    /// Transfers the release obligation to the caller.
    pub fn into_raw(self) -> n::iree_hal_module_debug_sink_t {
        std::mem::ManuallyDrop::new(self).0
    }
}

impl Default for DebugSink {
    /// A sink that ignores every event.
    fn default() -> Self {
        // SAFETY: pure constructor.
        Self(unsafe { n::iree_hal_module_debug_sink_null() })
    }
}

impl Drop for DebugSink {
    fn drop(&mut self) {
        if let Some(release) = self.0.release.fn_ {
            // SAFETY: this sink still owns its one release.
            unsafe { release(self.0.release.user_data) };
        }
    }
}

unsafe extern "C" fn release_trace(user_data: *mut c_void) {
    // SAFETY: called once with the box created by `DebugSink::new`.
    n::contain(|| drop(unsafe { Box::from_raw(user_data.cast::<Box<TraceCallback>>()) }));
}

unsafe extern "C" fn trace_buffer_views(
    user_data: *mut c_void,
    key: n::iree_string_view_t,
    count: usize,
    views: *mut *mut n::iree_hal_buffer_view_t,
    _allocator: n::iree_allocator_t,
) -> n::iree_status_t {
    n::callback_status(|| {
        // SAFETY: the sink owns the callback until release.
        let trace = unsafe { &*user_data.cast::<Box<TraceCallback>>() };
        // SAFETY: the key and views are borrowed for this call.
        let (key, views) = unsafe { (slice(key.data.cast::<u8>(), key.size), slice(views, count)) };
        let views: Vec<_> = views
            .iter()
            .map(|&raw| {
                // SAFETY: the view is live; retaining gives the wrapper its own reference.
                unsafe {
                    n::iree_hal_buffer_view_retain(raw);
                    BufferView::from_raw(raw)
                }
            })
            .collect();
        trace(key, &views)
    })
}

impl Module {
    /// Creates the HAL module over `devices`.
    pub fn hal(
        instance: &Instance,
        policy: HalModuleDevicePolicy,
        devices: &DeviceGroup,
        flags: HalModuleFlags,
        debug_sink: DebugSink,
    ) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: all handles are live; on success the module retains the group
        // and takes the sink's release.
        check(unsafe {
            n::iree_hal_module_create(
                instance.as_raw(),
                policy.0,
                devices.as_raw(),
                flags.bits(),
                debug_sink.0,
                crate::system(),
                &mut raw,
            )
        })?;
        std::mem::forget(debug_sink);
        // SAFETY: success returns one owned module reference.
        let mut module = unsafe { Self::from_parts(raw, instance.clone()) };
        module.retain_device_group(devices.clone());
        Ok(module)
    }

    /// Creates the inline HAL module, which runs on host memory from `allocator`.
    pub fn hal_inline(
        instance: &Instance,
        allocator: &Allocator,
        debug_sink: DebugSink,
    ) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: all handles are live; on success the module retains the
        // allocator and takes the sink's release.
        check(unsafe {
            n::iree_hal_inline_module_create(
                instance.as_raw(),
                n::IREE_HAL_INLINE_MODULE_FLAG_NONE as _,
                debug_sink.0,
                allocator.as_raw(),
                crate::system(),
                &mut raw,
            )
        })?;
        std::mem::forget(debug_sink);
        // SAFETY: success returns one owned module reference.
        Ok(unsafe { Self::from_parts(raw, instance.clone()) })
    }

    /// Creates the executable loader module used with [`Module::hal_inline`].
    #[cfg(any(feature = "cpu", feature = "local-sync", feature = "local-task"))]
    pub fn hal_loader(
        instance: &Instance,
        loaders: &[crate::hal::backends::local_loader::ExecutableLoader],
    ) -> Result<Self> {
        let mut pointers: Vec<_> = loaders.iter().map(|loader| loader.as_raw()).collect();
        let mut raw = std::ptr::null_mut();
        // SAFETY: the loaders are live during the call and the module retains them.
        check(unsafe {
            n::iree_hal_loader_module_create(
                instance.as_raw(),
                n::IREE_HAL_LOADER_MODULE_FLAG_NONE as _,
                pointers.len(),
                pointers.as_mut_ptr(),
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned module reference.
        Ok(unsafe { Self::from_parts(raw, instance.clone()) })
    }
}

/// State of a context's `hal` module.
pub struct HalModuleState<'a> {
    state: ModuleState<'a>,
    group: Option<&'a DeviceGroup>,
}

impl HalModuleState<'_> {
    /// Returns the number of devices.
    pub fn device_count(&self) -> usize {
        // SAFETY: the state belongs to a live HAL module.
        unsafe { n::iree_hal_module_state_device_count(self.state.as_raw()) }
    }

    /// Returns the device at `index`, or `None` when out of range.
    pub fn device(&self, index: usize) -> Option<RetainedDevice> {
        // SAFETY: the state belongs to a live HAL module.
        let raw = unsafe { n::iree_hal_module_state_device_get(self.state.as_raw(), index) };
        // SAFETY: the module state keeps this borrowed device live.
        (!raw.is_null()).then(|| unsafe { RetainedDevice::from_borrowed(raw) })
    }

    /// Returns a queue-capable handle when this module was created with a device group.
    pub fn grouped_device(&self, index: usize) -> Option<GroupedDevice> {
        self.group?.device(index)
    }
}

impl Context {
    /// Returns the state of a module constructed by [`Module::hal`].
    ///
    /// A module name alone does not establish its native state layout.
    pub fn resolve_hal_module_state<'a>(
        &'a self,
        module: &'a Module,
    ) -> Result<HalModuleState<'a>> {
        // Upstream casts any module state to HAL state without checking.
        if module.device_group().is_none() {
            return Err(Error::new(
                Code::InvalidArgument,
                "module is not the HAL module",
            ));
        }
        Ok(HalModuleState {
            state: self.resolve_module_state(module)?,
            group: module.device_group(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountDrop(Arc<AtomicUsize>);

    impl Drop for CountDrop {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn counted_sink(count: &Arc<AtomicUsize>) -> DebugSink {
        let capture = CountDrop(count.clone());
        DebugSink::new(move |_, _| {
            let _ = &capture;
            Ok(())
        })
    }

    fn trace(sink: &DebugSink, key: &[u8]) -> Result<()> {
        let raw = sink.as_raw();
        // SAFETY: sink owns this callback context and the key bytes live through the synchronous call.
        check(unsafe {
            raw.buffer_view_trace.fn_.unwrap()(
                raw.buffer_view_trace.user_data,
                n::iree_string_view_t {
                    data: key.as_ptr().cast(),
                    size: key.len(),
                },
                0,
                std::ptr::null_mut(),
                crate::system(),
            )
        })
    }

    #[test]
    fn named_native_module_cannot_spoof_hal_state() {
        let instance = Instance::new().unwrap();
        let module = super::super::NativeModule::builder("hal")
            .build(&instance)
            .unwrap();
        let context = Context::new(&instance, super::super::ContextFlags::default()).unwrap();
        let error = context.resolve_hal_module_state(&module).err().unwrap();
        assert_eq!(error.code(), Code::InvalidArgument);
    }

    #[test]
    fn unused_sink_releases_once() {
        let count = Arc::new(AtomicUsize::new(0));
        drop(counted_sink(&count));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn native_release_releases_once() {
        let count = Arc::new(AtomicUsize::new(0));
        let raw = counted_sink(&count).into_raw();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        // SAFETY: into_raw transferred this sink context and release is invoked exactly once.
        unsafe { raw.release.fn_.unwrap()(raw.release.user_data) };
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn errors_keep_their_code() {
        let sink = DebugSink::new(|_, _| Err(Error::new(Code::InvalidArgument, "rejected")));
        assert_eq!(
            trace(&sink, b"key").unwrap_err().code(),
            Code::InvalidArgument
        );
    }

    struct PanicOnDrop;

    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("panic payload drop");
        }
    }

    #[test]
    fn panics_are_contained() {
        let sink = DebugSink::new(|_, _| std::panic::panic_any(PanicOnDrop));
        assert_eq!(trace(&sink, b"key").unwrap_err().code(), Code::Internal);
        let capture = PanicOnDrop;
        drop(DebugSink::new(move |_, _| {
            let _ = &capture;
            Ok(())
        }));
    }

    proptest::proptest! {
        #[test]
        fn keys_are_passed_through(key in proptest::collection::vec(proptest::prelude::any::<u8>(), 0..128)) {
            let expected = key.clone();
            let sink = DebugSink::new(move |actual, views| {
                assert_eq!(actual, expected);
                assert!(views.is_empty());
                Ok(())
            });
            proptest::prop_assert!(trace(&sink, &key).is_ok());
        }
    }
}
