//! Callback-driven native futures and device timeline synchronization.
#[cfg(any(feature = "local-sync", feature = "cpu"))]
use iree::{Instance, Timeout};
use iree::{StatusCode, Timeline};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

#[derive(Default)]
struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn timepoints_complete_without_polling_threads() {
    let timeline = Timeline::new(0).unwrap();
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    let mut wait = timeline.wait(2).unwrap();
    assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    timeline.signal(1).unwrap();
    assert_eq!(counter.0.load(Ordering::SeqCst), 0);
    timeline.signal(2).unwrap();
    assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    assert!(matches!(
        Pin::new(&mut wait).poll(&mut context),
        Poll::Ready(Ok(()))
    ));
    assert_eq!(timeline.value().unwrap(), 2);
    assert_eq!(
        timeline.signal(1).unwrap_err().code(),
        StatusCode::INVALID_ARGUMENT
    );
}

#[test]
fn immediate_completion_failure_and_drop() {
    let timeline = Timeline::new(7).unwrap();
    let mut context = Context::from_waker(Waker::noop());
    let mut ready = timeline.wait(7).unwrap();
    assert!(matches!(
        Pin::new(&mut ready).poll(&mut context),
        Poll::Ready(Ok(()))
    ));
    drop(timeline.wait(100).unwrap());
    let mut pending = timeline.wait(100).unwrap();
    assert!(Pin::new(&mut pending).poll(&mut context).is_pending());
    timeline.cancel();
    let Poll::Ready(Err(error)) = Pin::new(&mut pending).poll(&mut context) else {
        panic!("failure must complete pending waits");
    };
    assert_eq!(error.code(), StatusCode::CANCELLED);
    let mut failed = timeline.wait(100).unwrap();
    assert!(matches!(
        Pin::new(&mut failed).poll(&mut context),
        Poll::Ready(Err(_))
    ));
    assert!(timeline.wait(u64::MAX).is_err());
    assert!(Timeline::new(u64::MAX).is_err());
}

#[cfg(feature = "task")]
#[test]
fn cross_thread_signal_and_cancellation_races() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Timeline>();
    for _ in 0..200 {
        let timeline = Timeline::new(0).unwrap();
        let wait = timeline.wait(1).unwrap();
        let other = timeline.clone();
        let signal = std::thread::spawn(move || other.signal(1).unwrap());
        drop(wait);
        signal.join().unwrap();
    }
    let timeline = Timeline::new(0).unwrap();
    let wait = timeline.wait(1).unwrap();
    let other = timeline.clone();
    std::thread::spawn(move || {
        other.signal(1).unwrap();
        let mut wait = wait;
        let mut context = Context::from_waker(Waker::noop());
        assert!(matches!(
            Pin::new(&mut wait).poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    })
    .join()
    .unwrap();
}

#[cfg(any(feature = "local-sync", feature = "cpu"))]
#[test]
fn hal_waits_preserve_timeouts_and_device_lifetime() {
    let device = Instance::new().unwrap().device("local-sync").unwrap();
    let semaphore = device.semaphore(0).unwrap();
    drop(device);
    assert_eq!(
        semaphore
            .wait_blocking(1, Timeout::Immediate)
            .unwrap_err()
            .code(),
        StatusCode::DEADLINE_EXCEEDED,
    );
    let mut wait = semaphore.wait(1).unwrap();
    let mut context = Context::from_waker(Waker::noop());
    assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    semaphore.signal(1).unwrap();
    assert!(matches!(
        Pin::new(&mut wait).poll(&mut context),
        Poll::Ready(Ok(()))
    ));
    semaphore.wait_blocking(1, Timeout::Infinite).unwrap();
    assert_eq!(semaphore.value().unwrap(), 1);
}

#[cfg(any(feature = "local-sync", feature = "cpu"))]
#[test]
fn immutable_fences_join_wait_all_and_propagate_failures() {
    use iree::Fence;
    let device = Instance::new().unwrap().device("local-sync").unwrap();
    let a = device.semaphore(0).unwrap();
    let b = device.semaphore(0).unwrap();
    let joined = Fence::join(&[a.at(2).unwrap(), b.at(1).unwrap()]).unwrap();
    assert!(!joined.is_ready().unwrap());
    let mut wait = joined.wait().unwrap();
    let mut context = Context::from_waker(Waker::noop());
    assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    a.signal(2).unwrap();
    assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    b.signal(1).unwrap();
    assert!(matches!(
        Pin::new(&mut wait).poll(&mut context),
        Poll::Ready(Ok(()))
    ));
    assert!(joined.is_ready().unwrap());
    joined.wait_blocking(Timeout::Infinite).unwrap();
    assert!(Fence::empty().is_ready().unwrap());
    assert!(Fence::join(&[]).unwrap().is_ready().unwrap());
    let failed = a.at(3).unwrap();
    let mut wait = failed.wait().unwrap();
    a.cancel();
    assert_eq!(failed.is_ready().unwrap_err().code(), StatusCode::CANCELLED);
    assert!(matches!(
        Pin::new(&mut wait).poll(&mut context),
        Poll::Ready(Err(_))
    ));
}
