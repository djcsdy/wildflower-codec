use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::rectangle::Rectangle;
use std::io::Result;

/// Defines a 9-slice grid that should be applied when scaling the specified
/// character.
#[derive(Clone, PartialEq, Debug)]
pub struct DefineScalingGridTag {
    pub character_id: u16,
    pub splitter: Rectangle,
}

impl DefineScalingGridTag {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let splitter = Rectangle::read(reader)?;
        Ok(Self {
            character_id,
            splitter,
        })
    }
}
