//! Executables, executable parameters, and executable caches.

use super::bitfield::scalar_value;
use super::{
    ExecutableCachingMode, ExecutableFunctionFlags, QueueAffinity, buffer::Buffer, device::Device,
};
use crate::{Code, Result, check};
use core::marker::PhantomData;
use iree_native as n;
use iree_native::{const_span, view_string};

scalar_value! {
    /// The kind of a reflected executable parameter.
    ParameterType: n::iree_hal_executable_function_parameter_type_t {
        /// A packed constant value.
        CONSTANT = n::IREE_HAL_EXECUTABLE_FUNCTION_PARAMETER_TYPE_CONSTANT,
        /// A buffer binding.
        BINDING = n::IREE_HAL_EXECUTABLE_FUNCTION_PARAMETER_TYPE_BINDING,
        /// A buffer pointer stored in constants.
        BUFFER_PTR = n::IREE_HAL_EXECUTABLE_FUNCTION_PARAMETER_TYPE_BUFFER_PTR,
    }
}

/// Reflection metadata for an executable function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutableFunctionInfo {
    /// Function name, when the executable carries one.
    pub name: Option<String>,
    /// Function behavior flags.
    pub flags: ExecutableFunctionFlags,
    /// Number of 32-bit constants.
    pub constant_count: u16,
    /// Number of buffer bindings.
    pub binding_count: u16,
    /// Number of logical parameters.
    pub parameter_count: u16,
    /// Static or minimum workgroup dimensions.
    pub workgroup_size: [u32; 3],
}

/// Reflection metadata for one executable parameter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutableFunctionParameter {
    /// Parameter representation.
    pub kind: ParameterType,
    /// Parameter byte size.
    pub size: u8,
    /// Parameter name, when the executable carries one.
    pub name: Option<String>,
    /// Byte offset or buffer-binding ordinal.
    pub offset: u16,
}

/// Copies a possibly empty native name.
///
/// # Safety
/// `name` must describe readable bytes.
unsafe fn name(name: n::iree_string_view_t) -> Option<String> {
    // SAFETY: guaranteed by the caller.
    (name.size != 0).then(|| unsafe { view_string(name) })
}

/// An owned `iree_hal_executable_t` reference.
pub struct Executable {
    raw: *mut n::iree_hal_executable_t,
}
impl Executable {
    /// Adopts one owned executable reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_executable_t) -> Self {
        Self { raw }
    }

    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_executable_t {
        self.raw
    }

    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_executable_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Returns the number of dispatchable functions.
    pub fn function_count(&self) -> usize {
        // SAFETY: the executable is live.
        unsafe { n::iree_hal_executable_function_count(self.raw) }
    }

    /// Returns reflected metadata for a function.
    pub fn function_info(&self, function: ExecutableFunction) -> Result<ExecutableFunctionInfo> {
        let mut info = n::iree_hal_executable_function_info_t::default();
        // SAFETY: the executable is live and `info` is writable.
        check(unsafe {
            n::iree_hal_executable_function_info(self.raw, function.raw(), &mut info)
        })?;
        Ok(ExecutableFunctionInfo {
            // SAFETY: the name views executable-owned metadata.
            name: unsafe { name(info.name) },
            flags: ExecutableFunctionFlags::from_bits_retain(info.flags),
            constant_count: info.constant_count,
            binding_count: info.binding_count,
            parameter_count: info.parameter_count,
            workgroup_size: info.workgroup_size,
        })
    }

    /// Returns reflected metadata for every parameter of a function.
    pub fn function_parameters(
        &self,
        function: ExecutableFunction,
    ) -> Result<Vec<ExecutableFunctionParameter>> {
        let count = usize::from(self.function_info(function)?.parameter_count);
        let mut native = vec![n::iree_hal_executable_function_parameter_t::default(); count];
        // SAFETY: `native` is writable for `count` elements.
        check(unsafe {
            n::iree_hal_executable_function_parameters(
                self.raw,
                function.raw(),
                count,
                native.as_mut_ptr(),
            )
        })?;
        Ok(native
            .into_iter()
            .map(|p| ExecutableFunctionParameter {
                kind: ParameterType::from_bits_retain(p.type_),
                size: p.size,
                // SAFETY: the name views executable-owned metadata.
                name: unsafe { name(p.name) },
                offset: p.offset,
            })
            .collect())
    }

    /// Looks up a function by name.
    pub fn lookup_function(&self, name: &str) -> Result<ExecutableFunction> {
        let mut out = n::iree_hal_executable_function_t::default();
        // SAFETY: the executable and name live through the call; `out` is writable.
        check(unsafe {
            n::iree_hal_executable_lookup_function_by_name(self.raw, crate::view(name), &mut out)
        })?;
        Ok(ExecutableFunction(out.value))
    }

    /// Looks up an executable global and returns a buffer aliasing its storage.
    pub fn lookup_global(&self, name: &str, queue_affinity: QueueAffinity) -> Result<Buffer> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the executable and name live through the call; `raw` is writable.
        check(unsafe {
            n::iree_hal_executable_lookup_global_by_name(
                self.raw,
                crate::view(name),
                queue_affinity.bits(),
                &mut raw,
            )
        })?;
        // SAFETY: successful lookup returns an owned buffer reference.
        Ok(unsafe { Buffer::from_raw(raw) })
    }
}
impl Clone for Executable {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_executable_retain(self.raw) };
        Self { raw: self.raw }
    }
}
impl Drop for Executable {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_executable_release(self.raw) }
    }
}

/// Executable preparation parameters borrowing their input bytes and constants.
pub struct ExecutableParams<'a> {
    raw: n::iree_hal_executable_params_t,
    _constants: PhantomData<&'a [u32]>,
}
impl<'a> ExecutableParams<'a> {
    /// Describes executable bytes in `format` with optional specialization constants.
    pub fn new(
        queue_affinity: QueueAffinity,
        caching_mode: ExecutableCachingMode,
        format: &'a str,
        data: &'a [u8],
        constants: &'a [u32],
    ) -> Self {
        // SAFETY: the params struct is plain data and is fully initialized by IREE.
        let mut raw = unsafe { core::mem::zeroed() };
        // SAFETY: `raw` is writable.
        unsafe { n::iree_hal_executable_params_initialize(&mut raw) };
        raw.queue_affinity = queue_affinity.bits();
        raw.caching_mode = caching_mode.bits();
        raw.executable_format = crate::view(format);
        raw.executable_data = const_span(data);
        raw.constant_count = constants.len();
        raw.constants = constants.as_ptr();
        Self {
            raw,
            _constants: PhantomData,
        }
    }
}

/// An owned `iree_hal_executable_cache_t` reference.
pub struct ExecutableCache(*mut n::iree_hal_executable_cache_t);
impl ExecutableCache {
    /// Creates a cache for the device and identifier.
    pub fn create(device: &Device, identifier: &str) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the device and identifier live through the call; `raw` is writable.
        check(unsafe {
            n::iree_hal_executable_cache_create(device.as_raw(), crate::view(identifier), &mut raw)
        })?;
        // SAFETY: successful creation returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned executable-cache reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_executable_cache_t) -> Self {
        Self(raw)
    }

    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_executable_cache_t {
        self.0
    }

    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_executable_cache_t {
        core::mem::ManuallyDrop::new(self).0
    }

    /// Infers the executable format of `data` and the number of bytes it occupies.
    pub fn infer_format(
        &self,
        caching_mode: ExecutableCachingMode,
        data: &[u8],
    ) -> Result<(String, usize)> {
        let mut format = vec![0u8; 64];
        loop {
            let mut inferred_size = 0;
            // SAFETY: `data` is readable and `format` is writable for its full length.
            let status = check(unsafe {
                n::iree_hal_executable_cache_infer_format(
                    self.0,
                    caching_mode.bits(),
                    const_span(data),
                    format.len(),
                    format.as_mut_ptr().cast(),
                    &mut inferred_size,
                )
            });
            match status {
                Ok(()) => {
                    let end = format.iter().position(|&b| b == 0).unwrap_or(format.len());
                    format.truncate(end);
                    let format = String::from_utf8_lossy(&format).into_owned();
                    return Ok((format, inferred_size));
                }
                Err(error) if error.code() == Code::OutOfRange && format.len() < 4096 => {
                    format.resize(format.len() * 2, 0);
                }
                Err(error) => return Err(error),
            }
        }
    }

    /// Returns whether this cache can prepare an executable format.
    pub fn can_prepare_format(&self, caching_mode: ExecutableCachingMode, format: &str) -> bool {
        // SAFETY: the cache and format live through the call.
        unsafe {
            n::iree_hal_executable_cache_can_prepare_format(
                self.0,
                caching_mode.bits(),
                crate::view(format),
            )
        }
    }

    /// Prepares an executable.
    ///
    /// [`ExecutableCachingMode::ALIAS_PROVIDED_DATA`] is cleared: borrowed input
    /// cannot outlive the cache and every native executable reference retained
    /// by command buffers or queues. Native preparation must own any bytes it
    /// needs after returning.
    ///
    /// # Safety
    /// The executable must be trusted and satisfy the selected loader's ABI.
    /// Native code may run during preparation and later dispatches; format
    /// verification does not establish memory safety.
    pub unsafe fn prepare(&self, params: &ExecutableParams<'_>) -> Result<Executable> {
        let mut native = params.raw;
        native.caching_mode &= !ExecutableCachingMode::ALIAS_PROVIDED_DATA.bits();
        let mut raw = core::ptr::null_mut();
        // SAFETY: params and their borrowed data live through the call; `raw` is writable.
        check(unsafe {
            n::iree_hal_executable_cache_prepare_executable(self.0, &native, &mut raw)
        })?;
        Ok(Executable { raw })
    }
}
impl Clone for ExecutableCache {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_executable_cache_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for ExecutableCache {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_executable_cache_release(self.0) }
    }
}
// SAFETY: the C header documents executable caches as thread-safe.
unsafe impl Send for ExecutableCache {}
// SAFETY: the C header documents executable caches as thread-safe.
unsafe impl Sync for ExecutableCache {}

/// An executable-local function token (`iree_hal_executable_function_t`).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExecutableFunction(u64);
impl ExecutableFunction {
    /// Selects a function by its dense table index.
    pub const fn from_index(index: u32) -> Self {
        Self(index as u64)
    }
    /// Wraps a raw executable-local token.
    pub const fn from_value(value: u64) -> Self {
        Self(value)
    }
    /// Returns the raw executable-local token.
    pub const fn value(self) -> u64 {
        self.0
    }
    /// Returns the dense table index, if the token is one.
    pub fn index(self) -> Option<u32> {
        u32::try_from(self.0).ok()
    }
    pub(crate) const fn raw(self) -> n::iree_hal_executable_function_t {
        n::iree_hal_executable_function_t { value: self.0 }
    }
}
