use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct MetadataTag {
    pub metadata: String,
}
