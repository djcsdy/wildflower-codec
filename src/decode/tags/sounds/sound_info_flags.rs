use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

bitflags! {
    pub struct SoundInfoFlags: u8 {
        const RESERVED = 0xc0;
        const SYNC_STOP = 0x20;
        const SYNC_NO_MULTIPLE = 0x10;
        const HAS_ENVELOPE = 0x08;
        const HAS_LOOPS = 0x04;
        const HAS_OUT_POINT = 0x02;
        const HAS_IN_POINT = 0x01;
    }
}

impl SoundInfoFlags {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self::from_bits(reader.read_u8()?).unwrap())
    }
}
