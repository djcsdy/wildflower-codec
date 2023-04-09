use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

bitflags! {
    pub struct DefineButton2Flags: u8 {
        const RESERVED = 0xfe;
        const TRACK_AS_MENU = 0x01;
    }
}

impl DefineButton2Flags {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(DefineButton2Flags::from_bits(reader.read_u8()?).unwrap())
    }
}
