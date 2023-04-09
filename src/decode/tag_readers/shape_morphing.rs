use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_read::SliceRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::shape_morphing::define_morph_shape::DefineMorphShapeTag;
use crate::decode::tags::shape_morphing::define_morph_shape_2::DefineMorphShape2Tag;
use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::shape_morphing::morph_line_style::MorphLineStyle;
use crate::decode::tags::shape_morphing::morph_line_style_2::MorphLineStyle2;
use crate::decode::tags::shapes::shape::Shape;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::Result;

pub fn read_define_morph_shape_tag(reader: &mut SwfSliceReader) -> Result<DefineMorphShapeTag> {
    let character_id = reader.read_u16()?;
    let start_bounds = Rectangle::read(reader)?;
    let end_bounds = Rectangle::read(reader)?;
    let offset = reader.read_u32()? as usize;
    let fill_styles = MorphFillStyle::read_array(reader)?;
    let line_styles = read_line_style_array(reader, &MorphLineStyle::read)?;
    let start_edges = Shape::read(&mut reader.slice(offset))?;
    let end_edges = Shape::read(reader)?;
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
    let line_styles = read_line_style_array(reader, &MorphLineStyle2::read)?;
    let start_edges = Shape::read(&mut reader.slice(offset))?;
    let end_edges = Shape::read(reader)?;
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
