use crate::decode::bit_read::BitRead;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TextRenderer {
    Standard = 0,
    Advanced = 1,
}

impl TextRenderer {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        Self::try_from(reader.read_ub8(2)?).map_err(|_| Error::from(InvalidData))
    }
}
