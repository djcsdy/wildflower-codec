use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::common::read_rectangle;
use crate::decode::tags::common::{Rectangle, String};
use crate::decode::tags::fonts::code_table_and_layout::CodeTableAndLayout;
use crate::decode::tags::fonts::define_font_2_flags::DefineFont2Flags;
use crate::decode::tags::fonts::glyph_shape_table::GlyphShapeTable;
use crate::decode::tags::fonts::glyphs_and_code_table_and_layout::GlyphsAndCodeTableAndLayout;
use crate::decode::tags::fonts::kerning::KerningRecord;
use crate::decode::tags::fonts::layout::FontLayout;
use crate::decode::tags::fonts::LanguageCode;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};
use std::mem::size_of;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont2Tag {
    pub font_id: u16,
    pub flags: DefineFont2Flags,
    pub language_code: Option<LanguageCode>,
    pub font_name: String,
    pub num_glyphs: u16,
    pub glyphs_and_code_table_and_layout: Vec<u8>,
}

impl DefineFont2Tag {
    pub fn read_glyphs_and_code_table_and_layout(
        &self,
        swf_version: u8,
    ) -> Result<GlyphsAndCodeTableAndLayout> {
        let mut reader = SwfSliceReader::new(&self.glyphs_and_code_table_and_layout, swf_version);
        let mut offset_table = Vec::with_capacity(self.num_glyphs as usize + 1);
        for _ in 0..self.num_glyphs + 1 {
            offset_table.push(if self.flags.contains(DefineFont2Flags::WIDE_OFFSETS) {
                reader.read_u16()? as usize
            } else {
                reader.read_u8()? as usize
            });
        }
        let shape_table_offset = *offset_table.first().unwrap_or(&0usize);
        let code_table_offset = *offset_table.last().unwrap_or(&0usize);
        let shape_table =
            &self.glyphs_and_code_table_and_layout[shape_table_offset..code_table_offset];
        let partial_layout = if self.flags.contains(DefineFont2Flags::HAS_LAYOUT) {
            let char_code_size_bytes = if self.flags.contains(DefineFont2Flags::WIDE_CODES) {
                size_of::<u16>()
            } else {
                size_of::<u8>()
            };
            let code_table_size_bytes = self.num_glyphs as usize * char_code_size_bytes;
            reader.seek(code_table_offset + code_table_size_bytes);
            Some(PartialFontLayout::read(
                &mut reader,
                self.num_glyphs as usize,
            )?)
        } else {
            None
        };
        let code_table_and_layout = if self.flags.contains(DefineFont2Flags::WIDE_CODES) {
            let layout = partial_layout.map_or(<Result<_>>::Ok(None), |partial| {
                Ok(Some(partial.read_kerning_table(
                    &mut reader,
                    &|reader| reader.read_u16(),
                    self.num_glyphs as usize,
                )?))
            })?;
            reader.seek(code_table_offset);
            let mut character_codes = Vec::with_capacity(self.num_glyphs as usize);
            for _ in 0..self.num_glyphs {
                character_codes.push(reader.read_u16()?);
            }
            if self.flags.contains(DefineFont2Flags::SHIFT_JIS) {
                CodeTableAndLayout::ShiftJis {
                    character_codes,
                    layout,
                }
            } else if self.flags.contains(DefineFont2Flags::ANSI) {
                return Err(Error::from(InvalidData));
            } else {
                CodeTableAndLayout::Ucs2 {
                    character_codes,
                    layout,
                }
            }
        } else {
            let layout = partial_layout.map_or(<Result<_>>::Ok(None), |partial| {
                Ok(Some(partial.read_kerning_table(
                    &mut reader,
                    &|reader| reader.read_u8(),
                    self.num_glyphs as usize,
                )?))
            })?;
            reader.seek(code_table_offset);
            let mut character_codes = Vec::with_capacity(self.num_glyphs as usize);
            for _ in 0..self.num_glyphs {
                character_codes.push(reader.read_u8()?);
            }
            if self.flags.contains(DefineFont2Flags::SHIFT_JIS) {
                CodeTableAndLayout::JisX0201 {
                    character_codes,
                    layout,
                }
            } else if self.flags.contains(DefineFont2Flags::ANSI) {
                CodeTableAndLayout::Windows1252 {
                    character_codes,
                    layout,
                }
            } else {
                return Err(Error::from(InvalidData));
            }
        };
        Ok(GlyphsAndCodeTableAndLayout {
            shape_table: GlyphShapeTable {
                swf_version,
                offset_table,
                shape_table,
            },
            code_table_and_layout,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
struct PartialFontLayout {
    pub ascent: u16,
    pub descent: u16,
    pub leading: i16,
    pub advance_table: Vec<i16>,
    pub bounds_table: Vec<Rectangle>,
}

impl PartialFontLayout {
    fn read(reader: &mut SwfSliceReader, num_glyphs: usize) -> Result<PartialFontLayout> {
        let ascent = reader.read_u16()?;
        let descent = reader.read_u16()?;
        let leading = reader.read_i16()?;
        let mut advance_table = Vec::with_capacity(num_glyphs);
        for _ in 0..num_glyphs {
            advance_table.push(reader.read_i16()?);
        }
        let mut bounds_table = Vec::with_capacity(num_glyphs);
        for _ in 0..num_glyphs {
            bounds_table.push(read_rectangle(reader)?);
        }
        Ok(PartialFontLayout {
            ascent,
            descent,
            leading,
            advance_table,
            bounds_table,
        })
    }

    fn read_kerning_table<
        CharacterCode,
        ReadCharacterCode: Fn(&mut SwfSliceReader) -> Result<CharacterCode>,
    >(
        self,
        reader: &mut SwfSliceReader,
        read_character_code: &ReadCharacterCode,
        num_glyphs: usize,
    ) -> Result<FontLayout<CharacterCode>> {
        let mut kerning_table = Vec::with_capacity(num_glyphs as usize);
        for _ in 0..num_glyphs {
            kerning_table.push(KerningRecord::read(reader, read_character_code)?);
        }
        Ok(FontLayout {
            ascent: self.ascent,
            descent: self.descent,
            leading: self.leading,
            advance_table: self.advance_table,
            bounds_table: self.bounds_table,
            kerning_table,
        })
    }
}
