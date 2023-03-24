use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::fixed_16::Fixed16;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GlowFilter {
    pub color: Rgba,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub strength: Fixed8,
    pub inner_glow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub passes: u8,
}

impl GlowFilter {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let color = Rgba::read(reader)?;
        let blur_x = Fixed16::read(reader)?;
        let blur_y = Fixed16::read(reader)?;
        let strength = Fixed8::read(reader)?;
        let inner_glow = reader.read_bit()?;
        let knockout = reader.read_bit()?;
        let composite_source = reader.read_bit()?;
        let passes = reader.read_ub8(5)?;
        Ok(Self {
            color,
            blur_x,
            blur_y,
            strength,
            inner_glow,
            knockout,
            composite_source,
            passes,
        })
    }
}
