use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct SoundStreamBlockTag {
    pub stream_sound_data: Vec<u8>,
}

impl SoundStreamBlockTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            stream_sound_data: reader.read_u8_to_end()?,
        })
    }
}
