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
    pub fn read(options: ReadGlyphEntryOptions) -> Result<Self> {
        let glyph_index = options.reader.read_ub(options.glyph_bits)?;
        let glyph_advance = options.reader.read_sb(options.advance_bits)?;
        Ok(Self {
            glyph_index,
            glyph_advance,
        })
    }
}
