use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::rgb::Rgb;
use std::io::Result;

pub(crate) fn read_pix15<R: BitRead>(reader: &mut R) -> Result<Rgb> {
    reader.read_bit()?;
    let red = reader.read_ub8(5)? << 3;
    let green = reader.read_ub8(5)? << 3;
    let blue = reader.read_ub8(5)? << 3;
    Ok(Rgb { red, green, blue })
}
