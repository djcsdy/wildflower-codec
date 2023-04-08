use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::shape_morphing::morph_gradient_record::MorphGradientRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct MorphFocalGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
    pub start_focal_point: Fixed8,
    pub end_focal_point: Fixed8,
}
