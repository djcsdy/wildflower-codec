use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct MetadataTag {
    pub metadata: String,
}

impl MetadataTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let metadata = String::read(reader)?;
        Ok(Self { metadata })
    }
}
