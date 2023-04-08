use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::text::glyph_entry::{GlyphEntry, ReadGlyphEntryOptions};
use crate::decode::tags::text::text_record_font::TextRecordFont;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct TextRecord<Color> {
    pub font: Option<TextRecordFont>,
    pub text_color: Option<Color>,
    pub x_offset: Option<i16>,
    pub y_offset: Option<i16>,
    pub glyph_entries: Vec<GlyphEntry>,
}

pub struct ReadTextRecordOptions<
    'reader,
    'buffer,
    Color,
    ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>,
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
    pub fn read<'reader, 'buffer, ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>>(
        ReadTextRecordOptions {
            reader,
            glyph_bits,
            advance_bits,
            read_color,
        }: ReadTextRecordOptions<'reader, 'buffer, Color, ReadColor>,
    ) -> Result<Option<Self>> {
        let flags = Flags::read(reader)?;
        Ok(if flags.is_empty() {
            None
        } else {
            let font_id = if flags.contains(Flags::HAS_FONT) {
                Some(reader.read_u16()?)
            } else {
                None
            };
            let text_color = if flags.contains(Flags::HAS_COLOR) {
                Some(read_color(reader)?)
            } else {
                None
            };
            let x_offset = if flags.contains(Flags::HAS_X_OFFSET) {
                Some(reader.read_i16()?)
            } else {
                None
            };
            let y_offset = if flags.contains(Flags::HAS_Y_OFFSET) {
                Some(reader.read_i16()?)
            } else {
                None
            };
            let font = if let Some(font_id) = font_id {
                Some(TextRecordFont {
                    font_id,
                    text_height: reader.read_u16()?,
                })
            } else {
                None
            };
            let glyph_count = reader.read_u8()?;
            let mut glyph_entries = Vec::with_capacity(glyph_count as usize);
            for _ in 0..glyph_count {
                glyph_entries.push(GlyphEntry::read(ReadGlyphEntryOptions {
                    reader,
                    glyph_bits,
                    advance_bits,
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

    pub fn read_all<
        'reader,
        'buffer,
        ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>,
    >(
        ReadTextRecordOptions {
            reader,
            glyph_bits,
            advance_bits,
            read_color,
        }: ReadTextRecordOptions<'reader, 'buffer, Color, ReadColor>,
    ) -> Result<Vec<Self>> {
        let mut records = Vec::new();
        while let Some(record) = Self::read(ReadTextRecordOptions {
            reader,
            glyph_bits,
            advance_bits,
            read_color: &read_color,
        })? {
            records.push(record)
        }
        Ok(records)
    }
}
