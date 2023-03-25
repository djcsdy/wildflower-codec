use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct Jump {
    pub offset: i16,
}

impl Jump {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let offset = reader.read_i16()?;
        Ok(Self { offset })
    }
}
