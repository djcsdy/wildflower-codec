use crate::decode::tags::common::string::String;
use crate::decode::tags::control::PortableCharacterRecord;

/// Imports characters from another SWF file.
#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssetsTag {
    /// URL of the source SWF file.
    pub url: String,

    pub imports: Vec<PortableCharacterRecord>,
}
