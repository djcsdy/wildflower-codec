use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

bitflags! {
    pub struct ButtonRecordFlags: u8 {
        const RESERVED = 0xc0;
        const HAS_BLEND_MODE = 0x20;
        const HAS_FILTER_LIST = 0x10;
        const STATE_HIT_TEST = 0x08;
        const STATE_DOWN = 0x04;
        const STATE_OVER = 0x02;
        const STATE_UP = 0x01;
    }
}

impl ButtonRecordFlags {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(Self::from_bits(reader.read_u8()?).unwrap())
    }
}
