//! Proactor pools: process-level, NUMA-aware factories of proactors that HAL
//! devices and other subsystems share.
use super::Proactor;
use crate::{Code, Error, Result, check, system};
use core::{marker::PhantomData, mem::ManuallyDrop, ptr};
use iree_native as n;
use std::{cell::RefCell, rc::Rc, sync::OnceLock};

/// A retained pool whose proactors are polled by IREE's own runners.
///
/// The runners are dedicated poll threads on native targets; targets without
/// threads (wasm) have none, and there a [`LocalProactorPool`] driven by the
/// host loop is the right choice. Proactors are created lazily, on first use
/// by a device, so an unused pool costs nothing. Devices retain the pool;
/// releasing the last reference stops its runners.
pub struct ProactorPool(ptr::NonNull<n::iree_async_proactor_pool_t>);

// SAFETY: the pool is reference counted atomically and IREE synchronizes lazy
// creation; this wrapper never polls, so runner ownership is never violated.
unsafe impl Send for ProactorPool {}
// SAFETY: as above; shared access only reads immutable configuration.
unsafe impl Sync for ProactorPool {}

impl ProactorPool {
    /// Creates a pool with one unpinned proactor per NUMA node.
    pub fn new() -> Result<Self> {
        // SAFETY: a plain query.
        create(unsafe { n::iree_numa_node_count() }, None, true).map(Self)
    }

    /// Creates a pool with one proactor per listed NUMA node, each runner
    /// pinned to its node.
    pub fn with_nodes(node_ids: &[u32]) -> Result<Self> {
        create(node_ids.len(), Some(node_ids), true).map(Self)
    }

    /// The process-wide pool, created by [`new`](Self::new) on first use.
    ///
    /// It is never released, so its runners live until the process exits.
    pub fn shared() -> Result<Self> {
        static SHARED: OnceLock<ProactorPool> = OnceLock::new();
        if let Some(pool) = SHARED.get() {
            return Ok(pool.clone());
        }
        let pool = Self::new()?;
        // A concurrent initializer may win; its pool is equivalent.
        Ok(SHARED.get_or_init(|| pool).clone())
    }

    /// Adopts one owned pool reference.
    ///
    /// # Safety
    /// `raw` must be a live pool whose proactors are polled only by its own
    /// runners, or by whoever owns them outside this wrapper.
    pub unsafe fn from_raw(raw: ptr::NonNull<n::iree_async_proactor_pool_t>) -> Self {
        Self(raw)
    }

    /// Borrows the native pool.
    pub fn as_raw(&self) -> *mut n::iree_async_proactor_pool_t {
        self.0.as_ptr()
    }

    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_async_proactor_pool_t {
        ManuallyDrop::new(self).as_raw()
    }

    /// The number of proactor slots.
    pub fn len(&self) -> usize {
        len(self.as_raw())
    }

    /// Whether the pool has no slots; never true for a created pool.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The NUMA node of slot `index`, or `None` when unspecified or out of
    /// range.
    pub fn node_id(&self, index: usize) -> Option<u32> {
        node_id(self.as_raw(), index)
    }
}

impl Clone for ProactorPool {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_async_proactor_pool_retain(self.as_raw()) };
        Self(self.0)
    }
}

impl Drop for ProactorPool {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_async_proactor_pool_release(self.as_raw()) }
    }
}

/// A retained pool of caller-driven proactors, bound to the creating thread.
///
/// There are no runners: whoever creates the pool polls its proactors through
/// [`get`](Self::get), for example from a host event loop. This is how IREE
/// runs without threads, and how a host integrates I/O into its own loop.
#[derive(Clone)]
pub struct LocalProactorPool {
    raw: ptr::NonNull<n::iree_async_proactor_pool_t>,
    proactors: Rc<RefCell<Vec<Option<Proactor>>>>,
}

impl LocalProactorPool {
    /// Creates a pool with one proactor per NUMA node.
    pub fn new() -> Result<Self> {
        // SAFETY: a plain query.
        let raw = create(unsafe { n::iree_numa_node_count() }, None, false)?;
        // SAFETY: the pool is new, runner-less, and owned by this thread.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Creates a pool with one proactor per listed NUMA node.
    pub fn with_nodes(node_ids: &[u32]) -> Result<Self> {
        let raw = create(node_ids.len(), Some(node_ids), false)?;
        // SAFETY: as in `new`.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned, caller-driven pool reference.
    ///
    /// # Safety
    /// The pool must be live, have no runners, and each of its proactors must
    /// be unpolled or polled only by the current thread, through this wrapper.
    pub unsafe fn from_raw(raw: ptr::NonNull<n::iree_async_proactor_pool_t>) -> Self {
        Self {
            raw,
            proactors: Rc::new(RefCell::new(vec![None; len(raw.as_ptr())])),
        }
    }

    /// Borrows the native pool.
    pub fn as_raw(&self) -> *mut n::iree_async_proactor_pool_t {
        self.raw.as_ptr()
    }

    /// Transfers one pool reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_async_proactor_pool_t {
        // SAFETY: the wrapper owns a live reference; the new one is returned
        // and this wrapper's own is released on drop.
        unsafe { n::iree_async_proactor_pool_retain(self.as_raw()) };
        self.as_raw()
    }

    /// The number of proactor slots.
    pub fn len(&self) -> usize {
        self.proactors.borrow().len()
    }

    /// Whether the pool has no slots; never true for a created pool.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The NUMA node of slot `index`, or `None` when unspecified or out of
    /// range.
    pub fn node_id(&self, index: usize) -> Option<u32> {
        node_id(self.as_raw(), index)
    }

    /// The proactor of slot `index`, created on first use.
    pub fn get(&self, index: usize) -> Result<Proactor> {
        let mut slots = self.proactors.borrow_mut();
        let slot = slots
            .get_mut(index)
            .ok_or_else(|| Error::new(Code::OutOfRange, "proactor pool index"))?;
        if let Some(proactor) = slot {
            return Ok(proactor.clone());
        }
        let mut raw = ptr::null_mut();
        // SAFETY: the pool is live and `index` is in range.
        check(unsafe { n::iree_async_proactor_pool_get(self.as_raw(), index, &mut raw) })?;
        // SAFETY: the pool lends the proactor; one retained reference is adopted
        // on this, its polling thread.
        let proactor = unsafe {
            n::iree_async_proactor_retain(raw);
            Proactor::from_raw(raw)
        };
        Ok(slot.insert(proactor).clone())
    }

    /// The proactor for NUMA node `node`, falling back to the first slot.
    pub fn get_for_node(&self, node: u32) -> Result<Proactor> {
        let index = (0..self.len())
            .find(|&index| self.node_id(index) == Some(node))
            .unwrap_or(0);
        self.get(index)
    }
}

impl Drop for LocalProactorPool {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_async_proactor_pool_release(self.as_raw()) }
    }
}

/// Borrows a proactor pool for one device-creation call.
///
/// IREE requires a pool on every creation path; the device retains it, so the
/// pool need not outlive this value.
#[derive(Clone, Copy)]
pub struct PoolRef<'a> {
    raw: *mut n::iree_async_proactor_pool_t,
    _pool: PhantomData<&'a ()>,
}

impl<'a> From<&'a ProactorPool> for PoolRef<'a> {
    fn from(pool: &'a ProactorPool) -> Self {
        Self {
            raw: pool.as_raw(),
            _pool: PhantomData,
        }
    }
}

impl<'a> From<&'a LocalProactorPool> for PoolRef<'a> {
    fn from(pool: &'a LocalProactorPool) -> Self {
        Self {
            raw: pool.as_raw(),
            _pool: PhantomData,
        }
    }
}

impl PoolRef<'_> {
    /// Borrows the native pool.
    pub fn as_raw(&self) -> *mut n::iree_async_proactor_pool_t {
        self.raw
    }
}

fn create(
    count: usize,
    node_ids: Option<&[u32]>,
    runners: bool,
) -> Result<ptr::NonNull<n::iree_async_proactor_pool_t>> {
    if count == 0 {
        return Err(Error::new(Code::InvalidArgument, "empty proactor pool"));
    }
    // SAFETY: returns plain platform defaults.
    let mut options = unsafe { n::iree_async_proactor_pool_options_default() };
    if !runners {
        // SAFETY: a zeroed factory means no runner, per `proactor_pool.h`.
        options.runner = unsafe { core::mem::zeroed() };
    }
    let mut raw = ptr::null_mut();
    // SAFETY: `node_ids`, when given, holds `count` entries; the output is
    // writable.
    check(unsafe {
        n::iree_async_proactor_pool_create(
            count,
            node_ids.map_or(ptr::null(), <[u32]>::as_ptr),
            options,
            system(),
            &mut raw,
        )
    })?;
    ptr::NonNull::new(raw).ok_or_else(|| Error::new(Code::Internal, "null proactor pool"))
}

fn len(raw: *mut n::iree_async_proactor_pool_t) -> usize {
    // SAFETY: callers pass a live pool.
    unsafe { n::iree_async_proactor_pool_count(raw) }
}

fn node_id(raw: *mut n::iree_async_proactor_pool_t, index: usize) -> Option<u32> {
    if index >= len(raw) {
        return None;
    }
    // SAFETY: the pool is live and `index` is in range.
    let id = unsafe { n::iree_async_proactor_pool_node_id(raw, index) };
    (id != u32::MAX).then_some(id)
}
