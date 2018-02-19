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