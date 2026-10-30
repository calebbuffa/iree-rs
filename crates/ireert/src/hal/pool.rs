//! Common HAL allocation-pool and reservation APIs.
use super::{Buffer, BufferParams, BufferUsage, GroupedDevice, MemoryType, QueueAffinity};
use crate::asynchronous::frontier::Frontier;
use crate::hal::DeviceSize;
use crate::{Code, Error, Result, check};
use iree_native as n;

/// Outcome of an attempted pool reservation.
pub enum ReservationAcquire {
    /// A reservation is available for immediate use.
    Reserved(PoolReservation),
    /// No immediately usable reservation is currently available.
    Exhausted,
    /// The configured logical allocation budget would be exceeded.
    OverBudget,
}

/// Device-owned ingredients for building queue-allocation pools.
///
/// The borrowed slab provider, notification and epoch query belong to the device.
/// Pools built from this backend retain the device, so they never outlive it.
pub struct QueuePoolBackend<'d> {
    device: &'d GroupedDevice,
    raw: n::iree_hal_queue_pool_backend_t,
}
impl<'d> QueuePoolBackend<'d> {
    /// Returns the device that owns this backend.
    pub fn device(&self) -> &'d GroupedDevice {
        self.device
    }

    /// Returns the borrowed native backend bundle.
    pub fn as_raw(&self) -> &n::iree_hal_queue_pool_backend_t {
        &self.raw
    }
}

impl GroupedDevice {
    /// Queries the backend used to build custom queue-allocation pools.
    ///
    /// The device must belong to a [`super::DeviceGroup`].
    pub fn queue_pool_backend(&self, affinity: QueueAffinity) -> Result<QueuePoolBackend<'_>> {
        let mut raw = n::iree_hal_queue_pool_backend_t::default();
        // SAFETY: the device is live and the output storage is writable.
        check(unsafe {
            n::iree_hal_device_query_queue_pool_backend(self.as_raw(), affinity.bits(), &mut raw)
        })?;
        Ok(QueuePoolBackend { device: self, raw })
    }
}

/// Options for a two-level segregated-fit pool.
#[derive(Clone, Copy, Debug)]
pub struct TlsfPoolOptions<'a> {
    /// Managed offset range in bytes; must be nonzero.
    pub range_length: DeviceSize,
    /// Power-of-two alignment of every offset and length.
    pub alignment: DeviceSize,
    /// Initial block-metadata capacity; zero selects the upstream default.
    pub initial_block_capacity: usize,
    /// Frontier entries kept per free block; zero selects the upstream default.
    pub frontier_capacity: u8,
    /// Logical allocation budget in bytes; zero means unlimited.
    pub budget_limit: DeviceSize,
    /// Tracing memory identifier; empty disables named tracing.
    pub trace_name: &'a str,
}
impl TlsfPoolOptions<'_> {
    /// Smallest permitted alignment.
    pub const MIN_ALIGNMENT: DeviceSize = n::IREE_HAL_MEMORY_TLSF_MIN_ALIGNMENT as DeviceSize;

    /// Creates options for `range_length` bytes with upstream defaults.
    pub fn new(range_length: DeviceSize) -> Self {
        Self {
            range_length,
            alignment: Self::MIN_ALIGNMENT,
            initial_block_capacity: 0,
            frontier_capacity: 0,
            budget_limit: 0,
            trace_name: "",
        }
    }
}

/// Options for a pool of equally sized blocks.
#[derive(Clone, Copy, Debug)]
pub struct FixedBlockPoolOptions<'a> {
    /// Size of each block in bytes; must be nonzero.
    pub block_size: DeviceSize,
    /// Number of blocks; at most [`Self::MAX_BLOCKS`].
    pub block_count: u32,
    /// Frontier entries kept per block; zero selects the upstream default.
    pub frontier_capacity: u16,
    /// Logical allocation budget in bytes; zero means unlimited.
    pub budget_limit: DeviceSize,
    /// Tracing memory identifier; empty disables named tracing.
    pub trace_name: &'a str,
}
impl FixedBlockPoolOptions<'_> {
    /// Largest permitted block count.
    pub const MAX_BLOCKS: u32 = n::IREE_HAL_MEMORY_FIXED_BLOCK_ALLOCATOR_MAX_BLOCKS;

    /// Creates options for `block_count` blocks of `block_size` bytes.
    pub fn new(block_size: DeviceSize, block_count: u32) -> Self {
        Self {
            block_size,
            block_count,
            frontier_capacity: 0,
            budget_limit: 0,
            trace_name: "",
        }
    }
}

/// An owned `iree_hal_pool_t` reference.
pub struct Pool {
    raw: *mut n::iree_hal_pool_t,
    device: Option<GroupedDevice>,
}
impl Pool {
    /// Creates a two-level segregated-fit pool over a device backend.
    pub fn tlsf(backend: &QueuePoolBackend<'_>, options: &TlsfPoolOptions<'_>) -> Result<Self> {
        let raw_options = n::iree_hal_tlsf_pool_options_t {
            tlsf_options: n::iree_hal_memory_tlsf_options_t {
                range_length: options.range_length,
                alignment: options.alignment,
                initial_block_capacity: options.initial_block_capacity,
                frontier_capacity: options.frontier_capacity,
            },
            budget_limit: options.budget_limit,
            trace_name: crate::view(options.trace_name),
        };
        Self::create(backend, |out| {
            // SAFETY: the backend is borrowed from a live device; the name is copied.
            unsafe {
                n::iree_hal_tlsf_pool_create(
                    raw_options,
                    backend.raw.slab_provider,
                    backend.raw.notification,
                    backend.raw.epoch_query,
                    crate::system(),
                    out,
                )
            }
        })
    }

    /// Creates a fixed-block pool over a device backend.
    pub fn fixed_block(
        backend: &QueuePoolBackend<'_>,
        options: &FixedBlockPoolOptions<'_>,
    ) -> Result<Self> {
        let raw_options = n::iree_hal_fixed_block_pool_options_t {
            block_allocator_options: n::iree_hal_memory_fixed_block_allocator_options_t {
                block_size: options.block_size,
                block_count: options.block_count,
                frontier_capacity: options.frontier_capacity,
            },
            budget_limit: options.budget_limit,
            trace_name: crate::view(options.trace_name),
        };
        Self::create(backend, |out| {
            // SAFETY: the backend is borrowed from a live device; the name is copied.
            unsafe {
                n::iree_hal_fixed_block_pool_create(
                    raw_options,
                    backend.raw.slab_provider,
                    backend.raw.notification,
                    backend.raw.epoch_query,
                    crate::system(),
                    out,
                )
            }
        })
    }

    fn create(
        backend: &QueuePoolBackend<'_>,
        call: impl FnOnce(*mut *mut n::iree_hal_pool_t) -> n::iree_status_t,
    ) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        check(call(&mut raw))?;
        Ok(Self {
            raw,
            device: Some(backend.device.clone()),
        })
    }

    /// Adopts one owned pool reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned pool reference that may be released exactly once,
    /// and any device state it borrows must outlive it.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_pool_t) -> Self {
        Self { raw, device: None }
    }

    /// Returns the borrowed native pool handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_pool_t {
        self.raw
    }

    /// Transfers ownership of the native pool handle to the caller.
    ///
    /// Any device retained by this pool is released, so the caller must keep it alive.
    pub fn into_raw(self) -> *mut n::iree_hal_pool_t {
        let mut this = core::mem::ManuallyDrop::new(self);
        drop(this.device.take());
        this.raw
    }

    /// Attempts to reserve storage without permitting hidden wait-frontier dependencies.
    pub fn acquire_reservation(
        &self,
        size: DeviceSize,
        alignment: DeviceSize,
        requester_frontier: Option<&Frontier>,
    ) -> Result<ReservationAcquire> {
        let mut reservation = n::iree_hal_pool_reservation_t::default();
        let mut info = n::iree_hal_pool_acquire_info_t::default();
        let mut result = n::IREE_HAL_POOL_ACQUIRE_EXHAUSTED as n::iree_hal_pool_acquire_result_t;
        // SAFETY: arguments and output storage remain valid for the synchronous native call.
        check(unsafe {
            n::iree_hal_pool_acquire_reservation(
                self.raw,
                size,
                alignment,
                requester_frontier.map_or(core::ptr::null(), |f| f.as_raw()),
                n::IREE_HAL_POOL_RESERVE_FLAG_NONE as n::iree_hal_pool_reserve_flags_t,
                &mut reservation,
                &mut info,
                &mut result,
            )
        })?;
        match result as n::iree_hal_pool_acquire_result_e {
            n::IREE_HAL_POOL_ACQUIRE_OK | n::IREE_HAL_POOL_ACQUIRE_OK_FRESH => {
                Ok(ReservationAcquire::Reserved(PoolReservation {
                    pool: self.clone(),
                    reservation,
                    released: false,
                }))
            }
            n::IREE_HAL_POOL_ACQUIRE_EXHAUSTED => Ok(ReservationAcquire::Exhausted),
            n::IREE_HAL_POOL_ACQUIRE_OVER_BUDGET => Ok(ReservationAcquire::OverBudget),
            n::IREE_HAL_POOL_ACQUIRE_OK_NEEDS_WAIT => {
                // SAFETY: the reservation was acquired above and is released exactly once.
                unsafe {
                    n::iree_hal_pool_release_reservation(self.raw, &reservation, info.wait_frontier)
                };
                Err(Error::new(
                    Code::FailedPrecondition,
                    "pool returned a wait-dependent reservation without permission",
                ))
            }
            _ => Err(Error::new(
                Code::Internal,
                "pool returned an unknown reservation result",
            )),
        }
    }

    /// Allocates a HAL buffer through this pool.
    pub fn allocate_buffer(
        &self,
        params: BufferParams,
        size: DeviceSize,
        requester_frontier: Option<&Frontier>,
        timeout: Option<core::time::Duration>,
    ) -> Result<Buffer> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: arguments and output storage remain valid for the synchronous native call.
        check(unsafe {
            n::iree_hal_pool_allocate_buffer(
                self.raw,
                params.raw(),
                size,
                requester_frontier.map_or(core::ptr::null(), |f| f.as_raw()),
                n::timeout(timeout)?,
                &mut raw,
            )
        })?;
        // SAFETY: successful allocation returns one owned buffer reference.
        Ok(unsafe { Buffer::from_raw(raw) })
    }

    /// Queries the memory capabilities of this pool.
    pub fn query_capabilities(&self) -> PoolCapabilities {
        let mut raw = n::iree_hal_pool_capabilities_t::default();
        // SAFETY: pool is live and raw storage is writable.
        unsafe { n::iree_hal_pool_query_capabilities(self.raw, &mut raw) };
        PoolCapabilities {
            memory_type: MemoryType::from_bits_retain(raw.memory_type),
            supported_usage: BufferUsage::from_bits_retain(raw.supported_usage),
            min_allocation_size: raw.min_allocation_size,
            max_allocation_size: raw.max_allocation_size,
        }
    }

    /// Captures an instantaneous snapshot of pool statistics.
    pub fn query_stats(&self) -> PoolStats {
        let mut raw = n::iree_hal_pool_stats_t::default();
        // SAFETY: pool is live and raw storage is writable.
        unsafe { n::iree_hal_pool_query_stats(self.raw, &mut raw) };
        PoolStats::from_raw(raw)
    }

    /// Trims unused pool storage.
    pub fn trim(&self) -> Result<()> {
        // SAFETY: pool is live for the trim call.
        check(unsafe { n::iree_hal_pool_trim(self.raw) })
    }
}
impl Clone for Pool {
    fn clone(&self) -> Self {
        // SAFETY: this wrapper owns a live pool reference.
        unsafe { n::iree_hal_pool_retain(self.raw) };
        Self {
            raw: self.raw,
            device: self.device.clone(),
        }
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one pool reference; the device drops afterwards.
        unsafe { n::iree_hal_pool_release(self.raw) }
    }
}

/// A pool reservation that releases automatically unless materialized.
pub struct PoolReservation {
    pool: Pool,
    reservation: n::iree_hal_pool_reservation_t,
    released: bool,
}
impl PoolReservation {
    /// Returns the reserved byte length.
    pub fn len(&self) -> DeviceSize {
        self.reservation.length
    }

    /// Returns whether the reservation is empty.
    pub fn is_empty(&self) -> bool {
        self.reservation.length == 0
    }

    /// Materializes the reservation as a buffer and transfers its release to that buffer.
    pub fn materialize(mut self, params: BufferParams) -> Result<Buffer> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: arguments and output storage remain valid for the synchronous native call.
        check(unsafe {
            n::iree_hal_pool_materialize_reservation(
                self.pool.as_raw(),
                params.raw(),
                &self.reservation,
                n::IREE_HAL_POOL_MATERIALIZE_FLAG_TRANSFER_RESERVATION_OWNERSHIP
                    as n::iree_hal_pool_materialize_flags_t,
                &mut raw,
            )
        })?;
        self.released = true;
        // SAFETY: successful materialization returns one owned buffer reference.
        Ok(unsafe { Buffer::from_raw(raw) })
    }

    /// Releases the reservation with an optional final-use frontier.
    pub fn release(mut self, death_frontier: Option<&Frontier>) {
        // SAFETY: pool and reservation are owned here; the frontier is borrowed through release.
        unsafe {
            n::iree_hal_pool_release_reservation(
                self.pool.as_raw(),
                &self.reservation,
                death_frontier.map_or(core::ptr::null(), |frontier| frontier.as_raw()),
            )
        };
        self.released = true;
    }
}
impl Drop for PoolReservation {
    fn drop(&mut self) {
        if !self.released {
            // SAFETY: pool and reservation are owned here; no death frontier was used.
            unsafe {
                n::iree_hal_pool_release_reservation(
                    self.pool.as_raw(),
                    &self.reservation,
                    core::ptr::null(),
                )
            };
        }
    }
}

/// Memory and allocation-size constraints provided by a pool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PoolCapabilities {
    /// Memory properties exposed by the pool.
    pub memory_type: MemoryType,
    /// Buffer usages supported by the pool.
    pub supported_usage: BufferUsage,
    /// Minimum allocation size in bytes.
    pub min_allocation_size: DeviceSize,
    /// Maximum allocation size in bytes; zero means no strategy-specific limit.
    pub max_allocation_size: DeviceSize,
}

/// Instantaneous counters reported by a pool.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PoolStats {
    /// Bytes in live reservations.
    pub bytes_reserved: DeviceSize,
    /// Bytes available for reservation.
    pub bytes_free: DeviceSize,
    /// Bytes of committed backing memory.
    pub bytes_committed: DeviceSize,
    /// Logical allocation budget; zero means unlimited.
    pub budget_limit: DeviceSize,
    /// Current reservation count.
    pub reservation_count: u32,
    /// Current slab count.
    pub slab_count: u32,
    /// Total successful reservation calls.
    pub reserve_count: u64,
    /// Total releases.
    pub release_count: u64,
    /// Reuses whose frontier was dominated.
    pub reuse_count: u64,
    /// Reuses that missed frontier dominance.
    pub reuse_miss_count: u64,
    /// Reservations from fresh offset space.
    pub fresh_count: u64,
    /// Exhausted acquisition results.
    pub exhausted_count: u64,
    /// Over-budget acquisition results.
    pub over_budget_count: u64,
    /// Wait-dependent acquisition results.
    pub wait_count: u64,
}
impl PoolStats {
    fn from_raw(raw: n::iree_hal_pool_stats_t) -> Self {
        Self {
            bytes_reserved: raw.bytes_reserved,
            bytes_free: raw.bytes_free,
            bytes_committed: raw.bytes_committed,
            budget_limit: raw.budget_limit,
            reservation_count: raw.reservation_count,
            slab_count: raw.slab_count,
            reserve_count: raw.reserve_count,
            release_count: raw.release_count,
            reuse_count: raw.reuse_count,
            reuse_miss_count: raw.reuse_miss_count,
            fresh_count: raw.fresh_count,
            exhausted_count: raw.exhausted_count,
            over_budget_count: raw.over_budget_count,
            wait_count: raw.wait_count,
        }
    }
}
