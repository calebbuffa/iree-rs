use crate::{Error, Result, sys};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, c_void},
    marker::PhantomData,
    ptr::{self, NonNull},
    rc::{Rc, Weak},
};

#[cfg(not(feature = "task"))]
use std::rc::Rc as InstanceOwnership;
#[cfg(feature = "task")]
use std::sync::Arc as InstanceOwnership;

pub(crate) fn check(status: sys::iree_status_t) -> Result<()> {
    if status.is_null() {
        return Ok(());
    }
    struct OwnedStatus(sys::iree_status_t);
    impl Drop for OwnedStatus {
        fn drop(&mut self) {
            unsafe { sys::iree_rs_status_free(self.0) };
        }
    }
    let status = OwnedStatus(status);
    let code = crate::StatusCode::from_native(unsafe { sys::iree_rs_status_code(status.0) });
    let mut length = 0;
    unsafe { sys::iree_status_format(status.0, 0, ptr::null_mut(), &mut length) };
    let capacity = length
        .checked_add(1)
        .ok_or_else(|| Error::new(code, "native diagnostic length overflows address space"))?;
    let mut message = vec![0 as std::ffi::c_char; capacity];
    if !unsafe {
        sys::iree_status_format(status.0, message.len(), message.as_mut_ptr(), &mut length)
    } {
        return Err(Error::new(
            code,
            "IREE could not format its native diagnostic",
        ));
    }
    let text = unsafe { CStr::from_ptr(message.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    Err(Error::new(code, text))
}

pub(crate) fn string_view(text: &str) -> sys::iree_string_view_t {
    sys::iree_string_view_t {
        data: text.as_ptr().cast(),
        size: text.len(),
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Primitive, byte-addressable tensor element supported by the safe API.
///
/// Sealed to types whose entire object representation is initialized and whose
/// every bit pattern is valid. Sub-byte formats need a separate packing API.
pub trait Element: sealed::Sealed + Copy + Default + 'static {
    /// IREE HAL element type corresponding to this Rust primitive.
    const TYPE: u32;
}

macro_rules! elements {
    ($($ty:ty => $code:expr),* $(,)?) => {$(
        impl sealed::Sealed for $ty {}
        impl Element for $ty {
            const TYPE: u32 = $code as sys::iree_hal_element_type_t;
        }
    )*};
}

elements! {
    f32 => sys::IREE_HAL_ELEMENT_TYPE_FLOAT_32,
    f64 => sys::IREE_HAL_ELEMENT_TYPE_FLOAT_64,
    i8 => sys::IREE_HAL_ELEMENT_TYPE_INT_8,
    i16 => sys::IREE_HAL_ELEMENT_TYPE_INT_16,
    i32 => sys::IREE_HAL_ELEMENT_TYPE_INT_32,
    i64 => sys::IREE_HAL_ELEMENT_TYPE_INT_64,
    u8 => sys::IREE_HAL_ELEMENT_TYPE_UINT_8,
    u16 => sys::IREE_HAL_ELEMENT_TYPE_UINT_16,
    u32 => sys::IREE_HAL_ELEMENT_TYPE_UINT_32,
    u64 => sys::IREE_HAL_ELEMENT_TYPE_UINT_64,
}

struct InstanceInner(NonNull<sys::iree_runtime_instance_t>);
// The runtime instance contract is thread-safe. Only native registrations and
// the system allocator are installed by the safe constructor.
#[cfg(feature = "task")]
unsafe impl Send for InstanceInner {}
#[cfg(feature = "task")]
unsafe impl Sync for InstanceInner {}
impl Drop for InstanceInner {
    fn drop(&mut self) {
        unsafe { sys::iree_runtime_instance_release(self.0.as_ptr()) };
    }
}

/// A runtime instance sharing driver registrations and VM types.
#[derive(Clone)]
pub struct Instance(InstanceOwnership<InstanceInner>);

impl Instance {
    /// Create a VM/runtime instance and register all compiled-in drivers.
    pub fn new() -> Result<Self> {
        let mut options = unsafe { std::mem::zeroed() };
        let mut raw = ptr::null_mut();
        unsafe {
            sys::iree_runtime_instance_options_initialize(&mut options);
            sys::iree_runtime_instance_options_use_all_available_drivers(&mut options);
            check(sys::iree_runtime_instance_create(
                &options,
                sys::iree_rs_allocator_system(),
                &mut raw,
            ))?;
        }
        let raw = NonNull::new(raw)
            .ok_or_else(|| Error::invalid("IREE returned a null runtime instance"))?;
        Ok(Self(InstanceOwnership::new(InstanceInner(raw))))
    }

    /// Borrow the native runtime instance without transferring ownership.
    ///
    /// The pointer is valid while this instance or any clone remains alive.
    /// Native calls through it must not release the wrapper's reference.
    pub fn as_raw(&self) -> *mut sys::iree_runtime_instance_t {
        self.0.0.as_ptr()
    }

    /// Create a registered device, e.g. `local-task`, `local-sync`, or `vulkan`.
    pub fn device(&self, driver: &str) -> Result<Device> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_runtime_instance_try_create_default_device(
                self.0.0.as_ptr(),
                string_view(driver),
                &mut raw,
            )
        })?;
        unsafe { Device::from_owned(self.clone(), raw, None) }
    }
}

pub(crate) struct DeviceInner {
    pub(crate) raw: NonNull<sys::iree_hal_device_t>,
    pub(crate) hal_module: NonNull<sys::iree_vm_module_t>,
    pub(crate) instance: Instance,
    pub(crate) allocation_access: RefCell<HashMap<usize, Weak<AllocationAccess>>>,
    // Keep the external native device alive until after HAL device destruction.
    pub(crate) _owner: Option<Box<dyn std::any::Any>>,
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe {
            sys::iree_vm_module_release(self.hal_module.as_ptr());
            sys::iree_hal_device_release(self.raw.as_ptr());
        }
    }
}

/// An owned HAL device using any compiled-in driver. Tensors retain their device.
#[derive(Clone)]
pub struct Device(pub(crate) Rc<DeviceInner>);

impl Device {
    /// Adopt one HAL device reference and create its shared VM HAL module.
    ///
    /// # Safety
    ///
    /// The device must be compatible with `instance`, and its queue frontiers
    /// must not already belong to a device group. Transfer exactly one owned
    /// reference. Externally owned handles, queues, allocators, and callback
    /// state must outlive this device and every derived tensor or session.
    /// This method consumes the reference even if HAL-module creation fails.
    pub unsafe fn from_raw(instance: Instance, raw: *mut sys::iree_hal_device_t) -> Result<Self> {
        unsafe { Self::from_owned(instance, raw, None) }
    }

    /// Adopt a native device reference while retaining an external lifetime owner.
    ///
    /// The owner is dropped after the native HAL module and device are released.
    /// Derived tensors and sessions retain it through this device.
    ///
    /// # Safety
    ///
    /// All requirements of [`Self::from_raw`] apply. `owner` must keep external
    /// native handles, allocators, queues, libraries, and callback state valid
    /// for this device's entire lifetime.
    pub unsafe fn from_raw_with_owner(
        instance: Instance,
        raw: *mut sys::iree_hal_device_t,
        owner: impl std::any::Any,
    ) -> Result<Self> {
        unsafe { Self::from_owned(instance, raw, Some(Box::new(owner))) }
    }

    /// Borrow the native HAL device without transferring ownership.
    ///
    /// Native operations must preserve the wrapper's shared device group and
    /// allocator. Mutating them can invalidate loaded modules and shared buffers.
    pub fn as_raw(&self) -> *mut sys::iree_hal_device_t {
        self.0.raw.as_ptr()
    }

    /// Borrow this device's shared HAL VM module without transferring ownership.
    ///
    /// The module owns the device's VM integration and device group. Native
    /// consumers retaining this pointer must retain any external lifetime owners
    /// separately; a native retain does not retain the Rust ownership graph.
    pub fn as_raw_hal_module(&self) -> *mut sys::iree_vm_module_t {
        self.0.hal_module.as_ptr()
    }

    /// Return the driver's runtime device identifier.
    ///
    /// This identifies the runtime device type/version, not a stable physical
    /// device enumeration ID. The returned string owns its storage.
    pub fn id(&self) -> String {
        unsafe { crate::driver::owned_string(sys::iree_hal_device_id(self.as_raw())) }
    }

    /// Query a native device configuration value.
    ///
    /// Categories and keys are driver-dependent. Well-known categories include
    /// `hal.device.id`, `hal.device.feature`, `hal.device.architecture`, and
    /// `hal.executable.format`; keys may be native wildcard patterns.
    /// Unsupported queries return the native error rather than a default value.
    pub fn query_i64(&self, category: &str, key: &str) -> Result<i64> {
        let mut value = 0;
        check(unsafe {
            sys::iree_hal_device_query_i64(
                self.as_raw(),
                string_view(category),
                string_view(key),
                &mut value,
            )
        })?;
        Ok(value)
    }

    /// Trim native device pools and caches while preserving live allocations.
    ///
    /// Suitable for low-memory notifications and parking a device. Drivers may
    /// synchronize internally; this method does not cancel outstanding work.
    pub fn trim(&self) -> Result<()> {
        check(unsafe { sys::iree_hal_device_trim(self.as_raw()) })
    }

    pub(crate) unsafe fn from_owned(
        instance: Instance,
        raw: *mut sys::iree_hal_device_t,
        owner: Option<Box<dyn std::any::Any>>,
    ) -> Result<Self> {
        let raw = NonNull::new(raw).ok_or_else(|| Error::invalid("IREE returned a null device"))?;
        let mut hal_module = ptr::null_mut();
        let status = unsafe {
            sys::iree_rs_device_hal_module_create(
                sys::iree_runtime_instance_vm_instance(instance.0.0.as_ptr()),
                raw.as_ptr(),
                sys::iree_rs_allocator_system(),
                &mut hal_module,
            )
        };
        if let Err(error) = check(status) {
            unsafe { sys::iree_hal_device_release(raw.as_ptr()) };
            return Err(error);
        }
        let Some(hal_module) = NonNull::new(hal_module) else {
            unsafe { sys::iree_hal_device_release(raw.as_ptr()) };
            return Err(Error::invalid("IREE returned a null HAL module"));
        };
        Ok(Self(Rc::new(DeviceInner {
            raw,
            hal_module,
            instance,
            allocation_access: RefCell::new(HashMap::new()),
            _owner: owner,
        })))
    }

    /// Upload initialized dense row-major values to device-owned tensor storage.
    ///
    /// The shape's element count must equal `data.len()`. An empty shape denotes
    /// a scalar; any zero dimension denotes an empty tensor. The returned tensor
    /// retains the device. This operation completes the upload before returning.
    pub fn tensor<T: Element>(&self, shape: &[usize], data: &[T]) -> Result<Tensor> {
        let byte_length = tensor_byte_length(shape, data)?;
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_buffer_view_allocate_buffer_copy(
                self.0.raw.as_ptr(),
                sys::iree_hal_device_allocator(self.0.raw.as_ptr()),
                shape.len(),
                shape.as_ptr(),
                T::TYPE,
                sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as sys::iree_hal_encoding_type_t,
                tensor_buffer_params(),
                sys::iree_const_byte_span_t {
                    data: data.as_ptr().cast(),
                    data_length: byte_length,
                },
                &mut raw,
            )
        })?;
        unsafe { Tensor::from_owned(self.clone(), raw) }
    }

    /// Load compiler output into a fresh isolated session.
    #[cfg(feature = "compiler")]
    pub fn load(&self, artifact: &crate::Artifact) -> Result<Session> {
        unsafe { self.load_bytes(artifact.bytes.clone()) }
    }

    /// Load compiler output with scoped native model-weight providers.
    ///
    /// The native parameter module retains providers and backing files for the
    /// entire session lifetime, independently of the Rust index/provider owners.
    #[cfg(feature = "compiler")]
    pub fn load_with_parameters(
        &self,
        artifact: &crate::Artifact,
        providers: &[crate::ParameterProvider],
    ) -> Result<Session> {
        unsafe { self.load_bytes_with_parameters(artifact.bytes.clone(), providers) }
    }

    /// Load trusted VM bytecode, retaining its storage for the session lifetime.
    ///
    /// # Safety
    ///
    /// Bytecode must be trusted compatible IREE output. Embedded native code is
    /// not sandboxed. Prefer `load` with an artifact from `Compiler`.
    pub unsafe fn load_bytes(&self, bytes: impl Into<std::sync::Arc<[u8]>>) -> Result<Session> {
        unsafe { self.load_bytes_with_parameters(bytes, &[]) }
    }

    /// Load trusted bytecode with scoped parameter providers.
    ///
    /// # Safety
    ///
    /// Bytecode must be trusted compatible IREE output, just as for
    /// [`Self::load_bytes`]. Embedded native code is not sandboxed.
    pub unsafe fn load_bytes_with_parameters(
        &self,
        bytes: impl Into<std::sync::Arc<[u8]>>,
        providers: &[crate::ParameterProvider],
    ) -> Result<Session> {
        let module = unsafe { self.0.instance.module_from_bytes(bytes) }?;
        self.session_builder()
            .parameters(providers)
            .module(&module)
            .build()
    }

    pub(crate) unsafe fn allocation_access(
        &self,
        buffer: *mut sys::iree_hal_buffer_t,
    ) -> Rc<AllocationAccess> {
        let root = unsafe { sys::iree_hal_buffer_allocated_buffer(buffer) } as usize;
        let mut registry = self.0.allocation_access.borrow_mut();
        registry.retain(|_, state| state.strong_count() != 0);
        if let Some(access) = registry.get(&root).and_then(Weak::upgrade) {
            access
        } else {
            let access = Rc::new(AllocationAccess(RefCell::new(Vec::new())));
            registry.insert(root, Rc::downgrade(&access));
            access
        }
    }
}

pub(crate) fn tensor_buffer_params() -> sys::iree_hal_buffer_params_t {
    sys::iree_hal_buffer_params_t {
        type_: sys::IREE_HAL_MEMORY_TYPE_DEVICE_LOCAL as sys::iree_hal_memory_type_t,
        usage: sys::IREE_HAL_BUFFER_USAGE_DEFAULT as sys::iree_hal_buffer_usage_t,
        access: sys::IREE_HAL_MEMORY_ACCESS_ALL as sys::iree_hal_memory_access_t,
        ..Default::default()
    }
}

pub(crate) fn tensor_byte_length<T: Element>(shape: &[usize], data: &[T]) -> Result<usize> {
    let elements = element_count(shape)?;
    if elements != data.len() {
        return Err(Error::invalid(format!(
            "tensor shape requires {elements} elements, received {}",
            data.len()
        )));
    }
    elements
        .checked_mul(std::mem::size_of::<T>())
        .filter(|&bytes| bytes <= isize::MAX as usize)
        .ok_or_else(|| Error::invalid("tensor byte size exceeds address space"))
}

pub(crate) fn element_count(shape: &[usize]) -> Result<usize> {
    if shape.contains(&0) {
        return Ok(0);
    }
    shape.iter().try_fold(1usize, |count, &dim| {
        count
            .checked_mul(dim)
            .ok_or_else(|| Error::invalid("tensor element count overflows usize"))
    })
}

/// A dense row-major HAL tensor. Clones share the same underlying storage.
pub struct Tensor {
    pub(crate) raw: NonNull<sys::iree_hal_buffer_view_t>,
    pub(crate) device: Device,
    access: Rc<AllocationAccess>,
    _thread_local: PhantomData<Rc<()>>,
}

pub(crate) struct AllocationAccess(pub(crate) RefCell<Vec<crate::Fence>>);

impl AllocationAccess {
    pub(crate) fn wait(&self) -> Result<()> {
        for fence in self.0.borrow().iter() {
            fence.wait_blocking(crate::Timeout::Infinite)?;
        }
        self.0.borrow_mut().clear();
        Ok(())
    }
}

impl Tensor {
    /// Borrow the device retaining this tensor's native allocation.
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Adopt one owned native buffer-view reference.
    ///
    /// # Safety
    ///
    /// `raw` must be a live buffer view allocated on `device`. Transfer exactly
    /// one reference; do not subsequently release that reference yourself.
    /// Storage must be initialized and valid for all reads and native accesses.
    pub unsafe fn from_raw(device: Device, raw: *mut sys::iree_hal_buffer_view_t) -> Result<Self> {
        unsafe { Self::from_owned(device, raw) }
    }

    /// Borrow the native view without transferring its reference.
    pub fn as_raw(&self) -> *mut sys::iree_hal_buffer_view_t {
        self.raw.as_ptr()
    }

    pub(crate) unsafe fn from_owned(
        device: Device,
        raw: *mut sys::iree_hal_buffer_view_t,
    ) -> Result<Self> {
        let raw = NonNull::new(raw).ok_or_else(|| Error::invalid("IREE returned a null tensor"))?;
        let buffer = unsafe { sys::iree_hal_buffer_view_buffer(raw.as_ptr()) };
        let access = unsafe { device.allocation_access(buffer) };
        Ok(Self {
            raw,
            device,
            access,
            _thread_local: PhantomData,
        })
    }

    /// Return the current dimensions in row-major logical order.
    pub fn shape(&self) -> Vec<usize> {
        let rank = unsafe { sys::iree_hal_buffer_view_shape_rank(self.raw.as_ptr()) };
        (0..rank)
            .map(|axis| unsafe {
                sys::iree_hal_buffer_view_shape_dim(self.raw.as_ptr(), axis) as usize
            })
            .collect()
    }

    /// Return the native HAL element-type code.
    pub fn element_type(&self) -> u32 {
        unsafe { sys::iree_hal_buffer_view_element_type(self.raw.as_ptr()) }
    }

    /// Return the tensor view's byte length, not its backing pool allocation size.
    pub fn byte_len(&self) -> usize {
        unsafe { sys::iree_hal_buffer_view_byte_length(self.raw.as_ptr()) }
    }

    /// Wait for safe API transfers and invocations accessing this allocation.
    ///
    /// This does not copy data or synchronize unrelated device allocations.
    /// External raw/graphics accesses remain the caller's responsibility.
    pub fn wait(&self) -> Result<()> {
        self.access.wait()
    }

    pub(crate) fn buffer(&self) -> *mut sys::iree_hal_buffer_t {
        unsafe { sys::iree_hal_buffer_view_buffer(self.raw.as_ptr()) }
    }

    /// Explicit device-to-host transfer, checking the requested element type.
    pub fn read<T: Element>(&self) -> Result<Vec<T>> {
        self.wait()?;
        if self.element_type() != T::TYPE {
            return Err(Error::invalid(
                "tensor element type does not match requested Rust type",
            ));
        }
        if unsafe { sys::iree_hal_buffer_view_encoding_type(self.raw.as_ptr()) }
            != sys::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR as sys::iree_hal_encoding_type_t
        {
            return Err(Error::invalid("read requires a dense row-major tensor"));
        }
        let elements = unsafe { sys::iree_hal_buffer_view_element_count(self.raw.as_ptr()) };
        let expected = elements
            .checked_mul(std::mem::size_of::<T>())
            .filter(|&bytes| bytes <= isize::MAX as usize)
            .ok_or_else(|| Error::invalid("tensor byte size exceeds address space"))?;
        if expected != self.byte_len() {
            return Err(Error::invalid(
                "tensor byte size does not match its element type",
            ));
        }
        let mut values = vec![T::default(); elements];
        if expected != 0 {
            check(unsafe {
                sys::iree_hal_device_transfer_d2h(
                    self.device.0.raw.as_ptr(),
                    self.buffer(),
                    0,
                    values.as_mut_ptr().cast::<c_void>(),
                    self.byte_len(),
                    0,
                    sys::iree_rs_infinite_timeout(),
                )
            })?;
        }
        Ok(values)
    }
}

impl Clone for Tensor {
    fn clone(&self) -> Self {
        unsafe { sys::iree_hal_buffer_view_retain(self.raw.as_ptr()) };
        Self {
            raw: self.raw,
            device: self.device.clone(),
            access: self.access.clone(),
            _thread_local: PhantomData,
        }
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        unsafe { sys::iree_hal_buffer_view_release(self.raw.as_ptr()) };
    }
}

/// An isolated VM context. Calls require exclusive access to session state.
pub struct Session {
    pub(crate) raw: NonNull<sys::iree_vm_context_t>,
    pub(crate) device: Device,
    pub(crate) _modules: Vec<crate::Module>,
    pub(crate) pending: Vec<Option<PendingCall>>,
}

pub(crate) struct PendingCall {
    outputs: VmList,
    _inputs: VmList,
    completion: crate::Fence,
    _dependencies: crate::Fence,
}

/// A coarse-fence invocation whose results become accessible after completion.
///
/// Polling waits on native timepoint callbacks, not a blocking Rust worker.
/// Dropping this future detaches it from the caller, but the session retains
/// native argument/result storage until the device finishes. Dropping that
/// session drains any abandoned calls before destroying its VM context.
#[must_use = "await the invocation to retrieve results or observe device failures"]
pub struct Invocation<'a> {
    session: &'a mut Session,
    index: usize,
    wait: crate::FenceWait,
    completion: crate::Fence,
    completed: bool,
}

impl Invocation<'_> {
    /// Borrow an immutable device completion fence for dependent submissions.
    pub fn completion(&self) -> &crate::Fence {
        &self.completion
    }
}

impl std::future::Future for Invocation<'_> {
    type Output = Result<Vec<Value>>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use std::task::Poll;
        if self.completed {
            return Poll::Ready(Err(Error::invalid(
                "invocation future was already consumed",
            )));
        }
        match std::pin::Pin::new(&mut self.wait).poll(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(status) => {
                self.completed = true;
                let index = self.index;
                let pending = self.session.pending[index]
                    .take()
                    .expect("owned pending invocation");
                Poll::Ready(status.and_then(|()| self.session.read_outputs(&pending.outputs)))
            }
        }
    }
}

/// A primitive VM value, tensor, or retained native object.
///
/// Integer values use VM signed types. Ref objects retain their native type and
/// may be passed to other functions on the same device without using `sys`.
pub enum Value {
    /// An absent value or null native reference.
    Null,
    /// Eight-bit signed integer.
    I8(i8),
    /// Sixteen-bit signed integer.
    I16(i16),
    /// Thirty-two-bit signed integer.
    I32(i32),
    /// Sixty-four-bit signed integer.
    I64(i64),
    /// IEEE single-precision floating-point value.
    F32(f32),
    /// IEEE double-precision floating-point value.
    F64(f64),
    /// A typed HAL buffer view.
    Tensor(Tensor),
    /// Any other native VM reference type.
    Object(Object),
}

/// An owned, dynamically typed native VM object returned by an invocation.
///
/// Its native type is preserved, and its originating device remains alive.
pub struct Object {
    reference: VmRef,
    device: Device,
}

impl Object {
    /// Registered native type name.
    pub fn type_name(&self) -> String {
        unsafe { crate::driver::owned_string(sys::iree_vm_ref_type_name(self.reference.0.type_)) }
    }
}

macro_rules! value_from {
    ($($ty:ty => $variant:ident),* $(,)?) => {$(
        impl From<$ty> for Value {
            fn from(value: $ty) -> Self { Self::$variant(value) }
        }
    )*};
}
value_from! { i8 => I8, i16 => I16, i32 => I32, i64 => I64, f32 => F32, f64 => F64, Tensor => Tensor, Object => Object }

impl Session {
    /// Borrow the VM context without transferring ownership.
    ///
    /// Native calls must not run concurrently with [`Self::call`] or release
    /// the wrapper's reference. The context retains its loaded native modules.
    pub fn as_raw(&self) -> *mut sys::iree_vm_context_t {
        self.raw.as_ptr()
    }

    /// Invoke a fully qualified function such as `module.main`.
    ///
    /// This synchronous API waits for the ordinary synchronous IREE ABI.
    /// Explicit coarse-fence ABIs are not exposed by this tensor-only interface.
    pub fn call(&mut self, function: &str, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        let values: Vec<_> = inputs.iter().cloned().map(Value::Tensor).collect();
        self.call_values(function, &values)?
            .into_iter()
            .map(|value| match value {
                Value::Tensor(tensor) => Ok(tensor),
                _ => Err(Error::invalid(
                    "function returned a non-tensor; use call_values",
                )),
            })
            .collect()
    }

    /// Invoke a VM function with scalars, tensors, and dynamically typed objects.
    ///
    /// Native argument and result types are checked by the VM. Device-backed
    /// objects must originate from this session's device.
    pub fn call_values(&mut self, function: &str, inputs: &[Value]) -> Result<Vec<Value>> {
        self.collect_finished()?;
        for value in inputs {
            if let Value::Tensor(tensor) = value {
                tensor.access.wait()?;
            }
        }
        let function_ref = self.resolve(function)?;
        let input_list = self.prepare_inputs(inputs)?;
        let output_list = VmList::new(0)?;
        self.invoke(function_ref, &input_list, &output_list)?;
        self.read_outputs(&output_list)
    }

    /// Read one function reflection attribute without invoking the function.
    ///
    /// Returns `None` if the attribute is absent. A missing function is an error.
    pub fn function_attribute(&self, function: &str, key: &str) -> Result<Option<String>> {
        let function = self.resolve(function)?;
        let value =
            unsafe { sys::iree_vm_function_lookup_attr_by_name(&function, string_view(key)) };
        if value.size != 0 {
            return Ok(Some(unsafe { crate::driver::owned_string(value) }));
        }
        // Native lookup uses the same empty view for absent and empty values.
        Ok(crate::FunctionInfo::copy(function)?
            .attributes
            .into_iter()
            .find(|attribute| attribute.key == key)
            .map(|attribute| attribute.value))
    }

    /// Submit a compiled coarse-fence function and await device completion.
    ///
    /// The function must advertise `iree.abi.model = "coarse-fences"`. IREE runs
    /// host-side dispatch preparation during this call, then schedules native
    /// device work. The returned future exposes no output storage until its
    /// completion semaphore is reached. No blocking worker is spawned.
    ///
    /// Dropping the future does not cancel submitted work. The session retains
    /// abandoned calls and reports their failures on the next invocation or
    /// [`Self::drain`]. Session destruction drains remaining calls.
    pub fn call_async(&mut self, function: &str, inputs: &[Value]) -> Result<Invocation<'_>> {
        self.call_async_after(function, inputs, &crate::Fence::empty())
    }

    /// Submit a coarse-fence function after an explicit device dependency.
    ///
    /// Pending safe tensor accesses are also chained on the device rather than
    /// blocked on the calling thread. All fence timepoints must belong to this
    /// device. Reading an input or caller-provided output tensor blocks until
    /// its submitted access completes, including through cloned tensor views.
    pub fn call_async_after(
        &mut self,
        function: &str,
        inputs: &[Value],
        dependency: &crate::Fence,
    ) -> Result<Invocation<'_>> {
        self.collect_finished()?;
        dependency.validate_device(&self.device)?;
        if self
            .function_attribute(function, "iree.abi.model")?
            .as_deref()
            != Some("coarse-fences")
        {
            return Err(Error::invalid(
                "call_async requires a compiled coarse-fences ABI",
            ));
        }
        let function_ref = self.resolve(function)?;
        let input_list = self.prepare_inputs(inputs)?;
        let output_list = VmList::new(0)?;
        let completion = self.device.semaphore(0)?.at(1)?;
        let mut dependencies = vec![dependency.clone()];
        for value in inputs {
            if let Value::Tensor(tensor) = value {
                dependencies.extend(tensor.access.0.borrow().iter().cloned());
            }
        }
        let dependencies = crate::Fence::join(&dependencies)?;
        let mut reference = VmRef(unsafe { sys::iree_hal_fence_retain_ref(dependencies.as_raw()) });
        check(unsafe { sys::iree_vm_list_push_ref_move(input_list.0, &mut reference.0) })?;
        let mut signal = VmRef(unsafe { sys::iree_hal_fence_retain_ref(completion.as_raw()) });
        check(unsafe { sys::iree_vm_list_push_ref_move(input_list.0, &mut signal.0) })?;
        let wait = completion.wait()?;
        self.invoke(function_ref, &input_list, &output_list)?;
        for value in inputs {
            if let Value::Tensor(tensor) = value {
                *tensor.access.0.borrow_mut() = vec![completion.clone()];
            }
        }
        let index = self.pending.len();
        self.pending.push(Some(PendingCall {
            outputs: output_list,
            _inputs: input_list,
            completion: completion.clone(),
            _dependencies: dependencies,
        }));
        Ok(Invocation {
            session: self,
            index,
            wait,
            completion,
            completed: false,
        })
    }

    /// Wait for detached calls and propagate the first native failure.
    ///
    /// All calls are drained even if one fails, so their storage is released
    /// only after native completion or device failure has been observed.
    pub fn drain(&mut self) -> Result<()> {
        let mut failure = None;
        for slot in &mut self.pending {
            if let Some(pending) = slot.take()
                && let Err(error) = pending.completion.wait_blocking(crate::Timeout::Infinite)
                && failure.is_none()
            {
                failure = Some(error);
            }
        }
        self.pending.clear();
        failure.map_or(Ok(()), Err)
    }

    fn collect_finished(&mut self) -> Result<()> {
        let mut failure = None;
        for slot in &mut self.pending {
            let Some(pending) = slot else { continue };
            match pending.completion.is_ready() {
                Ok(false) => {}
                Ok(true) => {
                    *slot = None;
                }
                Err(error) => {
                    *slot = None;
                    if failure.is_none() {
                        failure = Some(error);
                    }
                }
            }
        }
        while self.pending.last().is_some_and(Option::is_none) {
            self.pending.pop();
        }
        failure.map_or(Ok(()), Err)
    }

    pub(crate) fn resolve(&self, function: &str) -> Result<sys::iree_vm_function_t> {
        let mut function_ref = sys::iree_vm_function_t::default();
        check(unsafe {
            sys::iree_vm_context_resolve_function(
                self.raw.as_ptr(),
                string_view(function),
                &mut function_ref,
            )
        })?;
        Ok(function_ref)
    }

    fn prepare_inputs(&self, inputs: &[Value]) -> Result<VmList> {
        for value in inputs {
            let device = match value {
                Value::Tensor(tensor) => Some(&tensor.device),
                Value::Object(object) => Some(&object.device),
                _ => None,
            };
            if device.is_some_and(|device| !Rc::ptr_eq(&device.0, &self.device.0)) {
                return Err(Error::invalid("input tensor belongs to a different device"));
            }
        }
        let input_list = VmList::new(inputs.len())?;
        for value in inputs {
            let scalar = unsafe {
                match value {
                    Value::I8(value) => Some(sys::iree_vm_value_make_i8(*value)),
                    Value::I16(value) => Some(sys::iree_vm_value_make_i16(*value)),
                    Value::I32(value) => Some(sys::iree_vm_value_make_i32(*value)),
                    Value::I64(value) => Some(sys::iree_vm_value_make_i64(*value)),
                    Value::F32(value) => Some(sys::iree_vm_value_make_f32(*value)),
                    Value::F64(value) => Some(sys::iree_vm_value_make_f64(*value)),
                    _ => None,
                }
            };
            if let Some(scalar) = scalar {
                check(unsafe { sys::iree_vm_list_push_value(input_list.0, &scalar) })?;
                continue;
            }
            let mut reference = VmRef(unsafe { sys::iree_vm_ref_null() });
            match value {
                Value::Tensor(tensor) => {
                    reference.0 =
                        unsafe { sys::iree_hal_buffer_view_retain_ref(tensor.raw.as_ptr()) }
                }
                Value::Object(object) => {
                    let mut source = object.reference.0;
                    unsafe { sys::iree_vm_ref_retain(&mut source, &mut reference.0) };
                }
                Value::Null => {}
                _ => unreachable!("scalar inputs were handled above"),
            }
            check(unsafe { sys::iree_vm_list_push_ref_move(input_list.0, &mut reference.0) })?;
        }
        Ok(input_list)
    }

    fn invoke(
        &mut self,
        function_ref: sys::iree_vm_function_t,
        input_list: &VmList,
        output_list: &VmList,
    ) -> Result<()> {
        check(unsafe {
            sys::iree_vm_invoke(
                self.raw.as_ptr(),
                function_ref,
                0,
                ptr::null(),
                input_list.0,
                output_list.0,
                sys::iree_rs_allocator_system(),
            )
        })
    }

    fn read_outputs(&self, output_list: &VmList) -> Result<Vec<Value>> {
        let count = unsafe { sys::iree_vm_list_size(output_list.0) };
        let mut outputs = Vec::with_capacity(count);
        for index in 0..count {
            let mut variant = sys::iree_vm_variant_t::default();
            check(unsafe {
                sys::iree_vm_list_get_variant_assign(output_list.0, index, &mut variant)
            })?;
            if unsafe { sys::iree_vm_variant_is_value(variant) } {
                let scalar = unsafe { sys::iree_vm_variant_value(variant) };
                let value = unsafe {
                    match scalar.type_ {
                        sys::IREE_VM_VALUE_TYPE_I8 => Value::I8(scalar.__bindgen_anon_1.i8_),
                        sys::IREE_VM_VALUE_TYPE_I16 => Value::I16(scalar.__bindgen_anon_1.i16_),
                        sys::IREE_VM_VALUE_TYPE_I32 => Value::I32(scalar.__bindgen_anon_1.i32_),
                        sys::IREE_VM_VALUE_TYPE_I64 => Value::I64(scalar.__bindgen_anon_1.i64_),
                        sys::IREE_VM_VALUE_TYPE_F32 => Value::F32(scalar.__bindgen_anon_1.f32_),
                        sys::IREE_VM_VALUE_TYPE_F64 => Value::F64(scalar.__bindgen_anon_1.f64_),
                        _ => return Err(Error::invalid("unsupported native VM primitive type")),
                    }
                };
                outputs.push(value);
            } else if unsafe { sys::iree_vm_variant_is_ref(variant) } {
                let mut reference = VmRef(unsafe { sys::iree_vm_ref_null() });
                unsafe {
                    sys::iree_vm_ref_retain(&mut variant.__bindgen_anon_1.ref_, &mut reference.0)
                };
                if unsafe { sys::iree_hal_buffer_view_isa(reference.0) } {
                    let mut tensor = ptr::null_mut();
                    check(unsafe {
                        sys::iree_hal_buffer_view_check_deref(reference.0, &mut tensor)
                    })?;
                    unsafe { sys::iree_hal_buffer_view_retain(tensor) };
                    outputs.push(Value::Tensor(unsafe {
                        Tensor::from_owned(self.device.clone(), tensor)?
                    }));
                } else {
                    outputs.push(Value::Object(Object {
                        reference,
                        device: self.device.clone(),
                    }));
                }
            } else {
                outputs.push(Value::Null);
            }
        }
        Ok(outputs)
    }
}

struct VmRef(sys::iree_vm_ref_t);
impl Drop for VmRef {
    fn drop(&mut self) {
        unsafe { sys::iree_vm_ref_release(&mut self.0) };
    }
}

struct VmList(*mut sys::iree_vm_list_t);
impl VmList {
    fn new(capacity: usize) -> Result<Self> {
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_vm_list_create(
                sys::iree_rs_vm_undefined_type_def(),
                capacity,
                sys::iree_rs_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }
}
impl Drop for VmList {
    fn drop(&mut self) {
        unsafe { sys::iree_vm_list_release(self.0) };
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if let Err(error) = self.drain() {
            eprintln!("IREE detached invocation failed during session destruction: {error}");
        }
        // Release modules before their backing bytecode and device are dropped.
        unsafe { sys::iree_vm_context_release(self.raw.as_ptr()) };
    }
}
