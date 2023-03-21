use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::text::glyph_entry::{GlyphEntry, ReadGlyphEntryOptions};
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
    ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Color,
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

impl<Color> TextRecord<Color> {
    pub fn read<'reader, 'buffer, ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Color>(
        options: ReadTextRecordOptions<'reader, 'buffer, Color, ReadColor>,
    ) -> Result<Option<Self>> {
        let flags = Flags::read(options.reader)?;
        Ok(if flags.is_empty() {
            None
        } else {
            let font_id = if flags.contains(Flags::HAS_FONT) {
                Some(options.reader.read_u16()?)
            } else {
                None
            };
            let text_color = if flags.contains(Flags::HAS_COLOR) {
                Some((options.read_color)(options.reader))
            } else {
                None
            };
            let x_offset = if flags.contains(Flags::HAS_X_OFFSET) {
                Some(options.reader.read_i16()?)
            } else {
                None
            };
            let y_offset = if flags.contains(Flags::HAS_Y_OFFSET) {
                Some(options.reader.read_i16()?)
            } else {
                None
            };
            let font = if let Some(font_id) = font_id {
                Some(TextRecordFont {
                    font_id,
                    text_height: options.reader.read_u16()?,
                })
            } else {
                None
            };
            let glyph_count = options.reader.read_u8()?;
            let mut glyph_entries = Vec::with_capacity(glyph_count as usize);
            for _ in 0..glyph_count {
                glyph_entries.push(GlyphEntry::read(ReadGlyphEntryOptions {
                    reader: options.reader,
                    glyph_bits: options.glyph_bits,
                    advance_bits: options.advance_bits,
                })?);
            }
            Some(TextRecord {
                font,
                text_color,
                x_offset,
                y_offset,
                glyph_entries,
            })
        })
    }
}
