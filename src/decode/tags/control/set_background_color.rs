use crate::decode::tags::common::rgb::Rgb;

/// Sets the background color of the display.
#[derive(Clone, PartialEq, Debug)]
pub struct SetBackgroundColorTag {
    pub color: Rgb,
}
