use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use crate::decode::tags::fonts::define_font_4_flags::DefineFont4Flags;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont4Tag {
    pub font_id: u16,
    pub flags: DefineFont4Flags,
    pub font_name: String,
    pub font_data: Option<Vec<u8>>,
}

impl DefineFont4Tag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let font_id = reader.read_u16()?;
        let flags = DefineFont4Flags::read(reader)?;
        let font_name = String::read(reader)?;
        let font_data = if flags.contains(DefineFont4Flags::HAS_FONT_DATA) {
            Some(reader.read_u8_to_end()?)
        } else {
            None
        };
        Ok(Self {
            font_id,
            flags,
            font_name,
            font_data,
        })
    }
}
