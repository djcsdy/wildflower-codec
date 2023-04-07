use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct LineStyle<Color> {
    pub width: u16,
    pub color: Color,
}

impl<Color> LineStyle<Color> {
    pub fn read<Read: BitRead, ReadColor: Fn(&mut Read) -> Result<Color>>(
        reader: &mut Read,
        read_color: &ReadColor,
    ) -> Result<Self> {
        let width = reader.read_u16()?;
        let color = read_color(reader)?;
        Ok(Self { width, color })
    }
}
