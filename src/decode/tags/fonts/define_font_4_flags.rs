use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

bitflags! {
    pub struct DefineFont4Flags: u8 {
        const RESERVED = 0xf8;
        const HAS_FONT_DATA = 0x04;
        const ITALIC = 0x02;
        const BOLD = 0x01;
    }
}

impl DefineFont4Flags {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(DefineFont4Flags::from_bits(reader.read_u8()?).unwrap())
    }
}
