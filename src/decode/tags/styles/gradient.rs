use crate::decode::tags::styles::spread_mode::SpreadMode;
use crate::decode::tags::styles::{GradientRecord, InterpolationMode};

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
}
