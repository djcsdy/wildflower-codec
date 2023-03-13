use super::common::*;
use super::styles::{FillStyle, LineStyle, LineStyle2};

pub struct DefineShapeTag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgb, LineStyle<Rgb>>,
}

pub struct DefineShape2Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgb, LineStyle<Rgb>>,
}

pub struct DefineShape3Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub shape: ShapeWithStyle<Rgba, LineStyle<Rgba>>,
}

pub struct DefineShape4Tag {
    pub shape_id: u16,
    pub shape_bounds: Rectangle,
    pub edge_bounds: Rectangle,
    pub uses_fill_winding_rule: bool,
    pub uses_non_scaling_strokes: bool,
    pub uses_scaling_strokes: bool,
    pub shape: ShapeWithStyle<Rgba, LineStyle2>,
}

pub struct Shape<TColor, TLineStyle> {
    pub shape_records: Vec<ShapeRecord<TColor, TLineStyle>>,
}

pub struct ShapeWithStyle<TColor, TLineStyle> {
    pub fill_styles: Vec<FillStyle<TColor>>,
    pub line_styles: Vec<TLineStyle>,
    pub shape_records: Vec<ShapeRecord<TColor, TLineStyle>>,
}

pub enum ShapeRecord<TColor, TLineStyle> {
    EndShape,
    StyleChange(StyleChangeRecord<TColor, TLineStyle>),
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

#[derive(Clone, PartialEq, Debug)]
pub struct StyleChangeRecord<TColor, TLineStyle> {
    pub move_delta: (i16, i16),
    pub fill_style_0: u16,
    pub fill_style_1: u16,
    pub line_style: u16,
    pub fill_styles: Option<Vec<FillStyle<TColor>>>,
    pub line_styles: Option<Vec<TLineStyle>>,
}

pub struct StraightEdgeRecord {
    pub delta_x: i32,
    pub delta_y: i32,
}

pub struct CurvedEdgeRecord {
    pub control_delta_x: i32,
    pub control_delta_y: i32,
    pub anchor_delta_x: i32,
    pub anchor_delta_y: i32,
}
