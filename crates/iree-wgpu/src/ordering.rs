//! Cross-API ordering: the context lock, dirty tracking and timeline staging.
//!
//! Every WGPU submission and IREE invocation goes through `Inner`, which
//! records which API last wrote shared state and inserts the matching
//! semaphore wait (separate queues) or host wait (shared queue) before the
//! other API runs.

use crate::context::Inner;
use crate::context::QueueTopology;
use crate::marker::Watcher;
use ash::vk::{self, Handle};
use ireert::Code;
use ireert::Error;
use ireert::Result;
use ireert::hal::GroupedDevice;
use ireert::hal::QueueAffinity;
use ireert::hal::Semaphore;
use ireert::hal::SemaphoreCompatibility;
use ireert::hal::SemaphorePoint;
use ireert::hal::WaitFlags;
use ireert::hal::backends::vulkan;
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::MutexGuard;
use std::sync::PoisonError;

/// Ordering state, guarded by the context lock.
pub(crate) struct State {
    /// WGPU work (or a pending `write_*`) was submitted after the last marker.
    pub(crate) wgpu_dirty: bool,
    /// IREE submitted to the shared queue after the last barrier (`Shared`).
    pub(crate) iree_dirty: bool,
    /// Last value submitted on the private marker timeline.
    pub(crate) marker: u64,
    /// IREE work timeline and the value its last enqueued operation signals.
    pub(crate) work: Semaphore,
    pub(crate) work_vk: Option<vk::Semaphore>,
    pub(crate) launched: u64,
    /// Set when a failed IREE work timeline could not be replaced.
    pub(crate) poisoned: Option<String>,
    pub(crate) watcher: Option<Watcher>,
}

thread_local! {
    /// Contexts whose lock this thread holds, to report re-entrant calls
    /// (for example from a WGPU callback fired during a submission) instead
    /// of deadlocking.
    static HELD: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

/// The context lock, held by the current thread.
pub(crate) struct Locked<'a> {
    pub(crate) guard: MutexGuard<'a, State>,
    pub(crate) key: usize,
}

impl Deref for Locked<'_> {
    type Target = State;
    fn deref(&self) -> &State {
        &self.guard
    }
}

impl DerefMut for Locked<'_> {
    fn deref_mut(&mut self) -> &mut State {
        &mut self.guard
    }
}

impl Drop for Locked<'_> {
    fn drop(&mut self) {
        HELD.with_borrow_mut(|held| {
            if let Some(index) = held.iter().rposition(|key| *key == self.key) {
                held.swap_remove(index);
            }
        });
    }
}

pub(crate) fn next_value(value: u64) -> Result<u64> {
    value
        .checked_add(1)
        .filter(|next| *next < Semaphore::FAILURE_VALUE)
        .ok_or_else(|| Error::new(Code::OutOfRange, "iree-wgpu: timeline exhausted"))
}

impl Inner {
    pub(crate) fn key(&self) -> usize {
        self as *const Self as usize
    }

    pub(crate) fn lock(&self) -> Result<Locked<'_>> {
        let key = self.key();
        if HELD.with_borrow(|held| held.contains(&key)) {
            return Err(Error::new(
                Code::FailedPrecondition,
                "iree-wgpu: re-entrant context call, for example from a WGPU callback run during a \
                 submission; defer the call (for example through a channel)",
            ));
        }
        let guard = self.state.lock().map_err(|_| {
            Error::new(
                Code::Aborted,
                "iree-wgpu: a submission panicked; the context can no longer order work",
            )
        })?;
        HELD.with_borrow_mut(|held| held.push(key));
        Ok(Locked { guard, key })
    }

    /// The lock for teardown paths: recovers from a panic, but still refuses
    /// re-entrant use.
    pub(crate) fn lock_for_drop(&self) -> Option<Locked<'_>> {
        let key = self.key();
        if HELD.with_borrow(|held| held.contains(&key)) {
            return None;
        }
        let guard = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        HELD.with_borrow_mut(|held| held.push(key));
        Some(Locked { guard, key })
    }

    pub(crate) fn check(&self, state: &State) -> Result<()> {
        match &state.poisoned {
            Some(reason) => Err(Error::new(
                Code::Aborted,
                format!("iree-wgpu: the context could not recover from an IREE failure: {reason}"),
            )),
            None => Ok(()),
        }
    }

    /// Mark the context unable to order further IREE work.
    pub(crate) fn poison(&self, state: &mut State, error: &Error) {
        state.poisoned.get_or_insert_with(|| error.to_string());
    }

    /// Stage a GPU wait for all launched IREE work on WGPU's next submission.
    pub(crate) fn stage_iree_wait(&self, state: &mut State) {
        if self.topology != QueueTopology::Separate || state.launched == 0 {
            return;
        }
        match state.work.query() {
            Ok(reached) if reached >= state.launched => {}
            Ok(_) => {
                let staged = match state.work_vk {
                    // SAFETY: the HAL queue is only used to stage a semaphore
                    // wait on WGPU's next submission, which the caller makes
                    // under the context lock; nothing is submitted here.
                    Some(raw) => match unsafe { self.queue.as_hal::<wgpu::hal::api::Vulkan>() } {
                        Some(hal) => {
                            // A wait's second scope is one batch, so stage it every time.
                            hal.add_wait_semaphore(
                                raw,
                                Some(state.launched),
                                vk::PipelineStageFlags::ALL_COMMANDS,
                            );
                            true
                        }
                        None => false,
                    },
                    None => false,
                };
                // Not device-waitable: complete it on the host instead.
                if !staged
                    && state
                        .work
                        .wait(state.launched, None, WaitFlags::default())
                        .is_err()
                {
                    self.reset_work(state);
                }
            }
            // Failed work releases its waiters; start a fresh timeline.
            Err(_) => self.reset_work(state),
        }
    }

    /// Replace a failed IREE work timeline with a fresh one.
    ///
    /// Every queue access is serialized with WGPU's, so a failed IREE
    /// operation can only leave the buffers it writes with unspecified
    /// contents, and the context stays usable.
    pub(crate) fn reset_work(&self, state: &mut State) {
        let fresh = Semaphore::new(&self.iree, QueueAffinity::ANY, 0).and_then(|work| {
            // SAFETY: Context::create constructs `self.iree` with Vulkan's
            // wrap_device, whose create_semaphore callback creates Vulkan
            // semaphores. `work` was just created on that exact device.
            let raw = unsafe { device_waitable(&self.iree, &work)? };
            Ok((raw, work))
        });
        match fresh {
            Ok((raw, work)) => {
                let old = std::mem::replace(&mut state.work, work);
                state.work_vk = raw;
                state.launched = 0;
                // In-flight WGPU waits may still reference the old semaphore.
                self.queue.on_submitted_work_done(move || drop(old));
            }
            Err(error) => self.poison(state, &error),
        }
    }

    /// Submit WGPU work ordered after all previously launched IREE work.
    pub(crate) fn wgpu_submit(
        &self,
        state: &mut State,
        commands: Vec<wgpu::CommandBuffer>,
    ) -> Result<wgpu::SubmissionIndex> {
        match self.topology {
            QueueTopology::Separate => self.stage_iree_wait(state),
            QueueTopology::Shared => {
                if state.iree_dirty {
                    self.barrier(state)?;
                    state.iree_dirty = false;
                }
            }
        }
        let index = {
            let _held = self.wgpu_queue.lock();
            self.queue.submit(commands)
        };
        state.wgpu_dirty = true;
        Ok(index)
    }

    pub(crate) fn barrier(&self, state: &mut State) -> Result<()> {
        let next = next_value(state.marker)?;
        self.marker.barrier(next)?;
        state.marker = next;
        Ok(())
    }

    /// Mark all WGPU work submitted so far, including pending writes.
    ///
    /// Returns the marker value reached once that work completes.
    pub(crate) fn mark(&self, state: &mut State) -> Result<u64> {
        if !state.wgpu_dirty {
            return Ok(state.marker);
        }
        match self.topology {
            QueueTopology::Separate => {
                let next = next_value(state.marker)?;
                self.stage_iree_wait(state);
                {
                    // SAFETY: as in `stage_iree_wait`, the HAL queue only
                    // stages a signal on the submission made just below.
                    let hal = unsafe { self.queue.as_hal::<wgpu::hal::api::Vulkan>() }.ok_or_else(
                        || Error::new(Code::Internal, "iree-wgpu: the WGPU queue is not Vulkan"),
                    )?;
                    hal.add_signal_semaphore(self.marker.semaphore, Some(next));
                }
                // Flushes pending writes and forwards the staged semaphores.
                {
                    let _held = self.wgpu_queue.lock();
                    self.queue.submit(std::iter::empty());
                }
                state.marker = next;
                if let Some(watcher) = &state.watcher {
                    // A stopped watcher has already failed the handoff semaphore.
                    let _ = watcher.values.send(next);
                }
            }
            QueueTopology::Shared => {
                self.wgpu_submit(state, Vec::new())?;
                self.barrier(state)?;
            }
        }
        state.wgpu_dirty = false;
        Ok(state.marker)
    }

    /// Order the next IREE operation after all WGPU work submitted so far.
    ///
    /// With separate queues, returns the wait points the operation must take;
    /// with a shared queue, queue order and the barrier suffice.
    pub(crate) fn iree_waits(&self, state: &mut State) -> Result<Vec<(Semaphore, u64)>> {
        self.check(state)?;
        let marker = self.mark(state)?;
        let mut waits = Vec::with_capacity(2);
        if self.topology == QueueTopology::Separate {
            if marker > 0 {
                waits.push((self.marker.handoff.clone(), marker));
            }
            if state.launched > 0 {
                waits.push((state.work.clone(), state.launched));
            }
        }
        Ok(waits)
    }

    /// Complete all submitted work of both APIs on the host.
    pub(crate) fn quiesce(&self, state: &mut State) -> Result<()> {
        self.check(state)?;
        let marker = self.mark(state)?;
        self.marker.wait(marker)?;
        if state.launched > 0
            && let Err(error) = state.work.wait(state.launched, None, WaitFlags::default())
        {
            self.reset_work(state);
            return Err(error);
        }
        Ok(())
    }

    /// Run one IREE queue operation ordered on the logical queue.
    ///
    /// `op` receives the device, waits and signals.
    pub(crate) fn enqueue<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(&GroupedDevice, &[SemaphorePoint<'_>], &[SemaphorePoint<'_>]) -> Result<()>,
    {
        let mut state = self.lock()?;
        let waits = self.iree_waits(&mut state)?;
        let next = next_value(state.launched)?;
        let work = state.work.clone();
        let waits = waits
            .iter()
            .map(|(semaphore, value)| SemaphorePoint {
                semaphore,
                value: *value,
            })
            .collect::<Vec<_>>();
        let signals = [SemaphorePoint {
            semaphore: &work,
            value: next,
        }];
        match self.topology {
            QueueTopology::Separate => {
                // A rejected enqueue submitted nothing.
                op(&self.iree, &waits, &signals)?;
                state.launched = next;
            }
            QueueTopology::Shared => {
                let result = op(&self.iree, &waits, &signals)
                    .and_then(|()| work.wait(next, None, WaitFlags::default()));
                state.iree_dirty = true;
                if let Err(error) = result {
                    signals[0].semaphore.fail(Code::Aborted);
                    self.reset_work(&mut state);
                    return Err(error);
                }
                state.launched = next;
            }
        }
        Ok(())
    }

    /// Run synchronous IREE work (completed before returning) under the lock.
    pub(crate) fn blocking<T>(&self, op: impl FnOnce(&GroupedDevice) -> Result<T>) -> Result<T> {
        let mut state = self.lock()?;
        self.quiesce(&mut state)?;
        let result = op(&self.iree);
        if self.topology == QueueTopology::Shared {
            state.iree_dirty = true;
        }
        result
    }
}

/// # Safety
/// `device` must be a Vulkan HAL device and `semaphore` must have been created
/// by that device's Vulkan semaphore creation callback.
pub(crate) unsafe fn device_waitable(
    device: &GroupedDevice,
    semaphore: &Semaphore,
) -> Result<Option<vk::Semaphore>> {
    if !semaphore
        .compatibility_with(device)
        .contains(SemaphoreCompatibility::DEVICE_WAIT)
    {
        return Ok(None);
    }
    // SAFETY: the caller guarantees actual Vulkan semaphore provenance;
    // DEVICE_WAIT compatibility alone would not prove it for a generic device.
    let handle = unsafe { vulkan::semaphore_handle(semaphore)? };
    if handle.handle.is_null() {
        return Ok(None);
    }
    Ok(Some(vk::Semaphore::from_raw(handle.handle as u64)))
}
