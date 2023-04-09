use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontNameTag {
    pub font_id: u16,
    pub font_name: String,
    pub font_copyright: String,
}

impl DefineFontNameTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let font_id = reader.read_u16()?;
        let font_name = String::read(reader)?;
        let font_copyright = String::read(reader)?;
        Ok(Self {
            font_id,
            font_name,
            font_copyright,
        })
    }
}
