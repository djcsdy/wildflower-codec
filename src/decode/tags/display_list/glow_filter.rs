use crate::decode::tags::common::fixed_16::Fixed16;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct GlowFilter {
    pub color: Rgba,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub strength: Fixed8,
    pub inner_glow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub passes: u8,
}
