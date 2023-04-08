use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::{Error, Read};
use std::io;
use std::io::ErrorKind::InvalidData;
use crate::decode::read_ext::SwfTypesReadExt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum BitmapFormat {
    ColorMap8 = 3,
    Rgb15 = 4,
    Rgb24 = 5,
}

impl BitmapFormat {
    pub(crate) fn read<R: Read>(reader: &mut R) -> io::Result<BitmapFormat> {
        BitmapFormat::try_from(reader.read_u8()?).map_err(|_| Error::from(InvalidData))
    }
}