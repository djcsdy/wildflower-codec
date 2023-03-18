use crate::encoded_tags::tag_type::TagType;

pub struct EncodedTag {
    tag_type: TagType,
    body: Vec<u8>,
}

impl EncodedTag {
    pub fn new(tag_type: TagType, body: Vec<u8>) -> EncodedTag {
        EncodedTag { tag_type, body }
    }
}
