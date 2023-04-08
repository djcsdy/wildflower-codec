use std::io::Result;
use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::shapes::shape_with_style::{ReadShapeWithStyleOptions, ShapeWithStyle};
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::line_style::LineStyle;
use crate::decode::tags::styles::line_style_array::read_line_style_array;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShapeTag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgb, LineStyle<Rgb>>,
}

impl DefineShapeTag {
    pub fn read<R: BitRead + SizedRead>(reader: &mut R) -> Result<Self> {
        let shape_id = reader.read_u16()?;
        let shape_bounds = Rectangle::read(reader)?;
        let options = ReadShapeWithStyleOptions {
            reader,
            read_line_style_array: &|reader| {
                read_line_style_array(reader, &|reader| LineStyle::read(reader, &Rgb::read))
            },
            read_fill_style_array: &|reader| FillStyle::read_array(reader),
        };
        let shape = ShapeWithStyle::read(options)?;
        Ok(Self {
            shape_id,
            shape_bounds,
            shape,
        })
    }
}
