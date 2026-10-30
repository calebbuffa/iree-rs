//! Public causal-frontier cases corresponding to `frontier_test.cc`.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::asynchronous::{Axis, CausalDomain, Comparison, Frontier};

fn axis(ordinal: u64) -> Axis {
    Axis::new(1, 0, CausalDomain::Host, ordinal)
}

#[test]
fn axis_fields_round_trip() {
    let value = Axis::new(7, 9, CausalDomain::Host, 42);
    assert_eq!(value.session(), 7);
    assert_eq!(value.machine(), 9);
    assert_eq!(value.domain(), Some(CausalDomain::Host));
    assert_eq!(value.ordinal(), 42);
    let queue = Axis::queue(1, 2, 3, 4);
    assert_eq!(queue.device_index(), Some(3));
    assert_eq!(queue.queue_index(), Some(4));
}

#[test]
fn empty_sorted_duplicate_and_ordering_cases() {
    let empty = Frontier::default();
    assert!(empty.is_empty());
    assert_eq!(empty.compare(&empty), Comparison::Equal);
    let left = Frontier::new(&[(axis(2), 3), (axis(1), 2), (axis(1), 5)]).unwrap();
    assert_eq!(left.entries(), vec![(axis(1), 5), (axis(2), 3)]);
    let before = Frontier::new(&[(axis(1), 4)]).unwrap();
    let after = Frontier::new(&[(axis(1), 5)]).unwrap();
    assert_eq!(before.compare(&after), Comparison::Before);
    assert_eq!(after.compare(&before), Comparison::After);
    let concurrent = Frontier::new(&[(axis(2), 3)]).unwrap();
    assert_eq!(before.compare(&concurrent), Comparison::Concurrent);
}

#[test]
fn join_is_idempotent_commutative_and_satisfies_dependencies() {
    let a = Frontier::new(&[(axis(1), 3)]).unwrap();
    let b = Frontier::new(&[(axis(1), 5), (axis(2), 8)]).unwrap();
    let mut aa = a.clone();
    aa.merge(&a).unwrap();
    assert_eq!(aa.entries(), a.entries());
    let mut ab = a.clone();
    ab.merge(&b).unwrap();
    let mut ba = b.clone();
    ba.merge(&a).unwrap();
    assert_eq!(ab.entries(), ba.entries());
    assert!(a.is_satisfied(&ab));
    assert!(!b.is_satisfied(&a));
}

#[test]
fn frontier_capacity_limit_is_checked() {
    let entries: Vec<_> = (0..256).map(|id| (axis(id), id)).collect();
    assert!(Frontier::new(&entries).is_err());
}
