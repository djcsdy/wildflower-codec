use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::styles::focal_gradient::FocalGradient;
use crate::decode::tags::styles::gradient::Gradient;
use crate::decode::tags::styles::gradient_record::GradientRecord;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_gradient<Read: BitRead, Color, ReadColor: Fn(&mut Read) -> Result<Color>>(
    reader: &mut Read,
    read_color: &ReadColor,
) -> Result<Gradient<Color>> {
    let spread_mode = reader
        .read_ub8(2)?
        .try_into()
        .map_err(|_| Error::from(InvalidData))?;
    let interpolation_mode = reader
        .read_ub8(2)?
        .try_into()
        .map_err(|_| Error::from(InvalidData))?;
    let num_gradients = reader.read_ub8(4)?;
    let mut gradient_records = Vec::with_capacity(num_gradients as usize);
    for _ in 0..num_gradients {
        gradient_records.push(read_gradient_record(reader, &read_color)?);
    }
    Ok(Gradient {
        spread_mode,
        interpolation_mode,
        gradient_records,
    })
}

pub fn read_focal_gradient<Read: BitRead, Color, ReadColor: Fn(&mut Read) -> Result<Color>>(
    reader: &mut Read,
    read_color: &ReadColor,
) -> Result<FocalGradient<Color>> {
    let gradient = read_gradient(reader, read_color)?;
    let focal_point = Fixed8::read(reader)?;
    Ok(FocalGradient {
        spread_mode: gradient.spread_mode,
        interpolation_mode: gradient.interpolation_mode,
        gradient_records: gradient.gradient_records,
        focal_point,
    })
}

fn read_gradient_record<R: Read, Color, ReadColor: Fn(&mut R) -> Result<Color>>(
    reader: &mut R,
    read_color: &ReadColor,
) -> Result<GradientRecord<Color>> {
    let ratio = reader.read_u8()?;
    let color = read_color(reader)?;
    Ok(GradientRecord { ratio, color })
}
