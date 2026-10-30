//! Colors and source locations attached to command-buffer labels.

use iree_native as n;

/// A color for a command-buffer debug label.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LabelColor {
    /// Red component.
    pub red: u8,
    /// Green component.
    pub green: u8,
    /// Blue component.
    pub blue: u8,
    /// Alpha component.
    pub alpha: u8,
}
impl LabelColor {
    pub(crate) fn raw(self) -> n::iree_hal_label_color_t {
        n::iree_hal_label_color_t {
            __bindgen_anon_1: n::iree_hal_label_color_t__bindgen_ty_1 {
                __bindgen_anon_1: n::iree_hal_label_color_t__bindgen_ty_1__bindgen_ty_1 {
                    r: self.red,
                    g: self.green,
                    b: self.blue,
                    a: self.alpha,
                },
            },
        }
    }
}

/// Source location attached to a command-buffer label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LabelLocation<'a> {
    /// Source file path.
    pub file: &'a str,
    /// One-based source line.
    pub line: i32,
}
impl LabelLocation<'_> {
    pub(crate) fn raw(self) -> n::iree_hal_label_location_t {
        n::iree_hal_label_location_t {
            file: crate::view(self.file),
            line: self.line,
        }
    }
}
