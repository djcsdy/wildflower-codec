use super::common::{Matrix, Rectangle, Rgb, Rgba};

pub struct DefineTextTag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgb>>,
}

pub struct DefineText2Tag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgba>>,
}

pub struct TextRecord<TColor> {
    pub font_id: u16,
    pub text_color: Option<TColor>,
    pub x_offset: i16,
    pub y_offset: i16,
    pub text_height: u16,
    pub glyphs: Vec<GlyphEntry>,
}

pub struct GlyphEntry {
    pub glyph_index: u32,
    pub glyph_advance: i32,
}
