//! Public frontier-tracker cases corresponding to `frontier_tracker_test.cc`.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::asynchronous::{Axis, CausalDomain, FrontierTracker};

#[test]
fn initialize_register_advance_and_validate() {
    let tracker = FrontierTracker::new(4, 3, 5).unwrap();
    assert_eq!(tracker.session_epoch(), 3);
    assert_eq!(tracker.machine_index(), 5);
    let axis = Axis::new(3, 5, CausalDomain::Host, 1);
    tracker.register_axis(axis, None).unwrap();
    assert!(tracker.register_axis(axis, None).is_err());
    assert!(tracker.query_epoch(axis, 0));
    assert!(!tracker.query_epoch(axis, 1));
    assert_eq!(tracker.advance(axis, 2), 0);
    assert!(tracker.query_epoch(axis, 1));
    assert!(tracker.query_epoch(axis, 2));
    assert!(!tracker.query_epoch(axis, 3));
    assert_eq!(tracker.advance(axis, 1), 0);
    assert_eq!(
        tracker.advance(Axis::new(3, 5, CausalDomain::Host, 2), 1),
        0
    );
}

#[test]
fn capacity_and_axis_retirement() {
    let empty = FrontierTracker::new(0, 0, 0).unwrap();
    let no_capacity_axis = Axis::new(0, 0, CausalDomain::Host, 0);
    assert!(empty.register_axis(no_capacity_axis, None).is_err());
    let tracker = FrontierTracker::new(2, 1, 0).unwrap();
    let axis = Axis::new(1, 0, CausalDomain::Host, 7);
    tracker.register_axis(axis, None).unwrap();
    tracker.retire_axis(axis, ireert::Code::Cancelled);
}
