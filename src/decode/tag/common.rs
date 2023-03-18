use crate::ast::common::{Rgb, Rgba};
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

pub fn read_rgb<R: Read>(reader: &mut R) -> Result<Rgb> {
    let mut buf = [0u8; 3];
    reader.read_u8_into(&mut buf)?;
    Ok(Rgb {
        red: buf[0],
        green: buf[1],
        blue: buf[2],
    })
}

pub fn read_rgba<R: Read>(reader: &mut R) -> Result<Rgba> {
    let mut buf = [0u8; 4];
    reader.read_u8_into(&mut buf)?;
    Ok(Rgba {
        red: buf[0],
        green: buf[1],
        blue: buf[2],
        alpha: buf[3],
    })
}

pub fn read_argb<R: Read>(reader: &mut R) -> Result<Rgba> {
    let mut buf = [0u8; 4];
    reader.read_u8_into(&mut buf)?;
    Ok(Rgba {
        red: buf[1],
        green: buf[2],
        blue: buf[3],
        alpha: buf[0],
    })
}
