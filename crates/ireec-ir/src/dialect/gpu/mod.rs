//! GPU dialect attributes, target information, and lowering configuration.
//!
//! Laws: GPU enum discriminants are preserved exactly, and typed attribute
//! queries reject attributes outside their defining context or dialect.

mod lowering_config;
mod mma;
mod pipeline;
mod target;

pub use lowering_config::{LoweringConfigAttr, SubgroupBasis, TileSizes};
pub use mma::{
    MmaAttr, MmaInfo, MmaIntrinsic, MmaIntrinsicAttr, MmaOperand, MmaSingleSubgroupLayout,
    VirtualMmaAttr, VirtualMmaIntrinsic, VirtualMmaIntrinsicAttr, XorShuffleBounds,
};
pub use pipeline::{
    LoweringPipeline, PipelineAttr, PipelineOptions, PipelineOptionsAttr,
    ReorderWorkgroupsStrategy, ReorderWorkgroupsStrategyAttr,
};
pub(crate) use target::decode_target_info;
pub use target::{TargetInfo, TargetMmaIntrinsic};

#[cfg(test)]
mod tests;
