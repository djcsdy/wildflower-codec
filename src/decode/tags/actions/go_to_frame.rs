use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct GoToFrame {
    pub frame: u16,
}

impl GoToFrame {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let frame = reader.read_u16()?;
        Ok(Self { frame })
    }
}
