use super::common::*;

/// Sets the background color of the display.
pub struct SetBackgroundColorTag {
    color: Rgb
}

/// Labels the current frame with the specified name.
pub struct FrameLabelTag {
    name: String,

    named_anchor: bool,
}

pub struct ProtectTag {
    password_md5: Vec<u8>
}

/// Marks the end of a SWF file or of a sprite definition.
pub struct EndTag {}

/// Makes portions of the SWF file available for import by other SWF files
/// using `ImportAssetsTag`.
pub struct ExportAssetsTag {
    pub exports: Vec<PortableCharacterRecord>
}

/// Imports characters from another SWF file.
pub struct ImportAssetsTag {
    /// URL of the source SWF file.
    pub url: String,

    pub imports: Vec<PortableCharacterRecord>,
}

pub struct PortableCharacterRecord {
    character_id: u16,
    name: String,
}

pub struct EnableDebuggerTag {
    password_md5: Vec<u8>
}

pub struct EnableDebugger2Tag {
    password_md5: Vec<u8>
}

/// Overrides the default limits for AVM scripts.
pub struct ScriptLimitsTag {
    max_recursion_depth: u16,
    script_timeout_seconds: u16,
}

/// Sets the tab ordering of the character at the specified depth.
pub struct SetTabIndexTag {
    depth: u16,
    tab_index: u16,
}

pub struct FileAttributesTag {
    pub flags: FileAttributesFlag
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