use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::fonts::define_font_info_flags::DefineFontInfoFlags;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};
use crate::decode::sized_read::SizedRead;

#[derive(Clone, PartialEq, Debug)]
pub enum CodeTable {
    Windows1252(Vec<u8>),
    JisX0201(Vec<u8>),
    ShiftJis(Vec<u16>),
    Ucs2(Vec<u16>),
}

impl CodeTable {
    pub fn read(reader: &mut SwfSliceReader, flags: DefineFontInfoFlags) -> Result<Self> {
        if flags.contains(DefineFontInfoFlags::WIDE_CODES) {
            if flags.contains(DefineFontInfoFlags::SHIFT_JIS) {
                Ok(CodeTable::ShiftJis(reader.read_u16_to_end()?))
            } else if flags.contains(DefineFontInfoFlags::ANSI) {
                Err(Error::from(InvalidData))
            } else {
                Ok(CodeTable::Ucs2(reader.read_u16_to_end()?))
            }
        } else {
            if flags.contains(DefineFontInfoFlags::SHIFT_JIS) {
                Ok(CodeTable::JisX0201(reader.read_u8_to_end()?))
            } else if flags.contains(DefineFontInfoFlags::ANSI) {
                Ok(CodeTable::Windows1252(reader.read_u8_to_end()?))
            } else {
                Err(Error::from(InvalidData))
            }
        }
    }
}
