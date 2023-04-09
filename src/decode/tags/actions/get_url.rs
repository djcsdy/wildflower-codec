use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl {
    pub url: String,
    pub target: String,
}

impl GetUrl {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let url = String::read(reader)?;
        let target = String::read(reader)?;
        Ok(Self { url, target })
    }
}
