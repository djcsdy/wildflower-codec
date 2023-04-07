pub mod bevel_filter;
pub mod blend_mode;
pub mod blur_filter;
pub mod clip_action_record;
pub mod clip_actions;
pub mod color_matrix_filter;
pub mod convolution_filter;
pub mod drop_shadow_filter;
pub mod filter;
pub mod glow_filter;
pub mod gradient_bevel_filter;
pub mod gradient_glow_filter;
pub mod place_object;
pub mod place_object_2;
pub mod place_object_3;

bitflags! {
    pub struct ClipEventFlags: u32 {
        const KEY_UP = 0x8000_0000;
        const KEY_DOWN = 0x4000_0000;
        const MOUSE_UP = 0x2000_0000;
        const MOUSE_DOWN = 0x1000_0000;
        const MOUSE_MOVE = 0x0800_0000;
        const UNLOAD = 0x0400_0000;
        const ENTER_FRAME = 0x0200_0000;
        const LOAD = 0x0100_0000;
        const DRAG_OVER = 0x0080_0000;
        const ROLL_OUT = 0x0040_0000;
        const ROLL_OVER = 0x0020_0000;
        const RELEASE_OUTSIDE = 0x0010_0000;
        const RELEASE = 0x0008_0000;
        const PRESS = 0x0004_0000;
        const INITIALIZE = 0x0002_0000;
        const DATA = 0x0001_0000;
        const CONSTRUCT = 0x0000_0400;
        const KEY_PRESS = 0x0000_0200;
        const DRAG_OUT = 0x0000_0100;
        const RESERVED = 0x0000_f8ff;
    }
}

/// Removes the specified character at the specified depth from the display
/// list.
#[derive(Clone, PartialEq, Debug)]
pub struct RemoveObjectTag {
    /// ID of character to remove.
    pub character_id: u16,

    /// Depth of character to remove.
    pub depth: u16,
}

/// Removes the character at the specified depth from the display list.
#[derive(Clone, PartialEq, Debug)]
pub struct RemoveObject2Tag {
    /// Depth of character to remove.
    pub depth: u16,
}

/// Instructs the player to display the contents of the display list.
///
/// The player will wait at least the duration of one frame before the next
/// ShowFrame tag_readers takes effect.
#[derive(Clone, PartialEq, Debug)]
pub struct ShowFrameTag {}
