use crate::decode::read_ext::SwfTypesReadExt;
use std::io;
use std::io::Read;

#[derive(Clone, PartialEq, Debug)]
pub struct EnableDebuggerTag {
    pub password_md5: Vec<u8>,
}

impl EnableDebuggerTag {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<EnableDebuggerTag> {
        let password_md5 = reader.read_u8_until_null()?;
        Ok(EnableDebuggerTag { password_md5 })
    }
}
