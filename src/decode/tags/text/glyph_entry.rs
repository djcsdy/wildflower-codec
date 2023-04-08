use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

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

impl GlyphEntry {
    pub fn read(
        ReadGlyphEntryOptions {
            reader,
            glyph_bits,
            advance_bits,
        }: ReadGlyphEntryOptions,
    ) -> Result<Self> {
        let glyph_index = reader.read_ub(glyph_bits)?;
        let glyph_advance = reader.read_sb(advance_bits)?;
        Ok(Self {
            glyph_index,
            glyph_advance,
        })
    }
}
