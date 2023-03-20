use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::String;
use crate::decode::tags::fonts::CodeTable;
use std::io::{Error, ErrorKind::InvalidData, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontInfoTag {
    pub font_id: u16,
    pub font_name: String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub code_table: CodeTable,
}

impl DefineFontInfoTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
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
        Ok(Self {
            font_id,
            font_name,
            small_text,
            italic,
            bold,
            code_table,
        })
    }
}