use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GoToFrame {
    pub frame: u16,
}

impl GoToFrame {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let frame = reader.read_u16()?;
        Ok(Self { frame })
    }
}
