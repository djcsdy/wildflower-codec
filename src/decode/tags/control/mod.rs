use super::common::*;
use crate::decode::tags::common::rgb::Rgb;

/// Sets the background color of the display.
#[derive(Clone, PartialEq, Debug)]
pub struct SetBackgroundColorTag {
    pub color: Rgb,
}

/// Labels the current frame with the specified name.
#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelTag {
    pub name: String,

    pub named_anchor: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProtectTag {
    pub password_md5: Vec<u8>,
}

/// Marks the end of a SWF file or of a sprite definition.
#[derive(Clone, PartialEq, Debug)]
pub struct EndTag {}

/// Makes portions of the SWF file available for import by other SWF files
/// using `ImportAssetsTag`.
#[derive(Clone, PartialEq, Debug)]
pub struct ExportAssetsTag {
    pub exports: Vec<PortableCharacterRecord>,
}

/// Imports characters from another SWF file.
#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssetsTag {
    /// URL of the source SWF file.
    pub url: String,

    pub imports: Vec<PortableCharacterRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PortableCharacterRecord {
    pub character_id: u16,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnableDebuggerTag {
    pub password_md5: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnableDebugger2Tag {
    pub password_md5: Vec<u8>,
}

/// Overrides the default limits for AVM scripts.
#[derive(Clone, PartialEq, Debug)]
pub struct ScriptLimitsTag {
    pub max_recursion_depth: u16,
    pub script_timeout_seconds: u16,
}

/// Sets the tab ordering of the character at the specified depth.
#[derive(Clone, PartialEq, Debug)]
pub struct SetTabIndexTag {
    pub depth: u16,
    pub tab_index: u16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssets2Tag {
    pub url: String,
    pub imports: Vec<PortableCharacterRecord>,
}

/// Creates associations between characters in the SWF file and ActionScript 3
/// classes.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassTag {
    pub records: Vec<SymbolClassRecord>,
}

/// Associates an ActionScript 3 class with a character.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassRecord {
    /// The character ID to be associated.
    pub character_id: u16,

    /// The fully-qualified name of the ActionScript 3 class to be associated.
    pub class_name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MetadataTag {
    pub metadata: String,
}

/// Defines a 9-slice grid that should be applied when scaling the specified
/// character.
#[derive(Clone, PartialEq, Debug)]
pub struct DefineScalingGridTag {
    pub character_id: u16,
    pub splitter: Rectangle,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSceneAndFrameLabelDataTag {
    pub scenes: Vec<SceneRecord>,
    pub frame_labels: Vec<FrameLabelRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SceneRecord {
    pub offset: u32,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelRecord {
    pub frame_num: u32,
    pub frame_label: String,
}
