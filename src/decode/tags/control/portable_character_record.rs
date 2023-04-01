use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct PortableCharacterRecord {
    pub character_id: u16,
    pub name: String,
}
