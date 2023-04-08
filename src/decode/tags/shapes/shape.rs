use crate::decode::tags::shapes::ShapeRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct Shape<Color, LineStyle> {
    pub shape_records: Vec<ShapeRecord<Color, LineStyle>>,
}
