use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontNameTag {
    pub font_id: u16,
    pub font_name: String,
    pub font_copyright: String,
}

impl DefineFontNameTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let font_id = reader.read_u16()?;
        let font_name = reader.read_string()?;
        let font_copyright = reader.read_string()?;
        Ok(Self {
            font_id,
            font_name,
            font_copyright,
        })
    }
}
