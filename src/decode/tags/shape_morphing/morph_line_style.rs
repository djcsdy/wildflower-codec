use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::rgba::Rgba;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle {
    pub start_width: u16,
    pub end_width: u16,
    pub start_color: Rgba,
    pub end_color: Rgba,
}

impl MorphLineStyle {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let start_width = reader.read_u16()?;
        let end_width = reader.read_u16()?;
        let start_color = Rgba::read(reader)?;
        let end_color = Rgba::read(reader)?;
        Ok(Self {
            start_width,
            end_width,
            start_color,
            end_color,
        })
    }
}
