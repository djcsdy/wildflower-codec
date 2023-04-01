use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct SceneRecord {
    pub offset: u32,
    pub name: String,
}
