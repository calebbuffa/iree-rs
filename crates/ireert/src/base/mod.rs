//! Native values required by runtime C APIs.

pub(crate) mod string_builder;
pub(crate) mod time;
pub mod wait;

pub use wait::{WaitCallback, WaitSource, WaitStatus};
