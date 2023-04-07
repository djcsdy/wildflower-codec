pub mod cap_style;
pub mod fill_style;
pub mod focal_gradient;
pub mod gradient;
pub mod gradient_record;
pub mod interpolation_mode;
pub mod line_style;
pub mod line_style_2;
pub mod spread_mode;

use crate::decode::tags::common::fixed_8::Fixed8;

#[derive(Clone, PartialEq, Debug)]
pub enum JoinStyle {
    Round,
    Bevel,
    Miter { miter_limit_factor: Fixed8 },
}
