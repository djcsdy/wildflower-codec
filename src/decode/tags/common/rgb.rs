use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

/// An RGB color.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 3];
        reader.read_u8_into(&mut buf)?;
        Ok(Self {
            red: buf[0],
            green: buf[1],
            blue: buf[2],
        })
    }
}
