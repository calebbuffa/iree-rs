//! HAL channels for collective operations.

use super::bitfield::scalar_value;
use super::{QueueAffinity, device::Device};
use crate::{Result, check};
use core::marker::PhantomData;
use iree_native as n;
use iree_native::const_span;

/// Parameters used to create a collective channel.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChannelParams<'a> {
    /// Opaque backend-specific channel identifier; empty selects the provider default.
    pub id: &'a [u8],
    /// User-defined group key.
    pub group: &'a str,
    /// Participant rank; `None` uses the environment default.
    pub rank: Option<i32>,
    /// Participant count; `None` uses the environment default.
    pub count: Option<i32>,
}

/// An owned `iree_hal_channel_t` reference.
pub struct Channel {
    raw: *mut n::iree_hal_channel_t,
    _not_thread_safe: PhantomData<*mut ()>,
}
impl Channel {
    /// Creates a channel on `device` for the selected queue affinity.
    pub fn create(
        device: &Device,
        queue_affinity: QueueAffinity,
        params: ChannelParams<'_>,
    ) -> Result<Self> {
        let params = n::iree_hal_channel_params_t {
            flags: n::IREE_HAL_CHANNEL_FLAG_NONE as n::iree_hal_channel_flags_t,
            id: const_span(params.id),
            group: crate::view(params.group),
            rank: params.rank.unwrap_or(-1),
            count: params.count.unwrap_or(-1),
        };
        let mut raw = core::ptr::null_mut();
        // SAFETY: the device and borrowed parameter data live through the call.
        check(unsafe {
            n::iree_hal_channel_create(device.as_raw(), queue_affinity.bits(), params, &mut raw)
        })?;
        // SAFETY: successful create returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned channel reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_channel_t) -> Self {
        Self {
            raw,
            _not_thread_safe: PhantomData,
        }
    }

    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_channel_t {
        self.raw
    }

    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_channel_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Splits this channel into subgroups keyed by `color`, ordered by `key`.
    ///
    /// A `None` color opts this rank out and returns `Ok(None)`.
    pub fn split(&self, color: Option<i32>, key: i32) -> Result<Option<Self>> {
        let color = color.unwrap_or(-1);
        let mut raw = core::ptr::null_mut();
        // SAFETY: the channel is live and `raw` is writable.
        check(unsafe {
            n::iree_hal_channel_split(
                self.raw,
                color,
                key,
                n::IREE_HAL_CHANNEL_FLAG_NONE as n::iree_hal_channel_flags_t,
                &mut raw,
            )
        })?;
        // SAFETY: a non-null result is one owned reference.
        Ok((!raw.is_null()).then(|| unsafe { Self::from_raw(raw) }))
    }

    /// Returns this rank and the channel participant count.
    pub fn rank_and_count(&self) -> (i32, i32) {
        let (mut rank, mut count) = (0, 0);
        // SAFETY: the channel is live and both outputs are writable.
        unsafe { n::iree_hal_channel_query_rank_and_count(self.raw, &mut rank, &mut count) };
        (rank, count)
    }
}
impl Clone for Channel {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_channel_retain(self.raw) };
        Self {
            raw: self.raw,
            _not_thread_safe: PhantomData,
        }
    }
}
impl Drop for Channel {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_channel_release(self.raw) }
    }
}

scalar_value! {
    /// HAL collective operation kind (`iree_hal_collective_kind_t`).
    CollectiveKind: n::iree_hal_collective_kind_t {
        /// Gathers values from all ranks.
        ALL_GATHER = n::IREE_HAL_COLLECTIVE_KIND_ALL_GATHER,
        /// Reduces values across all ranks.
        ALL_REDUCE = n::IREE_HAL_COLLECTIVE_KIND_ALL_REDUCE,
        /// Exchanges distinct values between all ranks.
        ALL_TO_ALL = n::IREE_HAL_COLLECTIVE_KIND_ALL_TO_ALL,
        /// Broadcasts values from one rank.
        BROADCAST = n::IREE_HAL_COLLECTIVE_KIND_BROADCAST,
        /// Reduces values to one rank.
        REDUCE = n::IREE_HAL_COLLECTIVE_KIND_REDUCE,
        /// Reduces and scatters values.
        REDUCE_SCATTER = n::IREE_HAL_COLLECTIVE_KIND_REDUCE_SCATTER,
        /// Sends values to one rank.
        SEND = n::IREE_HAL_COLLECTIVE_KIND_SEND,
        /// Receives values from one rank.
        RECV = n::IREE_HAL_COLLECTIVE_KIND_RECV,
        /// Sends values to and receives values from one rank.
        SEND_RECV = n::IREE_HAL_COLLECTIVE_KIND_SEND_RECV,
    }
}

scalar_value! {
    /// HAL collective reduction (`iree_hal_collective_reduction_t`).
    CollectiveReduction: n::iree_hal_collective_reduction_t {
        /// No reduction.
        NONE = n::IREE_HAL_COLLECTIVE_REDUCTION_NONE,
        /// Sum.
        SUM = n::IREE_HAL_COLLECTIVE_REDUCTION_SUM,
        /// Product.
        PRODUCT = n::IREE_HAL_COLLECTIVE_REDUCTION_PRODUCT,
        /// Minimum.
        MINIMUM = n::IREE_HAL_COLLECTIVE_REDUCTION_MINIMUM,
        /// Maximum.
        MAXIMUM = n::IREE_HAL_COLLECTIVE_REDUCTION_MAXIMUM,
        /// Average.
        AVERAGE = n::IREE_HAL_COLLECTIVE_REDUCTION_AVERAGE,
    }
}

scalar_value! {
    /// HAL collective element type (`iree_hal_collective_element_type_t`).
    CollectiveElementType: n::iree_hal_collective_element_type_t {
        /// Signed 8-bit integer.
        SINT_8 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_SINT_8,
        /// Unsigned 8-bit integer.
        UINT_8 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_UINT_8,
        /// Signed 16-bit integer.
        SINT_16 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_SINT_16,
        /// Unsigned 16-bit integer.
        UINT_16 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_UINT_16,
        /// Signed 32-bit integer.
        SINT_32 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_SINT_32,
        /// Unsigned 32-bit integer.
        UINT_32 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_UINT_32,
        /// Signed 64-bit integer.
        SINT_64 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_SINT_64,
        /// Unsigned 64-bit integer.
        UINT_64 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_UINT_64,
        /// IEEE-754 16-bit floating point.
        FLOAT_16 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_FLOAT_16,
        /// IEEE-754 32-bit floating point.
        FLOAT_32 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_FLOAT_32,
        /// IEEE-754 64-bit floating point.
        FLOAT_64 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_FLOAT_64,
        /// BFloat16.
        BFLOAT_16 = n::IREE_HAL_COLLECTIVE_ELEMENT_TYPE_BFLOAT_16,
    }
}

/// Collective operation kind, reduction, and element type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CollectiveOp {
    /// Operation kind.
    pub kind: CollectiveKind,
    /// Reduction, or [`CollectiveReduction::NONE`] when unused.
    pub reduction: CollectiveReduction,
    /// Element type.
    pub element_type: CollectiveElementType,
}
impl CollectiveOp {
    pub(crate) fn raw(self) -> n::iree_hal_collective_op_t {
        n::iree_hal_collective_op_t {
            __bindgen_anon_1: n::iree_hal_collective_op_t__bindgen_ty_1 {
                kind: self.kind.bits(),
                reduction: self.reduction.bits(),
                element_type: self.element_type.bits(),
                reserved: 0,
            },
        }
    }
}
