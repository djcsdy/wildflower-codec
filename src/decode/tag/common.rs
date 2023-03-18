use crate::ast::common::{Rectangle, Rgb, Rgba};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
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

pub fn read_rectangle<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Rectangle> {
    reader.align_byte();
    let bits = reader.read_ub8(5)?;
    let x_min = reader.read_sb(bits)?;
    let x_max = reader.read_sb(bits)?;
    let y_min = reader.read_sb(bits)?;
    let y_max = reader.read_sb(bits)?;
    Ok(Rectangle {
        x_min,
        x_max,
        y_min,
        y_max,
    })
}
