use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::shape_morphing::morph_line_style::MorphLineStyle;
use crate::decode::tags::shapes::shape::Shape;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineMorphShapeTag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub fill_styles: Vec<MorphFillStyle>,
    pub line_styles: Vec<MorphLineStyle>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}

impl DefineMorphShapeTag {
    pub fn read<R: SliceRead + SizedRead + BitRead>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let start_bounds = Rectangle::read(reader)?;
        let end_bounds = Rectangle::read(reader)?;
        let offset = reader.read_u32()? as usize;
        let fill_styles = MorphFillStyle::read_array(reader)?;
        let line_styles = read_line_style_array(reader, &MorphLineStyle::read)?;
        let start_edges = Shape::read(&mut reader.slice(offset))?;
        let end_edges = Shape::read(reader)?;
        Ok(Self {
            character_id,
            start_bounds,
            end_bounds,
            fill_styles,
            line_styles,
            start_edges,
            end_edges,
        })
    }
}
