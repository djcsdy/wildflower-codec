use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GoToLabel {
    pub label: String,
}

impl GoToLabel {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let label = String::read(reader)?;
        Ok(Self { label })
    }
}
