//! Public proactor-pool and primitive cases corresponding to upstream tests.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::Code;
use ireert::asynchronous::{LocalProactorPool, Primitive, ProactorPool};
use std::time::Duration;

#[test]
fn local_pool_creation_lookup_and_retained_proactor() {
    assert!(LocalProactorPool::with_nodes(&[]).is_err());
    let pool = LocalProactorPool::with_nodes(&[0]).unwrap();
    assert_eq!(pool.len(), 1);
    assert!(!pool.is_empty());
    assert_eq!(pool.node_id(0), Some(0));
    assert_eq!(pool.node_id(1), None);
    assert!(pool.get(1).is_err());
    let proactor = pool.get(0).unwrap();
    let same_slot = pool.get_for_node(0).unwrap();
    assert_eq!(proactor.as_raw(), same_slot.as_raw());
    drop(pool);
    let poll_result = proactor.poll(Some(Duration::ZERO));
    assert!(match poll_result {
        Ok(_) => true,
        Err(error) => error.code() == Code::DeadlineExceeded,
    });
}

#[test]
fn standard_pool_and_empty_primitive_contracts() {
    let pool = ProactorPool::with_nodes(&[0]).unwrap();
    assert_eq!(pool.len(), 1);
    assert_eq!(pool.node_id(0), Some(0));
    let none = Primitive::none();
    assert!(none.is_none());
    assert!(none.try_clone().is_err());
}
