//! Public wait-source behavior corresponding to IREE base wait APIs.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::base::{WaitSource, WaitStatus};
use std::time::{Duration, Instant};

#[test]
fn immediate_source_is_resolved() {
    let source = WaitSource::immediate();
    assert!(source.is_immediate());
    assert!(!source.is_delay());
    assert_eq!(source.query().unwrap(), WaitStatus::Resolved);
    source.wait(Some(Duration::ZERO)).unwrap();
}

#[test]
fn expired_delay_is_resolved() {
    let source = WaitSource::delay_until(Instant::now());
    assert!(source.is_delay());
    source.wait(Some(Duration::from_secs(1))).unwrap();
    assert_eq!(source.query().unwrap(), WaitStatus::Resolved);
}
