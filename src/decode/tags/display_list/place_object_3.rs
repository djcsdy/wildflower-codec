use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::string::String;
use crate::decode::tags::display_list::blend_mode::BlendMode;
use crate::decode::tags::display_list::filter::Filter;
use crate::decode::tags::display_list::place_object_2::PlaceObject2Tag;

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
