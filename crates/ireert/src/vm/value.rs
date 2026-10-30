//! VM scalar values.
use crate::{Code, Error, Result};
use iree_native as n;

/// One immediate VM scalar value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    /// Signed 8-bit integer.
    I8(i8),
    /// Signed 16-bit integer.
    I16(i16),
    /// Signed 32-bit integer.
    I32(i32),
    /// Signed 64-bit integer.
    I64(i64),
    /// 32-bit floating-point value.
    F32(f32),
    /// 64-bit floating-point value.
    F64(f64),
}

impl Value {
    /// Returns this value's type.
    pub fn value_type(self) -> ValueType {
        match self {
            Self::I8(_) => ValueType::I8,
            Self::I16(_) => ValueType::I16,
            Self::I32(_) => ValueType::I32,
            Self::I64(_) => ValueType::I64,
            Self::F32(_) => ValueType::F32,
            Self::F64(_) => ValueType::F64,
        }
    }

    pub(crate) fn into_raw(self) -> n::iree_vm_value_t {
        let mut raw = n::iree_vm_value_t {
            type_: self.value_type().raw(),
            ..Default::default()
        };
        match self {
            Self::I8(v) => raw.__bindgen_anon_1.i8_ = v,
            Self::I16(v) => raw.__bindgen_anon_1.i16_ = v,
            Self::I32(v) => raw.__bindgen_anon_1.i32_ = v,
            Self::I64(v) => raw.__bindgen_anon_1.i64_ = v,
            Self::F32(v) => raw.__bindgen_anon_1.f32_ = v,
            Self::F64(v) => raw.__bindgen_anon_1.f64_ = v,
        }
        raw
    }

    pub(crate) fn from_raw(raw: n::iree_vm_value_t) -> Result<Self> {
        let value_type = ValueType::from_raw(raw.type_)
            .ok_or_else(|| Error::new(Code::InvalidArgument, "VM value has no scalar type"))?;
        // SAFETY: the tag selects the active union member, and every member
        // is a plain scalar valid for any bit pattern.
        Ok(unsafe {
            match value_type {
                ValueType::I8 => Self::I8(raw.__bindgen_anon_1.i8_),
                ValueType::I16 => Self::I16(raw.__bindgen_anon_1.i16_),
                ValueType::I32 => Self::I32(raw.__bindgen_anon_1.i32_),
                ValueType::I64 => Self::I64(raw.__bindgen_anon_1.i64_),
                ValueType::F32 => Self::F32(raw.__bindgen_anon_1.f32_),
                ValueType::F64 => Self::F64(raw.__bindgen_anon_1.f64_),
            }
        })
    }
}

/// Immediate scalar value types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueType {
    /// 8-bit signed integer.
    I8,
    /// 16-bit signed integer.
    I16,
    /// 32-bit signed integer.
    I32,
    /// 64-bit signed integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
}

impl ValueType {
    pub(crate) fn raw(self) -> n::iree_vm_value_type_t {
        match self {
            Self::I8 => n::IREE_VM_VALUE_TYPE_I8,
            Self::I16 => n::IREE_VM_VALUE_TYPE_I16,
            Self::I32 => n::IREE_VM_VALUE_TYPE_I32,
            Self::I64 => n::IREE_VM_VALUE_TYPE_I64,
            Self::F32 => n::IREE_VM_VALUE_TYPE_F32,
            Self::F64 => n::IREE_VM_VALUE_TYPE_F64,
        }
    }

    pub(crate) fn from_raw(value: n::iree_vm_value_type_t) -> Option<Self> {
        Some(match value {
            n::IREE_VM_VALUE_TYPE_I8 => Self::I8,
            n::IREE_VM_VALUE_TYPE_I16 => Self::I16,
            n::IREE_VM_VALUE_TYPE_I32 => Self::I32,
            n::IREE_VM_VALUE_TYPE_I64 => Self::I64,
            n::IREE_VM_VALUE_TYPE_F32 => Self::F32,
            n::IREE_VM_VALUE_TYPE_F64 => Self::F64,
            _ => return None,
        })
    }
}
