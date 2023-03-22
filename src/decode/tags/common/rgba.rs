use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

/// An RGB color with an alpha component.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_u8_into(&mut buf)?;
        Ok(Self {
            red: buf[0],
            green: buf[1],
            blue: buf[2],
            alpha: buf[3],
        })
    }

    pub fn read_argb<R: Read>(reader: &mut R) -> Result<Rgba> {
        let mut buf = [0u8; 4];
        reader.read_u8_into(&mut buf)?;
        Ok(Self {
            red: buf[1],
            green: buf[2],
            blue: buf[3],
            alpha: buf[0],
        })
    }
}
