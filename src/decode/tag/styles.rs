use crate::ast::common::Rgb;
use crate::ast::styles::{
    CapStyle, FillStyle, FocalGradient, Gradient, GradientRecord, JoinStyle, LineStyle, LineStyle2,
};
use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::common::{read_matrix, read_rgb, read_rgba};
use crate::decode::tag_body_reader::SwfTagBodyReader;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_fill_style_array<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<Vec<FillStyle<Rgb>>> {
    let fill_style_count = reader.read_u8()?;
    let mut fill_styles = Vec::with_capacity(fill_style_count as usize);
    for _ in 0..fill_style_count {
        fill_styles.push(read_fill_style(reader, &read_rgb)?);
    }
    Ok(fill_styles)
}

pub fn read_extended_fill_style_array<
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: &ReadColor,
) -> Result<Vec<FillStyle<Color>>> {
    let mut fill_style_count = reader.read_u8()? as u16;
    if fill_style_count == 0xff {
        fill_style_count = reader.read_u16()?;
    }
    let mut fill_styles = Vec::with_capacity(fill_style_count as usize);
    for _ in 0..fill_style_count {
        fill_styles.push(read_fill_style(reader, &read_color)?);
    }
    Ok(fill_styles)
}

pub fn read_fill_style<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: &ReadColor,
) -> Result<FillStyle<Color>> {
    let fill_style_type = read_fill_style_type(reader)?;
    Ok(match fill_style_type {
        FillStyleType::Solid => FillStyle::Solid(read_color(reader)?),
        FillStyleType::LinearGradient => {
            let matrix = read_matrix(reader)?;
            let gradient = read_gradient(reader, read_color)?;
            FillStyle::LinearGradient { matrix, gradient }
        }
        FillStyleType::RadialGradient => {
            let matrix = read_matrix(reader)?;
            let gradient = read_gradient(reader, read_color)?;
            FillStyle::RadialGradient { matrix, gradient }
        }
        FillStyleType::FocalRadialGradient => {
            let matrix = read_matrix(reader)?;
            let gradient = read_focal_gradient(reader, read_color)?;
            FillStyle::FocalRadialGradient { matrix, gradient }
        }
        FillStyleType::RepeatingBitmap => {
            let bitmap_id = reader.read_u16()?;
            let matrix = read_matrix(reader)?;
            FillStyle::RepeatingBitmap { bitmap_id, matrix }
        }
        FillStyleType::ClippedBitmap => {
            let bitmap_id = reader.read_u16()?;
            let matrix = read_matrix(reader)?;
            FillStyle::ClippedBitmap { bitmap_id, matrix }
        }
        FillStyleType::NonSmoothedRepeatingBitmap => {
            let bitmap_id = reader.read_u16()?;
            let matrix = read_matrix(reader)?;
            FillStyle::NonSmoothedRepeatingBitmap { bitmap_id, matrix }
        }
        FillStyleType::NonSmoothedClippedBitmap => {
            let bitmap_id = reader.read_u16()?;
            let matrix = read_matrix(reader)?;
            FillStyle::NonSmoothedClippedBitmap { bitmap_id, matrix }
        }
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum FillStyleType {
    Solid = 0x00,
    LinearGradient = 0x10,
    RadialGradient = 0x12,
    FocalRadialGradient = 0x13,
    RepeatingBitmap = 0x40,
    ClippedBitmap = 0x41,
    NonSmoothedRepeatingBitmap = 0x42,
    NonSmoothedClippedBitmap = 0x43,
}

pub fn read_fill_style_type<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<FillStyleType> {
    reader
        .read_u8()?
        .try_into()
        .map_err(|_| Error::from(InvalidData))
}

pub fn read_line_style_array<
    R: Read,
    LineStyle,
    ReadLineStyle: Fn(&mut SwfTagBodyReader<R>) -> Result<LineStyle>,
>(
    reader: &mut SwfTagBodyReader<R>,
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

pub fn read_line_style<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: &ReadColor,
) -> Result<LineStyle<Color>> {
    let width = reader.read_u16()?;
    let color = read_color(reader)?;
    Ok(LineStyle { width, color })
}

pub fn read_line_style2<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<LineStyle2> {
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
        Some(reader.read_fixed8()?)
    } else {
        None
    };
    let fill_style = if has_fill {
        read_fill_style(reader, &read_rgba)?
    } else {
        FillStyle::Solid(read_rgba(reader)?)
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

pub fn read_cap_style<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<CapStyle> {
    CapStyle::try_from(reader.read_ub8(2)?).map_err(|_| Error::from(InvalidData))
}

pub fn read_gradient<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    reader: &mut SwfTagBodyReader<R>,
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

pub fn read_focal_gradient<
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: &ReadColor,
) -> Result<FocalGradient<Color>> {
    let gradient = read_gradient(reader, read_color)?;
    let focal_point = reader.read_fixed8()?;
    Ok(FocalGradient {
        spread_mode: gradient.spread_mode,
        interpolation_mode: gradient.interpolation_mode,
        gradient_records: gradient.gradient_records,
        focal_point,
    })
}

fn read_gradient_record<
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: &ReadColor,
) -> Result<GradientRecord<Color>> {
    let ratio = reader.read_u8()?;
    let color = read_color(reader)?;
    Ok(GradientRecord { ratio, color })
}
