use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct WaitForFrame {
    pub frame: u16,
    pub skip_count: u8,
}

impl WaitForFrame {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let frame = reader.read_u16()?;
        let skip_count = reader.read_u8()?;
        Ok(Self { frame, skip_count })
    }
}
