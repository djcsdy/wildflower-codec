use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct Jump {
    pub offset: i16,
}

impl Jump {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let offset = reader.read_i16()?;
        Ok(Self { offset })
    }
}
