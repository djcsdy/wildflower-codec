use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::styles::gradient::Gradient;
use crate::decode::tags::styles::gradient_record::GradientRecord;
use crate::decode::tags::styles::interpolation_mode::InterpolationMode;
use crate::decode::tags::styles::spread_mode::SpreadMode;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct FocalGradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
    pub focal_point: Fixed8,
}

impl<Color> FocalGradient<Color> {
    pub fn read<Read: BitRead, ReadColor: Fn(&mut Read) -> Result<Color>>(
        reader: &mut Read,
        read_color: &ReadColor,
    ) -> Result<Self> {
        let gradient = Gradient::read(reader, read_color)?;
        let focal_point = Fixed8::read(reader)?;
        Ok(Self {
            spread_mode: gradient.spread_mode,
            interpolation_mode: gradient.interpolation_mode,
            gradient_records: gradient.gradient_records,
            focal_point,
        })
    }
}
