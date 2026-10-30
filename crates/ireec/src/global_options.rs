//! Process-wide compiler options that are not scoped to a compiler session.

use crate::{Compiler, Error, Result};
use std::sync::{Mutex, OnceLock};

/// Typed settings for the compiler's process-wide command-line options.
///
/// Unlike [`crate::Options`], these settings cannot be changed per session and
/// are applied only once for the process.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GlobalOptions {
    trace_dispatch_tensors: bool,
}

impl GlobalOptions {
    /// Enables tracing of runtime input and output tensors for every dispatch.
    pub fn with_trace_dispatch_tensors(mut self) -> Self {
        self.trace_dispatch_tensors = true;
        self
    }

    fn is_empty(&self) -> bool {
        !self.trace_dispatch_tensors
    }
}

static CONFIGURATION_LOCK: Mutex<()> = Mutex::new(());
static CONFIGURED_OPTIONS: OnceLock<GlobalOptions> = OnceLock::new();

impl Compiler {
    /// Sets process-wide compiler options.
    ///
    /// These options are irreversible and must be applied before compiling.
    /// Repeating the same configuration is a no-op; a different non-empty
    /// configuration returns an error.
    pub fn set_global_options(options: &GlobalOptions) -> Result<()> {
        if options.is_empty() {
            return Ok(());
        }

        let _guard = CONFIGURATION_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(configured) = CONFIGURED_OPTIONS.get() {
            return if configured == options {
                Ok(())
            } else {
                Err(Error::ConflictingGlobalOptions)
            };
        }

        Compiler::new()?.setup_global_cl(
            &["ireec", "--iree-flow-trace-dispatch-tensors"],
            "IREE compiler global options",
            false,
        )?;
        CONFIGURED_OPTIONS
            .set(options.clone())
            .map_err(|_| Error::ConflictingGlobalOptions)
    }
}
