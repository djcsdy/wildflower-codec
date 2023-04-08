use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shapes::ShapeWithStyle;
use crate::decode::tags::styles::line_style::LineStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShape3Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgba, LineStyle<Rgba>>,
}
