use crate::ast::shape_morphing::{
    MorphFillStyle, MorphFocalGradient, MorphGradient, MorphGradientRecord,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::styles::{read_fill_style_type, FillStyleType};
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

fn read_morph_fill_style_array<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<Vec<MorphFillStyle>> {
    let mut count = reader.read_u8()? as usize;
    if count == 0xff {
        count = reader.read_u16()? as usize
    }
    let mut fill_styles = Vec::with_capacity(count);
    for _ in 0..count {
        fill_styles.push(read_morph_fill_style(reader)?);
    }
    Ok(fill_styles)
}

fn read_morph_fill_style<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<MorphFillStyle> {
    let fill_style_type = read_fill_style_type(reader)?;
    Ok(match fill_style_type {
        FillStyleType::Solid => {
            let start_color = reader.read_rgba()?;
            let end_color = reader.read_rgba()?;
            MorphFillStyle::Solid {
                start_color,
                end_color,
            }
        }
        FillStyleType::LinearGradient => {
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            let gradient = read_morph_gradient(reader)?;
            MorphFillStyle::LinearGradient {
                start_matrix,
                end_matrix,
                gradient,
            }
        }
        FillStyleType::RadialGradient => {
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            let gradient = read_morph_gradient(reader)?;
            MorphFillStyle::RadialGradient {
                start_matrix,
                end_matrix,
                gradient,
            }
        }
        FillStyleType::FocalRadialGradient => {
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            let gradient = read_morph_focal_gradient(reader)?;
            MorphFillStyle::FocalRadialGradient {
                start_matrix,
                end_matrix,
                gradient,
            }
        }
        FillStyleType::RepeatingBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            MorphFillStyle::RepeatingBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
        FillStyleType::ClippedBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            MorphFillStyle::ClippedBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
        FillStyleType::NonSmoothedRepeatingBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            MorphFillStyle::NonSmoothedRepeatingBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
        FillStyleType::NonSmoothedClippedBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = reader.read_matrix()?;
            let end_matrix = reader.read_matrix()?;
            MorphFillStyle::NonSmoothedClippedBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
    })
}

fn read_morph_gradient<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<MorphGradient> {
    let num_gradients = reader.read_u8()? as usize;
    let mut gradient_records = Vec::with_capacity(num_gradients);
    for _ in 0..num_gradients {
        gradient_records.push(read_morph_gradient_record(reader)?);
    }
    Ok(MorphGradient { gradient_records })
}

fn read_morph_focal_gradient<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<MorphFocalGradient> {
    let morph_gradient = read_morph_gradient(reader)?;
    let start_focal_point = reader.read_fixed8()?;
    let end_focal_point = reader.read_fixed8()?;
    Ok(MorphFocalGradient {
        gradient_records: morph_gradient.gradient_records,
        start_focal_point,
        end_focal_point,
    })
}

fn read_morph_gradient_record<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<MorphGradientRecord> {
    let start_ratio = reader.read_u8()?;
    let start_color = reader.read_rgba()?;
    let end_ratio = reader.read_u8()?;
    let end_color = reader.read_rgba()?;
    Ok(MorphGradientRecord {
        start_ratio,
        start_color,
        end_ratio,
        end_color,
    })
}
