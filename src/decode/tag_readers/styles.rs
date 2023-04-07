use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::fill_style_type::FillStyleType;
use crate::decode::tags::styles::focal_gradient::FocalGradient;
use crate::decode::tags::styles::gradient::Gradient;
use crate::decode::tags::styles::gradient_record::GradientRecord;
use crate::decode::tags::styles::join_style::JoinStyle;
use crate::decode::tags::styles::line_style::LineStyle;
use crate::decode::tags::styles::line_style_2::LineStyle2;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_extended_fill_style_array<
    Read: BitRead,
    Color,
    ReadColor: Fn(&mut Read) -> Result<Color>,
>(
    reader: &mut Read,
    read_color: &ReadColor,
) -> Result<Vec<FillStyle<Color>>> {
    let mut fill_style_count = reader.read_u8()? as u16;
    if fill_style_count == 0xff {
        fill_style_count = reader.read_u16()?;
    }
    let mut fill_styles = Vec::with_capacity(fill_style_count as usize);
    for _ in 0..fill_style_count {
        fill_styles.push(FillStyle::read(reader, &read_color)?);
    }
    Ok(fill_styles)
}

pub(crate) fn read_fill_style_type<R: Read>(reader: &mut R) -> Result<FillStyleType> {
    reader
        .read_u8()?
        .try_into()
        .map_err(|_| Error::from(InvalidData))
}

pub fn read_line_style_array<
    Read: BitRead,
    LineStyle,
    ReadLineStyle: Fn(&mut Read) -> Result<LineStyle>,
>(
    reader: &mut Read,
    read_line_style: &ReadLineStyle,
) -> Result<Vec<LineStyle>> {
    let mut count = reader.read_u8()? as u16;
    if count == 0xff {
        count = reader.read_u16()?
    };
    let mut line_styles = Vec::with_capacity(count as usize);
    for _ in 0..count {
        line_styles.push(read_line_style(reader)?);
    }
    Ok(line_styles)
}

pub fn read_line_style<Read: BitRead, Color, ReadColor: Fn(&mut Read) -> Result<Color>>(
    reader: &mut Read,
    read_color: &ReadColor,
) -> Result<LineStyle<Color>> {
    let width = reader.read_u16()?;
    let color = read_color(reader)?;
    Ok(LineStyle { width, color })
}

pub fn read_line_style_2<R: BitRead>(reader: &mut R) -> Result<LineStyle2> {
    let width = reader.read_u16()?;
    let start_cap_style = read_cap_style(reader)?;
    let join_style = reader.read_ub8(2)?;
    let has_fill = reader.read_bit()?;
    let no_h_scale = reader.read_bit()?;
    let no_v_scale = reader.read_bit()?;
    let pixel_hinting = reader.read_bit()?;
    reader.read_ub8(5)?;
    let no_close = reader.read_bit()?;
    let end_cap_style = read_cap_style(reader)?;
    let miter_limit_factor = if join_style == 2 {
        Some(Fixed8::read(reader)?)
    } else {
        None
    };
    let fill_style = if has_fill {
        let read_color = &Rgba::read;
        FillStyle::read(reader, read_color)?
    } else {
        FillStyle::Solid(Rgba::read(reader)?)
    };
    Ok(LineStyle2 {
        width,
        start_cap_style,
        join_style: match join_style {
            0 => JoinStyle::Round,
            1 => JoinStyle::Bevel,
            2 => JoinStyle::Miter {
                miter_limit_factor: miter_limit_factor.unwrap(),
            },
            _ => return Err(Error::from(InvalidData)),
        },
        no_h_scale,
        no_v_scale,
        pixel_hinting,
        no_close,
        end_cap_style,
        fill_style,
    })
}

pub fn read_cap_style<R: BitRead>(reader: &mut R) -> Result<CapStyle> {
    CapStyle::try_from(reader.read_ub8(2)?).map_err(|_| Error::from(InvalidData))
}

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
