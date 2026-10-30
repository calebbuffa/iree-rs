//! Conversions between Rust time values and IREE's native time representation.

use iree_native as n;
use std::time::{Duration, Instant};

/// Converts a relative duration to an absolute deadline in IREE's monotonic clock.
pub(crate) fn deadline_after(duration: Option<Duration>) -> n::iree_time_t {
    let Some(duration) = duration else {
        return n::iree_time_t::MAX;
    };
    let delta = n::iree_time_t::try_from(duration.as_nanos()).unwrap_or(n::iree_time_t::MAX);
    // SAFETY: the clock query has no preconditions.
    unsafe { n::iree_time_now() }.saturating_add(delta)
}

/// Converts a Rust monotonic deadline to IREE's independent monotonic epoch.
pub(crate) fn deadline_from_instant(deadline: Option<Instant>) -> n::iree_time_t {
    deadline_after(deadline.map(|deadline| deadline.saturating_duration_since(Instant::now())))
}

/// Converts an IREE deadline to a Rust one; `None` is the infinite future.
pub(crate) fn instant_from_deadline(deadline_ns: n::iree_time_t) -> Option<Instant> {
    if deadline_ns == n::iree_time_t::MAX {
        return None;
    }
    // SAFETY: the clock query has no preconditions.
    let remaining = deadline_ns.saturating_sub(unsafe { n::iree_time_now() });
    // Deadlines already passed (including the infinite past) resolve to now;
    // ones beyond `Instant`'s range are effectively infinite.
    Instant::now().checked_add(Duration::from_nanos(u64::try_from(remaining).unwrap_or(0)))
}
