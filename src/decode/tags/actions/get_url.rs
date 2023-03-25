use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl {
    pub url: String,
    pub target: String,
}

impl GetUrl {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let url = String::read(reader)?;
        let target = String::read(reader)?;
        Ok(Self { url, target })
    }
}
