//! Laws: widths and signedness round-trip through integer constructors.
use super::Type;
use crate::{
    Context, Error, Result,
    string_ref::{copy_string_ref, string_ref},
};
use iree_native as n;
typed!(
    IntegerType,
    mlirTypeIsAInteger,
    "An integer type with explicit signedness."
);
typed!(
    IndexType,
    mlirTypeIsAIndex,
    "The platform-sized index type."
);
typed!(
    FloatType,
    mlirTypeIsAFloat,
    "A built-in floating point type."
);
typed!(NoneType, mlirTypeIsANone, "The none type.");
typed!(OpaqueType, mlirTypeIsAOpaque, "An opaque dialect type.");
/// Integer signedness, independent of bit width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signedness {
    /// No signedness is attached to the type.
    Signless,
    /// Signed integer interpretation.
    Signed,
    /// Unsigned integer interpretation.
    Unsigned,
}
impl<'c> IntegerType<'c> {
    /// Construct a nonzero-width integer type.
    pub fn new<O>(context: &'c Context<'_, O>, width: u32, signedness: Signedness) -> Result<Self> {
        if width == 0 || width >= (1 << 23) {
            return Err(Error::Invalid(
                "integer width is outside MLIR's supported range".into(),
            ));
        }
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirIntegerTypeGet`.
        let raw = unsafe {
            match signedness {
                Signedness::Signless => n::mlirIntegerTypeGet(context.raw, width),
                Signedness::Signed => n::mlirIntegerTypeSignedGet(context.raw, width),
                Signedness::Unsigned => n::mlirIntegerTypeUnsignedGet(context.raw, width),
            }
        };
        Ok(Self(Type::checked(raw)?))
    }
    /// Number of bits in this integer.
    pub fn width(&self) -> u32 {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerTypeGetWidth`.
        unsafe { n::mlirIntegerTypeGetWidth(self.raw) }
    }
    /// Signedness of this integer.
    pub fn signedness(&self) -> Signedness {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerTypeIsSigned`.
        if unsafe { n::mlirIntegerTypeIsSigned(self.raw) } {
            Signedness::Signed
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerTypeIsUnsigned`.
        } else if unsafe { n::mlirIntegerTypeIsUnsigned(self.raw) } {
            Signedness::Unsigned
        } else {
            Signedness::Signless
        }
    }
}
impl<'c> IndexType<'c> {
    /// Construct this type in the supplied context.
    pub fn new<O>(context: &'c Context<'_, O>) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirIndexTypeGet`.
        Ok(Self(Type::checked(unsafe {
            n::mlirIndexTypeGet(context.raw)
        })?))
    }
}
impl<'c> NoneType<'c> {
    /// Construct this type in the supplied context.
    pub fn new<O>(context: &'c Context<'_, O>) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirNoneTypeGet`.
        Ok(Self(Type::checked(unsafe {
            n::mlirNoneTypeGet(context.raw)
        })?))
    }
}
/// Built-in floating point storage formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatFormat {
    /// The MLIR F16 floating point format.
    F16,
    /// The MLIR BF16 floating point format.
    BF16,
    /// The MLIR F32 floating point format.
    F32,
    /// The MLIR F64 floating point format.
    F64,
    /// The MLIR TF32 floating point format.
    TF32,
    /// The MLIR F4E2M1FN floating point format.
    F4E2M1FN,
    /// The MLIR F6E2M3FN floating point format.
    F6E2M3FN,
    /// The MLIR F6E3M2FN floating point format.
    F6E3M2FN,
    /// The MLIR F8E5M2 floating point format.
    F8E5M2,
    /// The MLIR F8E4M3 floating point format.
    F8E4M3,
    /// The MLIR F8E4M3FN floating point format.
    F8E4M3FN,
    /// The MLIR F8E5M2FNUZ floating point format.
    F8E5M2FNUZ,
    /// The MLIR F8E4M3FNUZ floating point format.
    F8E4M3FNUZ,
    /// The MLIR F8E4M3B11FNUZ floating point format.
    F8E4M3B11FNUZ,
    /// The MLIR F8E3M4 floating point format.
    F8E3M4,
    /// The MLIR F8E8M0FNU floating point format.
    F8E8M0FNU,
    /// The MLIR F8E5M3FNU floating point format.
    F8E5M3FNU,
}
impl<'c> FloatType<'c> {
    /// Construct a built-in floating point format.
    pub fn new<O>(context: &'c Context<'_, O>, format: FloatFormat) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirF16TypeGet`.
        let raw = unsafe {
            match format {
                FloatFormat::F16 => n::mlirF16TypeGet(context.raw),
                FloatFormat::BF16 => n::mlirBF16TypeGet(context.raw),
                FloatFormat::F32 => n::mlirF32TypeGet(context.raw),
                FloatFormat::F64 => n::mlirF64TypeGet(context.raw),
                FloatFormat::TF32 => n::mlirTF32TypeGet(context.raw),
                FloatFormat::F4E2M1FN => n::mlirFloat4E2M1FNTypeGet(context.raw),
                FloatFormat::F6E2M3FN => n::mlirFloat6E2M3FNTypeGet(context.raw),
                FloatFormat::F6E3M2FN => n::mlirFloat6E3M2FNTypeGet(context.raw),
                FloatFormat::F8E5M2 => n::mlirFloat8E5M2TypeGet(context.raw),
                FloatFormat::F8E4M3 => n::mlirFloat8E4M3TypeGet(context.raw),
                FloatFormat::F8E4M3FN => n::mlirFloat8E4M3FNTypeGet(context.raw),
                FloatFormat::F8E5M2FNUZ => n::mlirFloat8E5M2FNUZTypeGet(context.raw),
                FloatFormat::F8E4M3FNUZ => n::mlirFloat8E4M3FNUZTypeGet(context.raw),
                FloatFormat::F8E4M3B11FNUZ => n::mlirFloat8E4M3B11FNUZTypeGet(context.raw),
                FloatFormat::F8E3M4 => n::mlirFloat8E3M4TypeGet(context.raw),
                FloatFormat::F8E8M0FNU => n::mlirFloat8E8M0FNUTypeGet(context.raw),
                FloatFormat::F8E5M3FNU => n::mlirFloat8E5M3FNUTypeGet(context.raw),
            }
        };
        Ok(Self(Type::checked(raw)?))
    }
    /// Identify the built-in format, absent for formats unknown to this API version.
    pub fn format(&self) -> Option<FloatFormat> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAF16`.
        if unsafe { n::mlirTypeIsAF16(self.raw) } {
            return Some(FloatFormat::F16);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsABF16`.
        if unsafe { n::mlirTypeIsABF16(self.raw) } {
            return Some(FloatFormat::BF16);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAF32`.
        if unsafe { n::mlirTypeIsAF32(self.raw) } {
            return Some(FloatFormat::F32);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAF64`.
        if unsafe { n::mlirTypeIsAF64(self.raw) } {
            return Some(FloatFormat::F64);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsATF32`.
        if unsafe { n::mlirTypeIsATF32(self.raw) } {
            return Some(FloatFormat::TF32);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat4E2M1FN`.
        if unsafe { n::mlirTypeIsAFloat4E2M1FN(self.raw) } {
            return Some(FloatFormat::F4E2M1FN);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat6E2M3FN`.
        if unsafe { n::mlirTypeIsAFloat6E2M3FN(self.raw) } {
            return Some(FloatFormat::F6E2M3FN);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat6E3M2FN`.
        if unsafe { n::mlirTypeIsAFloat6E3M2FN(self.raw) } {
            return Some(FloatFormat::F6E3M2FN);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E5M2`.
        if unsafe { n::mlirTypeIsAFloat8E5M2(self.raw) } {
            return Some(FloatFormat::F8E5M2);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E4M3`.
        if unsafe { n::mlirTypeIsAFloat8E4M3(self.raw) } {
            return Some(FloatFormat::F8E4M3);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E4M3FN`.
        if unsafe { n::mlirTypeIsAFloat8E4M3FN(self.raw) } {
            return Some(FloatFormat::F8E4M3FN);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E5M2FNUZ`.
        if unsafe { n::mlirTypeIsAFloat8E5M2FNUZ(self.raw) } {
            return Some(FloatFormat::F8E5M2FNUZ);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E4M3FNUZ`.
        if unsafe { n::mlirTypeIsAFloat8E4M3FNUZ(self.raw) } {
            return Some(FloatFormat::F8E4M3FNUZ);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E4M3B11FNUZ`.
        if unsafe { n::mlirTypeIsAFloat8E4M3B11FNUZ(self.raw) } {
            return Some(FloatFormat::F8E4M3B11FNUZ);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E3M4`.
        if unsafe { n::mlirTypeIsAFloat8E3M4(self.raw) } {
            return Some(FloatFormat::F8E3M4);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E8M0FNU`.
        if unsafe { n::mlirTypeIsAFloat8E8M0FNU(self.raw) } {
            return Some(FloatFormat::F8E8M0FNU);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIsAFloat8E5M3FNU`.
        if unsafe { n::mlirTypeIsAFloat8E5M3FNU(self.raw) } {
            return Some(FloatFormat::F8E5M3FNU);
        }
        None
    }
    /// Number of storage bits.
    pub fn width(&self) -> u32 {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirFloatTypeGetWidth`.
        unsafe { n::mlirFloatTypeGetWidth(self.raw) }
    }
}
impl<'c> OpaqueType<'c> {
    /// Construct an opaque type in a loaded or explicitly permitted dialect.
    pub fn new<O>(context: &'c Context<'_, O>, dialect: &str, data: &str) -> Result<Self> {
        super::validate_dialect(context.raw, dialect)?;
        let raw =
// SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns; the context handle also remains live.
            unsafe { n::mlirOpaqueTypeGet(context.raw, string_ref(dialect), string_ref(data)) };
        Ok(Self(Type::checked(raw)?))
    }
    /// Dialect namespace.
    pub fn dialect_namespace(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOpaqueTypeGetDialectNamespace`.
        copy_string_ref(unsafe { n::mlirOpaqueTypeGetDialectNamespace(self.raw) })
    }
    /// Opaque type payload.
    pub fn data(&self) -> Result<String> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOpaqueTypeGetData`.
        copy_string_ref(unsafe { n::mlirOpaqueTypeGetData(self.raw) })
    }
}
