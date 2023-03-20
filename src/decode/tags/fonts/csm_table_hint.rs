use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CsmTableHint {
    Thin = 0,
    Medium = 1,
    Thick = 2,
}

impl CsmTableHint {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(Self::try_from(reader.read_ub8(2)?).unwrap())
    }
}
