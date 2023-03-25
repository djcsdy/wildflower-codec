use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct SetTarget {
    pub target_name: String,
}

impl SetTarget {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let target_name = String::read(reader)?;
        Ok(Self { target_name })
    }
}
