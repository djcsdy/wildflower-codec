use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::fixed_16::Fixed16;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GradientBevelFilter {
    pub colors: Vec<Rgba>,
    pub ratio: Vec<u8>,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub inner_shadow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub on_top: bool,
    pub passes: u8,
}

impl GradientBevelFilter {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let num_colors = reader.read_u8()?;
        let mut colors = Vec::with_capacity(num_colors as usize);
        for _ in 0..num_colors {
            colors.push(Rgba::read(reader)?);
        }
        let mut ratio = Vec::with_capacity(num_colors as usize);
        for _ in 0..num_colors {
            ratio.push(reader.read_u8()?);
        }
        let blur_x = Fixed16::read(reader)?;
        let blur_y = Fixed16::read(reader)?;
        let angle = Fixed16::read(reader)?;
        let distance = Fixed16::read(reader)?;
        let strength = Fixed8::read(reader)?;
        let inner_shadow = reader.read_bit()?;
        let knockout = reader.read_bit()?;
        let composite_source = reader.read_bit()?;
        let on_top = reader.read_bit()?;
        let passes = reader.read_ub8(4)?;
        Ok(Self {
            colors,
            ratio,
            blur_x,
            blur_y,
            angle,
            distance,
            strength,
            inner_shadow,
            knockout,
            composite_source,
            on_top,
            passes,
        })
    }
}
