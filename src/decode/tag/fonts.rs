use crate::ast::fonts::{CodeTable, DefineFontInfoTag, DefineFontTag, LanguageCode};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::shapes::read_shape;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_define_font_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DefineFontTag> {
    let font_id = reader.read_u16()?;
    let end_offset = reader.remaining();
    let first_offset = reader.read_u16()? as usize;
    let num_glyphs = first_offset / 2;
    let mut length_table = Vec::with_capacity(num_glyphs);
    let mut prev_offset = first_offset;
    for _ in 1..num_glyphs {
        let offset = reader.read_u16()? as usize;
        length_table.push(offset - prev_offset);
        prev_offset = offset;
    }
    length_table.push(end_offset - prev_offset);
    let mut glyph_shapes = Vec::with_capacity(num_glyphs);
    for length in length_table {
        glyph_shapes.push(read_shape(&mut reader.with_max_length(length))?)
    }
    Ok(DefineFontTag {
        font_id,
        glyph_shapes,
    })
}

pub fn read_define_font_info_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<DefineFontInfoTag> {
    let font_id = reader.read_u16()?;
    let name_len = reader.read_u8()? as usize;
    let font_name = reader.read_fixed_string(name_len)?;
    let small_text = reader.read_bit()?;
    let shift_jis = reader.read_bit()?;
    let ansi = reader.read_bit()?;
    let italic = reader.read_bit()?;
    let bold = reader.read_bit()?;
    let wide_codes = reader.read_bit()?;
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
    Ok(DefineFontInfoTag {
        font_id,
        font_name,
        small_text,
        italic,
        bold,
        code_table,
    })
}

fn read_language_code<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<LanguageCode> {
    LanguageCode::try_from(reader.read_u8()?).map_err(|_| Error::from(InvalidData))
}
