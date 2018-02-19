use records::*;

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
    pub depth: u16,

    /// Affine transformation matrix applied to the character.
    pub matrix: Matrix,

    /// Color transformation applied to the character.
    pub color_transform: Option<ColorTransform>,
}