//! Public async semaphore cases corresponding to `semaphore_test.cc`.

use ireert::Code;
use ireert::asynchronous::semaphore::WaitMode;
use ireert::asynchronous::{LocalProactorPool, Semaphore};
use std::sync::mpsc;
use std::time::Duration;

fn semaphore() -> ireert::Result<Semaphore> {
    let pool = LocalProactorPool::with_nodes(&[0])?;
    let proactor = pool.get(0)?;
    Semaphore::new(&proactor, 0, 0)
}

#[test]
fn monotonic_signal_and_waits() -> ireert::Result<()> {
    let sem = semaphore()?;
    assert_eq!(sem.query(), 0);
    sem.signal(2, None)?;
    assert_eq!(sem.query(), 2);
    assert!(sem.signal(2, None).is_err());
    assert!(sem.signal(1, None).is_err());
    sem.wait(2, Some(Duration::ZERO))?;
    assert!(sem.wait(3, Some(Duration::ZERO)).is_err());
    Semaphore::wait_many(&[], WaitMode::All, Some(Duration::ZERO))?;
    Ok(())
}

#[test]
fn wait_many_all_and_any() -> ireert::Result<()> {
    let first = semaphore()?;
    let second = semaphore()?;
    first.signal(1, None)?;
    let all = [(&first, 1), (&second, 1)];
    assert!(Semaphore::wait_many(&all, WaitMode::All, Some(Duration::ZERO)).is_err());
    assert!(Semaphore::wait_many(&all, WaitMode::Any, Some(Duration::ZERO)).is_ok());
    second.signal(1, None)?;
    assert!(Semaphore::wait_many(&all, WaitMode::All, Some(Duration::ZERO)).is_ok());
    Ok(())
}

#[test]
fn callback_and_cancellation_lifetimes() -> Result<(), Box<dyn std::error::Error>> {
    let sem = semaphore()?;
    let (tx, rx) = mpsc::channel();
    let timepoint = sem.on_value(1, move |result| {
        assert!(
            tx.send(result.is_ok()).is_ok(),
            "callback receiver disconnected"
        );
    })?;
    timepoint.detach();
    sem.signal(1, None)?;
    assert!(rx.recv_timeout(Duration::from_secs(2))?);

    let (tx, rx) = mpsc::channel();
    let timepoint = sem.on_value(2, move |_| {
        assert!(
            tx.send(()).is_ok(),
            "cancellation callback receiver disconnected"
        );
    })?;
    assert!(timepoint.cancel());
    sem.signal(2, None)?;
    assert!(rx.recv_timeout(Duration::from_millis(20)).is_err());
    Ok(())
}

#[test]
fn failure_status_propagates() -> ireert::Result<()> {
    let sem = semaphore()?;
    sem.fail(Code::Cancelled);
    assert_eq!(sem.status().unwrap_err().code(), Code::Cancelled);
    Ok(())
}

#[test]
fn link_propagates_success_and_failure() -> ireert::Result<()> {
    let source = semaphore()?;
    let target = semaphore()?;
    let link = source.link(1, &target, 3)?;
    link.detach();
    source.signal(1, None)?;
    target.wait(3, Some(Duration::from_secs(2)))?;

    let failed_source = semaphore()?;
    let failed_target = semaphore()?;
    let link = failed_source.link(1, &failed_target, 3)?;
    link.detach();
    failed_source.fail(Code::Unavailable);
    assert_eq!(
        failed_target.status().unwrap_err().code(),
        Code::Unavailable
    );
    Ok(())
}

#[test]
fn dropping_pending_link_cancels_relay() -> ireert::Result<()> {
    let source = semaphore()?;
    let target = semaphore()?;
    let link = source.link(1, &target, 3)?;

    drop(link);
    source.signal(1, None)?;

    assert_eq!(source.query(), 1);
    assert_eq!(target.query(), 0);
    Ok(())
}
