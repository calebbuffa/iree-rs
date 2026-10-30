//! Causal timeline identifiers.
use iree_native as n;

/// The C causal domain vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CausalDomain {
    /// A device queue.
    Queue = n::IREE_ASYNC_CAUSAL_DOMAIN_QUEUE as u8,
    /// A collective channel.
    Collective = n::IREE_ASYNC_CAUSAL_DOMAIN_COLLECTIVE as u8,
    /// An epilogue timeline.
    Epilogue = n::IREE_ASYNC_CAUSAL_DOMAIN_EPILOGUE as u8,
    /// A host context.
    Host = n::IREE_ASYNC_CAUSAL_DOMAIN_HOST as u8,
}
/// A deterministic, value-semantic native axis identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Axis(u64);
impl Axis {
    /// Encodes an axis, masking the ordinal to the C API's low 40 bits.
    pub fn new(session: u8, machine: u8, domain: CausalDomain, ordinal: u64) -> Self {
        // SAFETY: Axis encoding operates only on by-value integers and has no pointer preconditions.
        Self(unsafe { n::iree_async_axis_make(session, machine, domain as u8, ordinal) })
    }
    /// Encodes a device queue axis.
    pub fn queue(session: u8, machine: u8, device: u8, queue: u8) -> Self {
        // SAFETY: Queue-axis encoding operates only on by-value indices and has no pointer preconditions.
        Self(unsafe { n::iree_async_axis_make_queue(session, machine, device, queue) })
    }
    /// Adopts a native axis value.
    ///
    /// # Safety
    /// The encoded domain must be meaningful for the native consumer.
    pub unsafe fn from_raw(raw: n::iree_async_axis_t) -> Self {
        Self(raw)
    }
    /// Returns the native axis value.
    pub fn as_raw(&self) -> n::iree_async_axis_t {
        self.0
    }
    /// Returns the encoded session epoch.
    pub fn session(self) -> u8 {
        // SAFETY: The packed axis is a by-value integer accepted by this accessor.
        unsafe { n::iree_async_axis_session(self.0) }
    }
    /// Returns the encoded machine index.
    pub fn machine(self) -> u8 {
        // SAFETY: The packed axis is a by-value integer accepted by this accessor.
        unsafe { n::iree_async_axis_machine(self.0) }
    }
    /// Returns the recognized causal domain, or `None` for an extension domain.
    pub fn domain(self) -> Option<CausalDomain> {
        [
            CausalDomain::Queue,
            CausalDomain::Collective,
            CausalDomain::Epilogue,
            CausalDomain::Host,
        ]
        .into_iter()
        // SAFETY: The packed axis is a by-value integer accepted by this accessor.
        .find(|domain| *domain as u8 == unsafe { n::iree_async_axis_domain(self.0) })
    }
    /// Returns the queue domain's device index, or `None` for other domains.
    pub fn device_index(self) -> Option<u8> {
        (self.domain() == Some(CausalDomain::Queue))
            // SAFETY: The queue domain was checked before reading its packed device index.
            .then(|| unsafe { n::iree_async_axis_device_index(self.0) })
    }
    /// Returns the queue domain's queue index, or `None` for other domains.
    pub fn queue_index(self) -> Option<u8> {
        (self.domain() == Some(CausalDomain::Queue))
            // SAFETY: The queue domain was checked before reading its packed queue index.
            .then(|| unsafe { n::iree_async_axis_queue_index(self.0) })
    }
    /// Returns the 40-bit ordinal.
    pub fn ordinal(self) -> u64 {
        // SAFETY: The packed axis is a by-value integer accepted by this accessor.
        unsafe { n::iree_async_axis_ordinal(self.0) }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn encoding_round_trip(session in any::<u8>(), machine in any::<u8>(), ordinal in any::<u64>()) {
            let axis = Axis::new(session, machine, CausalDomain::Host, ordinal);
            prop_assert_eq!(axis.session(), session);
            prop_assert_eq!(axis.machine(), machine);
            prop_assert_eq!(axis.ordinal(), ordinal & 0xFF_FFFF_FFFF);
        }
    }
}
