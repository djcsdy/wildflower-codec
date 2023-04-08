use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradientRecord {
    pub start_ratio: u8,
    pub start_color: Rgba,
    pub end_ratio: u8,
    pub end_color: Rgba,
}
