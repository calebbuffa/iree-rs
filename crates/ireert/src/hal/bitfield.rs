//! Shared constructors for HAL bitfields and scalar value wrappers.

macro_rules! scalar_value {
    (
        $(#[$meta:meta])*
        $name:ident: $raw:ty {
            $($(#[$constant_meta:meta])* $constant:ident = $value:path,)*
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
        pub struct $name($raw);
        impl $name {
            $(
                $(#[$constant_meta])*
                pub const $constant: Self = Self($value as $raw);
            )*
            /// Creates a value while preserving unrecognized native values.
            pub const fn from_bits_retain(bits: $raw) -> Self {
                Self(bits)
            }
            /// Returns the native value.
            pub const fn bits(self) -> $raw {
                self.0
            }
        }
    };
}

macro_rules! bitfield {
    (
        $(#[$meta:meta])*
        $name:ident: $raw:ty {
            $($(#[$constant_meta:meta])* $constant:ident = $value:path,)*
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
        pub struct $name($raw);
        impl $name {
            $(
                $(#[$constant_meta])*
                pub const $constant: Self = Self($value as $raw);
            )*
            /// Creates a value from raw C bits, keeping bits unknown to this binding version.
            pub const fn from_bits_retain(bits: $raw) -> Self { Self(bits) }
            /// Returns the raw C bits.
            pub const fn bits(self) -> $raw { self.0 }
            /// Returns whether every bit of `other` is set.
            pub const fn contains(self, other: Self) -> bool { self.0 & other.0 == other.0 }
            /// Returns whether no bit is set.
            pub const fn is_empty(self) -> bool { self.0 == 0 }
            /// Returns whether `self` and `other` share any bit.
            pub const fn intersects(self, other: Self) -> bool { self.0 & other.0 != 0 }
        }
        impl core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
        }
        impl core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
        }
        impl core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
        }
    };
}

pub(crate) use bitfield;
pub(crate) use scalar_value;
