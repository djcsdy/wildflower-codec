use crate::decode::tags::metadata::FileAttributesFlags;

#[derive(Clone, PartialEq, Debug)]
pub struct FileAttributesTag {
    pub flags: FileAttributesFlags,
}
