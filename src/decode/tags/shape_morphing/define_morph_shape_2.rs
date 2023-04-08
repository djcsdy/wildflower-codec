use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::shape_morphing::{MorphFillStyle, MorphLineStyle2};
use crate::decode::tags::shapes::shape::Shape;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineMorphShape2Tag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub start_edge_bounds: Rectangle,
    pub end_edge_bounds: Rectangle,
    pub uses_non_scaling_strokes: bool,
    pub uses_scaling_strokes: bool,
    pub fill_styles: Vec<MorphFillStyle>,
    pub line_styles: Vec<MorphLineStyle2>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}
