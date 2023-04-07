pub mod bevel_filter;
pub mod blend_mode;
pub mod blur_filter;
pub mod color_matrix_filter;
pub mod convolution_filter;
pub mod drop_shadow_filter;
pub mod filter;
pub mod glow_filter;
pub mod gradient_bevel_filter;
pub mod gradient_glow_filter;
pub mod place_object;
pub mod place_object_2;

use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::string::String;
use blend_mode::BlendMode;
use filter::Filter;
use place_object_2::PlaceObject2Tag;

/// Defines one or more event handlers to be invoked when their
/// corresponding events occur.
#[derive(Clone, PartialEq, Debug)]
pub struct ClipActions {
    pub all_event_flags: ClipEventFlags,
    pub clip_action_records: Vec<ClipActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClipActionRecord {
    pub event_flags: ClipEventFlags,
    pub key_code: Option<u8>,
    pub actions: ActionList<Vec<u8>>,
}

/// Adds a character to the display list.
///
/// Equivalent to PlaceObject2Tag, but with extended functionality.
#[derive(Clone, PartialEq, Debug)]
pub struct PlaceObject3Tag {
    /// Fields shared with PlaceObject2Tag.
    pub place_object_2: PlaceObject2Tag,

    /// True if the character has an opaque background.
    pub opaque_background: bool,

    /// The ActionScript 3 class name of the character to place.
    ///
    /// Used instead of character ID when the character was defined in an
    /// external SWF and has not been imported into the character dictionary
    /// of this SWF.
    pub class_name: Option<String>,

    /// List of filters applied to this character.
    pub surface_filter_list: Option<Vec<Filter>>,

    /// The blend mode to use when compositing the character onto the stage.
    pub blend_mode: Option<BlendMode>,

    /// True if the player should cache its rendering of the character as a
    /// bitmap.
    ///
    /// This slows down initial rendering of the character, but allows the
    /// player to render the character more quickly in subsequent redraws so
    /// long as it remains unchanged.
    pub bitmap_cache: Option<bool>,

    /// True if the character should be visible. False if it should be hidden.
    pub visible: Option<bool>,

    pub background_color: Option<Rgba>,
}

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
