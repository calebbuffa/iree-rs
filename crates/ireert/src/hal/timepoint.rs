//! External semaphore timepoints: one payload value of a HAL timeline
//! semaphore expressed as a binary, platform- or API-native handle.
//!
//! A timepoint is *unsignaled* while `semaphore < value` and *signaled* once
//! `semaphore >= value`. Two directions exist:
//! - **import**: a foreign handle signals the HAL semaphore to `value` when it
//!   becomes ready (foreign producer → IREE consumer);
//! - **export**: IREE creates a foreign handle that becomes ready when the HAL
//!   semaphore reaches `value` (IREE producer → foreign consumer).
//!
//! Platform primitives (eventfd, Win32 events, Mach ports) go through the
//! semaphore's proactor and work on every driver whose semaphores were created
//! with a proactor. Driver-native events (HIP/CUDA) require backend-specific
//! interop instead of these platform primitives.
//!
//! # Laws
//! - Import transfers ownership of the primitive only on success; on failure
//!   the primitive is closed by its Rust owner exactly once.
//! - A safely exported primitive is fully owned: no native writer can touch it
//!   after it is returned, so dropping it at any time is sound.
use super::{QueueAffinity, Semaphore, SemaphoreCompatibility};
use crate::asynchronous::Primitive;
use crate::{Code, Error, Result, check};
use iree_native as n;

/// Rejects semaphores whose proactor is absent: the base layer dereferences it
/// unconditionally (only a debug assertion guards it natively).
fn require_proactor(semaphore: &Semaphore) -> Result<()> {
    // SAFETY: every HAL semaphore begins with an `iree_async_semaphore_t`
    // header (toll-free bridged in `hal/semaphore.h`); reading the pointer
    // field does not race because it is immutable after initialization.
    let proactor = unsafe { (*semaphore.as_raw().cast::<n::iree_async_semaphore_t>()).proactor };
    if proactor.is_null() {
        return Err(Error::new(
            Code::FailedPrecondition,
            "semaphore has no proactor; platform timepoints are unavailable",
        ));
    }
    Ok(())
}

fn require_payload(value: u64) -> Result<()> {
    if value > n::IREE_HAL_SEMAPHORE_MAX_VALUE as u64 {
        return Err(Error::new(
            Code::InvalidArgument,
            "timepoint value exceeds the maximum semaphore payload",
        ));
    }
    Ok(())
}

/// Imports a native external timepoint.
///
/// # Safety
/// `external` must be valid for its `type_`; ownership of its handle is
/// transferred to the semaphore on success, per the type's documented rules.
/// Every object the semaphore's signal path depends on (its device, driver
/// and loaded libraries) must stay alive until the timepoint retires.
pub(crate) unsafe fn import_external(
    semaphore: &Semaphore,
    value: u64,
    affinity: QueueAffinity,
    external: n::iree_hal_external_timepoint_t,
) -> Result<()> {
    require_payload(value)?;
    // SAFETY: the caller upholds the handle and lifetime contracts.
    check(unsafe {
        n::iree_hal_semaphore_import_timepoint(semaphore.as_raw(), value, affinity.bits(), external)
    })
}

/// Exports a native external timepoint of `requested_type`.
///
/// The returned handle's ownership and destruction rules are those of its type.
pub(crate) fn export_external(
    semaphore: &Semaphore,
    value: u64,
    affinity: QueueAffinity,
    requested_type: n::iree_hal_external_timepoint_type_t,
) -> Result<n::iree_hal_external_timepoint_t> {
    require_payload(value)?;
    // SAFETY: all-zero is the documented empty timepoint (the C API memsets it).
    let mut out: n::iree_hal_external_timepoint_t = unsafe { core::mem::zeroed() };
    // SAFETY: semaphore is live and `out` is writable.
    check(unsafe {
        n::iree_hal_semaphore_export_timepoint(
            semaphore.as_raw(),
            value,
            affinity.bits(),
            requested_type,
            n::IREE_HAL_EXTERNAL_TIMEPOINT_FLAG_NONE as _,
            &mut out,
        )
    })?;
    Ok(out)
}

impl Semaphore {
    /// Signals this semaphore to `value` when `primitive` becomes ready.
    ///
    /// On success the semaphore's proactor owns and eventually closes the
    /// primitive; keep a [`Primitive::try_clone`] duplicate to signal it. On
    /// failure (including platforms whose proactor lacks fence import, such as
    /// IOCP) the primitive is closed here.
    ///
    /// # Safety
    /// The proactor retains the semaphore object but not what its signal path
    /// depends on: the creating device, driver and loaded libraries must stay
    /// alive until the primitive becomes ready or the import is cancelled by
    /// proactor shutdown.
    pub unsafe fn import_primitive_timepoint(
        &self,
        value: u64,
        affinity: QueueAffinity,
        primitive: Primitive,
    ) -> Result<()> {
        require_proactor(self)?;
        let external = n::iree_hal_external_timepoint_t {
            type_: n::IREE_HAL_EXTERNAL_TIMEPOINT_TYPE_ASYNC_PRIMITIVE,
            flags: n::IREE_HAL_EXTERNAL_TIMEPOINT_FLAG_NONE as _,
            compatibility: SemaphoreCompatibility::HOST_ONLY.bits(),
            handle: n::iree_hal_external_timepoint_t__bindgen_ty_1 {
                async_primitive: primitive.as_raw(),
            },
        };
        // SAFETY: the primitive is live and owned; the caller upholds the
        // signal-path lifetime contract.
        unsafe { import_external(self, value, affinity, external) }?;
        // Ownership moved to the proactor; do not close our copy.
        let _ = primitive.into_raw();
        Ok(())
    }

    /// Exports a timepoint the timeline has **already reached** as an owned
    /// primitive (already ready).
    ///
    /// Restricting export to reached values is what makes this safe: the
    /// native writer then runs synchronously inside registration, so nothing
    /// writes to the handle after it is returned. Pending values return
    /// `FailedPrecondition`; see
    /// [`Semaphore::export_pending_primitive_timepoint`].
    pub fn export_reached_primitive_timepoint(
        &self,
        value: u64,
        affinity: QueueAffinity,
    ) -> Result<Primitive> {
        require_proactor(self)?;
        if self.query()? < value {
            return Err(Error::new(
                Code::FailedPrecondition,
                "timepoint not yet reached; pending export is unsafe",
            ));
        }
        // SAFETY: the value is reached, so the writer has retired (see above).
        unsafe { self.export_pending_primitive_timepoint(value, affinity) }
    }

    /// Exports a timepoint as an owned primitive that becomes ready when the
    /// timeline reaches `value`.
    ///
    /// # Safety
    /// Until the timeline reaches `value` (or fails), the proactor's writer
    /// still targets the returned handle: it must not be closed (dropped)
    /// before then, or the write may land on a reused descriptor.
    pub unsafe fn export_pending_primitive_timepoint(
        &self,
        value: u64,
        affinity: QueueAffinity,
    ) -> Result<Primitive> {
        require_proactor(self)?;
        let out = export_external(
            self,
            value,
            affinity,
            n::IREE_HAL_EXTERNAL_TIMEPOINT_TYPE_ASYNC_PRIMITIVE,
        )?;
        // SAFETY: on success the caller owns the exported primitive.
        Ok(unsafe { Primitive::from_raw(out.handle.async_primitive) })
    }
}
