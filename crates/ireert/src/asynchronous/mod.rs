//! Caller-driven low-level asynchronous I/O and synchronization.
//!
//! Native objects retain their dependencies. Proactors stay on their creating
//! thread: only that thread may drive completion callbacks with `poll`.
//! No Rust worker, scheduler, or reactor is introduced.
//!
//! # Ownership and cancellation laws
//!
//! Each submitted operation owns its storage, handles, arrays, buffers, and one
//! callback allocation until final native completion. Dropping its Rust handle
//! requests cancellation; it does not free in-flight data or imply cancellation
//! won. Continue polling until final completion, even after dropping a handle.
//! Stopping all progress with outstanding operations leaves their native-owned
//! storage live. Single-shot owners preserve the first intermediate `MORE` failure and
//! reclaim callback data only at final completion. Callback invocation and
//! payload destruction are separately panic-contained.
//!
//! Frontier wait cancellation is different: a successful native cancel removes
//! the waiter and suppresses its callback, so Rust reclaims its payload itself.
//! A failed cancel leaves payload reclamation to the dispatched callback.
//!
//! # Scope
//!
//! Only the asynchronous objects that HAL public APIs consume are bound:
//! proactor pools (device creation), proactors, owned primitives (semaphore
//! timepoints), timeline semaphores and their wait/signal operations, and
//! causal frontiers/axes/trackers (semaphore signal and device groups). Sockets, files, relays, events,
//! notifications, slabs, regions and buffer pools are proactor-service
//! infrastructure with no HAL consumer and are deliberately not bound; use
//! Rust's I/O ecosystem for those.

pub mod axis;
pub mod frontier;
pub mod frontier_tracker;
pub mod operation;
pub mod primitive;
pub mod proactor;
pub mod proactor_pool;
pub mod semaphore;
pub mod timepoint;

pub use axis::{Axis, CausalDomain};
pub use frontier::{Comparison, Frontier};
pub use frontier_tracker::FrontierTracker;
pub use operation::{SemaphoreSignalOperation, SemaphoreWaitOperation};
pub use primitive::Primitive;
pub use proactor::Proactor;
pub use proactor_pool::{LocalProactorPool, PoolRef, ProactorPool};
pub use semaphore::Semaphore;
pub use timepoint::Timepoint;
