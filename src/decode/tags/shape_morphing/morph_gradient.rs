use crate::decode::tags::shape_morphing::morph_gradient_record::MorphGradientRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
}
