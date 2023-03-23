use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use crate::decode::tags::fonts::code_table::CodeTable;
use crate::decode::tags::fonts::define_font_info_flags::DefineFontInfoFlags;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontInfoTag {
    pub font_id: u16,
    pub font_name: String,
    pub flags: DefineFontInfoFlags,
    pub code_table: CodeTable,
}

impl DefineFontInfoTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let font_id = reader.read_u16()?;
        let name_len = reader.read_u8()? as usize;
        let font_name = reader.read_fixed_string(name_len)?;
        let flags = DefineFontInfoFlags::read(reader)?;
        let code_table = CodeTable::read(reader, flags)?;
        Ok(Self {
            font_id,
            font_name,
            flags,
            code_table,
        })
    }
}
