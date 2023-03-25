use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct WaitForFrame {
    pub frame: u16,
    pub skip_count: u8,
}

impl WaitForFrame {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let frame = reader.read_u16()?;
        let skip_count = reader.read_u8()?;
        Ok(Self { frame, skip_count })
    }
}
