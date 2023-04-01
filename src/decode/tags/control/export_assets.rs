use crate::decode::tags::control::portable_character_record::PortableCharacterRecord;

/// Makes portions of the SWF file available for import by other SWF files
/// using `ImportAssetsTag`.
#[derive(Clone, PartialEq, Debug)]
pub struct ExportAssetsTag {
    pub exports: Vec<PortableCharacterRecord>,
}
