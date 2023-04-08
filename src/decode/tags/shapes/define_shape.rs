use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::shapes::ShapeWithStyle;
use crate::decode::tags::styles::line_style::LineStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShapeTag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgb, LineStyle<Rgb>>,
}
