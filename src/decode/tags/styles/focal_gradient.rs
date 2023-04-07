use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::styles::{GradientRecord, InterpolationMode, SpreadMode};

#[derive(Clone, PartialEq, Debug)]
pub struct FocalGradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
    pub focal_point: Fixed8,
}
