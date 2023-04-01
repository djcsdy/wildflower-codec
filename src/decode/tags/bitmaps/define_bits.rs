use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsTag {
    pub character_id: u16,
    pub jpeg_data: Vec<u8>,
}

impl DefineBitsTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let jpeg_data = reader.read_u8_to_end()?;
        Ok(Self {
            character_id,
            jpeg_data,
        })
    }
}
