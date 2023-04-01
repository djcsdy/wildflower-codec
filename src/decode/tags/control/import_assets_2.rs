use crate::decode::tags::common::string::String;
use crate::decode::tags::control::portable_character_record::PortableCharacterRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssets2Tag {
    pub url: String,
    pub imports: Vec<PortableCharacterRecord>,
}
