//! Compiled tokenizer regular expressions and incremental matching.
use crate::{Code, Error, Offset, Result, check, n, view};
use std::{
    ffi::CStr,
    mem::MaybeUninit,
    ptr::{self, NonNull},
};

/// Flags for compiling a tokenizer regular expression.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct RegexCompileFlags(u32);
impl RegexCompileFlags {
    /// No compile options.
    pub const NONE: Self = Self(0);
    /// Match ASCII letters without regard to case.
    pub const CASE_INSENSITIVE: Self = Self(1);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    /// Returns the flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Properties of a compiled tokenizer regular expression.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct RegexDfaFlags(u16);
impl RegexDfaFlags {
    /// No DFA properties.
    pub const NONE: Self = Self(0);
    /// The DFA uses lookahead.
    pub const HAS_LOOKAHEAD: Self = Self(1);
    /// The DFA uses Unicode categories.
    pub const UNICODE: Self = Self(2);
    /// The DFA is case-insensitive.
    pub const CASE_INSENSITIVE: Self = Self(4);
    /// The DFA uses anchors.
    pub const HAS_ANCHORS: Self = Self(8);
    /// The DFA tracks alternation branches.
    pub const HAS_BRANCHES: Self = Self(16);
    /// The DFA includes Unicode range transitions.
    pub const HAS_RANGES: Self = Self(32);
    /// Constructs flags from raw bits, preserving unknown bits.
    pub const fn from_bits_retain(bits: u16) -> Self {
        Self(bits)
    }
    /// Returns the flag bits.
    pub const fn bits(self) -> u16 {
        self.0
    }
}

/// An owned compiled regular-expression DFA and its backing bytes.
pub struct RegexDfa {
    pub(crate) raw: n::iree_tokenizer_regex_dfa_t,
    pub(crate) storage: NonNull<u8>,
}
// SAFETY: the DFA owns system-allocator storage with no thread-affine resources.
// Native query APIs take a const DFA and keep execution state in caller storage.
unsafe impl Send for RegexDfa {}
// SAFETY: concurrent queries only read that immutable data and use independent
// caller-owned execution state.
unsafe impl Sync for RegexDfa {}
impl Drop for RegexDfa {
    fn drop(&mut self) {
        // SAFETY: storage was allocated by the system allocator for this DFA.
        unsafe { n::iree_allocator_free(crate::status::system(), self.storage.as_ptr().cast()) }
    }
}
impl RegexDfa {
    /// Compiles a regex pattern into an owned DFA.
    ///
    /// Errors include the native diagnostic position, span, and message.
    pub fn compile(pattern: &str, flags: RegexCompileFlags) -> Result<Self> {
        let mut raw = MaybeUninit::uninit();
        let mut storage = ptr::null_mut();
        let mut diagnostic = MaybeUninit::<n::iree_tokenizer_regex_compile_error_t>::zeroed();
        // SAFETY: Pattern and output storage remain valid for the synchronous native call.
        let status = unsafe {
            n::iree_tokenizer_regex_compile_and_load(
                view(pattern),
                flags.bits(),
                crate::status::system(),
                raw.as_mut_ptr(),
                &mut storage,
                diagnostic.as_mut_ptr(),
            )
        };
        // SAFETY: The zero-initialized diagnostic storage is always valid to read.
        let diagnostic = unsafe { diagnostic.assume_init() };
        if let Err(source) = check(status) {
            let message = if diagnostic.message.is_null() {
                None
            } else {
                // SAFETY: The compiler supplies a static NUL-terminated diagnostic string.
                Some(
                    unsafe { CStr::from_ptr(diagnostic.message) }
                        .to_string_lossy()
                        .into_owned(),
                )
            };
            let position = (!diagnostic.message.is_null()).then_some(diagnostic.position);
            let span = (diagnostic.length > 0).then(|| {
                diagnostic.position..diagnostic.position.saturating_add(diagnostic.length)
            });
            let mut details = source.message().to_owned();
            if let Some(position) = position {
                details.push_str(&format!(" at byte {position}"));
                if let Some(span) = span {
                    details.push_str(&format!(" (span {}..{})", span.start, span.end));
                }
            }
            for payload in source.payload() {
                details.push('\n');
                details.push_str(&payload.text);
            }
            if let Some(message) = message {
                details.push_str(": ");
                details.push_str(&message);
            }
            return Err(Error::new(source.code(), details));
        }
        // SAFETY: The successful native call initialized this output value.
        let raw = unsafe { raw.assume_init() };
        let storage = crate::status::nonnull(
            storage.cast::<u8>(),
            "DFA compilation returned null storage",
        )?;
        Ok(Self { raw, storage })
    }

    /// Loads and validates an owned copy of binary DFA data.
    pub fn load(binary: &[u8]) -> Result<Self> {
        if binary.is_empty() {
            return Err(Error::new(Code::InvalidArgument, "DFA data is empty"));
        }
        let allocator = crate::status::system();
        let mut storage = ptr::null_mut();
        // SAFETY: The allocator/output slot are valid; success gives the wrapper unique ownership.
        check(unsafe { n::iree_allocator_malloc(allocator, binary.len(), &mut storage) })?;
        let storage = crate::status::nonnull(
            storage.cast::<u8>(),
            "DFA storage allocation succeeded with a null output",
        )?;
        // SAFETY: Both slices are valid and their allocations do not overlap.
        unsafe {
            ptr::copy_nonoverlapping(binary.as_ptr(), storage.as_ptr(), binary.len());
        }
        let span = n::iree_const_byte_span_t {
            data: storage.as_ptr(),
            data_length: binary.len(),
        };
        let mut raw = MaybeUninit::uninit();
        if let Err(error) =
            // SAFETY: The owned input span is valid; the output slot is initialized on success.
            check(unsafe { n::iree_tokenizer_regex_dfa_load(span, raw.as_mut_ptr()) })
        {
            // SAFETY: The allocation is uniquely owned here and came from this allocator.
            unsafe { n::iree_allocator_free(allocator, storage.as_ptr().cast()) };
            return Err(error);
        }
        // SAFETY: The successful native call initialized this output value.
        let raw = unsafe { raw.assume_init() };
        // SAFETY: The DFA backing allocation remains live during validation.
        if let Err(error) = check(unsafe { n::iree_tokenizer_regex_dfa_validate(&raw) }) {
            // SAFETY: This allocation came from the matching allocator and is uniquely owned here.
            unsafe { n::iree_allocator_free(allocator, storage.as_ptr().cast()) };
            return Err(error);
        }
        Ok(Self { raw, storage })
    }

    /// Returns the number of DFA states.
    pub fn state_count(&self) -> u16 {
        // SAFETY: The referenced DFA and buffers remain live for this call.
        unsafe { n::iree_tokenizer_regex_dfa_state_count(&self.raw) }
    }

    /// Returns the DFA's start state.
    pub fn start_state(&self) -> u16 {
        // SAFETY: The referenced DFA and buffers remain live for this call.
        unsafe { n::iree_tokenizer_regex_dfa_start_state(&self.raw) }
    }

    /// Returns compile-time properties of the DFA.
    pub fn flags(&self) -> RegexDfaFlags {
        // SAFETY: The referenced DFA and buffers remain live for this call.
        RegexDfaFlags::from_bits_retain(unsafe { n::iree_tokenizer_regex_dfa_flags(&self.raw) })
    }

    /// Whether matching uses Unicode category transitions.
    pub fn uses_unicode(&self) -> bool {
        // SAFETY: The referenced DFA and buffers remain live for this call.
        unsafe { n::iree_tokenizer_regex_dfa_uses_unicode(&self.raw) }
    }

    /// Performs one transition using a caller-supplied byte or Unicode category.
    pub fn step(&self, state: u16, codepoint: char, category_byte: u8) -> Option<u16> {
        if state >= self.state_count() {
            return None;
        }
        // SAFETY: The referenced DFA and buffers remain live for this call.
        let next = unsafe {
            n::iree_tokenizer_regex_dfa_step(&self.raw, state, codepoint as u32, category_byte)
        };
        (next != u16::MAX).then_some(next)
    }

    /// Whether `state` is accepting.
    pub fn is_accepting(&self, state: u16) -> bool {
        state < self.state_count()
            // SAFETY: The referenced DFA and buffers remain live for this call.
            && unsafe { n::iree_tokenizer_regex_dfa_is_accepting(&self.raw, state) }
    }

    /// Calls `visitor` for each match without collecting ranges.
    pub fn for_each_match<F>(&self, text: &str, mut visitor: F) -> Result<()>
    where
        F: FnMut(Offset) -> Result<()>,
    {
        let user_data = ptr::from_mut(&mut visitor).cast::<core::ffi::c_void>();
        // SAFETY: The DFA and callback context remain live throughout this synchronous call;
        // the callback converts panics into an IREE status before returning to C.
        check(unsafe {
            n::iree_tokenizer_regex_exec(
                &self.raw,
                view(text),
                Some(regex_match_callback::<F>),
                user_data,
            )
        })
    }

    /// Returns every match range found in `text`.
    pub fn matches(&self, text: &str) -> Result<Vec<Offset>> {
        let mut output = Vec::new();
        self.for_each_match(text, |matched| {
            output.push(matched);
            Ok(())
        })?;
        Ok(output)
    }

    /// Allocates optional ASCII stride acceleration for repeated execution.
    pub fn accelerate(&self) -> Result<RegexStride<'_>> {
        let mut raw = ptr::null_mut();
        // SAFETY: The DFA remains live for the resulting stride borrow.
        check(unsafe {
            n::iree_tokenizer_regex_stride_allocate(&self.raw, crate::status::system(), &mut raw)
        })?;
        Ok(RegexStride {
            dfa: self,
            raw: crate::status::nonnull(raw, "regex stride allocation returned null")?,
        })
    }

    /// Creates an incremental matcher for this DFA.
    pub fn exec_state(&self) -> RegexExecState<'_> {
        RegexExecState::new(self)
    }

    /// Counts non-overlapping matches in `text` without allocating ranges.
    pub fn count_matches(&self, text: &str) -> Result<usize> {
        let mut count = 0;
        // SAFETY: The referenced DFA and buffers remain live for this call.
        check(unsafe { n::iree_tokenizer_regex_count_matches(&self.raw, view(text), &mut count) })?;
        Ok(count)
    }

    /// Tests whether the pattern matches anywhere in `text`.
    pub fn has_match(&self, text: &str) -> Result<bool> {
        let mut matched = false;
        // SAFETY: The referenced DFA and buffers remain live for this call.
        check(unsafe { n::iree_tokenizer_regex_has_match(&self.raw, view(text), &mut matched) })?;
        Ok(matched)
    }
}

/// Owned stride data tied to the DFA from which it was computed.
pub struct RegexStride<'a> {
    dfa: &'a RegexDfa,
    raw: NonNull<n::iree_tokenizer_regex_stride_t>,
}
impl Drop for RegexStride<'_> {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the native stride allocation.
        unsafe { n::iree_tokenizer_regex_stride_free(self.raw.as_ptr(), crate::status::system()) }
    }
}
impl RegexStride<'_> {
    /// Returns the precomputed bitmap of states eligible for the fast path.
    pub fn boring_bitmap(&self) -> &[u8] {
        // SAFETY: The native stride and its trailing bitmap remain live for this borrow.
        let bitmap = unsafe { n::iree_tokenizer_regex_stride_boring_bitmap(self.raw.as_ptr()) };
        // SAFETY: self holds a live, non-null stride allocation.
        let length = (unsafe { self.raw.as_ref().num_states } as usize).div_ceil(8);
        // SAFETY: The header's bitmap accessor points to at least ceil(states / 8) bytes.
        unsafe { n::slice(bitmap, length) }
    }

    /// Returns whether a state can use the accelerated scan path.
    pub fn is_boring(&self, state: u16) -> Option<bool> {
        // SAFETY: self holds a live, non-null stride allocation.
        let num_states = unsafe { self.raw.as_ref().num_states };
        if state >= num_states {
            return None;
        }
        // SAFETY: The state is in range and the stride data remains live.
        Some(unsafe { n::iree_tokenizer_regex_stride_is_boring(self.raw.as_ptr(), state) })
    }

    /// Returns the ASCII self-loop category for a state, if one exists.
    pub fn self_loop_category(&self, state: u16) -> Option<u8> {
        // SAFETY: self holds a live, non-null stride allocation.
        let num_states = unsafe { self.raw.as_ref().num_states };
        if state >= num_states {
            return None;
        }
        // SAFETY: The state is in range and the stride data remains live.
        let category =
            unsafe { n::iree_tokenizer_regex_stride_self_loop_category(self.raw.as_ptr(), state) };
        (category != u8::MAX).then_some(category)
    }
}

/// Incremental matcher borrowing its compiled DFA.
pub struct RegexExecState<'a> {
    dfa: &'a RegexDfa,
    raw: n::iree_tokenizer_regex_exec_state_t,
    offset: usize,
    finalized: bool,
}
impl<'a> RegexExecState<'a> {
    fn new(dfa: &'a RegexDfa) -> Self {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: The output has native state layout and the borrowed DFA outlives the state.
        unsafe { n::iree_tokenizer_regex_exec_initialize(raw.as_mut_ptr(), &dfa.raw) };
        Self {
            dfa,
            // SAFETY: The successful native call initialized this output value.
            raw: unsafe { raw.assume_init() },
            offset: 0,
            finalized: false,
        }
    }

    /// Feeds a UTF-8 chunk and returns matches completed by the chunk.
    pub fn feed(&mut self, chunk: &str) -> Result<Vec<Offset>> {
        self.feed_using_stride(chunk, None)
    }

    /// Feeds a chunk using an accelerator built for this exact DFA.
    pub fn feed_with_stride(
        &mut self,
        chunk: &str,
        stride: &RegexStride<'_>,
    ) -> Result<Vec<Offset>> {
        if !ptr::eq(self.dfa, stride.dfa) {
            return Err(Error::new(
                Code::InvalidArgument,
                "regex stride belongs to another DFA",
            ));
        }
        self.feed_using_stride(chunk, Some(stride))
    }

    fn feed_using_stride(
        &mut self,
        chunk: &str,
        stride: Option<&RegexStride<'_>>,
    ) -> Result<Vec<Offset>> {
        let mut output = Vec::new();
        self.feed_visit_using_stride(chunk, stride, |matched| {
            output.push(matched);
            Ok(())
        })?;
        Ok(output)
    }

    /// Feeds a chunk and invokes `visitor` for each newly completed match.
    pub fn feed_each<F>(&mut self, chunk: &str, visitor: F) -> Result<()>
    where
        F: FnMut(Offset) -> Result<()>,
    {
        self.feed_visit_using_stride(chunk, None, visitor)
    }

    /// Feeds a chunk, invoking `visitor` with optional stride acceleration.
    pub fn feed_each_with_stride<F>(
        &mut self,
        chunk: &str,
        stride: &RegexStride<'_>,
        visitor: F,
    ) -> Result<()>
    where
        F: FnMut(Offset) -> Result<()>,
    {
        if !ptr::eq(self.dfa, stride.dfa) {
            return Err(Error::new(
                Code::InvalidArgument,
                "regex stride belongs to another DFA",
            ));
        }
        self.feed_visit_using_stride(chunk, Some(stride), visitor)
    }

    fn feed_visit_using_stride<F>(
        &mut self,
        chunk: &str,
        stride: Option<&RegexStride<'_>>,
        mut visitor: F,
    ) -> Result<()>
    where
        F: FnMut(Offset) -> Result<()>,
    {
        if self.finalized {
            return Err(Error::new(
                Code::FailedPrecondition,
                "regex execution state is finalized",
            ));
        }
        let next_offset = self
            .offset
            .checked_add(chunk.len())
            .ok_or_else(|| Error::new(Code::OutOfRange, "regex input length overflow"))?;
        let user_data = ptr::from_mut(&mut visitor).cast::<core::ffi::c_void>();
        let stride = stride.map_or(ptr::null(), |value| value.raw.as_ptr());
        // SAFETY: DFA, optional stride, chunk, and callback context remain live for this call.
        let status = unsafe {
            n::iree_tokenizer_regex_exec_feed(
                &self.dfa.raw,
                &mut self.raw,
                view(chunk),
                self.offset,
                stride,
                Some(regex_match_callback::<F>),
                user_data,
            )
        };
        if let Err(error) = check(status) {
            self.finalized = true;
            return Err(error);
        }
        self.offset = next_offset;
        Ok(())
    }

    /// Completes matching and returns any final pending ranges.
    pub fn finalize(&mut self) -> Result<Vec<Offset>> {
        let mut output = Vec::new();
        self.finalize_each(|matched| {
            output.push(matched);
            Ok(())
        })?;
        Ok(output)
    }

    /// Finalizes matching and invokes `visitor` for remaining matches.
    pub fn finalize_each<F>(&mut self, mut visitor: F) -> Result<()>
    where
        F: FnMut(Offset) -> Result<()>,
    {
        if self.finalized {
            return Err(Error::new(
                Code::FailedPrecondition,
                "regex execution state is already finalized",
            ));
        }
        let user_data = ptr::from_mut(&mut visitor).cast::<core::ffi::c_void>();
        // SAFETY: The DFA and callback context remain live for this synchronous call.
        let status = unsafe {
            n::iree_tokenizer_regex_exec_finalize(
                &self.dfa.raw,
                &mut self.raw,
                self.offset,
                Some(regex_match_callback::<F>),
                user_data,
            )
        };
        self.finalized = true;
        check(status)
    }
}

unsafe extern "C" fn regex_match_callback<F>(
    user_data: *mut std::ffi::c_void,
    matched: n::iree_tokenizer_regex_match_t,
) -> n::iree_status_t
where
    F: FnMut(Offset) -> Result<()>,
{
    n::callback_status(|| {
        if user_data.is_null() {
            return Err(Error::new(
                Code::InvalidArgument,
                "regex callback context is null",
            ));
        }
        // SAFETY: Native execution invokes this callback synchronously with the live context.
        let visitor = unsafe { &mut *user_data.cast::<F>() };
        visitor(Offset {
            start: matched.start,
            end: matched.end,
        })
    })
}
