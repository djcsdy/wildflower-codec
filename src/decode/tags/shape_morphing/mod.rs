pub mod define_morph_shape;
pub mod define_morph_shape_2;
pub mod morph_fill_style;
pub mod morph_focal_gradient;
pub mod morph_gradient;
pub mod morph_gradient_record;

use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::join_style::JoinStyle;
use morph_fill_style::MorphFillStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle {
    pub start_width: u16,
    pub end_width: u16,
    pub start_color: Rgba,
    pub end_color: Rgba,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle2 {
    pub start_width: u16,
    pub end_width: u16,
    pub start_cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub no_h_scale: bool,
    pub no_v_scale: bool,
    pub pixel_hinting: bool,
    pub no_close: bool,
    pub end_cap_style: CapStyle,
    pub fill_style: MorphFillStyle,
}
