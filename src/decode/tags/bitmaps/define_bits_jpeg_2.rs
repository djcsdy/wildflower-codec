use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg2Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
}

impl DefineBitsJpeg2Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let image_data = reader.read_u8_to_end()?;
        Ok(Self {
            character_id,
            image_data,
        })
    }
}
