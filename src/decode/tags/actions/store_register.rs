use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct StoreRegister {
    pub register_number: u8,
}

impl StoreRegister {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let register_number = reader.read_u8()?;
        Ok(Self { register_number })
    }
}
