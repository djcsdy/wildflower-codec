use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shapes::shape_with_style::{ReadShapeWithStyleOptions, ShapeWithStyle};
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::line_style_2::LineStyle2;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShape4Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub edge_bounds: Rectangle,
    pub uses_fill_winding_rule: bool,
    pub uses_non_scaling_strokes: bool,
    pub uses_scaling_strokes: bool,
    pub shape: ShapeWithStyle<Rgba, LineStyle2>,
}

impl DefineShape4Tag {
    pub fn read<R: BitRead + SizedRead>(reader: &mut R) -> Result<Self> {
        let shape_id = reader.read_u16()?;
        let shape_bounds = Rectangle::read(reader)?;
        let edge_bounds = Rectangle::read(reader)?;
        reader.read_ub8(5)?;
        let uses_fill_winding_rule = reader.read_bit()?;
        let uses_non_scaling_strokes = reader.read_bit()?;
        let uses_scaling_strokes = reader.read_bit()?;
        let options = ReadShapeWithStyleOptions {
            reader,
            read_line_style_array: &|reader| read_line_style_array(reader, &LineStyle2::read),
            read_fill_style_array: &|reader| FillStyle::read_extended_array(reader, &Rgba::read),
        };
        let shape = ShapeWithStyle::read(options)?;
        Ok(Self {
            shape_id,
            shape_bounds,
            edge_bounds,
            uses_fill_winding_rule,
            uses_non_scaling_strokes,
            uses_scaling_strokes,
            shape,
        })
    }
}
