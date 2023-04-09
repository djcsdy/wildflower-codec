use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::shape_morphing::morph_line_style_2::MorphLineStyle2;
use crate::decode::tags::shapes::shape::Shape;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineMorphShape2Tag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub start_edge_bounds: Rectangle,
    pub end_edge_bounds: Rectangle,
    pub uses_non_scaling_strokes: bool,
    pub uses_scaling_strokes: bool,
    pub fill_styles: Vec<MorphFillStyle>,
    pub line_styles: Vec<MorphLineStyle2>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}

impl DefineMorphShape2Tag {
    pub fn read<R: SliceRead + SizedRead + BitRead>(reader: &mut R) -> Result<Self> {
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
        Ok(Self {
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
}
