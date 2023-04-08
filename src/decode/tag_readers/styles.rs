use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::styles::focal_gradient::FocalGradient;
use crate::decode::tags::styles::gradient::Gradient;
use std::io::Result;

pub fn read_focal_gradient<Read: BitRead, Color, ReadColor: Fn(&mut Read) -> Result<Color>>(
    reader: &mut Read,
    read_color: &ReadColor,
) -> Result<FocalGradient<Color>> {
    let gradient = Gradient::read(reader, read_color)?;
    let focal_point = Fixed8::read(reader)?;
    Ok(FocalGradient {
        spread_mode: gradient.spread_mode,
        interpolation_mode: gradient.interpolation_mode,
        gradient_records: gradient.gradient_records,
        focal_point,
    })
}
