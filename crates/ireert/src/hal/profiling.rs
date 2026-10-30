//! HAL device profiling, sinks, profile-file parsing, metrics, and statistics.

use super::{
    CounterSetSelectionFlags, MetricId, ProfileChunkFlags, ProfileDataFamilies,
    ProfileMetricPlotHint, ProfileMetricSemantic, ProfileMetricUnit, ProfileMetricValueKind,
    ProfileRecordType, ProfileStatisticsRowFlags, ProfileStatisticsRowType,
    ProfileStatisticsTimeDomain, ProfilingFlags, device::Device,
};
use crate::io::{FileAccess, FileHandle};
use crate::{Code, Result, check, system};
use core::marker::PhantomData;
use iree_native as n;
use n::{const_span, view_bytes, view_string};

/// Rust-owned metadata attached to a profile chunk.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProfileChunkMetadata {
    /// MIME or schema identifier for the chunk.
    pub content_type: String,
    /// Human-readable chunk name.
    pub name: String,
    /// Session identifier.
    pub session_id: u64,
    /// Stream identifier.
    pub stream_id: u64,
    /// Event identifier.
    pub event_id: u64,
    /// Executable identifier.
    pub executable_id: u64,
    /// Command-buffer identifier.
    pub command_buffer_id: u64,
    /// Physical device ordinal.
    pub physical_device_ordinal: u32,
    /// Queue ordinal.
    pub queue_ordinal: u32,
    /// Native profile chunk flags.
    pub flags: ProfileChunkFlags,
    /// Number of dropped records represented by this chunk.
    pub dropped_record_count: u64,
}
impl ProfileChunkMetadata {
    fn raw(&self) -> n::iree_hal_profile_chunk_metadata_t {
        n::iree_hal_profile_chunk_metadata_t {
            content_type: crate::view(&self.content_type),
            name: crate::view(&self.name),
            session_id: self.session_id,
            stream_id: self.stream_id,
            event_id: self.event_id,
            executable_id: self.executable_id,
            command_buffer_id: self.command_buffer_id,
            physical_device_ordinal: self.physical_device_ordinal,
            queue_ordinal: self.queue_ordinal,
            flags: self.flags.bits(),
            dropped_record_count: self.dropped_record_count,
        }
    }
}

/// An owned `iree_hal_profile_sink_t` reference.
pub struct ProfileSink<'a> {
    raw: *mut n::iree_hal_profile_sink_t,
    _file_handle: Option<FileHandle<'a>>,
}
impl<'a> ProfileSink<'a> {
    /// Returns the borrowed native profiling-sink handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_profile_sink_t {
        self.raw
    }
    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(mut self) -> *mut n::iree_hal_profile_sink_t {
        // The native sink itself retains the handle.
        drop(self._file_handle.take());
        core::mem::ManuallyDrop::new(self).raw
    }
    /// Begins a profiling session.
    pub fn begin_session(&self, metadata: &ProfileChunkMetadata) -> Result<()> {
        let raw = metadata.raw();
        // SAFETY: self owns a live sink and metadata is a valid by-value native record.
        check(unsafe { n::iree_hal_profile_sink_begin_session(self.raw, &raw) })
    }
    /// Writes binary chunks to the profiling sink.
    pub fn write(&self, metadata: &ProfileChunkMetadata, chunks: &[&[u8]]) -> Result<()> {
        let spans: Vec<_> = chunks.iter().map(|bytes| const_span(bytes)).collect();
        let raw = metadata.raw();
        // SAFETY: self owns the sink and the spans borrow the chunks through this synchronous write.
        check(unsafe {
            n::iree_hal_profile_sink_write(self.raw, &raw, spans.len(), spans.as_ptr())
        })
    }
    /// Ends the profiling session with the session's final status code.
    pub fn end_session(&self, metadata: &ProfileChunkMetadata, status: Code) -> Result<()> {
        let raw = metadata.raw();
        // SAFETY: self owns a live sink and metadata is a valid by-value native record.
        check(unsafe { n::iree_hal_profile_sink_end_session(self.raw, &raw, status.to_raw()) })
    }
}
impl ProfileSink<'static> {
    /// Adopts one owned profiling-sink reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_profile_sink_t) -> Self {
        Self {
            raw,
            _file_handle: None,
        }
    }
}
impl Clone for ProfileSink<'_> {
    fn clone(&self) -> Self {
        // SAFETY: Clone retains the live sink before creating another owner.
        unsafe { n::iree_hal_profile_sink_retain(self.raw) };
        Self {
            raw: self.raw,
            _file_handle: self._file_handle.clone(),
        }
    }
}
impl Drop for ProfileSink<'_> {
    fn drop(&mut self) {
        // SAFETY: Drop releases the one sink reference owned by self.
        unsafe { n::iree_hal_profile_sink_release(self.raw) }
    }
}

/// Creates a file-backed profile sink from a native I/O handle.
pub fn file_sink<'a>(handle: &FileHandle<'a>) -> Result<ProfileSink<'a>> {
    handle.require_access(FileAccess::WRITE)?;
    let mut raw = core::ptr::null_mut();
    // SAFETY: handle is live and raw is a writable sink output slot.
    check(unsafe { n::iree_hal_profile_file_sink_create(handle.as_raw(), system(), &mut raw) })?;
    Ok(ProfileSink {
        raw,
        _file_handle: Some(handle.clone()),
    })
}

/// Rust-owned profile-file header values.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProfileFileHeader {
    /// File format magic.
    pub magic: u32,
    /// Major format version.
    pub version_major: u16,
    /// Minor format version.
    pub version_minor: u16,
    /// Header length in bytes.
    pub header_length: u32,
    /// Format flags.
    pub flags: u32,
    /// Total file length in bytes.
    pub file_length: u64,
    /// Offset of the first record.
    pub first_record_offset: usize,
}
/// Rust-owned profile record header values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProfileRecordHeader {
    /// Total record length.
    pub record_length: u64,
    /// Payload length.
    pub payload_length: u64,
    /// Session identifier.
    pub session_id: u64,
    /// Stream identifier.
    pub stream_id: u64,
    /// Event identifier.
    pub event_id: u64,
    /// Executable identifier.
    pub executable_id: u64,
    /// Command-buffer identifier.
    pub command_buffer_id: u64,
    /// Header length.
    pub header_length: u32,
    /// Content-type byte length.
    pub content_type_length: u32,
    /// Name byte length.
    pub name_length: u32,
    /// Physical device ordinal.
    pub physical_device_ordinal: u32,
    /// Queue ordinal.
    pub queue_ordinal: u32,
    /// Profile chunk flags.
    pub chunk_flags: ProfileChunkFlags,
    /// Number of dropped records.
    pub dropped_record_count: u64,
    /// Status code associated with the record.
    pub session_status_code: Code,
    /// Record category.
    pub record_type: ProfileRecordType,
    /// Record flags.
    pub flags: u16,
}
impl ProfileRecordHeader {
    fn from_raw(h: n::iree_hal_profile_file_record_header_t) -> Self {
        Self {
            record_length: h.record_length,
            payload_length: h.payload_length,
            session_id: h.session_id,
            stream_id: h.stream_id,
            event_id: h.event_id,
            executable_id: h.executable_id,
            command_buffer_id: h.command_buffer_id,
            header_length: h.header_length,
            content_type_length: h.content_type_length,
            name_length: h.name_length,
            physical_device_ordinal: h.physical_device_ordinal,
            queue_ordinal: h.queue_ordinal,
            chunk_flags: ProfileChunkFlags::from_bits_retain(h.chunk_flags),
            dropped_record_count: h.dropped_record_count,
            session_status_code: Code::from_raw(h.session_status_code as n::iree_status_code_t)
                .unwrap_or(Code::Unknown),
            record_type: ProfileRecordType::from_bits_retain(h.record_type),
            flags: h.flags,
        }
    }
}
/// Borrowed view of one profile-file record.
#[derive(Clone, Copy)]
pub struct ProfileRecord<'a> {
    /// Parsed record header.
    pub header: ProfileRecordHeader,
    /// Content type bytes.
    pub content_type: &'a [u8],
    /// Record name bytes.
    pub name: &'a [u8],
    /// Record payload bytes.
    pub payload: &'a [u8],
}
/// Parses profile-file data using the native encoding.
pub struct ProfileFile<'a> {
    bytes: &'a [u8],
    header: ProfileFileHeader,
}
impl<'a> ProfileFile<'a> {
    /// Parses the header of a profile-file byte slice.
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        let span = const_span(bytes);
        let mut header = n::iree_hal_profile_file_header_t::default();
        let mut offset = 0;
        // SAFETY: span borrows bytes for this parse and both native result slots are writable.
        check(unsafe { n::iree_hal_profile_file_parse_header(span, &mut header, &mut offset) })?;
        Ok(Self {
            bytes,
            header: ProfileFileHeader {
                magic: header.magic,
                version_major: header.version_major,
                version_minor: header.version_minor,
                header_length: header.header_length,
                flags: header.flags,
                file_length: header.file_length,
                first_record_offset: offset,
            },
        })
    }
    /// Returns parsed file header metadata.
    pub fn header(&self) -> ProfileFileHeader {
        self.header
    }
    /// Parses a record at `offset` and returns its next-record offset.
    pub fn record(&self, offset: usize) -> Result<(ProfileRecord<'a>, usize)> {
        let mut record = n::iree_hal_profile_file_record_t::default();
        let mut next = 0;
        // SAFETY: The input span borrows self.bytes and record/next are writable result slots.
        check(unsafe {
            n::iree_hal_profile_file_parse_record(
                const_span(self.bytes),
                offset,
                &mut record,
                &mut next,
            )
        })?;
        // SAFETY: the parser returns views into `bytes`, which outlives the result.
        let record = unsafe {
            ProfileRecord {
                header: ProfileRecordHeader::from_raw(record.header),
                content_type: view_bytes(record.content_type),
                name: view_bytes(record.name),
                payload: n::slice(record.payload.data, record.payload.data_length),
            }
        };
        Ok((record, next))
    }
}

/// Rust-owned profile statistics row.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProfileStatisticsRow {
    /// Row category.
    pub row_type: ProfileStatisticsRowType,
    /// Timestamp domain.
    pub time_domain: ProfileStatisticsTimeDomain,
    /// Row flags.
    pub flags: ProfileStatisticsRowFlags,
    /// Physical device ordinal.
    pub physical_device_ordinal: u32,
    /// Queue ordinal.
    pub queue_ordinal: u32,
    /// Event type.
    pub event_type: u32,
    /// Executable identifier.
    pub executable_id: u64,
    /// Command-buffer identifier.
    pub command_buffer_id: u64,
    /// Function ordinal.
    pub function_ordinal: u32,
    /// Command index.
    pub command_index: u32,
    /// Valid sample count.
    pub sample_count: u64,
    /// Invalid sample count.
    pub invalid_sample_count: u64,
    /// Operation count.
    pub operation_count: u64,
    /// Payload bytes.
    pub payload_bytes: u64,
    /// Tile count.
    pub tile_count: u64,
    /// Sum of tile durations in nanoseconds.
    pub tile_duration_sum_ns: u64,
    /// First start timestamp.
    pub first_start_time: u64,
    /// Last end timestamp.
    pub last_end_time: u64,
    /// Total duration in the row time domain.
    pub total_duration: u64,
    /// Minimum duration in the row time domain.
    pub minimum_duration: u64,
    /// Maximum duration in the row time domain.
    pub maximum_duration: u64,
}
impl ProfileStatisticsRow {
    fn from_raw(r: n::iree_hal_profile_statistics_row_t) -> Self {
        Self {
            row_type: ProfileStatisticsRowType::from_bits_retain(r.row_type),
            time_domain: ProfileStatisticsTimeDomain::from_bits_retain(r.time_domain),
            flags: ProfileStatisticsRowFlags::from_bits_retain(r.flags),
            physical_device_ordinal: r.physical_device_ordinal,
            queue_ordinal: r.queue_ordinal,
            event_type: r.event_type,
            executable_id: r.executable_id,
            command_buffer_id: r.command_buffer_id,
            function_ordinal: r.function_ordinal,
            command_index: r.command_index,
            sample_count: r.sample_count,
            invalid_sample_count: r.invalid_sample_count,
            operation_count: r.operation_count,
            payload_bytes: r.payload_bytes,
            tile_count: r.tile_count,
            tile_duration_sum_ns: r.tile_duration_sum_ns,
            first_start_time: r.first_start_time,
            last_end_time: r.last_end_time,
            total_duration: r.total_duration,
            minimum_duration: r.minimum_duration,
            maximum_duration: r.maximum_duration,
        }
    }
}
impl ProfileStatisticsRow {
    fn raw(self) -> n::iree_hal_profile_statistics_row_t {
        n::iree_hal_profile_statistics_row_t {
            row_type: self.row_type.bits(),
            time_domain: self.time_domain.bits(),
            flags: self.flags.bits(),
            physical_device_ordinal: self.physical_device_ordinal,
            queue_ordinal: self.queue_ordinal,
            event_type: self.event_type,
            executable_id: self.executable_id,
            command_buffer_id: self.command_buffer_id,
            function_ordinal: self.function_ordinal,
            command_index: self.command_index,
            sample_count: self.sample_count,
            invalid_sample_count: self.invalid_sample_count,
            operation_count: self.operation_count,
            payload_bytes: self.payload_bytes,
            tile_count: self.tile_count,
            tile_duration_sum_ns: self.tile_duration_sum_ns,
            first_start_time: self.first_start_time,
            last_end_time: self.last_end_time,
            total_duration: self.total_duration,
            minimum_duration: self.minimum_duration,
            maximum_duration: self.maximum_duration,
        }
    }
}

/// An owned native profile-statistics sink.
pub struct ProfileStatisticsSink(*mut n::iree_hal_profile_statistics_sink_t);
impl ProfileStatisticsSink {
    /// Creates an empty statistics sink.
    pub fn new() -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: raw is a writable output slot for the newly created statistics sink.
        check(unsafe { n::iree_hal_profile_statistics_sink_create(system(), &mut raw) })?;
        Ok(Self(raw))
    }
    /// Adopts one owned native statistics-sink reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_profile_statistics_sink_t) -> Self {
        Self(raw)
    }
    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_profile_statistics_sink_t {
        self.0
    }
    /// Transfers ownership of the native statistics sink to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_profile_statistics_sink_t {
        let p = self.0;
        core::mem::forget(self);
        p
    }
    /// Returns a retained base profile sink for attaching to device profiling.
    pub fn profile_sink(&self) -> ProfileSink<'static> {
        // SAFETY: self owns the statistics sink whose base sink is borrowed for this call.
        let raw = unsafe { n::iree_hal_profile_statistics_sink_base(self.0) };
        // SAFETY: The returned base sink is retained before it is wrapped as an owned reference.
        unsafe {
            n::iree_hal_profile_sink_retain(raw);
            ProfileSink::from_raw(raw)
        }
    }
    /// Returns the row count.
    pub fn row_count(&self) -> usize {
        // SAFETY: self owns a live statistics sink.
        unsafe { n::iree_hal_profile_statistics_sink_row_count(self.0) }
    }
    /// Returns the count of dropped records.
    pub fn dropped_record_count(&self) -> u64 {
        // SAFETY: self owns a live statistics sink.
        unsafe { n::iree_hal_profile_statistics_sink_dropped_record_count(self.0) }
    }
    /// Copies all available rows into owned Rust values.
    pub fn rows(&self) -> Result<Vec<ProfileStatisticsRow>> {
        let mut rows = Vec::with_capacity(self.row_count());
        let callback = n::iree_hal_profile_statistics_row_callback_t {
            fn_: Some(collect_row),
            user_data: (&mut rows as *mut Vec<ProfileStatisticsRow>).cast(),
        };
        // SAFETY: self owns the sink and callback context points to rows for this synchronous iteration.
        check(unsafe { n::iree_hal_profile_statistics_sink_for_each_row(self.0, callback) })?;
        Ok(rows)
    }
    /// Finds a function name for an executable-local function ordinal.
    pub fn function_name(&self, executable_id: u64, function_ordinal: u32) -> Option<String> {
        let mut out = n::iree_string_view_t {
            data: core::ptr::null(),
            size: 0,
        };
        // SAFETY: self owns the sink and out is writable for the returned name view.
        if !unsafe {
            n::iree_hal_profile_statistics_sink_find_function_name(
                self.0,
                executable_id,
                function_ordinal,
                &mut out,
            )
        } {
            return None;
        }
        // SAFETY: successful lookup returns a view into sink-owned storage.
        Some(unsafe { view_string(out) })
    }
    /// Converts a duration to nanoseconds when the row time domain is known.
    pub fn duration_ns(&self, row: ProfileStatisticsRow, duration: u64) -> Option<u64> {
        let raw = row.raw();
        let mut out = 0;
        // SAFETY: self owns the sink and raw row/out value are valid synchronous inputs.
        unsafe {
            n::iree_hal_profile_statistics_sink_scale_duration_to_ns(
                self.0, &raw, duration, &mut out,
            )
        }
        .then_some(out)
    }
}
impl Clone for ProfileStatisticsSink {
    fn clone(&self) -> Self {
        // SAFETY: Clone retains the live statistics sink before creating another owner.
        unsafe { n::iree_hal_profile_statistics_sink_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for ProfileStatisticsSink {
    fn drop(&mut self) {
        // SAFETY: Drop releases the one statistics-sink reference owned by self.
        unsafe { n::iree_hal_profile_statistics_sink_release(self.0) }
    }
}
unsafe extern "C" fn collect_row(
    user_data: *mut core::ffi::c_void,
    row: *const n::iree_hal_profile_statistics_row_t,
) -> n::iree_status_t {
    n::callback_status(|| {
        // SAFETY: the native callback supplies a valid row during the call,
        // and the user data points to our live Vec.
        let row = unsafe { *row };
        // SAFETY: The native visitor supplies a live row and user_data is the Vec passed by rows().
        unsafe { &mut *user_data.cast::<Vec<ProfileStatisticsRow>>() }
            .push(ProfileStatisticsRow::from_raw(row));
        Ok(())
    })
}

/// A profile counter-set selection.
#[derive(Clone, Copy)]
pub struct CounterSetSelection<'a> {
    /// Selection flags.
    pub flags: CounterSetSelectionFlags,
    /// Counter-set name.
    pub name: &'a str,
    /// Counter names selected from the set.
    pub counter_names: &'a [&'a str],
}
/// Capture-filter criteria for device profiling; `None` fields match everything.
#[derive(Clone, Copy, Debug, Default)]
pub struct ProfileCaptureFilter<'a> {
    /// Function-name pattern.
    pub executable_function_pattern: Option<&'a str>,
    /// Command-buffer identifier.
    pub command_buffer_id: Option<u64>,
    /// Command index.
    pub command_index: Option<u32>,
    /// Physical-device ordinal.
    pub physical_device_ordinal: Option<u32>,
    /// Queue ordinal.
    pub queue_ordinal: Option<u32>,
}
impl ProfileCaptureFilter<'_> {
    /// Tests whether a command location matches this filter.
    pub fn matches_location(
        self,
        command_buffer_id: u64,
        command_index: u32,
        physical_device_ordinal: u32,
        queue_ordinal: u32,
    ) -> bool {
        let raw = self.raw();
        // SAFETY: `raw` borrows `self` for the duration of the call.
        unsafe {
            n::iree_hal_profile_capture_filter_matches_location(
                &raw,
                command_buffer_id,
                command_index,
                physical_device_ordinal,
                queue_ordinal,
            )
        }
    }

    fn raw(self) -> n::iree_hal_profile_capture_filter_t {
        let flag = |set: bool, bit: n::iree_hal_profile_capture_filter_flag_bits_t| {
            if set {
                bit as n::iree_hal_profile_capture_filter_flags_t
            } else {
                0
            }
        };
        n::iree_hal_profile_capture_filter_t {
            flags: flag(
                self.executable_function_pattern.is_some(),
                n::IREE_HAL_PROFILE_CAPTURE_FILTER_FLAG_EXECUTABLE_FUNCTION_PATTERN,
            ) | flag(
                self.command_buffer_id.is_some(),
                n::IREE_HAL_PROFILE_CAPTURE_FILTER_FLAG_COMMAND_BUFFER_ID,
            ) | flag(
                self.command_index.is_some(),
                n::IREE_HAL_PROFILE_CAPTURE_FILTER_FLAG_COMMAND_INDEX,
            ) | flag(
                self.physical_device_ordinal.is_some(),
                n::IREE_HAL_PROFILE_CAPTURE_FILTER_FLAG_PHYSICAL_DEVICE_ORDINAL,
            ) | flag(
                self.queue_ordinal.is_some(),
                n::IREE_HAL_PROFILE_CAPTURE_FILTER_FLAG_QUEUE_ORDINAL,
            ),
            executable_function_pattern: crate::view(
                self.executable_function_pattern.unwrap_or_default(),
            ),
            command_buffer_id: self.command_buffer_id.unwrap_or_default(),
            command_index: self.command_index.unwrap_or_default(),
            physical_device_ordinal: self.physical_device_ordinal.unwrap_or_default(),
            queue_ordinal: self.queue_ordinal.unwrap_or_default(),
            reserved0: 0,
        }
    }
}
/// Borrowed profiling options tied to their sink and counter-set data.
pub struct ProfilingOptions<'a> {
    raw: n::iree_hal_device_profiling_options_t,
    _counter_names: Vec<Vec<n::iree_string_view_t>>,
    _counter_sets: Vec<n::iree_hal_profile_counter_set_selection_t>,
    _borrow: PhantomData<&'a ()>,
}
impl<'a> ProfilingOptions<'a> {
    /// Creates options attached to an owned sink, with optional filters and counter selections.
    ///
    /// The device can retain the sink beyond these options, so its backing
    /// file cannot borrow host memory.
    pub fn new(
        sink: &'a ProfileSink<'static>,
        flags: ProfilingFlags,
        data_families: ProfileDataFamilies,
        capture_filter: ProfileCaptureFilter<'a>,
        counter_sets: &[CounterSetSelection<'a>],
    ) -> Self {
        let counter_names: Vec<Vec<n::iree_string_view_t>> = counter_sets
            .iter()
            .map(|selection| {
                selection
                    .counter_names
                    .iter()
                    .map(|name| crate::view(name))
                    .collect()
            })
            .collect();
        let native_sets: Vec<n::iree_hal_profile_counter_set_selection_t> = counter_sets
            .iter()
            .enumerate()
            .map(
                |(index, selection)| n::iree_hal_profile_counter_set_selection_t {
                    flags: selection.flags.bits(),
                    name: crate::view(selection.name),
                    counter_name_count: counter_names[index].len(),
                    counter_names: counter_names[index].as_ptr(),
                },
            )
            .collect();
        let raw = n::iree_hal_device_profiling_options_t {
            flags: flags.bits(),
            data_families: data_families.bits(),
            sink: sink.as_raw(),
            capture_filter: capture_filter.raw(),
            counter_set_count: native_sets.len(),
            counter_sets: native_sets.as_ptr(),
        };
        Self {
            raw,
            _counter_names: counter_names,
            _counter_sets: native_sets,
            _borrow: PhantomData,
        }
    }

    /// Returns whether the options request any of the specified profile data families.
    pub fn requests_data(&self, families: ProfileDataFamilies) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_data(&self.raw, families.bits()) }
    }

    /// Returns whether lightweight statistics are requested.
    pub fn requests_lightweight_statistics(&self) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_lightweight_statistics(&self.raw) }
    }

    /// Returns whether counter samples are requested.
    pub fn requests_counter_samples(&self) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_counter_samples(&self.raw) }
    }

    /// Returns whether counter ranges are requested.
    pub fn requests_counter_ranges(&self) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_counter_ranges(&self.raw) }
    }

    /// Returns whether counters are requested.
    pub fn requests_counters(&self) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_counters(&self.raw) }
    }

    /// Returns whether executable traces are requested.
    pub fn requests_executable_traces(&self) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_executable_traces(&self.raw) }
    }

    /// Returns whether device metrics are requested.
    pub fn requests_device_metrics(&self) -> bool {
        // SAFETY: raw is a valid by-value profiling-options struct.
        unsafe { n::iree_hal_device_profiling_options_requests_device_metrics(&self.raw) }
    }
}

impl Device {
    /// Begins device profiling with the specified options.
    pub fn profiling_begin(&self, options: &ProfilingOptions<'_>) -> Result<()> {
        // SAFETY: The device and borrowed options backing arrays remain live for this call.
        check(unsafe { n::iree_hal_device_profiling_begin(self.as_raw(), &options.raw) })
    }
}

/// Rust-owned profile metric metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileMetricDescriptor {
    /// Metric identifier.
    pub id: MetricId,
    /// Metric unit.
    pub unit: ProfileMetricUnit,
    /// Sample representation.
    pub value_kind: ProfileMetricValueKind,
    /// Sampling semantic.
    pub semantic: ProfileMetricSemantic,
    /// Visualization hint.
    pub plot_hint: ProfileMetricPlotHint,
    /// Metric name.
    pub name: String,
    /// Metric description.
    pub description: String,
}
fn descriptor(
    raw: *const n::iree_hal_profile_metric_descriptor_t,
) -> Option<ProfileMetricDescriptor> {
    // SAFETY: non-null descriptors live in the static built-in registry.
    let raw = unsafe { raw.as_ref() }?;
    // SAFETY: descriptor text is stored in the static built-in registry.
    let text = |view| unsafe { view_string(view) };
    Some(ProfileMetricDescriptor {
        id: MetricId::from_bits_retain(raw.metric_id),
        unit: ProfileMetricUnit::from_bits_retain(raw.unit),
        value_kind: ProfileMetricValueKind::from_bits_retain(raw.value_kind),
        semantic: ProfileMetricSemantic::from_bits_retain(raw.semantic),
        plot_hint: ProfileMetricPlotHint::from_bits_retain(raw.plot_hint),
        name: text(raw.name),
        description: text(raw.description),
    })
}
/// Returns whether a metric identifier is a built-in identifier.
pub fn metric_id_is_builtin(id: MetricId) -> bool {
    // SAFETY: The metric identifier is passed as a by-value integer.
    unsafe { n::iree_hal_profile_metric_id_is_builtin(id.bits()) }
}
/// Returns whether a metric identifier is producer-specific.
pub fn metric_id_is_producer_specific(id: MetricId) -> bool {
    // SAFETY: The metric identifier is passed as a by-value integer.
    unsafe { n::iree_hal_profile_metric_id_is_producer_specific(id.bits()) }
}
/// Returns the number of registered built-in metric descriptors.
pub fn builtin_metric_descriptor_count() -> usize {
    // SAFETY: The descriptor-count query has no pointers or mutable state.
    unsafe { n::iree_hal_profile_builtin_metric_descriptor_count() }
}
/// Looks up a built-in metric descriptor by index.
pub fn builtin_metric_descriptor_at(index: usize) -> Option<ProfileMetricDescriptor> {
    // SAFETY: The registry lookup accepts a by-value index and returns a static descriptor pointer.
    descriptor(unsafe { n::iree_hal_profile_builtin_metric_descriptor_at(index) })
}
/// Looks up a built-in metric descriptor by identifier.
pub fn builtin_metric_descriptor(id: MetricId) -> Option<ProfileMetricDescriptor> {
    // SAFETY: The registry lookup accepts a by-value identifier and returns a static descriptor pointer.
    descriptor(unsafe { n::iree_hal_profile_builtin_metric_descriptor_lookup(id.bits()) })
}
/// Looks up a built-in metric descriptor by its name.
pub fn builtin_metric_descriptor_by_name(name: &str) -> Option<ProfileMetricDescriptor> {
    // SAFETY: name is borrowed for the synchronous lookup and returned text is static.
    descriptor(unsafe {
        n::iree_hal_profile_builtin_metric_descriptor_lookup_name(crate::view(name))
    })
}
/// Returns the native name for a metric unit value.
pub fn metric_unit_name(value: ProfileMetricUnit) -> String {
    // SAFETY: The unit-name function returns a view into static native storage.
    static_text(unsafe { n::iree_hal_profile_metric_unit_string(value.bits()) })
}
/// Returns the native name for a metric value-kind value.
pub fn metric_value_kind_name(value: ProfileMetricValueKind) -> String {
    // SAFETY: The value-kind function returns a view into static native storage.
    static_text(unsafe { n::iree_hal_profile_metric_value_kind_string(value.bits()) })
}
/// Returns the native name for a metric semantic value.
pub fn metric_semantic_name(value: ProfileMetricSemantic) -> String {
    // SAFETY: The semantic-name function returns a view into static native storage.
    static_text(unsafe { n::iree_hal_profile_metric_semantic_string(value.bits()) })
}
/// Returns the native name for a metric plot-hint value.
pub fn metric_plot_hint_name(value: ProfileMetricPlotHint) -> String {
    // SAFETY: The plot-hint function returns a view into static native storage.
    static_text(unsafe { n::iree_hal_profile_metric_plot_hint_string(value.bits()) })
}
fn static_text(view: n::iree_string_view_t) -> String {
    // SAFETY: enum-name functions return views into static strings.
    unsafe { view_string(view) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn metric_id_partition_is_disjoint(id in any::<u64>()) {
            let id = MetricId::from_bits_retain(id);
            prop_assert!(!(metric_id_is_builtin(id) && metric_id_is_producer_specific(id)));
        }
        #[test]
        fn builtin_lookup_by_index_is_bounded(index in any::<usize>()) {
            if index >= builtin_metric_descriptor_count() {
                prop_assert!(builtin_metric_descriptor_at(index).is_none());
            }
        }
    }
}
