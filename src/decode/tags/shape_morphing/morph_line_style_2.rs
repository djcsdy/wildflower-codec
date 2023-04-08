use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::join_style::JoinStyle;

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
