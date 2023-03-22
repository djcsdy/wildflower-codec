use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DoAbcTag {
    pub flags: u32,
    pub name: String,
    pub abc_data: Vec<u8>,
}

impl DoAbcTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let flags = reader.read_u32()?;
        let name = reader.read_string()?;
        let abc_data = reader.read_u8_to_end()?;
        Ok(Self {
            flags,
            name,
            abc_data,
        })
    }
}
