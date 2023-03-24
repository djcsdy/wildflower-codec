use crate::decode::tags::common::fixed_16::Fixed16;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct BevelFilter {
    pub shadow_color: Rgba,
    pub highlight_color: Rgba,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub inner_shadow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub on_top: bool,
    pub passes: u8,
}
