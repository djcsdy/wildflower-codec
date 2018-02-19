use super::common::*;

/// Sets the background color of the display.
pub struct SetBackgroundColorTag {
    color: Rgb
}

/// Labels the current frame with the specified name.
pub struct FrameLabelTag {
    name: String,

    named_anchor: bool,
}