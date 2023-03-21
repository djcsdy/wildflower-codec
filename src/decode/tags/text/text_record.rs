use crate::decode::slice_reader::SwfSliceReader;
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

pub struct ReadTextRecordOptions<
    'reader,
    'buffer,
    Color,
    ReadColor: Fn(&'reader SwfSliceReader<'buffer>) -> Color,
> {
    pub reader: &'reader mut SwfSliceReader<'buffer>,
    pub glyph_bits: u8,
    pub advance_bits: u8,
    pub read_color: ReadColor,
}
