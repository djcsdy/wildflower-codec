use crate::decode::slice_reader::SwfSliceReader;

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphEntry {
    pub glyph_index: u32,
    pub glyph_advance: i32,
}

pub struct ReadGlyphEntryOptions<'reader, 'buffer> {
    pub reader: &'reader mut SwfSliceReader<'buffer>,
    pub glyph_bits: u8,
    pub advance_bits: u8,
}
