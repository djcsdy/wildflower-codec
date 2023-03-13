use super::common::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineTextTag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgb>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineText2Tag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgba>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextRecord<TColor> {
    pub font_id: u16,
    pub text_color: Option<TColor>,
    pub x_offset: i16,
    pub y_offset: i16,
    pub text_height: u16,
    pub glyphs: Vec<GlyphEntry>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphEntry {
    pub glyph_index: u32,
    pub glyph_advance: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextTag {
    pub character_id: u16,
    pub bounds: Rectangle,
    pub word_wrap: bool,
    pub multiline: bool,
    pub password: bool,
    pub read_only: bool,
    pub auto_size: bool,
    pub no_select: bool,
    pub border: bool,
    pub was_static: bool,
    pub html: bool,
    pub use_outlines: bool,
    pub font_id: u16,
    pub font_class: String,
    pub font_height: u16,
    pub text_color: Option<Rgba>,
    pub max_length: Option<u16>,
    pub align: Align,
    pub left_margin: u16,
    pub right_margin: u16,
    pub indent: u16,
    pub leading: i16,
    pub variable_name: String,
    pub initial_text: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Align {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
}

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
