use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::fixed_16::Fixed16;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct BevelFilter {
    pub shadow_color: Rgba,
    pub highlight_color: Rgba,
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

impl BevelFilter {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let shadow_color = Rgba::read(reader)?;
        let highlight_color = Rgba::read(reader)?;
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
            shadow_color,
            highlight_color,
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
