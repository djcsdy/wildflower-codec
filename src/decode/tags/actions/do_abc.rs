use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DoAbcTag {
    pub flags: u32,
    pub name: String,
    pub abc_data: Vec<u8>,
}

impl DoAbcTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u32()?;
        let name = String::read(reader)?;
        let abc_data = reader.read_u8_to_end()?;
        Ok(Self {
            flags,
            name,
            abc_data,
        })
    }
}
