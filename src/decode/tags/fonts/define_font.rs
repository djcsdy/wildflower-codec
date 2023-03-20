use crate::decode::tags::shapes::Shape;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontTag {
    pub font_id: u16,
    pub glyph_shapes: Vec<Shape<(), ()>>,
}
