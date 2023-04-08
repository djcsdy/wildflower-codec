use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::rgba::Rgba;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradientRecord {
    pub start_ratio: u8,
    pub start_color: Rgba,
    pub end_ratio: u8,
    pub end_color: Rgba,
}

impl MorphGradientRecord {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let start_ratio = reader.read_u8()?;
        let start_color = Rgba::read(reader)?;
        let end_ratio = reader.read_u8()?;
        let end_color = Rgba::read(reader)?;
        Ok(Self {
            start_ratio,
            start_color,
            end_ratio,
            end_color,
        })
    }
}
