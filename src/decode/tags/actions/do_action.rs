use crate::decode::tags::actions::action_list::ActionList;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DoActionTag {
    pub actions: ActionList<Vec<u8>>,
}

impl DoActionTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let actions = ActionList::read_to_end(reader)?;
        Ok(Self { actions })
    }
}
