use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg3Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}

impl DefineBitsJpeg3Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let alpha_data_offset = reader.read_u32()? as usize;
        let mut image_data = vec![0u8; alpha_data_offset];
        reader.read_u8_into(&mut image_data)?;
        let bitmap_alpha_data = reader.read_u8_to_end()?;
        Ok(Self {
            character_id,
            image_data,
            bitmap_alpha_data,
        })
    }
}
