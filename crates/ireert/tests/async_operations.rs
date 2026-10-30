//! Public semaphore operation cases for async proactor-driven tests.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::asynchronous::semaphore::WaitMode;
use ireert::asynchronous::{
    LocalProactorPool, Semaphore, SemaphoreSignalOperation, SemaphoreWaitOperation,
};
use std::{cell::RefCell, rc::Rc, time::Duration};

#[test]
fn signal_and_wait_operations_complete_through_polling() {
    let pool = LocalProactorPool::with_nodes(&[0]).unwrap();
    let proactor = pool.get(0).unwrap();
    let semaphore = Semaphore::new(&proactor, 0, 0).unwrap();
    let wait_result = Rc::new(RefCell::new(None));
    let wait_result_cb = wait_result.clone();
    let wait = SemaphoreWaitOperation::submit(
        &proactor,
        &[(&semaphore, 2)],
        WaitMode::All,
        move |result| *wait_result_cb.borrow_mut() = Some(result),
    )
    .unwrap();
    let signal_result = Rc::new(RefCell::new(None));
    let signal_result_cb = signal_result.clone();
    let signal =
        SemaphoreSignalOperation::submit(&proactor, &[(&semaphore, 2)], None, move |result| {
            *signal_result_cb.borrow_mut() = Some(result)
        })
        .unwrap();

    for _ in 0..100 {
        proactor.poll(Some(Duration::from_millis(10))).unwrap();
        if wait.is_complete() && signal.is_complete() {
            break;
        }
    }
    assert!(wait.is_complete());
    assert!(signal.is_complete());
    assert_eq!(semaphore.query(), 2);
    assert!(wait_result.borrow().as_ref().unwrap().is_ok());
    assert!(signal_result.borrow().as_ref().unwrap().is_ok());
}
