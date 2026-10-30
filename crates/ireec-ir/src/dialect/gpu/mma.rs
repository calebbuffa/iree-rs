//! MMA intrinsic enums, layouts, and operand distribution attributes.
use crate::{Attribute, Context, Error, Result, Type};
use iree_native as n;

macro_rules! gpu_enum {
    ($(#[$meta:meta])* $name:ident { $($(#[$variant_meta:meta])* $variant:ident = $value:expr,)+ }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u32)]
        #[doc = concat!("IREE GPU ", stringify!($name), " values.")]
        pub enum $name { $($(#[$variant_meta])* $variant = $value,)+ }
        impl $name {
            /// Every known native value, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];
            /// Decode a native discriminant, rejecting unknown values.
            /// Numeric value used by the pinned C API.
            pub const fn value(self) -> u32 { self as u32 }
            /// Decode a native discriminant, rejecting unknown values.
            pub fn from_value(value: u32) -> Result<Self> {
                match value { $($value => Ok(Self::$variant),)+ _ => Err(Error::Invalid(format!("unknown {} value {value}", stringify!($name)))) }
            }
        }
    };
}

gpu_enum! {
    MmaIntrinsic {
        /// Pinned C enum case.
        MfmaF32x16x16x4F32 = 0x1010,
        /// Pinned C enum case.
        MfmaF32x16x16x16F16 = 0x1020,
        /// Pinned C enum case.
        MfmaF32x32x32x8F16 = 0x1021,
        /// Pinned C enum case.
        MfmaI32x16x16x16I8 = 0x10C0,
        /// Pinned C enum case.
        MfmaI32x32x32x8I8 = 0x10C1,
        /// Pinned C enum case.
        MfmaF32x16x16x8BF16 = 0x1120,
        /// Pinned C enum case.
        MfmaF32x32x32x4BF16 = 0x1121,
        /// Pinned C enum case.
        MfmaF64x16x16x4F64 = 0x1100,
        /// Pinned C enum case.
        MfmaF32x16x16x16BF16 = 0x1220,
        /// Pinned C enum case.
        MfmaF32x32x32x8BF16 = 0x1221,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E5M2FNUZ = 0x1230,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E5M2FNUZF8E4M3FNUZ = 0x1231,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E4M3FNUZ = 0x1232,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E4M3FNUZF8E5M2FNUZ = 0x1233,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E5M2FNUZ = 0x1234,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E5M2FNUZF8E4M3FNUZ = 0x1235,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E4M3FNUZ = 0x1236,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E4M3FNUZF8E5M2FNUZ = 0x1237,
        /// Pinned C enum case.
        MfmaI32x16x16x32I8 = 0x12C0,
        /// Pinned C enum case.
        MfmaI32x32x32x16I8 = 0x12C1,
        /// Pinned C enum case.
        MfmaF32x4x4x4x16BF16 = 0x1222,
        /// Pinned C enum case.
        MfmaF32x16x16x4x4BF16 = 0x1223,
        /// Pinned C enum case.
        MfmaF32x32x32x4x2BF16 = 0x1224,
        /// Pinned C enum case.
        MfmaF32x4x4x4x16BBF16 = 0x1225,
        /// Pinned C enum case.
        MfmaF32x16x16x4x4BBF16 = 0x1226,
        /// Pinned C enum case.
        MfmaF32x32x32x4x2BBF16 = 0x1227,
        /// Pinned C enum case.
        MfmaF32x4x4x1x16BF32 = 0x1210,
        /// Pinned C enum case.
        MfmaF32x16x16x1x4BF32 = 0x1211,
        /// Pinned C enum case.
        MfmaF32x32x32x1x2BF32 = 0x1212,
        /// Pinned C enum case.
        MfmaF64x4x4x4x4BF64 = 0x1200,
        /// Pinned C enum case.
        MfmaI32x4x4x4x16BI8 = 0x12C2,
        /// Pinned C enum case.
        MfmaI32x16x16x4x4BI8 = 0x12C3,
        /// Pinned C enum case.
        MfmaI32x32x32x4x2BI8 = 0x12C4,
        /// Pinned C enum case.
        MfmaF32x16x16x32F16 = 0x1320,
        /// Pinned C enum case.
        MfmaF32x32x32x16F16 = 0x1321,
        /// Pinned C enum case.
        MfmaF32x16x16x32BF16 = 0x1322,
        /// Pinned C enum case.
        MfmaF32x32x32x16BF16 = 0x1323,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E5M2 = 0x1330,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E5M2F8E4M3FN = 0x1331,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E4M3FN = 0x1332,
        /// Pinned C enum case.
        MfmaF32x16x16x32F8E4M3FNF8E5M2 = 0x1333,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E5M2 = 0x1334,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E5M2F8E4M3FN = 0x1335,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E4M3FN = 0x1336,
        /// Pinned C enum case.
        MfmaF32x32x32x16F8E4M3FNF8E5M2 = 0x1337,
        /// Pinned C enum case.
        MfmaF32x16x16x128F8E5M2 = 0x1338,
        /// Pinned C enum case.
        MfmaF32x16x16x128F8E5M2F8E4M3FN = 0x1339,
        /// Pinned C enum case.
        MfmaF32x16x16x128F8E4M3FN = 0x133A,
        /// Pinned C enum case.
        MfmaF32x16x16x128F8E4M3FNF8E5M2 = 0x133B,
        /// Pinned C enum case.
        MfmaF32x32x32x64F8E5M2 = 0x133C,
        /// Pinned C enum case.
        MfmaF32x32x32x64F8E5M2F8E4M3FN = 0x133D,
        /// Pinned C enum case.
        MfmaF32x32x32x64F8E4M3FN = 0x133E,
        /// Pinned C enum case.
        MfmaF32x32x32x64F8E4M3FNF8E5M2 = 0x133F,
        /// Pinned C enum case.
        MfmaI32x16x16x64I8 = 0x13C0,
        /// Pinned C enum case.
        MfmaI32x32x32x32I8 = 0x13C1,
        /// Pinned C enum case.
        WMMAR3F32x16x16x16F16 = 0x1820,
        /// Pinned C enum case.
        WMMAR3F16x16x16x16F16 = 0x1821,
        /// Pinned C enum case.
        WMMAR3F32x16x16x16BF16 = 0x1822,
        /// Pinned C enum case.
        WMMAR3BF16x16x16x16BF16 = 0x1823,
        /// Pinned C enum case.
        WMMAR3I32x16x16x16I8 = 0x18C0,
        /// Pinned C enum case.
        WMMAR4F32x16x16x16F16 = 0x1920,
        /// Pinned C enum case.
        WMMAR4F16x16x16x16F16 = 0x1921,
        /// Pinned C enum case.
        WMMAR4F32x16x16x16BF16 = 0x1922,
        /// Pinned C enum case.
        WMMAR4BF16x16x16x16BF16 = 0x1923,
        /// Pinned C enum case.
        WMMAR4F32x16x16x16F8E5M2 = 0x1930,
        /// Pinned C enum case.
        WMMAR4F32x16x16x16F8E5M2F8E4M3FN = 0x1931,
        /// Pinned C enum case.
        WMMAR4F32x16x16x16F8E4M3FN = 0x1932,
        /// Pinned C enum case.
        WMMAR4F32x16x16x16F8E4M3FNF8E5M2 = 0x1933,
        /// Pinned C enum case.
        WMMAR4I32x16x16x16I8 = 0x19C0,
        /// Pinned C enum case.
        WmmaF32x16x16x4F32 = 0x1A10,
        /// Pinned C enum case.
        WmmaF32x16x16x32F16 = 0x1A20,
        /// Pinned C enum case.
        WmmaF32x16x16x32BF16 = 0x1A21,
        /// Pinned C enum case.
        WmmaF16x16x16x32F16 = 0x1A22,
        /// Pinned C enum case.
        WmmaBF16x16x16x32BF16 = 0x1A23,
        /// Pinned C enum case.
        WmmaF32x16x16x64F8E4M3FN = 0x1A30,
        /// Pinned C enum case.
        WmmaF32x16x16x64F8E4M3FNF8E5M2 = 0x1A31,
        /// Pinned C enum case.
        WmmaF32x16x16x64F8E5M2 = 0x1A32,
        /// Pinned C enum case.
        WmmaF32x16x16x64F8E5M2F8E4M3FN = 0x1A33,
        /// Pinned C enum case.
        WmmaF16x16x16x64F8E4M3FN = 0x1A34,
        /// Pinned C enum case.
        WmmaF16x16x16x64F8E4M3FNF8E5M2 = 0x1A35,
        /// Pinned C enum case.
        WmmaF16x16x16x64F8E5M2 = 0x1A36,
        /// Pinned C enum case.
        WmmaF16x16x16x64F8E5M2F8E4M3FN = 0x1A37,
        /// Pinned C enum case.
        WmmaI32x16x16x64I8 = 0x1AC0,
        /// Pinned C enum case.
        WmmaF32x16x16x128F8E5M2 = 0x1A38,
        /// Pinned C enum case.
        WmmaF32x16x16x128F8E5M2F8E4M3FN = 0x1A39,
        /// Pinned C enum case.
        WmmaF32x16x16x128F8E4M3FN = 0x1A3A,
        /// Pinned C enum case.
        WmmaF32x16x16x128F8E4M3FNF8E5M2 = 0x1A3B,
        /// Pinned C enum case.
        WmmaF16x16x16x128F8E5M2 = 0x1A3C,
        /// Pinned C enum case.
        WmmaF16x16x16x128F8E5M2F8E4M3FN = 0x1A3D,
        /// Pinned C enum case.
        WmmaF16x16x16x128F8E4M3FN = 0x1A3E,
        /// Pinned C enum case.
        WmmaF16x16x16x128F8E4M3FNF8E5M2 = 0x1A3F,
        /// Pinned C enum case.
        NvMmaSyncF32x16x8x16F16 = 0x2020,
        /// Pinned C enum case.
        NvMmaSyncF16x16x8x16F16 = 0x2021,
        /// Pinned C enum case.
        NvMmaSyncF32x16x8x16BF16 = 0x2022,
        /// Pinned C enum case.
        NvMmaSyncF32x16x8x32F8E4M3FN = 0x2023,
        /// Pinned C enum case.
        NvMmaSyncF32x16x8x32F8E5M2 = 0x2024,
        /// Pinned C enum case.
        NvWmmaF32x16x16x16F16 = 0x2120,
        /// Pinned C enum case.
        NvWmmaF16x16x16x16F16 = 0x2121,
    }
}

gpu_enum! {
    VirtualMmaIntrinsic {
        /// Pinned C enum case.
        VmfmaF32x16x16x32F16 = 0,
        /// Pinned C enum case.
        VmfmaF32x32x32x16F16 = 1,
        /// Pinned C enum case.
        VmfmaF32x16x16x32F8E4M3FNUZ = 2,
        /// Pinned C enum case.
        VmfmaF32x32x32x16F8E4M3FNUZ = 3,
        /// Pinned C enum case.
        VdmfmaF32x8x16x64x2F16 = 4,
        /// Pinned C enum case.
        VdmfmaF32x8x16x64x2BF16 = 6,
        /// Pinned C enum case.
        VdmfmaI32x8x16x128x2I8 = 5,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x2F8E5M2FNUZ = 7,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x2F8E5M2FNUZF8E4M3FNUZ = 8,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x2F8E4M3FNUZF8E5M2FNUZ = 9,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x2F8E4M3FNUZ = 10,
        /// Pinned C enum case.
        VdmfmaF32x8x16x64x1F16 = 15,
        /// Pinned C enum case.
        VdmfmaF32x8x16x64x1BF16 = 16,
        /// Pinned C enum case.
        VdmfmaI32x8x16x128x1I8 = 17,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x1F8E5M2 = 11,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x1F8E5M2F8E4M3FN = 12,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x1F8E4M3FNF8E5M2 = 13,
        /// Pinned C enum case.
        VdmfmaF32x8x16x128x1F8E4M3FN = 14,
    }
}

/// The selected operand whose MMA subgroup layout is queried.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MmaOperand {
    /// Left-hand matrix operand.
    Lhs = 0,
    /// Right-hand matrix operand.
    Rhs = 1,
    /// Accumulator operand.
    Acc = 2,
}

/// An IREE GPU MMA intrinsic attribute.
pub struct MmaIntrinsicAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// An IREE GPU virtual MMA intrinsic attribute.
pub struct VirtualMmaIntrinsicAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// An IREE GPU MMA layout attribute.
pub struct MmaAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// An IREE GPU virtual MMA layout attribute.
pub struct VirtualMmaAttr<'ctx> {
    attribute: Attribute<'ctx>,
}
/// MMA element types and tile extents.
pub struct MmaInfo<'ctx> {
    /// Element type of the left operand.
    pub a_element_type: Type<'ctx>,
    /// Element type of the right operand.
    pub b_element_type: Type<'ctx>,
    /// Accumulator element type.
    pub c_element_type: Type<'ctx>,
    /// Per-thread left-operand vector type.
    pub a_vector_type: Type<'ctx>,
    /// Per-thread right-operand vector type.
    pub b_vector_type: Type<'ctx>,
    /// Per-thread accumulator vector type.
    pub c_vector_type: Type<'ctx>,
    /// M tile size.
    pub m: i64,
    /// N tile size.
    pub n: i64,
    /// K tile size.
    pub k: i64,
}

macro_rules! typed_attribute {
    ($type:ident, $predicate:ident, $label:literal) => {
        impl<'ctx> $type<'ctx> {
            /// Validate and wrap an attribute of this dialect-specific kind.
            pub fn from_attribute(attribute: Attribute<'ctx>) -> Result<Self> {
                // SAFETY: `attribute` retains its checked handle and context while the native predicate checks its kind.
                if !unsafe { n::$predicate(attribute.raw) } {
                    return Err(Error::Invalid(concat!("attribute is not ", $label).into()));
                }
                Ok(Self { attribute })
            }
            /// Borrow the generic attribute.
            pub fn attribute(&self) -> &Attribute<'ctx> {
                &self.attribute
            }
        }
    };
}

typed_attribute!(
    MmaIntrinsicAttr,
    ireeAttributeIsAGPUMMAIntrinsicAttr,
    "an IREE GPU MMA intrinsic"
);
typed_attribute!(
    VirtualMmaIntrinsicAttr,
    ireeAttributeIsAGPUVirtualMMAIntrinsicAttr,
    "an IREE GPU virtual MMA intrinsic"
);
typed_attribute!(
    MmaAttr,
    ireeAttributeIsAGPUMMAAttr,
    "an IREE GPU MMA layout"
);
typed_attribute!(
    VirtualMmaAttr,
    ireeAttributeIsAGPUVirtualMMAAttr,
    "an IREE GPU virtual MMA layout"
);

impl MmaIntrinsicAttr<'_> {
    /// Decode the hardware intrinsic enum.
    pub fn value(&self) -> Result<MmaIntrinsic> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUMMAIntrinsicAttrGetValue`.
        MmaIntrinsic::from_value(unsafe { n::ireeGPUMMAIntrinsicAttrGetValue(self.attribute.raw) })
    }
    /// Query the per-operand single-subgroup distribution layout.
    pub fn subgroup_layout(&self, operand: MmaOperand) -> Result<MmaSingleSubgroupLayout> {
        read_subgroup_layout(self.attribute.raw, operand)
    }
}
impl VirtualMmaIntrinsicAttr<'_> {
    /// Decode the virtual intrinsic enum.
    pub fn value(&self) -> Result<VirtualMmaIntrinsic> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUVirtualMMAIntrinsicAttrGetValue`.
        VirtualMmaIntrinsic::from_value(unsafe {
            n::ireeGPUVirtualMMAIntrinsicAttrGetValue(self.attribute.raw)
        })
    }
    /// Query the per-operand single-subgroup distribution layout.
    pub fn subgroup_layout(&self, operand: MmaOperand) -> Result<MmaSingleSubgroupLayout> {
        read_subgroup_layout(self.attribute.raw, operand)
    }
}
impl<'ctx> MmaAttr<'ctx> {
    /// Whether the layout is column-major.
    pub fn col_major(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUMMAAttrGetColMajor`.
        unsafe { n::ireeGPUMMAAttrGetColMajor(self.attribute.raw) }
    }
    /// Read native element/vector types and matrix dimensions.
    pub fn info(&self) -> Result<MmaInfo<'ctx>> {
        mma_info(&self.attribute)
    }
    /// Decode the virtual intrinsics associated with this layout.
    pub fn virtual_intrinsics(&self) -> Result<Vec<VirtualMmaIntrinsic>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUMMAAttrGetVirtualMMAIntrinsic`.
        let raw = unsafe { n::ireeGPUMMAAttrGetVirtualMMAIntrinsic(self.attribute.raw) };
        integer_array(raw, "virtual MMA intrinsic list")?
            .into_iter()
            .map(|value| {
                u32::try_from(value)
                    .ok()
                    .and_then(|value| VirtualMmaIntrinsic::from_value(value).ok())
                    .ok_or_else(|| Error::Invalid(format!("unknown virtual MMA intrinsic {value}")))
            })
            .collect()
    }
    /// Return XOR-shuffle bounds for an operand, when supported.
    pub fn xor_shuffle_bounds(&self, operand: MmaOperand) -> Option<XorShuffleBounds> {
        xor_shuffle_bounds(self.attribute.raw, operand)
    }
}
impl<'ctx> VirtualMmaAttr<'ctx> {
    /// Whether the layout is column-major.
    pub fn col_major(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUVirtualMMAAttrGetColMajor`.
        unsafe { n::ireeGPUVirtualMMAAttrGetColMajor(self.attribute.raw) }
    }
    /// Read native element/vector types and matrix dimensions.
    pub fn info(&self) -> Result<MmaInfo<'ctx>> {
        mma_info(&self.attribute)
    }
    /// Return XOR-shuffle bounds for an operand, when supported.
    pub fn xor_shuffle_bounds(&self, operand: MmaOperand) -> Option<XorShuffleBounds> {
        xor_shuffle_bounds(self.attribute.raw, operand)
    }
}

fn mma_info<'ctx>(attribute: &Attribute<'ctx>) -> Result<MmaInfo<'ctx>> {
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeGPUMMAAttrGetInfo`.
    let raw = unsafe { n::ireeGPUMMAAttrGetInfo(attribute.raw) };
    let types = [
        raw.aElementType,
        raw.bElementType,
        raw.cElementType,
        raw.aVectorType,
        raw.bVectorType,
        raw.cVectorType,
    ];
    if types.iter().any(|ty| ty.ptr.is_null()) {
        return Err(Error::Null("MMA element or vector type"));
    }
    let [
        a_element_type,
        b_element_type,
        c_element_type,
        a_vector_type,
        b_vector_type,
        c_vector_type,
    ] = types.map(Type::borrowed);
    Ok(MmaInfo {
        a_element_type,
        b_element_type,
        c_element_type,
        a_vector_type,
        b_vector_type,
        c_vector_type,
        m: raw.mElements,
        n: raw.nElements,
        k: raw.kElements,
    })
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Construct a hardware MMA intrinsic attribute.
    pub fn mma_intrinsic<'a>(&'a self, value: MmaIntrinsic) -> Result<MmaIntrinsicAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUMMAIntrinsicAttrGet`.
        let raw = unsafe { n::ireeGPUMMAIntrinsicAttrGet(self.raw, value as u32) };
        Ok(MmaIntrinsicAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
    /// Construct a virtual MMA intrinsic attribute.
    pub fn virtual_mma_intrinsic<'a>(
        &'a self,
        value: VirtualMmaIntrinsic,
    ) -> Result<VirtualMmaIntrinsicAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUVirtualMMAIntrinsicAttrGet`.
        let raw = unsafe { n::ireeGPUVirtualMMAIntrinsicAttrGet(self.raw, value as u32) };
        Ok(VirtualMmaIntrinsicAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
    /// Construct a hardware MMA layout.
    pub fn mma<'a>(&'a self, value: MmaIntrinsic, col_major: bool) -> Result<MmaAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUMMAAttrGet`.
        let raw = unsafe { n::ireeGPUMMAAttrGet(self.raw, value as u32, col_major) };
        Ok(MmaAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
    /// Construct a virtual MMA layout.
    pub fn virtual_mma<'a>(
        &'a self,
        value: VirtualMmaIntrinsic,
        col_major: bool,
    ) -> Result<VirtualMmaAttr<'a>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `ireeGPUVirtualMMAAttrGet`.
        let raw = unsafe { n::ireeGPUVirtualMMAAttrGet(self.raw, value as u32, col_major) };
        Ok(VirtualMmaAttr {
            // SAFETY: `self` retains this valid native handle and its context throughout `this call`.
            attribute: unsafe { Attribute::from_raw(self, raw)? },
        })
    }
    /// Test IREE's XOR-shuffle legality predicate.
    pub fn xor_shuffle_valid(
        &self,
        row_elements: i64,
        access_elements: i64,
        total_tile_elements: i64,
    ) -> bool {
        // SAFETY: this native predicate accepts only the three scalar tile extents passed by value.
        unsafe { n::ireeGPUIsXORShuffleValid(row_elements, access_elements, total_tile_elements) }
    }
}

/// Per-thread distribution details for one MMA operand.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MmaSingleSubgroupLayout {
    /// Repetition counts outside the distributed tile.
    pub outer: Vec<i64>,
    /// Number of threads along each dimension.
    pub thread: Vec<i64>,
    /// Thread strides for each dimension.
    pub thread_strides: Vec<i64>,
    /// Contiguous elements per thread.
    pub element: Vec<i64>,
}

pub(super) fn integer_array(attribute: n::MlirAttribute, label: &str) -> Result<Vec<i64>> {
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsAArray`.
    if attribute.ptr.is_null() || !unsafe { n::mlirAttributeIsAArray(attribute) } {
        return Err(Error::Invalid(format!("{label} is not an array attribute")));
    }
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirArrayAttrGetNumElements`.
    let count = unsafe { n::mlirArrayAttrGetNumElements(attribute) };
    if count < 0 {
        return Err(Error::Invalid(format!("{label} has negative length")));
    }
    (0..count)
        .map(|index| {
            // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `mlirArrayAttrGetElement`.
            let value = unsafe { n::mlirArrayAttrGetElement(attribute, index) };
            // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirAttributeIsAInteger`.
            if value.ptr.is_null() || !unsafe { n::mlirAttributeIsAInteger(value) } {
                return Err(Error::Invalid(format!("{label} contains a non-integer")));
            }
            // SAFETY: `value` is a live native handle; their wrappers keep them valid throughout `mlirIntegerAttrGetValueInt`.
            Ok(unsafe { n::mlirIntegerAttrGetValueInt(value) })
        })
        .collect()
}

fn read_subgroup_layout(
    attribute: n::MlirAttribute,
    operand: MmaOperand,
) -> Result<MmaSingleSubgroupLayout> {
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeGPUGetSingleSubgroupLayout`.
    let raw = unsafe { n::ireeGPUGetSingleSubgroupLayout(attribute, operand as u32) };
    Ok(MmaSingleSubgroupLayout {
        outer: integer_array(raw.outer, "MMA outer layout")?,
        thread: integer_array(raw.thread, "MMA thread layout")?,
        thread_strides: integer_array(raw.tstrides, "MMA thread strides")?,
        element: integer_array(raw.element, "MMA element layout")?,
    })
}

fn xor_shuffle_bounds(
    attribute: n::MlirAttribute,
    operand: MmaOperand,
) -> Option<XorShuffleBounds> {
    let mut min_access_elements = 0;
    let mut total_tile_elements = 0;
    // SAFETY: `attribute` is a live native handle; their wrappers keep them valid throughout `ireeGPUGetXorShuffleBounds`.
    if unsafe {
        n::ireeGPUGetXorShuffleBounds(
            attribute,
            operand as i32,
            &mut min_access_elements,
            &mut total_tile_elements,
        )
    } {
        Some(XorShuffleBounds {
            min_access_elements,
            total_tile_elements,
        })
    } else {
        None
    }
}

/// Compute XOR shuffle bounds for a native MMA intrinsic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XorShuffleBounds {
    /// Minimum elements each thread must access.
    pub min_access_elements: i64,
    /// Total distributed tile elements.
    pub total_tile_elements: i64,
}
