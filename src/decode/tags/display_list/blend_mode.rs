use crate::decode::read_ext::SwfTypesReadExt;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BlendMode {
    #[num_enum(alternatives = [0])]
    Normal = 1,
    Layer = 2,
    Multiply = 3,
    Screen = 4,
    Lighten = 5,
    Darken = 6,
    Difference = 7,
    Add = 8,
    Subtract = 9,
    Invert = 10,
    Alpha = 11,
    Erase = 12,
    Overlay = 13,
    Hardlight = 14,
}

impl BlendMode {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Self::try_from(reader.read_u8()?).map_err(|_| Error::from(InvalidData))
    }
}
