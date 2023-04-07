pub mod fill_style;

use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use fill_style::FillStyle;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, PartialEq, Debug)]
pub struct LineStyle<Color> {
    pub width: u16,
    pub color: Color,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LineStyle2 {
    pub width: u16,
    pub start_cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub no_h_scale: bool,
    pub no_v_scale: bool,
    pub pixel_hinting: bool,
    pub no_close: bool,
    pub end_cap_style: CapStyle,
    pub fill_style: FillStyle<Rgba>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FocalGradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
    pub focal_point: Fixed8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SpreadMode {
    Pad,
    Reflect,
    Repeat,
}

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
