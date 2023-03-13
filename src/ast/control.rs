use super::common::*;

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
pub struct EndTag {}

/// Makes portions of the SWF file available for import by other SWF files
/// using `ImportAssetsTag`.
pub struct ExportAssetsTag {
    pub exports: Vec<PortableCharacterRecord>,
}

/// Imports characters from another SWF file.
pub struct ImportAssetsTag {
    /// URL of the source SWF file.
    pub url: String,

    pub imports: Vec<PortableCharacterRecord>,
}

pub struct PortableCharacterRecord {
    pub character_id: u16,
    pub name: String,
}

pub struct EnableDebuggerTag {
    pub password_md5: Vec<u8>,
}

pub struct EnableDebugger2Tag {
    pub password_md5: Vec<u8>,
}

/// Overrides the default limits for AVM scripts.
pub struct ScriptLimitsTag {
    pub max_recursion_depth: u16,
    pub script_timeout_seconds: u16,
}

/// Sets the tab ordering of the character at the specified depth.
pub struct SetTabIndexTag {
    pub depth: u16,
    pub tab_index: u16,
}

pub struct FileAttributesTag {
    pub flags: FileAttributesFlags,
}

bitflags! {
    pub struct FileAttributesFlags: u32 {
        const USE_DIRECT_BLIT = 0x4000_0000;
        const USE_GPU = 0x2000_0000;
        const HAS_METADATA = 0x1000_0000;
        const ACTION_SCRIPT_3 = 0x0800_0000;
        const USE_NETWORK = 0x0100_0000;
        const RESERVED = 0x86ff_ffff;
    }
}

pub struct ImportAssets2Tag {
    pub url: String,
    pub imports: Vec<PortableCharacterRecord>,
}

/// Creates associations between characters in the SWF file and ActionScript 3
/// classes.
pub struct SymbolClassTag {
    pub records: Vec<SymbolClassRecord>,
}

/// Associates an ActionScript 3 class with a character.
pub struct SymbolClassRecord {
    /// The character ID to be associated.
    pub character_id: u16,

    /// The fully-qualified name of the ActionScript 3 class to be associated.
    pub class_name: String,
}

pub struct MetadataTag {
    pub metadata: String,
}

/// Defines a 9-slice grid that should be applied when scaling the specified
/// character.
pub struct DefineScalingGridTag {
    pub character_id: u16,
    pub splitter: Rectangle,
}

pub struct DefineSceneAndFrameLabelDataTag {
    pub scenes: Vec<SceneRecord>,
    pub frame_labels: Vec<FrameLabelRecord>,
}

pub struct SceneRecord {
    pub offset: u32,
    pub name: String,
}

pub struct FrameLabelRecord {
    pub frame_num: u32,
    pub frame_label: String,
}
