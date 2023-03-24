use crate::decode::tags::common::fixed_16::Fixed16;

#[derive(Clone, PartialEq, Debug)]
pub struct BlurFilter {
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub passes: u8,
}
