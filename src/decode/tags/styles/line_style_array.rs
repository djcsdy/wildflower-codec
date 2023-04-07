use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::Result;

pub fn read_line_style_array<
    Read: BitRead,
    LineStyle,
    ReadLineStyle: Fn(&mut Read) -> Result<LineStyle>,
>(
    reader: &mut Read,
    read_line_style: &ReadLineStyle,
) -> Result<Vec<LineStyle>> {
    let mut count = reader.read_u8()? as u16;
    if count == 0xff {
        count = reader.read_u16()?
    };
    let mut line_styles = Vec::with_capacity(count as usize);
    for _ in 0..count {
        line_styles.push(read_line_style(reader)?);
    }
    Ok(line_styles)
}
