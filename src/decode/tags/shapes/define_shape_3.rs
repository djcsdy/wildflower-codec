use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shapes::shape_with_style::{ReadShapeWithStyleOptions, ShapeWithStyle};
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::line_style::LineStyle;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShape3Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgba, LineStyle<Rgba>>,
}

impl DefineShape3Tag {
    pub fn read<R: BitRead + SizedRead>(reader: &mut R) -> Result<Self> {
        let shape_id = reader.read_u16()?;
        let shape_bounds = Rectangle::read(reader)?;
        let options = ReadShapeWithStyleOptions {
            reader,
            read_line_style_array: &|reader| {
                read_line_style_array(reader, &|reader| LineStyle::read(reader, &Rgba::read))
            },
            read_fill_style_array: &|reader| FillStyle::read_extended_array(reader, &Rgba::read),
        };
        let shape = ShapeWithStyle::read(options)?;
        Ok(Self {
            shape_id,
            shape_bounds,
            shape,
        })
    }
}
