use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::metadata::file_attributes_flags::FileAttributesFlags;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct FileAttributesTag {
    pub flags: FileAttributesFlags,
}

impl FileAttributesTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let flags = FileAttributesFlags::from_bits_truncate(reader.read_u32()?);
        Ok(Self { flags })
    }
}
