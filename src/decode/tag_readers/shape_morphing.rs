use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::shapes::read_shape;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shape_morphing::define_morph_shape::DefineMorphShapeTag;
use crate::decode::tags::shape_morphing::define_morph_shape_2::DefineMorphShape2Tag;
use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::shape_morphing::morph_line_style::MorphLineStyle;
use crate::decode::tags::shape_morphing::morph_line_style_2::MorphLineStyle2;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::join_style::JoinStyle;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_define_morph_shape_tag(reader: &mut SwfSliceReader) -> Result<DefineMorphShapeTag> {
    let character_id = reader.read_u16()?;
    let start_bounds = Rectangle::read(reader)?;
    let end_bounds = Rectangle::read(reader)?;
    let offset = reader.read_u32()? as usize;
    let fill_styles = MorphFillStyle::read_array(reader)?;
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
    let fill_styles = MorphFillStyle::read_array(reader)?;
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

fn read_morph_line_style<R: Read>(reader: &mut R) -> Result<MorphLineStyle> {
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

fn read_morph_line_style_2<R: BitRead>(reader: &mut R) -> Result<MorphLineStyle2> {
    let start_width = reader.read_u16()?;
    let end_width = reader.read_u16()?;
    let start_cap_style = CapStyle::read(reader)?;
    let join_style = reader.read_ub8(2)?;
    let has_fill = reader.read_bit()?;
    let no_h_scale = reader.read_bit()?;
    let no_v_scale = reader.read_bit()?;
    let pixel_hinting = reader.read_bit()?;
    reader.read_ub8(5)?;
    let no_close = reader.read_bit()?;
    let end_cap_style = CapStyle::read(reader)?;
    let miter_limit_factor = if join_style == 2 {
        Some(Fixed8::read(reader)?)
    } else {
        None
    };
    let fill_style = if has_fill {
        MorphFillStyle::read(reader)?
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
