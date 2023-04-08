use crate::decode::bit_read::BitRead;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::shapes::shape_record::{ReadShapeRecordArrayOptions, ShapeRecord};
use crate::decode::tags::styles::fill_style::FillStyle;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ShapeWithStyle<Color, LineStyle> {
    pub fill_styles: Vec<FillStyle<Color>>,
    pub line_styles: Vec<LineStyle>,
    pub shape_records: Vec<ShapeRecord<Color, LineStyle>>,
}

impl<Color, LineStyle> ShapeWithStyle<Color, LineStyle> {
    pub fn read<
        Read: BitRead + SizedRead,
        ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
        ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
    >(
        ReadShapeWithStyleOptions {
            reader,
            read_line_style_array,
            read_fill_style_array,
        }: ReadShapeWithStyleOptions<
            Read,
            Color,
            LineStyle,
            ReadLineStyleArray,
            ReadFillStyleArray,
        >,
    ) -> Result<Self> {
        let fill_styles = (read_fill_style_array)(reader)?;
        let line_styles = (read_line_style_array)(reader)?;
        let num_fill_bits = reader.read_ub8(4)?;
        let num_line_bits = reader.read_ub8(4)?;
        let shape_records = ShapeRecord::read_array(ReadShapeRecordArrayOptions {
            reader,
            num_fill_bits,
            num_line_bits,
            read_line_style_array,
            read_fill_style_array,
        })?;
        Ok(Self {
            fill_styles,
            line_styles,
            shape_records,
        })
    }
}

pub struct ReadShapeWithStyleOptions<
    'reader,
    'read_line_style_array,
    'read_fill_style_array,
    Read: BitRead + SizedRead,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
> {
    pub reader: &'reader mut Read,
    pub read_line_style_array: &'read_line_style_array ReadLineStyleArray,
    pub read_fill_style_array: &'read_fill_style_array ReadFillStyleArray,
}
