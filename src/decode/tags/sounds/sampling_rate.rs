use crate::decode::bit_read::BitRead;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SamplingRate {
    Khz5 = 0,
    Khz11 = 1,
    Khz22 = 2,
    Khz44 = 3,
}

impl SamplingRate {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        Ok(Self::try_from(reader.read_ub8(2)?).unwrap())
    }
}
