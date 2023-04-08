use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::shape_morphing::MorphLineStyle;
use crate::decode::tags::shapes::shape::Shape;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineMorphShapeTag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub fill_styles: Vec<MorphFillStyle>,
    pub line_styles: Vec<MorphLineStyle>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}
