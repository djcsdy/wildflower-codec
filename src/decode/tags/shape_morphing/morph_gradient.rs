use crate::decode::tags::shape_morphing::MorphGradientRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
}
