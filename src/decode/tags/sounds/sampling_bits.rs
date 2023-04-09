use crate::decode::bit_read::BitRead;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SamplingBits {
    Bits8 = 0,
    Bits16 = 1,
}

impl SamplingBits {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        Ok(Self::try_from(reader.read_ub8(1)?).unwrap())
    }
}
