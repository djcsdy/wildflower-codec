use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct GoToLabel {
    pub label: String,
}

impl GoToLabel {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let label = String::read(reader)?;
        Ok(Self { label })
    }
}
