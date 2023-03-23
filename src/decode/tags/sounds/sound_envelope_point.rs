use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundEnvelopePoint {
    pub pos_samples_44: u32,
    pub left_level: u16,
    pub right_level: u16,
}

impl SoundEnvelopePoint {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let pos_samples_44 = reader.read_u32()?;
        let left_level = reader.read_u16()?;
        let right_level = reader.read_u16()?;
        Ok(Self {
            pos_samples_44,
            left_level,
            right_level,
        })
    }
}
