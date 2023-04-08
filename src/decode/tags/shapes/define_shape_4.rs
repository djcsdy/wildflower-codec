use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shapes::shape_with_style::ShapeWithStyle;
use crate::decode::tags::styles::line_style_2::LineStyle2;

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
