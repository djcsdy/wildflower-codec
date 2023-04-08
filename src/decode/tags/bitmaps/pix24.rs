use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::rgb::Rgb;
use std::io::Result;

pub fn read_pix24(reader: &mut SwfSliceReader) -> Result<Rgb> {
    reader.read_u8()?;
    let red = reader.read_u8()?;
    let green = reader.read_u8()?;
    let blue = reader.read_u8()?;
    Ok(Rgb { red, green, blue })
}
