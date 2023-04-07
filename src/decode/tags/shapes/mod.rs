use super::styles::LineStyle2;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::line_style::LineStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShapeTag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgb, LineStyle<Rgb>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShape2Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgb, LineStyle<Rgb>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineShape3Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgba, LineStyle<Rgba>>,
}

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

#[derive(Clone, PartialEq, Debug)]
pub struct Shape<Color, LineStyle> {
    pub shape_records: Vec<ShapeRecord<Color, LineStyle>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ShapeWithStyle<Color, LineStyle> {
    pub fill_styles: Vec<FillStyle<Color>>,
    pub line_styles: Vec<LineStyle>,
    pub shape_records: Vec<ShapeRecord<Color, LineStyle>>,
}

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
