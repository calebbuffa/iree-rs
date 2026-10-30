//! CPU executable loaders and executable import plugins.
//!
//! Loaders retain any plugin manager, VM instance and modules they are given.
#[cfg(feature = "cpu")]
use crate::Result;
#[cfg(feature = "cpu")]
use crate::{Code, Error, check, system, view};
use iree_native as n;
#[cfg(feature = "cpu")]
use std::ptr;
#[cfg(feature = "cpu")]
use std::sync::Arc;

/// Query function exported by a statically linked IREE executable library.
#[cfg(feature = "cpu")]
pub type ExecutableLibraryQueryFn =
    unsafe extern "C" fn(
        n::iree_hal_executable_library_version_t,
        *const n::iree_hal_executable_environment_v0_t,
    ) -> *mut *const n::iree_hal_executable_library_header_t;

/// A retained local executable loader.
pub struct ExecutableLoader {
    raw: *mut n::iree_hal_executable_loader_t,
    #[cfg(feature = "cpu")]
    _isolated_type_registry: Option<Arc<crate::vm::IsolatedTypeRegistryGuard>>,
}

impl ExecutableLoader {
    /// Adopts one owned executable-loader reference.
    ///
    /// # Safety
    /// `raw` must be a live loader with one transferred reference and must not
    /// own an isolated VMVX instance that rewrites process-global VM type IDs.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_executable_loader_t) -> Self {
        Self {
            raw,
            #[cfg(feature = "cpu")]
            _isolated_type_registry: None,
        }
    }

    /// Borrows the native loader pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_executable_loader_t {
        self.raw
    }

    #[cfg(any(feature = "local-task", feature = "local-sync"))]
    pub(crate) fn uses_isolated_type_registry(&self) -> bool {
        #[cfg(feature = "cpu")]
        {
            self._isolated_type_registry.is_some()
        }
        #[cfg(not(feature = "cpu"))]
        {
            false
        }
    }

    /// Creates every loader linked into this build, with default options.
    ///
    /// # Safety
    /// No VM instance outside this wrapper may be live: native enumeration
    /// creates an isolated instance and rewrites process-global VM type IDs.
    /// Returned loaders prevent this crate's safe shared instance from being
    /// created until they are all dropped. Safe local-device constructors
    /// reject these loaders; use [`Self::vmvx`] with a shared instance instead.
    #[cfg(feature = "cpu")]
    pub unsafe fn all_available(manager: Option<&ExecutablePluginManager>) -> Result<Vec<Self>> {
        let isolated_type_registry = Arc::new(crate::vm::IsolatedTypeRegistryGuard::begin()?);
        // Upstream links at most three loaders and does not report the
        // required capacity on overflow, so query once with headroom.
        let mut raw = [ptr::null_mut(); 8];
        let mut count = 0;
        // SAFETY: `raw` has `raw.len()` writable slots; on failure upstream
        // releases any loaders it created. The caller excludes live VM registries.
        check(unsafe {
            n::iree_hal_create_all_available_executable_loaders(
                manager.map_or(ptr::null_mut(), ExecutablePluginManager::as_raw),
                raw.len(),
                &mut count,
                raw.as_mut_ptr(),
                system(),
            )
        })?;
        // SAFETY: each of the first `count` slots holds one owned reference.
        Ok(raw
            .into_iter()
            .take(count)
            .map(|raw| Self {
                raw,
                _isolated_type_registry: Some(isolated_type_registry.clone()),
            })
            .collect())
    }

    /// Creates the linked loader registered under `name`, such as `embedded-elf`.
    #[cfg(feature = "cpu")]
    pub fn by_name(name: &str, manager: Option<&ExecutablePluginManager>) -> Result<Self> {
        // Avoid native isolated creation for VMVX; use the shared instance.
        if name.starts_with("vmvx-module") {
            return Self::vmvx(&crate::vm::Instance::new()?, &[]);
        }
        let mut raw = ptr::null_mut();
        // SAFETY: the name is borrowed for the call; the loader retains the manager.
        check(unsafe {
            n::iree_hal_create_executable_loader_by_name(
                view(name),
                manager.map_or(ptr::null_mut(), ExecutablePluginManager::as_raw),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _isolated_type_registry: None,
        })
    }

    /// Creates a loader for embedded ELF executables.
    #[cfg(feature = "cpu")]
    pub fn embedded_elf(manager: Option<&ExecutablePluginManager>) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: the optional manager is live; the loader retains it.
        check(unsafe {
            n::iree_hal_embedded_elf_loader_create(
                manager.map_or(ptr::null_mut(), ExecutablePluginManager::as_raw),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _isolated_type_registry: None,
        })
    }

    /// Creates a loader for platform dynamic-library executables.
    #[cfg(feature = "cpu")]
    pub fn system_library(manager: Option<&ExecutablePluginManager>) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: the optional manager is live; the loader retains it.
        check(unsafe {
            n::iree_hal_system_library_loader_create(
                manager.map_or(ptr::null_mut(), ExecutablePluginManager::as_raw),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _isolated_type_registry: None,
        })
    }

    /// Creates a loader for statically linked executable libraries.
    ///
    /// # Safety
    /// Each query function must be a compiler-generated library query symbol
    /// that outlives this loader and every executable it loads.
    #[cfg(feature = "cpu")]
    pub unsafe fn static_library(query_functions: &[ExecutableLibraryQueryFn]) -> Result<Self> {
        let functions: Vec<n::iree_hal_executable_library_query_fn_t> =
            query_functions.iter().copied().map(Some).collect();
        let mut raw = ptr::null_mut();
        // SAFETY: `functions` outlives the call; the loader copies the list.
        check(unsafe {
            n::iree_hal_static_library_loader_create(
                functions.len(),
                functions.as_ptr(),
                n::iree_hal_executable_import_provider_t::default(),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _isolated_type_registry: None,
        })
    }

    /// Creates a VMVX loader in `instance` with optional user modules.
    #[cfg(feature = "cpu")]
    pub fn vmvx(instance: &crate::vm::Instance, modules: &[crate::vm::Module]) -> Result<Self> {
        let mut modules: Vec<_> = modules.iter().map(crate::vm::Module::as_raw).collect();
        let mut raw = ptr::null_mut();
        // SAFETY: the instance and modules are live; the loader retains them.
        check(unsafe {
            n::iree_hal_vmvx_module_loader_create(
                instance.as_raw(),
                modules.len(),
                modules.as_mut_ptr(),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _isolated_type_registry: None,
        })
    }

    /// Creates a VMVX loader with its own VM instance and optional user modules.
    ///
    /// # Safety
    /// No VM instance outside this crate may be live. The returned loader
    /// prevents this crate's safe shared instance from being created until it
    /// is dropped. Safe local-device constructors reject this loader; use
    /// [`Self::vmvx`] with a shared instance instead.
    #[cfg(feature = "cpu")]
    pub unsafe fn vmvx_isolated(modules: &[crate::vm::Module]) -> Result<Self> {
        let isolated_type_registry = Arc::new(crate::vm::IsolatedTypeRegistryGuard::begin()?);
        let mut modules: Vec<_> = modules.iter().map(crate::vm::Module::as_raw).collect();
        let mut raw = ptr::null_mut();
        // SAFETY: the modules are live; the loader retains them.
        check(unsafe {
            n::iree_hal_vmvx_module_loader_create_isolated(
                modules.len(),
                modules.as_mut_ptr(),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            _isolated_type_registry: Some(isolated_type_registry),
        })
    }
}

impl Clone for ExecutableLoader {
    fn clone(&self) -> Self {
        // SAFETY: the live loader is retained before creating another owner.
        unsafe { n::iree_hal_executable_loader_retain(self.raw) };
        Self {
            raw: self.raw,
            #[cfg(feature = "cpu")]
            _isolated_type_registry: self._isolated_type_registry.clone(),
        }
    }
}

impl Drop for ExecutableLoader {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one loader reference.
        unsafe { n::iree_hal_executable_loader_release(self.raw) }
    }
}

#[cfg(any(feature = "local-task", feature = "local-sync"))]
pub(crate) fn ensure_shared_type_registry(loaders: &[ExecutableLoader]) -> crate::Result<()> {
    if loaders
        .iter()
        .any(ExecutableLoader::uses_isolated_type_registry)
    {
        return Err(crate::Error::new(
            crate::Code::FailedPrecondition,
            "safe local-device constructors require VMVX loaders bound to the shared VM instance",
        ));
    }
    Ok(())
}

/// Retained registry of executable import plugins.
#[cfg(feature = "cpu")]
pub struct ExecutablePluginManager(*mut n::iree_hal_executable_plugin_manager_t);

#[cfg(feature = "cpu")]
impl ExecutablePluginManager {
    /// Creates a manager with room for `capacity` plugins.
    pub fn new(capacity: usize) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: `raw` is writable.
        check(unsafe {
            n::iree_hal_executable_plugin_manager_create(capacity, system(), &mut raw)
        })?;
        Ok(Self(raw))
    }

    /// Borrows the native manager pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_executable_plugin_manager_t {
        self.0
    }

    /// Registers a plugin; the manager retains it.
    pub fn register(&self, plugin: &ExecutablePlugin) -> Result<()> {
        // SAFETY: both objects are live.
        check(unsafe { n::iree_hal_executable_plugin_manager_register_plugin(self.0, plugin.0) })
    }
}

#[cfg(feature = "cpu")]
impl Clone for ExecutablePluginManager {
    fn clone(&self) -> Self {
        // SAFETY: the live manager is retained before creating another owner.
        unsafe { n::iree_hal_executable_plugin_manager_retain(self.0) };
        Self(self.0)
    }
}

#[cfg(feature = "cpu")]
impl Drop for ExecutablePluginManager {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one manager reference.
        unsafe { n::iree_hal_executable_plugin_manager_release(self.0) }
    }
}

// SAFETY: upstream documents the manager as thread-safe.
#[cfg(feature = "cpu")]
unsafe impl Send for ExecutablePluginManager {}
// SAFETY: upstream documents the manager as thread-safe.
#[cfg(feature = "cpu")]
unsafe impl Sync for ExecutablePluginManager {}

/// A retained executable import plugin.
#[cfg(feature = "cpu")]
pub struct ExecutablePlugin(*mut n::iree_hal_executable_plugin_t);

#[cfg(feature = "cpu")]
impl ExecutablePlugin {
    /// Loads a standalone embedded-ELF plugin from bytes.
    ///
    /// # Safety
    /// The executable code and its constructors must be trusted and obey
    /// the IREE plugin ABI, including every imported function contract.
    pub unsafe fn embedded_elf_from_memory(bytes: &[u8], params: &[(&str, &str)]) -> Result<Self> {
        let params = raw_params(params);
        let mut raw = ptr::null_mut();
        // SAFETY: `bytes` and `params` outlive the call; the plugin copies what it keeps.
        check(unsafe {
            n::iree_hal_embedded_elf_executable_plugin_load_from_memory(
                n::const_span(bytes),
                params.len(),
                params.as_ptr(),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }

    /// Loads a standalone embedded-ELF plugin from a file.
    ///
    /// # Safety
    /// The file's executable code and constructors must be trusted and obey
    /// the IREE plugin ABI, including every imported function contract.
    pub unsafe fn embedded_elf_from_file(
        path: impl AsRef<std::path::Path>,
        params: &[(&str, &str)],
    ) -> Result<Self> {
        let path = crate::io::path_str(path.as_ref())?;
        let params = raw_params(params);
        let mut raw = ptr::null_mut();
        // SAFETY: `path` and `params` outlive the call.
        check(unsafe {
            n::iree_hal_embedded_elf_executable_plugin_load_from_file(
                view(path),
                params.len(),
                params.as_ptr(),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }

    /// Loads a platform dynamic-library plugin from a file.
    ///
    /// # Safety
    /// The library's executable code and constructors must be trusted and
    /// obey the IREE plugin ABI, including every imported function contract.
    pub unsafe fn system_library_from_file(
        path: impl AsRef<std::path::Path>,
        params: &[(&str, &str)],
    ) -> Result<Self> {
        let path = std::ffi::CString::new(crate::io::path_str(path.as_ref())?)
            .map_err(|_| Error::new(Code::InvalidArgument, "plugin path contains NUL"))?;
        let params = raw_params(params);
        let mut raw = ptr::null_mut();
        // SAFETY: `path` and `params` outlive the call.
        check(unsafe {
            n::iree_hal_system_library_executable_plugin_load_from_file(
                path.as_ptr(),
                params.len(),
                params.as_ptr(),
                system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }

    /// Borrows the native plugin pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_executable_plugin_t {
        self.0
    }
}

#[cfg(feature = "cpu")]
impl Clone for ExecutablePlugin {
    fn clone(&self) -> Self {
        // SAFETY: the live plugin is retained before creating another owner.
        unsafe { n::iree_hal_executable_plugin_retain(self.0) };
        Self(self.0)
    }
}

#[cfg(feature = "cpu")]
impl Drop for ExecutablePlugin {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one plugin reference.
        unsafe { n::iree_hal_executable_plugin_release(self.0) }
    }
}

#[cfg(feature = "cpu")]
fn raw_params(params: &[(&str, &str)]) -> Vec<n::iree_string_pair_t> {
    params
        .iter()
        .map(|(key, value)| {
            let mut pair = n::iree_string_pair_t::default();
            pair.__bindgen_anon_1.key = view(key);
            pair.__bindgen_anon_2.value = view(value);
            pair
        })
        .collect()
}
