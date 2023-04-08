pub mod define_shape;
pub mod define_shape_2;
pub mod define_shape_3;
pub mod define_shape_4;
pub mod shape;
pub mod shape_with_style;

use crate::decode::tags::styles::fill_style::FillStyle;

#[derive(Clone, PartialEq, Debug)]
pub enum ShapeRecord<Color, LineStyle> {
    EndShape,
    StyleChange(StyleChangeRecord<Color, LineStyle>),
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

#[derive(Clone, PartialEq, Debug)]
pub struct StyleChangeRecord<Color, LineStyle> {
    pub move_delta: (i16, i16),
    pub fill_style_0: u16,
    pub fill_style_1: u16,
    pub line_style: u16,
    pub fill_styles: Option<Vec<FillStyle<Color>>>,
    pub line_styles: Option<Vec<LineStyle>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct StraightEdgeRecord {
    pub delta_x: i32,
    pub delta_y: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CurvedEdgeRecord {
    pub control_delta_x: i32,
    pub control_delta_y: i32,
    pub anchor_delta_x: i32,
    pub anchor_delta_y: i32,
}
