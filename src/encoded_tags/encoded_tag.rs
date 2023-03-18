use crate::encoded_tags::tag_type::TagType;

pub struct EncodedTag {
    tag_type: TagType,
    body: Vec<u8>,
}
