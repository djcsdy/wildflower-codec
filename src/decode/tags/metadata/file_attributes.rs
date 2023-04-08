use crate::decode::tags::metadata::file_attributes_flags::FileAttributesFlags;

#[derive(Clone, PartialEq, Debug)]
pub struct FileAttributesTag {
    pub flags: FileAttributesFlags,
}
