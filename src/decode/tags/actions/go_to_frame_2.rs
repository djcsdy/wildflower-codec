use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GoToFrame2 {
    pub play: bool,
    pub scene_bias: u16,
}

impl GoToFrame2 {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        reader.read_ub8(6)?;
        let scene_bias_flag = reader.read_bit()?;
        let play = reader.read_bit()?;
        let scene_bias = if scene_bias_flag {
            reader.read_u16()?
        } else {
            0
        };
        Ok(Self { play, scene_bias })
    }
}
