use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct SetTarget {
    pub target_name: String,
}

impl SetTarget {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let target_name = String::read(reader)?;
        Ok(Self { target_name })
    }
}
