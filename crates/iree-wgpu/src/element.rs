//! Host element types accepted by typed transfers.

use ireert::hal::ElementType;

/// A host element type with a matching IREE element type.
///
/// Implemented for the fixed-width integers (`i8` through `u64`) and for
/// `f32` and `f64`. Typed transfers such as [`Context::upload`] and
/// [`Context::read`] use it to describe host data to IREE.
///
/// This trait is sealed: its element-type tags must match the Rust types'
/// layouts, so it cannot be implemented outside this crate.
///
/// [`Context::upload`]: crate::Context::upload
/// [`Context::read`]: crate::Context::read
pub trait Element: bytemuck::Pod + sealed::Sealed {
    /// The IREE element type with the layout of `Self`.
    const TYPE: ElementType;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! elements {
    ($($rust:ty => $hal:ident),* $(,)?) => {
        $(
            impl sealed::Sealed for $rust {}
            impl Element for $rust {
                const TYPE: ElementType = ElementType::$hal;
            }
        )*
    };
}

elements! {
    i8 => SINT_8, u8 => UINT_8, i16 => SINT_16, u16 => UINT_16,
    i32 => SINT_32, u32 => UINT_32, i64 => SINT_64, u64 => UINT_64,
    f32 => FLOAT_32, f64 => FLOAT_64,
}
