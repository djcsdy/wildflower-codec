pub mod bevel_filter;
pub mod blend_mode;
pub mod blur_filter;
pub mod clip_action_record;
pub mod clip_actions;
pub mod clip_event_flags;
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
