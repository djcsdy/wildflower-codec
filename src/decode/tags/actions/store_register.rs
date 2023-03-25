use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct StoreRegister {
    pub register_number: u8,
}

impl StoreRegister {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let register_number = reader.read_u8()?;
        Ok(Self { register_number })
    }
}
