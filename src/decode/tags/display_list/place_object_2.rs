use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::string;
use crate::decode::tags::display_list::ClipActions;

/// Adds a character to the display list.
///
/// Equivalent to PlaceObjectTag, but with extended functionality.
#[derive(Clone, PartialEq, Debug)]
pub struct PlaceObject2Tag {
    /// If true, then this tag_readers modifies or replaces an existing character at
    /// the specified depth.
    ///
    /// If false, then this tag_readers creates a new character at the specified depth.
    /// There must not be an existing character at the specified depth.
    ///
    /// Called `PlaceFlagMove` in the SWF Specification.
    pub modify: bool,

    /// Depth of the character.
    ///
    /// Characters with higher depth values appear above characters with lower
    /// depth values.
    pub depth: u16,

    /// The ID of the character to place.
    ///
    /// The character must have previously been defined by another tag_readers.
    ///
    /// Required if `modify` is false, otherwise optional.
    pub character_id: Option<u16>,

    /// Affine transformation matrix applied to the character.
    pub matrix: Option<Matrix>,

    /// Color transformation applied to the character.
    pub color_transform: Option<ColorTransformWithAlpha>,

    pub ratio: Option<u16>,

    /// Specifies a name for the character.
    pub name: Option<string::String>,

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
