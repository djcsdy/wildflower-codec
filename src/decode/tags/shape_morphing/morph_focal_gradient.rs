use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::shape_morphing::morph_gradient::MorphGradient;
use crate::decode::tags::shape_morphing::morph_gradient_record::MorphGradientRecord;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct MorphFocalGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
    pub start_focal_point: Fixed8,
    pub end_focal_point: Fixed8,
}

impl MorphFocalGradient {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let morph_gradient = MorphGradient::read(reader)?;
        let start_focal_point = Fixed8::read(reader)?;
        let end_focal_point = Fixed8::read(reader)?;
        Ok(Self {
            gradient_records: morph_gradient.gradient_records,
            start_focal_point,
            end_focal_point,
        })
    }
}
