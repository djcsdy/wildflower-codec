use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct JpegTablesTag {
    pub jpeg_data: Vec<u8>,
}

impl JpegTablesTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let jpeg_data = reader.read_u8_to_end()?;
        Ok(Self { jpeg_data })
    }
}
