//! Textual and bytecode printing of operations to Rust writers.
use crate::{Error, Operation, Result, callback::print_to};
use iree_native as n;
use std::io::Write;

/// Configurable flags for textual operation printing.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PrintingOptions {
    /// Replace dense elements larger than this element count with an elision.
    pub large_elements_limit: Option<usize>,
    /// Omit resource strings larger than this byte count.
    pub large_resources_limit: Option<usize>,
    /// Print source debug information.
    pub debug_info: bool,
    /// Print debug locations in a human-readable, non-round-trippable form.
    pub pretty_debug_info: bool,
    /// Always print generic operation syntax.
    pub generic_form: bool,
    /// Restrict SSA name computation to local scope.
    pub local_scope: bool,
    /// Prefix SSA names with named source locations when possible.
    pub name_location_prefix: bool,
    /// Skip nested regions.
    pub skip_regions: bool,
    /// Skip verification checks in custom printers.
    pub assume_verified: bool,
}

/// Owned MLIR operation-printing flags.
pub struct OpPrintingFlags {
    pub(crate) raw: n::MlirOpPrintingFlags,
}

impl OpPrintingFlags {
    /// Create native flags configured by `options`.
    pub fn new(options: &PrintingOptions) -> Result<Self> {
        let limit = |value: Option<usize>, what: &str| {
            value
                .map(isize::try_from)
                .transpose()
                .map_err(|_| Error::Invalid(format!("{what} elision limit exceeds native range")))
        };
        let elements = limit(options.large_elements_limit, "element")?;
        let resources = limit(options.large_resources_limit, "resource")?;
        // SAFETY: this constructor takes no inputs and returns an owned flags handle kept alive through printing.
        let raw = unsafe { n::mlirOpPrintingFlagsCreate() };
        if raw.ptr.is_null() {
            return Err(Error::Null("operation printing flags"));
        }
        let flags = Self { raw };
        // SAFETY: `raw` was returned by MLIR and remains live for `mlirOpPrintingFlagsElideLargeElementsAttrs`.
        unsafe {
            if let Some(limit) = elements {
                n::mlirOpPrintingFlagsElideLargeElementsAttrs(raw, limit);
            }
            if let Some(limit) = resources {
                n::mlirOpPrintingFlagsElideLargeResourceString(raw, limit);
            }
            n::mlirOpPrintingFlagsEnableDebugInfo(
                raw,
                options.debug_info,
                options.pretty_debug_info,
            );
            if options.generic_form {
                n::mlirOpPrintingFlagsPrintGenericOpForm(raw);
            }
            if options.local_scope {
                n::mlirOpPrintingFlagsUseLocalScope(raw);
            }
            if options.name_location_prefix {
                n::mlirOpPrintingFlagsPrintNameLocAsPrefix(raw);
            }
            if options.skip_regions {
                n::mlirOpPrintingFlagsSkipRegions(raw);
            }
            if options.assume_verified {
                n::mlirOpPrintingFlagsAssumeVerified(raw);
            }
        }
        Ok(flags)
    }

    /// Borrow the native printing-flags handle.
    pub fn as_raw(&self) -> n::MlirOpPrintingFlags {
        self.raw
    }
}

impl Drop for OpPrintingFlags {
    fn drop(&mut self) {
        // SAFETY: `self` uniquely owns the native handle released by `mlirOpPrintingFlagsDestroy`.
        unsafe { n::mlirOpPrintingFlagsDestroy(self.raw) };
    }
}

/// Write an operation's textual form using `flags`.
pub fn write_operation<Ownership>(
    operation: &Operation<'_, Ownership>,
    flags: &OpPrintingFlags,
    writer: &mut impl Write,
) -> Result<()> {
    let ((), written) = print_to(writer, |callback, data| {
        // SAFETY: `operation` and `flags` are live native handles; their wrappers keep them valid throughout `mlirOperationPrintWithFlags`.
        unsafe { n::mlirOperationPrintWithFlags(operation.raw, flags.raw, callback, data) }
    });
    Ok(written?)
}

/// Write an operation's bytecode, optionally requesting a bytecode `version`.
pub fn write_bytecode<Ownership>(
    operation: &Operation<'_, Ownership>,
    version: Option<i64>,
    writer: &mut impl Write,
) -> Result<()> {
    let Some(version) = version else {
        let ((), written) = print_to(writer, |callback, data| {
            // SAFETY: `operation` is a live native handle; their wrappers keep them valid throughout `mlirOperationWriteBytecode`.
            unsafe { n::mlirOperationWriteBytecode(operation.raw, callback, data) }
        });
        return Ok(written?);
    };
    // SAFETY: this constructor takes no inputs and returns an owned config handle destroyed after bytecode emission.
    let config = unsafe { n::mlirBytecodeWriterConfigCreate() };
    if config.ptr.is_null() {
        return Err(Error::Null("bytecode writer configuration"));
    }
    // SAFETY: `config` is a live native handle; their wrappers keep them valid throughout `mlirBytecodeWriterConfigDesiredEmitVersion`.
    unsafe { n::mlirBytecodeWriterConfigDesiredEmitVersion(config, version) };
    let (result, written) = print_to(writer, |callback, data| {
        // SAFETY: `operation` and `config` are live native handles; their wrappers keep them valid throughout `mlirOperationWriteBytecodeWithConfig`.
        unsafe { n::mlirOperationWriteBytecodeWithConfig(operation.raw, config, callback, data) }
    });
    // SAFETY: `config` is uniquely owned here, and `mlirBytecodeWriterConfigDestroy` releases it exactly once.
    unsafe { n::mlirBytecodeWriterConfigDestroy(config) };
    written?;
    if n::compiler::logical_result::is_success(result) {
        Ok(())
    } else {
        Err(Error::Invalid(format!(
            "MLIR cannot emit bytecode version {version}"
        )))
    }
}
