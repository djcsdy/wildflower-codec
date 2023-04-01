use crate::decode::tags::common::rgb::Rgb;
use std::io::{Read, Result};

/// Sets the background color of the display.
#[derive(Clone, PartialEq, Debug)]
pub struct SetBackgroundColorTag {
    pub color: Rgb,
}

impl SetBackgroundColorTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let color = Rgb::read(reader)?;
        Ok(Self { color })
    }
}
