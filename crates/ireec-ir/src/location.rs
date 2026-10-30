//! Laws: source locations are immutable context-owned values; equality and
//! printing are independent of the lifetime of a particular wrapper copy.
use crate::{
    Attribute, Borrowed, Context, Error, Result,
    callback::print_with,
    string_ref::{copy_string_ref, string_ref},
    types::{count, same_context},
};
use iree_native as n;
use std::{fmt, marker::PhantomData};

/// A source position represented by one-based line and column numbers.
pub type SourcePosition = (u32, u32);

/// A source filename and its inclusive start and end positions.
pub type FileRange = (String, SourcePosition, SourcePosition);

/// A context-owned MLIR source location.
#[derive(Clone, Copy)]
pub struct Location<'c> {
    pub(crate) raw: n::MlirLocation,
    pub(crate) _context: PhantomData<&'c ()>,
}
impl<'c> Location<'c> {
    fn checked(raw: n::MlirLocation) -> Result<Self> {
        if raw.ptr.is_null() {
            return Err(Error::Null("MLIR location"));
        }
        Ok(Self {
            raw,
            _context: PhantomData,
        })
    }
    /// Borrow a native location in its context.
    ///
    /// # Safety
    /// `raw` must be a non-null location in `context`.
    pub unsafe fn from_raw<O>(_context: &'c Context<'_, O>, raw: n::MlirLocation) -> Result<Self> {
        Self::checked(raw)
    }
    /// The native location handle.
    pub fn as_raw(&self) -> n::MlirLocation {
        self.raw
    }
    /// Create an unknown location.
    pub fn unknown<O>(context: &'c Context<'_, O>) -> Self {
        Self {
            // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirLocationUnknownGet`.
            raw: unsafe { n::mlirLocationUnknownGet(context.raw) },
            _context: PhantomData,
        }
    }
    /// Create a file, line, and column location (MLIR uses one-based positions).
    pub fn file_line_col<O>(
        context: &'c Context<'_, O>,
        filename: &str,
        line: u32,
        column: u32,
    ) -> Result<Self> {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        Self::checked(unsafe {
            n::mlirLocationFileLineColGet(context.raw, string_ref(filename), line, column)
        })
    }
    /// Create a file range with inclusive start and end source positions.
    pub fn file_line_col_range<O>(
        context: &'c Context<'_, O>,
        filename: &str,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Result<Self> {
        if end < start {
            return Err(Error::Invalid(
                "location range ends before it starts".into(),
            ));
        }
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        Self::checked(unsafe {
            n::mlirLocationFileLineColRangeGet(
                context.raw,
                string_ref(filename),
                start.0,
                start.1,
                end.0,
                end.1,
            )
        })
    }
    /// Create a named location with an optional child location.
    pub fn name<O>(context: &'c Context<'_, O>, name: &str, child: Option<Self>) -> Result<Self> {
        let child = child.unwrap_or_else(|| Self::unknown(context));
        // SAFETY: `child` is a live native handle; their wrappers keep them valid throughout `mlirLocationGetContext`.
        same_context(context.raw, unsafe { n::mlirLocationGetContext(child.raw) })?;
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
        Self::checked(unsafe { n::mlirLocationNameGet(context.raw, string_ref(name), child.raw) })
    }
    /// Create a callsite location, preserving the callee and caller ordering.
    pub fn callsite(callee: Self, caller: Self) -> Result<Self> {
        // SAFETY: `callee` is a live native handle; their wrappers keep them valid throughout `mlirLocationGetContext`.
        same_context(unsafe { n::mlirLocationGetContext(callee.raw) }, unsafe {
            n::mlirLocationGetContext(caller.raw)
        })?;
        // SAFETY: `callee` and `caller` are live native handles; their wrappers keep them valid throughout `mlirLocationCallSiteGet`.
        Self::checked(unsafe { n::mlirLocationCallSiteGet(callee.raw, caller.raw) })
    }
    /// Fuse locations and optional metadata from the same context.
    pub fn fused<O>(
        context: &'c Context<'_, O>,
        locations: &[Self],
        metadata: Option<Attribute<'c>>,
    ) -> Result<Self> {
        for loc in locations {
            // SAFETY: `loc` is a live location handle, and this getter only reads its owning context.
            same_context(context.raw, unsafe { n::mlirLocationGetContext(loc.raw) })?;
        }
        let metadata = if let Some(attr) = metadata {
            same_context(context.raw, attr.context_raw())?;
            attr.raw
        } else {
            n::MlirAttribute {
                ptr: std::ptr::null(),
            }
        };
        let raw: Vec<_> = locations.iter().map(|loc| loc.raw).collect();
        // SAFETY: `raw` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Self::checked(unsafe {
            n::mlirLocationFusedGet(context.raw, count(raw.len())?, raw.as_ptr(), metadata)
        })
    }
    /// Borrow the owning context.
    pub fn context(&self) -> Result<Context<'c, Borrowed>> {
        // SAFETY: the context handle comes from the live receiver and is borrowed only for the receiver-bound lifetime.
        unsafe { Context::borrow_raw(n::mlirLocationGetContext(self.raw)) }
    }
    /// Print MLIR source-location assembly.
    pub(crate) fn print(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationPrint`.
        Ok(print_with(|callback, data| unsafe {
            n::mlirLocationPrint(self.raw, callback, data)
        }))
    }
    /// Convert this location to its underlying attribute.
    pub fn as_attribute(&self) -> Result<Attribute<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationGetAttribute`.
        Attribute::checked(unsafe { n::mlirLocationGetAttribute(self.raw) })
    }
    /// Whether this location is unknown.
    pub fn is_unknown(&self) -> Result<bool> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationIsAUnknown`.
        Ok(unsafe { n::mlirLocationIsAUnknown(self.raw) })
    }
    /// Source file and start/end positions, absent for other location kinds.
    pub fn file_range(&self) -> Result<Option<FileRange>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationIsAFileLineColRange`.
        if !unsafe { n::mlirLocationIsAFileLineColRange(self.raw) } {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationFileLineColRangeGetFilename`.
        let filename = unsafe { n::mlirLocationFileLineColRangeGetFilename(self.raw) };
        // SAFETY: `filename` is the interned identifier returned by the live location; its context keeps the identifier storage valid.
        let filename = copy_string_ref(unsafe { n::mlirIdentifierStr(filename) })?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationFileLineColRangeGetStartLine`.
        let start = unsafe {
            (
                n::mlirLocationFileLineColRangeGetStartLine(self.raw) as u32,
                n::mlirLocationFileLineColRangeGetStartColumn(self.raw) as u32,
            )
        };
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationFileLineColRangeGetEndLine`.
        let end = unsafe {
            (
                n::mlirLocationFileLineColRangeGetEndLine(self.raw) as u32,
                n::mlirLocationFileLineColRangeGetEndColumn(self.raw) as u32,
            )
        };
        Ok(Some((filename, start, end)))
    }
    /// Callee and caller, absent for non-callsite locations.
    pub fn callsite_parts(&self) -> Result<Option<(Self, Self)>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationIsACallSite`.
        if !unsafe { n::mlirLocationIsACallSite(self.raw) } {
            return Ok(None);
        }
        Ok(Some((
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationCallSiteGetCallee`.
            Self::checked(unsafe { n::mlirLocationCallSiteGetCallee(self.raw) })?,
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationCallSiteGetCaller`.
            Self::checked(unsafe { n::mlirLocationCallSiteGetCaller(self.raw) })?,
        )))
    }
    /// Name and child, absent for unnamed locations.
    pub fn name_parts(&self) -> Result<Option<(String, Self)>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationIsAName`.
        if !unsafe { n::mlirLocationIsAName(self.raw) } {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationNameGetName`.
        let name = unsafe { n::mlirLocationNameGetName(self.raw) };
        Ok(Some((
            // SAFETY: `name` is the interned identifier returned by the live location; its context keeps the identifier storage valid.
            copy_string_ref(unsafe { n::mlirIdentifierStr(name) })?,
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationNameGetChildLoc`.
            Self::checked(unsafe { n::mlirLocationNameGetChildLoc(self.raw) })?,
        )))
    }
    /// Fused children and metadata, absent for other location kinds.
    pub fn fused_parts(&self) -> Result<Option<(Vec<Self>, Option<Attribute<'c>>)>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationIsAFused`.
        if !unsafe { n::mlirLocationIsAFused(self.raw) } {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationFusedGetNumLocations`.
        let count = unsafe { n::mlirLocationFusedGetNumLocations(self.raw) } as usize;
        let mut raw = vec![
            n::MlirLocation {
                ptr: std::ptr::null()
            };
            count
        ];
        // SAFETY: `raw` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results.
        unsafe { n::mlirLocationFusedGetLocations(self.raw, raw.as_mut_ptr()) };
        let children = raw
            .into_iter()
            .map(Self::checked)
            .collect::<Result<Vec<_>>>()?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirLocationFusedGetMetadata`.
        let metadata = unsafe { n::mlirLocationFusedGetMetadata(self.raw) };
        let metadata = (!metadata.ptr.is_null()).then(|| Attribute::borrowed(metadata));
        Ok(Some((children, metadata)))
    }
}
impl<'c> TryFrom<Attribute<'c>> for Location<'c> {
    type Error = Error;
    fn try_from(value: Attribute<'c>) -> Result<Self> {
        // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsALocation`.
        if !unsafe { n::mlirAttributeIsALocation(value.raw) } {
            return Err(Error::Invalid("expected location attribute".into()));
        }
        // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirLocationFromAttribute`.
        Self::checked(unsafe { n::mlirLocationFromAttribute(value.raw) })
    }
}
impl PartialEq for Location<'_> {
    fn eq(&self, rhs: &Self) -> bool {
        self.raw.ptr == rhs.raw.ptr
    }
}
impl Eq for Location<'_> {}
impl fmt::Debug for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print()
            .map_err(|_| fmt::Error)
            .and_then(|text| f.write_str(&text))
    }
}
impl fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn file_locations_are_interned(line in 1u32..100_000, column in 1u32..10_000) {
            let context = Context::new().unwrap();
            let first = Location::file_line_col(&context, "example.mlir", line, column).unwrap();
            let second = Location::file_line_col(&context, "example.mlir", line, column).unwrap();
            prop_assert_eq!(first, second);
            prop_assert_eq!(Location::try_from(first.as_attribute().unwrap()).unwrap(), first);
            prop_assert_eq!(first.file_range().unwrap().unwrap().1, (line, column));
        }
    }
    #[test]
    fn names_callsites_and_fusion_preserve_parts() {
        let context = Context::new().unwrap();
        let unknown = Location::unknown(&context);
        let named = Location::name(&context, "callee", Some(unknown)).unwrap();
        assert_eq!(
            named.name_parts().unwrap(),
            Some(("callee".into(), unknown))
        );
        let callsite = Location::callsite(named, unknown).unwrap();
        assert_eq!(callsite.callsite_parts().unwrap(), Some((named, unknown)));
        assert!(!callsite.print().unwrap().is_empty());
        let fused = Location::fused(&context, &[named, callsite], None).unwrap();
        assert!(fused.fused_parts().unwrap().is_some());
    }
}
