use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::rgb::Rgb;
use std::io::{Read, Result};

pub fn read_pix24<R: Read>(reader: &mut R) -> Result<Rgb> {
    reader.read_u8()?;
    let red = reader.read_u8()?;
    let green = reader.read_u8()?;
    let blue = reader.read_u8()?;
    Ok(Rgb { red, green, blue })
}
