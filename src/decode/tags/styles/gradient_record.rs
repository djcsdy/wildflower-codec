use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct GradientRecord<Color> {
    pub ratio: u8,
    pub color: Color,
}

impl<Color> GradientRecord<Color> {
    pub fn read<R: Read, ReadColor: Fn(&mut R) -> Result<Color>>(
        reader: &mut R,
        read_color: &ReadColor,
    ) -> Result<Self> {
        let ratio = reader.read_u8()?;
        let color = read_color(reader)?;
        Ok(Self { ratio, color })
    }
}
