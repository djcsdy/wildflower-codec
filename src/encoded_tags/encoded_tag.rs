use crate::encoded_tags::tag_type::TagType;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct EncodedTag {
    swf_version: u8,
    tag_type: TagType,
    body: Vec<u8>,
}

impl EncodedTag {
    pub fn new(swf_version: u8, tag_type: TagType, body: Vec<u8>) -> EncodedTag {
        EncodedTag {
            swf_version,
            tag_type,
            body,
        }
    }
}
