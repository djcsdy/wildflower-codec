use crate::decode::tags::common::{Matrix, Rectangle, Rgb};
use crate::decode::tags::text::TextRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineTextTag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgb>>,
}
