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

/// Adds a character to the display list.
///
/// Equivalent to PlaceObject2Tag, but with extended functionality.
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

pub enum BlendMode {
    Normal = 1,
    Layer = 2,
    Multiply = 3,
    Screen = 4,
    Lighten = 5,
    Darken = 6,
    Difference = 7,
    Add = 8,
    Subtract = 9,
    Invert = 10,
    Alpha = 11,
    Erase = 12,
    Overlay = 13,
    Hardlight = 14,
}

pub enum Filter {
    DropShadow(DropShadowFilter),
    Blur(BlurFilter),
    Glow(GlowFilter),
    Bevel(BevelFilter),
    GradientGlow(GradientGlowFilter),
    Convolution(ConvolutionFilter),
    ColorMatrix(ColorMatrixFilter),
    GradientBevel(GradientBevelFilter),
}

pub struct ColorMatrixFilter {
    pub matrix: [f32; 20]
}

pub struct ConvolutionFilter {
    pub divisor: f32,
    pub bias: f32,
    pub matrix: Vec<Vec<f32>>,
    pub default_color: Rgba,
    pub clamp: bool,
    pub preserve_alpha: bool,
}

pub struct BlurFilter {
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub passes: u8,
}

pub struct DropShadowFilter {
    pub color: Rgba,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub inner_shadow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub passes: u8,
}

pub struct GlowFilter {
    pub color: Rgba,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub strength: Fixed8,
    pub inner_glow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub passes: u8,
}

pub struct BevelFilter {
    pub shadow_color: Rgba,
    pub highlight_color: Rgba,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub inner_shadow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub on_top: bool,
    pub passes: u8,
}

pub struct GradientGlowFilter {
    pub colors: Vec<Rgba>,
    pub ratio: Vec<u8>,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub inner_shadow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub on_top: bool,
    pub passes: u8,
}

pub struct GradientBevelFilter {
    pub colors: Vec<Rgba>,
    pub ratio: Vec<u8>,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub inner_shadow: bool,
    pub knockout: bool,
    pub composite_source: bool,
    pub on_top: bool,
    pub passes: u8,
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
pub struct RemoveObjectTag {
    /// ID of character to remove.
    pub character_id: u16,

    /// Depth of character to remove.
    pub depth: u16,
}

/// Removes the character at the specified depth from the display list.
pub struct RemoveObject2Tag {
    /// Depth of character to remove.
    pub depth: u16
}

/// Instructs the player to display the contents of the display list.
///
/// The player will wait at least the duration of one frame before the next
/// ShowFrame tag takes effect.
pub struct ShowFrameTag {
}