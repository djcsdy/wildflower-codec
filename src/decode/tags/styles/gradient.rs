use crate::decode::tags::styles::gradient_record::GradientRecord;
use crate::decode::tags::styles::interpolation_mode::InterpolationMode;
use crate::decode::tags::styles::spread_mode::SpreadMode;

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
}
