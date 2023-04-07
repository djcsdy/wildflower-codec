use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelRecord {
    pub frame_num: u32,
    pub frame_label: String,
}

impl FrameLabelRecord {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let frame_num = reader.read_encoded_u32()?;
        let frame_label = String::read(reader)?;
        Ok(Self {
            frame_num,
            frame_label,
        })
    }
}
