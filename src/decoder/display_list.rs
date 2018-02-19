use super::common::*;
use super::actions::ActionRecord;

/// A Tag with an unknown or unsupported tag code.
pub struct UnknownTag {
    pub tag_code: u16,
    pub data: [u8],
}

/// A tag with a supported tag code but invalid data.
///
/// The tag does not adhere to the SWF Specification, and Wildflower was unable
/// to parse it even using known workarounds.
pub struct InvalidTag {
    pub tag_code: u16,
    pub data: [u8],
}

/// Adds a character to the display list.
pub struct PlaceObjectTag {
    /// The ID of the character to place.
    ///
    /// The character must have previously be defined by another tag.
    pub character_id: u16,

    /// Depth of the character.
    ///
    /// Characters with higher depth values appear above characters with lower
    /// depth values.
    ///
    /// If there is already a character with this depth then the existing
    /// character is replaced.
    pub depth: u16,

    /// Affine transformation matrix applied to the character.
    pub matrix: Matrix,

    /// Color transformation applied to the character.
    pub color_transform: Option<ColorTransform>,
}

/// Adds a character to the display list.
///
/// Equivalent to PlaceObjectTag, but with extended functionality.
pub struct PlaceObject2Tag {
    /// Depth of the character.
    ///
    /// Characters with higher depth values appear above characters with lower
    /// depth values.
    ///
    /// If there is already a character with this depth then the existing
    /// character is modified or replaced.
    pub depth: u16,

    /// The ID of the character to place.
    ///
    /// The character must have previously been defined by another tag.
    ///
    /// If no character ID is specified, then there must be an existing
    /// character at `depth`, which will be modified by the properties of
    /// this tag.
    pub character_id: Option<u16>,

    /// Affine transformation matrix applied to the character.
    pub matrix: Option<Matrix>,

    /// Color transformation applied to the character.
    pub color_transform: Option<ColorTransformationWithAlpha>,

    pub ratio: Option<u16>,

    /// Specifies a name for the character.
    pub name: Option<String>,

    /// The top-most depth that will be masked by the character.
    ///
    /// None or zero specifies that this character does not mask other
    /// characters.
    pub clip_depth: Option<u16>,

    /// Defines one or more event handlers to be invoked when their
    /// corresponding events occur.
    ///
    /// Valid only when placing or modifying sprite characters.
    pub clip_actions: Option<ClipActions>,
}

/// Defines one or more event handlers to be invoked when their
/// corresponding events occur.
pub struct ClipActions {
    pub all_event_flags: ClipEventFlags,
    pub clip_action_records: Vec<ClipActionRecord>,
}

pub struct ClipActionRecord {
    pub event_flags: ClipEventFlags,
    pub key_code: Option<u8>,
    pub actions: Vec<ActionRecord>,
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