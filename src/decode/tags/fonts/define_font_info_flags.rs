use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

bitflags! {
    pub struct DefineFontInfoFlags: u8 {
        const RESERVED = 0xc0;
        const SMALL_TEXT = 0x20;
        const SHIFT_JIS = 0x10;
        const ANSI = 0x08;
        const ITALIC = 0x04;
        const BOLD = 0x02;
        const WIDE_CODES = 0x01;
    }
}

impl DefineFontInfoFlags {
    pub fn read<R: Read>(reader: &mut R) -> Result<DefineFontInfoFlags> {
        Ok(DefineFontInfoFlags::from_bits(reader.read_u8()?).unwrap())
    }
}
