use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::portable_character_record::PortableCharacterRecord;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssets2Tag {
    pub url: String,
    pub imports: Vec<PortableCharacterRecord>,
}

impl ImportAssets2Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let url = String::read(reader)?;
        reader.read_u16()?;
        let count = reader.read_u16()?;
        let mut imports = Vec::with_capacity(count as usize);
        for _ in 0..count {
            imports.push(PortableCharacterRecord::read(reader)?);
        }
        Ok(Self { url, imports })
    }
}
