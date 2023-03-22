use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Align {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
}

impl Align {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Self::try_from(reader.read_u8()?).map_err(|_| Error::from(InvalidData))
    }
}
