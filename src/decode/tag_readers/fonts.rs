use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::fonts::define_font_2::DefineFont2Tag;
use crate::decode::tags::fonts::define_font_flags::DefineFontFlags;
use crate::decode::tags::fonts::language_code::LanguageCode;
use std::io::Result;

pub fn read_define_font_2_tag(reader: &mut SwfSliceReader) -> Result<DefineFont2Tag> {
    let font_id = reader.read_u16()?;
    let flags = DefineFontFlags::from_bits(reader.read_u8()?).unwrap();
    let language_code = LanguageCode::read_optional(reader)?;
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
