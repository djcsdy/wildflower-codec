use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::fixed_8::Fixed8;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg4Tag {
    pub character_id: u16,
    pub deblock_param: Fixed8,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}

impl DefineBitsJpeg4Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let alpha_data_offset = reader.read_u32()? as usize;
        let deblock_param = Fixed8::read(reader)?;
        let mut image_data = vec![0u8; alpha_data_offset];
        reader.read_u8_into(&mut image_data)?;
        let bitmap_alpha_data = reader.read_u8_to_end()?;
        Ok(Self {
            character_id,
            deblock_param,
            image_data,
            bitmap_alpha_data,
        })
    }
}
