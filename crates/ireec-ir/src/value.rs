//! MLIR SSA values and their associated types.
use crate::{Type, callback::print_with};
use iree_native as n;
use std::{fmt, marker::PhantomData};

/// A non-owning MLIR SSA value tied to its context and owner lifetime.
pub struct Value<'owner> {
    pub(crate) raw: n::MlirValue,
    pub(crate) context: n::MlirContext,
    _owner: PhantomData<&'owner ()>,
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirValuePrint`.
        f.write_str(&print_with(|callback, data| unsafe {
            n::mlirValuePrint(self.raw, callback, data)
        }))
    }
}

impl<'owner> Value<'owner> {
    pub(crate) fn borrowed(raw: n::MlirValue, context: n::MlirContext) -> Value<'owner> {
        Value {
            raw,
            context,
            _owner: PhantomData,
        }
    }

    /// The borrowed native value handle.
    pub fn as_raw(&self) -> n::MlirValue {
        self.raw
    }

    /// Borrow the value's MLIR type.
    pub fn ty(&self) -> Type<'_> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirValueGetType`.
        let raw = unsafe { n::mlirValueGetType(self.raw) };
        Type::borrowed(raw)
    }
}
