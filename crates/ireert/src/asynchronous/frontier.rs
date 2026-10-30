//! Compact causal frontiers: finite maps with component-wise maximum as join.
use super::Axis;
use crate::{Code, Error, Result, check};
use iree_native as n;

/// The native causal partial order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    /// Both frontiers describe the same cut.
    Equal,
    /// This cut precedes the other cut.
    Before,
    /// This cut follows the other cut.
    After,
    /// Neither cut dominates the other.
    Concurrent,
}
/// An aligned, owned native frontier with room for all 255 entries.
///
/// Entries are sorted by axis and unique. Join is idempotent, commutative,
/// and associative whenever the union fits the native 255-axis limit.
#[derive(Clone)]
pub struct Frontier(Box<[u64]>);
impl Frontier {
    /// Constructs a frontier, merging duplicate axes by maximum epoch.
    pub fn new(entries: &[(Axis, u64)]) -> Result<Self> {
        let mut map = std::collections::BTreeMap::new();
        for &(axis, epoch) in entries {
            map.entry(axis)
                .and_modify(|v: &mut u64| *v = (*v).max(epoch))
                .or_insert(epoch);
        }
        if map.len() > u8::MAX as usize {
            return Err(Error::new(
                Code::ResourceExhausted,
                "frontier exceeds 255 axes",
            ));
        }
        let mut value = Self::default();
        // SAFETY: the allocation holds 255 entries and `map.len()` fits in `u8`.
        unsafe {
            n::iree_async_frontier_initialize(value.as_mut_ptr(), map.len() as u8);
            let entries = (*value.as_mut_ptr()).entries.as_mut_ptr();
            for (index, (axis, epoch)) in map.into_iter().enumerate() {
                entries.add(index).write(n::iree_async_frontier_entry_t {
                    axis: axis.as_raw(),
                    epoch,
                });
            }
        }
        // SAFETY: value was initialized with a valid entry count and remains exclusively owned here.
        check(unsafe { n::iree_async_frontier_validate(value.as_raw()) })?;
        Ok(value)
    }
    /// Borrows the native frontier without permitting safe mutation.
    pub fn as_raw(&self) -> *const n::iree_async_frontier_t {
        self.0.as_ptr().cast()
    }
    pub(crate) fn as_mut_ptr(&mut self) -> *mut n::iree_async_frontier_t {
        self.0.as_mut_ptr().cast()
    }
    /// Returns the number of active entries.
    pub fn len(&self) -> usize {
        // SAFETY: Frontier owns this initialized, aligned native allocation.
        unsafe { (*self.as_raw()).entry_count as usize }
    }
    /// Returns whether there are no dependencies.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the sorted axis/epoch entries using Rust values only.
    pub fn entries(&self) -> Vec<(Axis, u64)> {
        // SAFETY: Frontier owns the entries array and len is its initialized entry count.
        unsafe { n::slice((*self.as_raw()).entries.as_ptr(), self.len()) }
            .iter()
            // SAFETY: Each stored axis originated from a valid Axis value.
            .map(|e| (unsafe { Axis::from_raw(e.axis) }, e.epoch))
            .collect()
    }
    /// Compares the two native cuts.
    pub fn compare(&self, other: &Self) -> Comparison {
        // SAFETY: both frontiers are valid, initialized native frontiers.
        match unsafe { n::iree_async_frontier_compare(self.as_raw(), other.as_raw()) }
            as n::iree_async_frontier_comparison_e
        {
            n::IREE_ASYNC_FRONTIER_EQUAL => Comparison::Equal,
            n::IREE_ASYNC_FRONTIER_BEFORE => Comparison::Before,
            n::IREE_ASYNC_FRONTIER_AFTER => Comparison::After,
            _ => Comparison::Concurrent,
        }
    }
    /// Joins another frontier, leaving `self` unchanged on capacity failure.
    pub fn merge(&mut self, other: &Self) -> Result<()> {
        let mut next = self.clone();
        // SAFETY: Both frontiers are initialized and next has capacity for the 255-axis union.
        if !unsafe { n::iree_async_frontier_merge(next.as_mut_ptr(), u8::MAX, other.as_raw()) } {
            return Err(Error::new(
                Code::ResourceExhausted,
                "frontier union exceeds 255 axes",
            ));
        }
        *self = next;
        Ok(())
    }
    /// Tests whether this dependency cut is satisfied by the current cut.
    pub fn is_satisfied(&self, current: &Self) -> bool {
        // SAFETY: The current frontier owns an initialized entries array covering current.len().
        unsafe {
            n::iree_async_frontier_is_satisfied(
                self.as_raw(),
                (*current.as_raw()).entries.as_ptr(),
                current.len(),
            )
        }
    }
}
impl Default for Frontier {
    fn default() -> Self {
        let mut value = Self(vec![0u64; 1 + 2 * u8::MAX as usize].into_boxed_slice());
        // SAFETY: the aligned allocation has capacity for every native entry.
        unsafe { n::iree_async_frontier_initialize(value.as_mut_ptr(), 0) };
        value
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    fn frontier(entries: Vec<(u8, u64)>) -> Frontier {
        Frontier::new(
            &entries
                .into_iter()
                .map(|(a, e)| {
                    (
                        Axis::new(1, 0, super::super::CausalDomain::Host, a as u64),
                        e,
                    )
                })
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
    proptest! {
        #[test]
        fn join_laws(
            a in prop::collection::vec((any::<u8>(), 1u64..10000), 0..30),
            b in prop::collection::vec((any::<u8>(), 1u64..10000), 0..30),
            c in prop::collection::vec((any::<u8>(), 1u64..10000), 0..30),
        ) {
            let a = frontier(a);
            let b = frontier(b);
            let c = frontier(c);
            let mut aa = a.clone();
            aa.merge(&a).unwrap();
            prop_assert_eq!(aa.entries(), a.entries());
            let mut ab = a.clone();
            ab.merge(&b).unwrap();
            let mut ba = b.clone();
            ba.merge(&a).unwrap();
            prop_assert_eq!(ab.entries(), ba.entries());
            let mut left = ab;
            left.merge(&c).unwrap();
            let mut bc = b;
            bc.merge(&c).unwrap();
            let mut right = a;
            right.merge(&bc).unwrap();
            prop_assert_eq!(left.entries(), right.entries());
        }
    }
}
