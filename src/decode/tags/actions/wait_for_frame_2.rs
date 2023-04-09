use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct WaitForFrame2 {
    pub skip_count: u8,
}

impl WaitForFrame2 {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let skip_count = reader.read_u8()?;
        Ok(Self { skip_count })
    }
}
