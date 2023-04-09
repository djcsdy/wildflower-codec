use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ControlTagList<Buffer: AsRef<[u8]>> {
    pub buffer: Buffer,
}

impl ControlTagList<Vec<u8>> {
    pub fn read_to_end<R: Read>(reader: &mut R) -> Result<Self> {
        let buffer = reader.read_u8_to_end()?;
        Ok(Self { buffer })
    }
}
