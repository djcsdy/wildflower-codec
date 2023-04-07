use crate::decode::tags::common::color_transform::ColorTransform;
use crate::decode::tags::common::matrix::Matrix;

/// Adds a character to the display list.
#[derive(Clone, PartialEq, Debug)]
pub struct PlaceObjectTag {
    /// The ID of the character to place.
    ///
    /// The character must have previously be defined by another tag_readers.
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
