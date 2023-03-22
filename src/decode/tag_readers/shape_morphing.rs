use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::common::read_matrix;
use crate::decode::tag_readers::shapes::read_shape;
use crate::decode::tag_readers::styles::{
    read_cap_style, read_fill_style_type, read_line_style_array, FillStyleType,
};
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shape_morphing::{
    DefineMorphShape2Tag, DefineMorphShapeTag, MorphFillStyle, MorphFocalGradient, MorphGradient,
    MorphGradientRecord, MorphLineStyle, MorphLineStyle2,
};
use crate::decode::tags::styles::JoinStyle;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

pub fn read_define_morph_shape_tag(reader: &mut SwfSliceReader) -> Result<DefineMorphShapeTag> {
    let character_id = reader.read_u16()?;
    let start_bounds = Rectangle::read(reader)?;
    let end_bounds = Rectangle::read(reader)?;
    let offset = reader.read_u32()? as usize;
    let fill_styles = read_morph_fill_style_array(reader)?;
    let line_styles = read_line_style_array(reader, &read_morph_line_style)?;
    let start_edges = read_shape(&mut reader.slice(offset))?;
    let end_edges = read_shape(reader)?;
    Ok(DefineMorphShapeTag {
        character_id,
        start_bounds,
        end_bounds,
        fill_styles,
        line_styles,
        start_edges,
        end_edges,
    })
}

pub fn read_define_morph_shape_2_tag(reader: &mut SwfSliceReader) -> Result<DefineMorphShape2Tag> {
    let character_id = reader.read_u16()?;
    let start_bounds = Rectangle::read(reader)?;
    let end_bounds = Rectangle::read(reader)?;
    let start_edge_bounds = Rectangle::read(reader)?;
    let end_edge_bounds = Rectangle::read(reader)?;
    reader.read_ub8(6)?;
    let uses_non_scaling_strokes = reader.read_bit()?;
    let uses_scaling_strokes = reader.read_bit()?;
    let offset = reader.read_u32()? as usize;
    let fill_styles = read_morph_fill_style_array(reader)?;
    let line_styles = read_line_style_array(reader, &read_morph_line_style_2)?;
    let start_edges = read_shape(&mut reader.slice(offset))?;
    let end_edges = read_shape(reader)?;
    Ok(DefineMorphShape2Tag {
        character_id,
        start_bounds,
        end_bounds,
        start_edge_bounds,
        end_edge_bounds,
        uses_non_scaling_strokes,
        uses_scaling_strokes,
        fill_styles,
        line_styles,
        start_edges,
        end_edges,
    })
}

fn read_morph_fill_style_array(reader: &mut SwfSliceReader) -> Result<Vec<MorphFillStyle>> {
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

fn read_morph_fill_style(reader: &mut SwfSliceReader) -> Result<MorphFillStyle> {
    let fill_style_type = read_fill_style_type(reader)?;
    Ok(match fill_style_type {
        FillStyleType::Solid => {
            let start_color = Rgba::read(reader)?;
            let end_color = Rgba::read(reader)?;
            MorphFillStyle::Solid {
                start_color,
                end_color,
            }
        }
        FillStyleType::LinearGradient => {
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            let gradient = read_morph_gradient(reader)?;
            MorphFillStyle::LinearGradient {
                start_matrix,
                end_matrix,
                gradient,
            }
        }
        FillStyleType::RadialGradient => {
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            let gradient = read_morph_gradient(reader)?;
            MorphFillStyle::RadialGradient {
                start_matrix,
                end_matrix,
                gradient,
            }
        }
        FillStyleType::FocalRadialGradient => {
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            let gradient = read_morph_focal_gradient(reader)?;
            MorphFillStyle::FocalRadialGradient {
                start_matrix,
                end_matrix,
                gradient,
            }
        }
        FillStyleType::RepeatingBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            MorphFillStyle::RepeatingBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
        FillStyleType::ClippedBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            MorphFillStyle::ClippedBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
        FillStyleType::NonSmoothedRepeatingBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            MorphFillStyle::NonSmoothedRepeatingBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
        FillStyleType::NonSmoothedClippedBitmap => {
            let bitmap_id = reader.read_u16()?;
            let start_matrix = read_matrix(reader)?;
            let end_matrix = read_matrix(reader)?;
            MorphFillStyle::NonSmoothedClippedBitmap {
                bitmap_id,
                start_matrix,
                end_matrix,
            }
        }
    })
}

fn read_morph_gradient(reader: &mut SwfSliceReader) -> Result<MorphGradient> {
    let num_gradients = reader.read_u8()? as usize;
    let mut gradient_records = Vec::with_capacity(num_gradients);
    for _ in 0..num_gradients {
        gradient_records.push(read_morph_gradient_record(reader)?);
    }
    Ok(MorphGradient { gradient_records })
}

fn read_morph_focal_gradient(reader: &mut SwfSliceReader) -> Result<MorphFocalGradient> {
    let morph_gradient = read_morph_gradient(reader)?;
    let start_focal_point = reader.read_fixed_8()?;
    let end_focal_point = reader.read_fixed_8()?;
    Ok(MorphFocalGradient {
        gradient_records: morph_gradient.gradient_records,
        start_focal_point,
        end_focal_point,
    })
}

fn read_morph_gradient_record(reader: &mut SwfSliceReader) -> Result<MorphGradientRecord> {
    let start_ratio = reader.read_u8()?;
    let start_color = Rgba::read(reader)?;
    let end_ratio = reader.read_u8()?;
    let end_color = Rgba::read(reader)?;
    Ok(MorphGradientRecord {
        start_ratio,
        start_color,
        end_ratio,
        end_color,
    })
}

fn read_morph_line_style(reader: &mut SwfSliceReader) -> Result<MorphLineStyle> {
    let start_width = reader.read_u16()?;
    let end_width = reader.read_u16()?;
    let start_color = Rgba::read(reader)?;
    let end_color = Rgba::read(reader)?;
    Ok(MorphLineStyle {
        start_width,
        end_width,
        start_color,
        end_color,
    })
}

fn read_morph_line_style_2(reader: &mut SwfSliceReader) -> Result<MorphLineStyle2> {
    let start_width = reader.read_u16()?;
    let end_width = reader.read_u16()?;
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
        Some(reader.read_fixed_8()?)
    } else {
        None
    };
    let fill_style = if has_fill {
        read_morph_fill_style(reader)?
    } else {
        let start_color = Rgba::read(reader)?;
        let end_color = Rgba::read(reader)?;
        MorphFillStyle::Solid {
            start_color,
            end_color,
        }
    };
    Ok(MorphLineStyle2 {
        start_width,
        end_width,
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
