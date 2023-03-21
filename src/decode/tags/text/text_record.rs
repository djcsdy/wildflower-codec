use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::text::glyph_entry::GlyphEntry;
use crate::decode::tags::text::text_record_font::TextRecordFont;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct TextRecord<TColor> {
    pub font: Option<TextRecordFont>,
    pub text_color: Option<TColor>,
    pub x_offset: Option<i16>,
    pub y_offset: Option<i16>,
    pub glyph_entries: Vec<GlyphEntry>,
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

bitflags! {
    struct Flags: u8 {
        const RESERVED = 0xf0;
        const HAS_FONT = 0x08;
        const HAS_COLOR = 0x04;
        const HAS_Y_OFFSET = 0x02;
        const HAS_X_OFFSET = 0x01;
    }
}

impl Flags {
    fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(Self::from_bits(reader.read_u8()?).unwrap())
    }
}
