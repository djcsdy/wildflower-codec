use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::fixed_16::Fixed16;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct BlurFilter {
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub passes: u8,
}

impl BlurFilter {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let blur_x = Fixed16::read(reader)?;
        let blur_y = Fixed16::read(reader)?;
        let passes = reader.read_ub8(5)?;
        reader.read_ub8(3)?;
        Ok(Self {
            blur_x,
            blur_y,
            passes,
        })
    }
}
