use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct EnableDebugger2Tag {
    pub password_md5: Vec<u8>,
}

impl EnableDebugger2Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        reader.read_u16()?;
        let password_md5 = reader.read_u8_until_null()?;
        Ok(Self { password_md5 })
    }
}
