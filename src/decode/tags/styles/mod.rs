pub mod fill_style;
pub mod focal_gradient;
pub mod gradient;
pub mod line_style;
pub mod line_style_2;
pub mod spread_mode;

use crate::decode::tags::common::fixed_8::Fixed8;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InterpolationMode {
    Gamma,
    Linear,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GradientRecord<Color> {
    pub ratio: u8,
    pub color: Color,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CapStyle {
    Round = 0,
    None = 1,
    Square = 2,
}

#[derive(Clone, PartialEq, Debug)]
pub enum JoinStyle {
    Round,
    Bevel,
    Miter { miter_limit_factor: Fixed8 },
}
