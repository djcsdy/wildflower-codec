use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::read_options::ReadOptions;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::string::String;
use crate::decode::tags::display_list::clip_actions::ClipActions;
use std::io::Result;

/// Adds a character to the display list.
///
/// Equivalent to PlaceObjectTag, but with extended functionality.
#[derive(Clone, PartialEq, Debug)]
pub struct PlaceObject2Tag {
    /// If true, then this tag modifies or replaces an existing character at
    /// the specified depth.
    ///
    /// If false, then this tag creates a new character at the specified depth.
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
    /// The character must have previously been defined by another tag.
    ///
    /// Required if `modify` is false, otherwise optional.
    pub character_id: Option<u16>,

    /// Affine transformation matrix applied to the character.
    pub matrix: Option<Matrix>,

    /// Color transformation applied to the character.
    pub color_transform: Option<ColorTransformWithAlpha>,

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

impl PlaceObject2Tag {
    pub fn read<R: BitRead + SliceRead>(
        ReadOptions {
            reader,
            swf_version,
        }: ReadOptions<R>,
    ) -> Result<Self> {
        let has_clip_actions = reader.read_bit()?;
        let has_clip_depth = reader.read_bit()?;
        let has_name = reader.read_bit()?;
        let has_ratio = reader.read_bit()?;
        let has_color_transform = reader.read_bit()?;
        let has_matrix = reader.read_bit()?;
        let has_character = reader.read_bit()?;
        let modify = reader.read_bit()?;
        let depth = reader.read_u16()?;
        let character_id = if has_character {
            Some(reader.read_u16()?)
        } else {
            None
        };
        let matrix = if has_matrix {
            Some(Matrix::read(reader)?)
        } else {
            None
        };
        let color_transform = if has_color_transform {
            Some(ColorTransformWithAlpha::read(reader)?)
        } else {
            None
        };
        let ratio = if has_ratio {
            Some(reader.read_u16()?)
        } else {
            None
        };
        let name = if has_name {
            Some(String::read(reader)?)
        } else {
            None
        };
        let clip_depth = if has_clip_depth {
            Some(reader.read_u16()?)
        } else {
            None
        };
        let clip_actions = if has_clip_actions {
            Some(ClipActions::read(ReadOptions {
                reader,
                swf_version,
            })?)
        } else {
            None
        };
        Ok(Self {
            modify,
            depth,
            character_id,
            matrix,
            color_transform,
            ratio,
            name,
            clip_depth,
            clip_actions,
        })
    }
}
