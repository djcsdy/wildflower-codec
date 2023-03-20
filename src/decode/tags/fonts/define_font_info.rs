use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::String;
use crate::decode::tags::fonts::code_table::CodeTable;
use crate::decode::tags::fonts::define_font_info_flags::DefineFontInfoFlags;
use std::io::{Error, ErrorKind::InvalidData, Result};

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
        let code_table = if flags.contains(DefineFontInfoFlags::WIDE_CODES) {
            if flags.contains(DefineFontInfoFlags::SHIFT_JIS) {
                CodeTable::ShiftJis(reader.read_u16_to_end()?)
            } else if flags.contains(DefineFontInfoFlags::ANSI) {
                return Err(Error::from(InvalidData));
            } else {
                CodeTable::Ucs2(reader.read_u16_to_end()?)
            }
        } else {
            if flags.contains(DefineFontInfoFlags::SHIFT_JIS) {
                CodeTable::JisX0201(reader.read_u8_to_end()?)
            } else if flags.contains(DefineFontInfoFlags::ANSI) {
                CodeTable::Windows1252(reader.read_u8_to_end()?)
            } else {
                return Err(Error::from(InvalidData));
            }
        };
        Ok(Self {
            font_id,
            font_name,
            flags,
            code_table,
        })
    }
}
