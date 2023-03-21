use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod define_edit_text;
pub mod define_text;
pub mod define_text_2;
pub mod glyph_entry;
pub mod text_record;
pub mod text_record_font;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Align {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CsmTextSettingsTag {
    pub text_id: u16,
    pub text_renderer: TextRenderer,
    pub grid_fit: GridFit,
    pub thickness: f32,
    pub sharpness: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TextRenderer {
    Standard = 0,
    Advanced = 1,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum GridFit {
    None = 0,
    PixelFit = 1,
    SubPixelFit = 2,
}
