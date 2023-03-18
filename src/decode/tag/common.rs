use crate::ast::common::Rgb;
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
