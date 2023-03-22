use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod align;
pub mod csm_text_settings;
pub mod define_edit_text;
pub mod define_edit_text_flags;
pub mod define_edit_text_layout;
pub mod define_text;
pub mod define_text_2;
pub mod font_ref;
pub mod glyph_entry;
pub mod text_record;
pub mod text_record_font;
pub mod text_renderer;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum GridFit {
    None = 0,
    PixelFit = 1,
    SubPixelFit = 2,
}
