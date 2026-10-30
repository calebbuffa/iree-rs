//! Public event-loop dispatch and completion behavior.

use ireert::base::WaitSource;
use ireert::vm::InlineLoop;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

#[test]
fn inline_loop_calls_and_waits_until_deadlines() {
    let mut loop_ = InlineLoop::new();
    let callbacks = Arc::new(AtomicUsize::new(0));
    let called = callbacks.clone();
    loop_
        .as_loop()
        .call(move |_loop, status| {
            status.unwrap();
            called.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .unwrap();
    let waited = callbacks.clone();
    loop_
        .as_loop()
        .wait_until(
            Instant::now() + Duration::from_millis(1),
            move |_loop, status| {
                status.unwrap();
                waited.fetch_add(1, Ordering::SeqCst);
                Ok(())
            },
        )
        .unwrap();
    loop_.take_error().unwrap();
    assert_eq!(callbacks.load(Ordering::SeqCst), 2);
}

#[test]
fn inline_wait_any_preserves_upstream_first_source_fallback() {
    let mut loop_ = InlineLoop::new();
    let (send, receive) = std::sync::mpsc::channel();
    let sources = [
        WaitSource::delay_until(Instant::now() + Duration::from_secs(60)),
        WaitSource::immediate(),
    ];
    // SAFETY: both sources have static backing and the inline loop invokes the
    // callback before this call returns.
    unsafe {
        loop_
            .as_loop()
            .wait_any(&sources, Some(Duration::ZERO), move |_, status| {
                let _ = send.send(status);
                Ok(())
            })
            .unwrap();
    }
    assert_eq!(
        receive
            .try_recv()
            .expect("inline wait callback runs before return")
            .unwrap_err()
            .code(),
        ireert::Code::DeadlineExceeded
    );
    loop_.take_error().unwrap();
}
