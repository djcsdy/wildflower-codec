use crate::decode::tags::styles::{GradientRecord, InterpolationMode, SpreadMode};

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
}
