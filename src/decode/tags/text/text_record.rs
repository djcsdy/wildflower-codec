use crate::decode::tags::text::GlyphEntry;

#[derive(Clone, PartialEq, Debug)]
pub struct TextRecord<TColor> {
    pub font_id: u16,
    pub text_color: Option<TColor>,
    pub x_offset: i16,
    pub y_offset: i16,
    pub text_height: u16,
    pub glyphs: Vec<GlyphEntry>,
}
