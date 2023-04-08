use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle {
    pub start_width: u16,
    pub end_width: u16,
    pub start_color: Rgba,
    pub end_color: Rgba,
}
