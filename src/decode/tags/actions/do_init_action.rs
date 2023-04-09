use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::actions::action_list::ActionList;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DoInitActionTag {
    pub sprite_id: u16,
    pub actions: ActionList<Vec<u8>>,
}

impl DoInitActionTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let sprite_id = reader.read_u16()?;
        let actions = ActionList::read_to_end(reader)?;
        Ok(Self { sprite_id, actions })
    }
}
