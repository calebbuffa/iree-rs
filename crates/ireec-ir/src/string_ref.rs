//! MLIR string-reference construction and checked string copying.
use iree_native as n;
use std::ffi::c_char;

pub(crate) fn string_ref(text: &str) -> n::MlirStringRef {
    n::MlirStringRef {
        data: text.as_ptr().cast::<c_char>(),
        length: text.len(),
    }
}

pub(crate) fn copy_string_ref(value: n::MlirStringRef) -> crate::Result<String> {
    if value.length > isize::MAX as usize {
        return Err(crate::Error::Invalid(
            "native string exceeds the maximum Rust slice size".into(),
        ));
    }
    if value.length > 0 && value.data.is_null() {
        return Err(crate::Error::Null("string reference"));
    }
    let bytes = if value.length == 0 {
        &[][..]
    } else {
        // SAFETY: the callback rejects null pointers and oversized lengths before forming this borrowed slice.
        unsafe { n::slice(value.data.cast::<u8>(), value.length) }
    };
    String::from_utf8(bytes.to_vec())
        .map_err(|_| crate::Error::Invalid("native string is not UTF-8".into()))
}

#[cfg(test)]
mod tests {
    use crate::string_ref::string_ref;
    use proptest::prelude::*;

    proptest! {
                    #[test]
                    fn string_ref_preserves_utf8_bytes(text in ".*") {
                        let reference = string_ref(&text);
                        prop_assert_eq!(reference.length, text.len());
    // SAFETY: the callback rejects null pointers and oversized lengths before forming this borrowed slice.
                        let bytes = unsafe {
                            iree_native::slice(
                                reference.data.cast::<u8>(),
                                reference.length,
                            )
                        };
                        prop_assert_eq!(bytes, text.as_bytes());
                    }
                }
}
