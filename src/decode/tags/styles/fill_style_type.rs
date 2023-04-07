use crate::decode::read_ext::SwfTypesReadExt;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum FillStyleType {
    Solid = 0x00,
    LinearGradient = 0x10,
    RadialGradient = 0x12,
    FocalRadialGradient = 0x13,
    RepeatingBitmap = 0x40,
    ClippedBitmap = 0x41,
    NonSmoothedRepeatingBitmap = 0x42,
    NonSmoothedClippedBitmap = 0x43,
}

pub(crate) fn read_fill_style_type<R: Read>(reader: &mut R) -> Result<FillStyleType> {
    FillStyleType::read(reader)
}

impl FillStyleType {
    pub(crate) fn read<R: Read>(reader: &mut R) -> Result<FillStyleType> {
        reader
            .read_u8()?
            .try_into()
            .map_err(|_| Error::from(InvalidData))
    }
}
