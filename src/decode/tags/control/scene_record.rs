use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct SceneRecord {
    pub offset: u32,
    pub name: String,
}

impl SceneRecord {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let offset = reader.read_encoded_u32()?;
        let name = String::read(reader)?;
        Ok(Self { offset, name })
    }
}
