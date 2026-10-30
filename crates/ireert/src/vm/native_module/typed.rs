//! Typed argument and result conversion for native exports.

use super::{NativeCall, Ref};
use crate::{Code, Error, Result};

/// Arguments accepted by [`super::NativeModule::export_typed`].
///
/// Implementations must describe the same VM calling convention as the value
/// they decode. Built-in implementations cover scalars, references, and tuples
/// of up to six values.
#[doc(hidden)]
pub trait NativeArguments: Sized {
    /// Implementations must make this fragment exactly describe the slots
    /// read by `decode`.
    /// Returns the VM ABI fragment for these arguments.
    fn abi_fragment() -> String;

    /// Decodes arguments from a live native call.
    fn decode(call: &NativeCall<'_>) -> Result<Self>;
}

/// Results returned by [`super::NativeModule::export_typed`].
///
/// Implementations must describe the same VM calling convention as the value
/// they encode. Built-in implementations cover scalars, references, and tuples
/// of up to six values.
#[doc(hidden)]
pub trait NativeResults {
    /// Implementations must make this fragment exactly describe the slots
    /// written by `encode`.
    /// Returns the VM ABI fragment for these results.
    fn abi_fragment() -> String;

    /// Encodes results into the native call's result storage.
    fn encode(self, call: &mut NativeCall<'_>) -> Result<()>;
}

/// One value that can cross a typed native-module export boundary.
///
/// Implement this for Rust newtypes when their conversions match one supported
/// VM ABI slot. This enables them in typed export arguments and results while
/// keeping byte-offset handling out of the callback.
///
/// Booleans use the `i` slot with the canonical values zero and one; other
/// incoming values are rejected. Unsigned integers use the same-width bit
/// representation as their signed VM slot.
pub trait NativeValue: Sized {
    /// Implementations must make this slot type agree with `read` and `write`.
    /// VM calling-convention slot: `i`, `I`, `f`, `F`, or `r`.
    const ABI: char;

    /// Reads one value from an argument slot.
    fn read(call: &NativeCall<'_>, index: usize) -> Result<Self>;

    /// Writes one value to a result slot.
    fn write(self, call: &mut NativeCall<'_>, index: usize) -> Result<()>;
}

fn argument_offset(call: &NativeCall<'_>, index: usize) -> Result<usize> {
    call.argument_offset(index)
        .ok_or_else(|| Error::new(Code::OutOfRange, "native argument index is out of range"))
}

fn result_offset(call: &NativeCall<'_>, index: usize) -> Result<usize> {
    call.result_offset(index)
        .ok_or_else(|| Error::new(Code::OutOfRange, "native result index is out of range"))
}

macro_rules! scalar_value {
    ($ty:ty, $abi:literal) => {
        impl NativeValue for $ty {
            const ABI: char = $abi;

            fn read(call: &NativeCall<'_>, index: usize) -> Result<Self> {
                call.argument(argument_offset(call, index)?)
            }

            fn write(self, call: &mut NativeCall<'_>, index: usize) -> Result<()> {
                call.set_result(result_offset(call, index)?, self)
            }
        }
    };
}

scalar_value!(i32, 'i');
scalar_value!(u32, 'i');
scalar_value!(i64, 'I');
scalar_value!(u64, 'I');
scalar_value!(f32, 'f');
scalar_value!(f64, 'F');

impl NativeValue for bool {
    const ABI: char = 'i';

    fn read(call: &NativeCall<'_>, index: usize) -> Result<Self> {
        match call.argument::<i32>(argument_offset(call, index)?)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new(
                Code::InvalidArgument,
                "native boolean argument must be zero or one",
            )),
        }
    }

    fn write(self, call: &mut NativeCall<'_>, index: usize) -> Result<()> {
        call.set_result(
            result_offset(call, index)?,
            if self { 1_i32 } else { 0_i32 },
        )
    }
}

impl NativeValue for Ref {
    const ABI: char = 'r';

    fn read(call: &NativeCall<'_>, index: usize) -> Result<Self> {
        call.argument_ref(argument_offset(call, index)?)?
            .ok_or_else(|| Error::new(Code::InvalidArgument, "null native reference argument"))
    }

    fn write(self, call: &mut NativeCall<'_>, index: usize) -> Result<()> {
        call.set_result_ref(result_offset(call, index)?, Some(self))
    }
}

impl NativeValue for Option<Ref> {
    const ABI: char = 'r';

    fn read(call: &NativeCall<'_>, index: usize) -> Result<Self> {
        call.argument_ref(argument_offset(call, index)?)
    }

    fn write(self, call: &mut NativeCall<'_>, index: usize) -> Result<()> {
        call.set_result_ref(result_offset(call, index)?, self)
    }
}

impl NativeArguments for () {
    fn abi_fragment() -> String {
        "v".to_owned()
    }

    fn decode(_call: &NativeCall<'_>) -> Result<Self> {
        Ok(())
    }
}

impl<T: NativeValue> NativeArguments for T {
    fn abi_fragment() -> String {
        T::ABI.to_string()
    }

    fn decode(call: &NativeCall<'_>) -> Result<Self> {
        T::read(call, 0)
    }
}

impl NativeResults for () {
    fn abi_fragment() -> String {
        "v".to_owned()
    }

    fn encode(self, _call: &mut NativeCall<'_>) -> Result<()> {
        Ok(())
    }
}

impl<T: NativeValue> NativeResults for T {
    fn abi_fragment() -> String {
        T::ABI.to_string()
    }

    fn encode(self, call: &mut NativeCall<'_>) -> Result<()> {
        self.write(call, 0)
    }
}

macro_rules! tuple_values {
    ($($type:ident : $index:tt),+ $(,)?) => {
        impl<$($type: NativeValue),+> NativeArguments for ($($type,)+) {
            fn abi_fragment() -> String {
                [$(<$type as NativeValue>::ABI),+].iter().copied().collect()
            }

            fn decode(call: &NativeCall<'_>) -> Result<Self> {
                Ok(($($type::read(call, $index)?,)+))
            }
        }

        impl<$($type: NativeValue),+> NativeResults for ($($type,)+) {
            fn abi_fragment() -> String {
                [$(<$type as NativeValue>::ABI),+].iter().copied().collect()
            }

            fn encode(self, call: &mut NativeCall<'_>) -> Result<()> {
                $(self.$index.write(call, $index)?;)+
                Ok(())
            }
        }
    };
}

tuple_values!(A: 0, B: 1);
tuple_values!(A: 0, B: 1, C: 2);
tuple_values!(A: 0, B: 1, C: 2, D: 3);
tuple_values!(A: 0, B: 1, C: 2, D: 3, E: 4);
tuple_values!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
