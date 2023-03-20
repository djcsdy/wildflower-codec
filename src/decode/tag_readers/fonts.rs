use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::fonts::define_font_2::DefineFont2Tag;
use crate::decode::tags::fonts::define_font_2_flags::DefineFont2Flags;
use crate::decode::tags::fonts::define_font_info_2::DefineFontInfo2Tag;
use crate::decode::tags::fonts::language_code::LanguageCode;
use crate::decode::tags::fonts::CodeTable;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

pub fn read_define_font_info_2_tag(reader: &mut SwfSliceReader) -> Result<DefineFontInfo2Tag> {
    let font_id = reader.read_u16()?;
    let name_len = reader.read_u8()? as usize;
    let font_name = reader.read_fixed_string(name_len)?;
    reader.read_ub8(2)?;
    let small_text = reader.read_bit()?;
    let shift_jis = reader.read_bit()?;
    let ansi = reader.read_bit()?;
    let italic = reader.read_bit()?;
    let bold = reader.read_bit()?;
    let wide_codes = reader.read_bit()?;
    let language_code = read_language_code(reader)?;
    let code_table = if wide_codes {
        if ansi {
            return Err(Error::from(InvalidData));
        } else if shift_jis {
            CodeTable::ShiftJis(reader.read_u16_to_end()?)
        } else {
            CodeTable::Ucs2(reader.read_u16_to_end()?)
        }
    } else {
        if ansi {
            CodeTable::Windows1252(reader.read_u8_to_end()?)
        } else if shift_jis {
            CodeTable::JisX0201(reader.read_u8_to_end()?)
        } else {
            return Err(Error::from(InvalidData));
        }
    };
    Ok(DefineFontInfo2Tag {
        font_id,
        font_name,
        small_text,
        italic,
        bold,
        language_code,
        code_table,
    })
}

fn read_optional_language_code(reader: &mut SwfSliceReader) -> Result<Option<LanguageCode>> {
    let code = reader.read_u8()?;
    if code == 0 {
        Ok(None)
    } else {
        LanguageCode::try_from(code)
            .map(Some)
            .map_err(|_| Error::from(InvalidData))
    }
}

fn read_language_code(reader: &mut SwfSliceReader) -> Result<LanguageCode> {
    read_optional_language_code(reader)?.ok_or_else(|| Error::from(InvalidData))
}

pub fn read_define_font_2_tag(reader: &mut SwfSliceReader) -> Result<DefineFont2Tag> {
    let font_id = reader.read_u16()?;
    let flags = DefineFont2Flags::from_bits(reader.read_u8()?).unwrap();
    let language_code = read_optional_language_code(reader)?;
    let name_len = reader.read_u8()? as usize;
    let font_name = reader.read_fixed_string(name_len)?;
    let num_glyphs = reader.read_u16()?;
    let glyphs_and_layout = reader.read_u8_to_end()?;

    Ok(DefineFont2Tag {
        font_id,
        flags,
        language_code,
        font_name,
        num_glyphs,
        glyphs_and_code_table_and_layout: glyphs_and_layout,
    })
}
