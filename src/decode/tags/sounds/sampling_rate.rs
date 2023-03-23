use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
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
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(Self::try_from(reader.read_ub8(2)?).unwrap())
    }
}
