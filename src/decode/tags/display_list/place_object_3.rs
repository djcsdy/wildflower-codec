use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::read_options::ReadOptions;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::string::String;
use crate::decode::tags::display_list::blend_mode::BlendMode;
use crate::decode::tags::display_list::clip_actions::ClipActions;
use crate::decode::tags::display_list::filter::Filter;
use crate::decode::tags::display_list::place_object_2::PlaceObject2Tag;
use std::io::Result;

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

impl PlaceObject3Tag {
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
        let opaque_background = reader.read_bit()?;
        let has_visible = reader.read_bit()?;
        let has_image = reader.read_bit()?;
        let has_class_name = reader.read_bit()?;
        let has_cache_as_bitmap = reader.read_bit()?;
        let has_blend_mode = reader.read_bit()?;
        let has_filter_list = reader.read_bit()?;
        let depth = reader.read_u16()?;
        let class_name = if has_class_name || (has_image && has_character) {
            Some(String::read(reader)?)
        } else {
            None
        };
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
        let surface_filter_list = if has_filter_list {
            Some(Filter::read_list(reader)?)
        } else {
            None
        };
        let blend_mode = if has_blend_mode {
            Some(BlendMode::read(reader)?)
        } else {
            None
        };
        let bitmap_cache = if has_cache_as_bitmap {
            Some(reader.read_u8()? != 0)
        } else {
            None
        };
        let visible = if has_visible {
            Some(reader.read_u8()? != 0)
        } else {
            None
        };
        let background_color = if has_visible {
            Some(Rgba::read(reader)?)
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
            place_object_2: PlaceObject2Tag {
                modify,
                depth,
                character_id,
                matrix,
                color_transform,
                ratio,
                name,
                clip_depth,
                clip_actions,
            },
            opaque_background,
            class_name,
            surface_filter_list,
            blend_mode,
            bitmap_cache,
            visible,
            background_color,
        })
    }
}
