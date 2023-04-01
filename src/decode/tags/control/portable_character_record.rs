use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct PortableCharacterRecord {
    pub character_id: u16,
    pub name: String,
}

impl PortableCharacterRecord {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let name = String::read(reader)?;
        Ok(Self { character_id, name })
    }
}
