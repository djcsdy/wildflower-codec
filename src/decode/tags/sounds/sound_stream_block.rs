use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundStreamBlockTag {
    pub stream_sound_data: Vec<u8>,
}

impl SoundStreamBlockTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(Self {
            stream_sound_data: reader.read_u8_to_end()?,
        })
    }
}
