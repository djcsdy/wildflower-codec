use super::common::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub enum FillStyle<TColor> {
    Solid(TColor),
    LinearGradient {
        matrix: Matrix,
        gradient: Gradient<TColor>,
    },
    RadialGradient {
        matrix: Matrix,
        gradient: Gradient<TColor>,
    },
    FocalRadialGradient {
        matrix: Matrix,
        gradient: FocalGradient<TColor>,
    },
    RepeatingBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    ClippedBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    NonSmoothedRepeatingBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    NonSmoothedClippedBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct LineStyle<TColor> {
    pub width: u16,
    pub color: TColor,
}

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
pub struct Gradient<TColor> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<TColor>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FocalGradient<TColor> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<TColor>>,
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
pub struct GradientRecord<TColor> {
    pub ratio: u8,
    pub color: TColor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CapStyle {
    Round,
    None,
    Square,
}

#[derive(Clone, PartialEq, Debug)]
pub enum JoinStyle {
    Round,
    Bevel,
    Miter { miter_limit_factor: Fixed8 },
}
