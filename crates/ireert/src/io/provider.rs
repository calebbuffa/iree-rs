//! Parameter providers: scoped key-to-bytes sources that move parameter data
//! between storage and device buffers on a device queue.
//!
//! Laws:
//! - Every transfer is ordered after all `waits` and signals all `signals`
//!   when complete; the device retains every buffer it uses until then.
//! - `load` yields exactly one retained buffer per span, in span order.
//! - `load` and `gather` reject overlapping buffer ranges, and `scatter`
//!   rejects repeated keys, before submission (both are undefined natively).
//! - Spans are only read during the call; nothing borrowed outlives it.

use crate::hal::semaphore::SemaphoreList;
use crate::hal::{Buffer, BufferParams, Device, DeviceSize, QueueAffinity, SemaphorePoint};
use crate::{Code, Error, Result, check};
use core::ffi::c_void;
use iree_native as n;
use std::collections::HashSet;

/// One parameter range and its placement in a device buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParameterSpan<'a> {
    /// Parameter key within the operation's scope.
    pub key: &'a str,
    /// Byte offset within the parameter.
    pub parameter_offset: u64,
    /// Byte offset within the device buffer.
    pub buffer_offset: DeviceSize,
    /// Byte length of the range.
    pub length: DeviceSize,
}

/// Provider lifecycle signals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderSignal {
    /// Resume parameter requests.
    Resume,
    /// Suspend parameter requests.
    Suspend,
    /// Notify the provider of memory pressure.
    LowMemory,
}

/// The queue position of a provider transfer.
#[derive(Clone, Copy)]
pub struct Submission<'a> {
    /// Device whose queue performs the transfer.
    pub device: &'a Device,
    /// Queues permitted to execute the transfer.
    pub affinity: QueueAffinity,
    /// Dependencies that must be reached before the transfer starts.
    pub waits: &'a [SemaphorePoint<'a>],
    /// Values signaled when the transfer completes.
    pub signals: &'a [SemaphorePoint<'a>],
}

/// A retained parameter provider.
pub struct ParameterProvider(*mut n::iree_io_parameter_provider_t);
impl ParameterProvider {
    /// Adopts one owned provider reference.
    ///
    /// # Safety
    /// `raw` must be a live provider pointer with one owned reference.
    pub unsafe fn from_raw(raw: *mut n::iree_io_parameter_provider_t) -> Self {
        Self(raw)
    }
    /// Borrows the native provider pointer.
    pub fn as_raw(&self) -> *mut n::iree_io_parameter_provider_t {
        self.0
    }
    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_io_parameter_provider_t {
        core::mem::ManuallyDrop::new(self).0
    }
    /// Notifies the provider about runtime pressure or lifecycle changes.
    pub fn notify(&self, signal: ProviderSignal) -> Result<()> {
        let signal = match signal {
            ProviderSignal::Resume => n::IREE_IO_PARAMETER_PROVIDER_SIGNAL_RESUME,
            ProviderSignal::Suspend => n::IREE_IO_PARAMETER_PROVIDER_SIGNAL_SUSPEND,
            ProviderSignal::LowMemory => n::IREE_IO_PARAMETER_PROVIDER_SIGNAL_LOW_MEMORY,
        };
        // SAFETY: provider is live and signal is a valid native enum value.
        check(unsafe { n::iree_io_parameter_provider_notify(self.0, signal) })
    }
    /// Returns whether this provider handles `scope`.
    pub fn supports(&self, scope: &str) -> bool {
        // SAFETY: provider and scope view remain valid for the query.
        unsafe { n::iree_io_parameter_provider_query_support(self.0, crate::view(scope)) }
    }
    /// Loads each span into a device buffer, returning them in span order.
    ///
    /// Providers may return mapped or shared storage, and several results may
    /// be views of one allocation.
    pub fn load(
        &self,
        at: Submission<'_>,
        scope: &str,
        params: BufferParams,
        spans: &[ParameterSpan<'_>],
    ) -> Result<Vec<Buffer>> {
        require_disjoint_ranges(spans)?;
        let (waits, signals) = lists(&at);
        let mut emitted: Vec<Option<Buffer>> = (0..spans.len()).map(|_| None).collect();
        let emitter = n::iree_io_parameter_emitter_t {
            fn_: Some(emit_buffer),
            user_data: (&mut emitted as *mut Vec<Option<Buffer>>).cast(),
        };
        // SAFETY: provider, device, lists, spans and `emitted` outlive this
        // synchronous call; callbacks are only invoked before it returns.
        check(unsafe {
            n::iree_io_parameter_provider_load(
                self.0,
                at.device.as_raw(),
                at.affinity.bits(),
                waits.as_raw(),
                signals.as_raw(),
                crate::view(scope),
                params.raw(),
                spans.len(),
                enumerator(&spans),
                emitter,
            )
        })?;
        emitted
            .into_iter()
            .map(|buffer| {
                buffer.ok_or_else(|| Error::new(Code::Internal, "provider skipped a span"))
            })
            .collect()
    }
    /// Reads `length` bytes of one parameter into `target`.
    pub fn read(
        &self,
        at: Submission<'_>,
        scope: &str,
        span: ParameterSpan<'_>,
        target: &Buffer,
    ) -> Result<()> {
        let (waits, signals) = lists(&at);
        // SAFETY: all handles are live and views are valid for the call; the
        // device retains `target` until the transfer completes.
        check(unsafe {
            n::iree_io_parameter_provider_read(
                self.0,
                at.device.as_raw(),
                at.affinity.bits(),
                waits.as_raw(),
                signals.as_raw(),
                crate::view(scope),
                crate::view(span.key),
                span.parameter_offset,
                target.as_raw(),
                span.buffer_offset,
                span.length,
            )
        })
    }
    /// Writes `length` bytes from `source` into one parameter.
    pub fn write(
        &self,
        at: Submission<'_>,
        scope: &str,
        source: &Buffer,
        span: ParameterSpan<'_>,
    ) -> Result<()> {
        let (waits, signals) = lists(&at);
        // SAFETY: all handles are live and views are valid for the call; the
        // device retains `source` until the transfer completes.
        check(unsafe {
            n::iree_io_parameter_provider_write(
                self.0,
                at.device.as_raw(),
                at.affinity.bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.as_raw(),
                span.buffer_offset,
                crate::view(scope),
                crate::view(span.key),
                span.parameter_offset,
                span.length,
            )
        })
    }
    /// Gathers parameter ranges into disjoint ranges of `target`.
    pub fn gather(
        &self,
        at: Submission<'_>,
        scope: &str,
        target: &Buffer,
        spans: &[ParameterSpan<'_>],
    ) -> Result<()> {
        require_disjoint_ranges(spans)?;
        let (waits, signals) = lists(&at);
        // SAFETY: handles and spans are live for the synchronous call; target
        // ranges were checked disjoint.
        check(unsafe {
            n::iree_io_parameter_provider_gather(
                self.0,
                at.device.as_raw(),
                at.affinity.bits(),
                waits.as_raw(),
                signals.as_raw(),
                crate::view(scope),
                target.as_raw(),
                spans.len(),
                enumerator(&spans),
            )
        })
    }
    /// Scatters ranges of `source` into distinct parameters.
    pub fn scatter(
        &self,
        at: Submission<'_>,
        scope: &str,
        source: &Buffer,
        spans: &[ParameterSpan<'_>],
    ) -> Result<()> {
        require_distinct_keys(spans)?;
        let (waits, signals) = lists(&at);
        // SAFETY: handles and spans are live for the synchronous call; target
        // keys were checked distinct.
        check(unsafe {
            n::iree_io_parameter_provider_scatter(
                self.0,
                at.device.as_raw(),
                at.affinity.bits(),
                waits.as_raw(),
                signals.as_raw(),
                source.as_raw(),
                crate::view(scope),
                spans.len(),
                enumerator(&spans),
            )
        })
    }
}

impl Clone for ParameterProvider {
    fn clone(&self) -> Self {
        // SAFETY: `self.0` is a live ref-counted provider.
        unsafe { n::iree_io_parameter_provider_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for ParameterProvider {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one provider reference.
        unsafe { n::iree_io_parameter_provider_release(self.0) }
    }
}

fn lists<'a>(at: &Submission<'a>) -> (SemaphoreList<'a>, SemaphoreList<'a>) {
    (
        SemaphoreList::from_points(at.waits),
        SemaphoreList::from_points(at.signals),
    )
}

/// Builds an enumerator over `spans`; `spans` must outlive every native use.
fn enumerator(spans: &&[ParameterSpan<'_>]) -> n::iree_io_parameter_enumerator_t {
    n::iree_io_parameter_enumerator_t {
        fn_: Some(enumerate_span),
        user_data: (spans as *const &[ParameterSpan<'_>]).cast_mut().cast(),
    }
}

unsafe extern "C" fn enumerate_span(
    user_data: *mut c_void,
    index: n::iree_host_size_t,
    out_key: *mut n::iree_string_view_t,
    out_span: *mut n::iree_io_parameter_span_t,
) -> n::iree_status_t {
    // SAFETY: `enumerator` passes a pointer to a live span slice.
    let spans = unsafe { *user_data.cast::<&[ParameterSpan<'_>]>() };
    let Some(span) = spans.get(index) else {
        return n::abi::status::from_code(n::IREE_STATUS_OUT_OF_RANGE);
    };
    // SAFETY: native passes writable outputs; the key bytes outlive the call.
    unsafe {
        *out_key = crate::view(span.key);
        *out_span = n::iree_io_parameter_span_t {
            parameter_offset: span.parameter_offset,
            buffer_offset: span.buffer_offset,
            length: span.length,
        };
        n::abi::status::ok()
    }
}

unsafe extern "C" fn emit_buffer(
    user_data: *mut c_void,
    index: n::iree_host_size_t,
    buffer: *mut n::iree_hal_buffer_t,
) -> n::iree_status_t {
    // SAFETY: `load` passes its exclusively borrowed result vector.
    let emitted = unsafe { &mut *user_data.cast::<Vec<Option<Buffer>>>() };
    let Some(slot) = emitted.get_mut(index).filter(|_| !buffer.is_null()) else {
        return n::abi::status::from_code(n::IREE_STATUS_OUT_OF_RANGE);
    };
    // SAFETY: the emitted buffer is borrowed for the callback; retaining it
    // gives the returned wrapper its own reference.
    unsafe {
        n::iree_hal_buffer_retain(buffer);
        *slot = Some(Buffer::from_raw(buffer));
        n::abi::status::ok()
    }
}

fn require_disjoint_ranges(spans: &[ParameterSpan<'_>]) -> Result<()> {
    let mut ranges = spans
        .iter()
        .filter(|span| span.length != 0)
        .map(|span| {
            span.buffer_offset
                .checked_add(span.length)
                .map(|end| (span.buffer_offset, end))
                .ok_or_else(|| Error::new(Code::OutOfRange, "span range overflows the device size"))
        })
        .collect::<Result<Vec<_>>>()?;
    ranges.sort_unstable();
    if ranges.windows(2).any(|pair| pair[0].1 > pair[1].0) {
        return Err(Error::new(
            Code::InvalidArgument,
            "parameter spans have overlapping buffer ranges",
        ));
    }
    Ok(())
}

fn require_distinct_keys(spans: &[ParameterSpan<'_>]) -> Result<()> {
    let mut keys = HashSet::with_capacity(spans.len());
    if spans.iter().all(|span| keys.insert(span.key)) {
        Ok(())
    } else {
        Err(Error::new(
            Code::InvalidArgument,
            "scatter spans repeat a target parameter",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn span(buffer_offset: DeviceSize, length: DeviceSize) -> ParameterSpan<'static> {
        ParameterSpan {
            key: "k",
            parameter_offset: 0,
            buffer_offset,
            length,
        }
    }

    proptest! {
        #[test]
        fn disjoint_check_matches_pairwise_overlap(ranges in proptest::collection::vec((0usize..256, 0usize..64), 0..12)) {
            let spans: Vec<_> = ranges.iter().map(|&(offset, length)| span(offset, length)).collect();
            let overlapping = ranges.iter().enumerate().any(|(i, &(a, al))| {
                ranges[i + 1..].iter().any(|&(b, bl)| al != 0 && bl != 0 && a < b + bl && b < a + al)
            });
            prop_assert_eq!(require_disjoint_ranges(&spans).is_err(), overlapping);
        }

        #[test]
        fn distinct_check_matches_set_size(keys in proptest::collection::vec("[a-c]{1,2}", 0..12)) {
            let spans: Vec<_> = keys.iter().map(|key| ParameterSpan { key, parameter_offset: 0, buffer_offset: 0, length: 0 }).collect();
            let unique = keys.iter().collect::<HashSet<_>>().len() == keys.len();
            prop_assert_eq!(require_distinct_keys(&spans).is_ok(), unique);
        }
    }

    #[test]
    fn enumerator_reports_each_span_then_out_of_range() {
        let spans = [span(0, 4), span(8, 4)];
        let slice: &[ParameterSpan<'_>] = &spans;
        let native = enumerator(&slice);
        for (index, expected) in spans.iter().enumerate() {
            let mut key = crate::view("");
            let mut out = n::iree_io_parameter_span_t::default();
            // SAFETY: the enumerator user data points at `slice`, which is live.
            let status = unsafe { enumerate_span(native.user_data, index, &mut key, &mut out) };
            assert!(check(status).is_ok());
            assert_eq!(
                (out.buffer_offset, out.length),
                (expected.buffer_offset, expected.length)
            );
        }
        // SAFETY: as above; an out-of-range index must fail without writing.
        let status = unsafe {
            enumerate_span(
                native.user_data,
                spans.len(),
                &mut crate::view(""),
                &mut n::iree_io_parameter_span_t::default(),
            )
        };
        assert_eq!(check(status).unwrap_err().code(), Code::OutOfRange);
    }
}
