use crate::decode::tags::shapes::shape_record::ShapeRecord;
use crate::decode::tags::styles::fill_style::FillStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct ShapeWithStyle<Color, LineStyle> {
    pub fill_styles: Vec<FillStyle<Color>>,
    pub line_styles: Vec<LineStyle>,
    pub shape_records: Vec<ShapeRecord<Color, LineStyle>>,
}
