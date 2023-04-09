use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

bitflags! {
    pub struct AlignZoneFlags: u8 {
        const RESERVED = 0xfc;
        const ZONE_MASK_Y = 0x02;
        const ZONE_MASK_X = 0x01;
    }
}

impl AlignZoneFlags {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self::from_bits(reader.read_u8()?).unwrap())
    }
}
