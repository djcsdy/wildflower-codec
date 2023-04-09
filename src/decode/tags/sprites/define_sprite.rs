use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::sprites::control_tag_list::ControlTagList;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSpriteTag {
    pub sprite_id: u16,
    pub frame_count: u16,
    pub control_tags: ControlTagList<Vec<u8>>,
}

impl DefineSpriteTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let sprite_id = reader.read_u16()?;
        let frame_count = reader.read_u16()?;
        let control_tags = ControlTagList::read_to_end(reader)?;
        Ok(Self {
            sprite_id,
            frame_count,
            control_tags,
        })
    }
}
