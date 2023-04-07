use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::JoinStyle;

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
