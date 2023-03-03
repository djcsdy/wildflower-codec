use crate::ast::common::Rgb;
use crate::ast::styles::{FillStyle, FocalGradient, Gradient, GradientRecord, LineStyle};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_fill_style_array<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<Vec<FillStyle<Rgb>>> {
    let fill_style_count = reader.read_u8()?;
    let mut fill_styles = Vec::with_capacity(fill_style_count as usize);
    for _ in 0..fill_style_count {
        fill_styles.push(read_fill_style(reader, &SwfTagBodyReader::read_rgb)?);
    }
    Ok(fill_styles)
}

pub fn read_extended_fill_style_array<
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: ReadColor,
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
    read_color: ReadColor,
) -> Result<FillStyle<Color>> {
    let fill_style_type = reader.read_u8()?;
    Ok(match fill_style_type {
        0x00 => FillStyle::Solid(read_color(reader)?),
        0x10 => {
            let matrix = reader.read_matrix()?;
            let gradient = read_gradient(reader, read_color)?;
            FillStyle::LinearGradient { matrix, gradient }
        }
        0x12 => {
            let matrix = reader.read_matrix()?;
            let gradient = read_gradient(reader, read_color)?;
            FillStyle::RadialGradient { matrix, gradient }
        }
        0x13 => {
            let matrix = reader.read_matrix()?;
            let gradient = read_focal_gradient(reader, read_color)?;
            FillStyle::FocalRadialGradient { matrix, gradient }
        }
        0x40 => {
            let bitmap_id = reader.read_u16()?;
            let matrix = reader.read_matrix()?;
            FillStyle::RepeatingBitmap { bitmap_id, matrix }
        }
        0x41 => {
            let bitmap_id = reader.read_u16()?;
            let matrix = reader.read_matrix()?;
            FillStyle::ClippedBitmap { bitmap_id, matrix }
        }
        0x42 => {
            let bitmap_id = reader.read_u16()?;
            let matrix = reader.read_matrix()?;
            FillStyle::NonSmoothedRepeatingBitmap { bitmap_id, matrix }
        }
        0x43 => {
            let bitmap_id = reader.read_u16()?;
            let matrix = reader.read_matrix()?;
            FillStyle::NonSmoothedClippedBitmap { bitmap_id, matrix }
        }
        _ => return Err(Error::from(InvalidData)),
    })
}

pub fn read_line_style_array<
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: ReadColor,
) -> Result<Vec<LineStyle<Color>>> {
    let mut count = reader.read_u8()? as u16;
    if count == 0xff {
        count = reader.read_u16()?
    };
    let mut line_styles = Vec::with_capacity(count as usize);
    for _ in 0..count {
        line_styles.push(read_line_style(reader, &read_color)?);
    }
    Ok(line_styles)
}

pub fn read_line_style<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: ReadColor,
) -> Result<LineStyle<Color>> {
    let width = reader.read_u16()?;
    let color = read_color(reader)?;
    Ok(LineStyle { width, color })
}

pub fn read_gradient<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: ReadColor,
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
    read_color: ReadColor,
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
    read_color: ReadColor,
) -> Result<GradientRecord<Color>> {
    let ratio = reader.read_u8()?;
    let color = read_color(reader)?;
    Ok(GradientRecord { ratio, color })
}
