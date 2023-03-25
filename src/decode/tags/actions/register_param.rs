use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub enum RegisterParam {
    Register(u8),
    Name(String),
}

impl RegisterParam {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let register = reader.read_u8()?;
        let name = String::read(reader)?;
        Ok(if register == 0 {
            Self::Name(name)
        } else {
            Self::Register(register)
        })
    }
}
