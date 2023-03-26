use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

pub struct ControlTagList<Buffer: AsRef<[u8]>> {
    pub buffer: Buffer,
}

impl ControlTagList<Vec<u8>> {
    pub fn read_to_end(reader: &mut SwfSliceReader) -> Result<Self> {
        let buffer = reader.read_u8_to_end()?;
        Ok(Self { buffer })
    }
}
