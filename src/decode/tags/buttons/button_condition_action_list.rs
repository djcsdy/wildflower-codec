use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_read::SliceRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::buttons::button_condition::ButtonCondition;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonConditionActionList {
    pub condition: ButtonCondition,
    pub actions: ActionList<Vec<u8>>,
}

impl ButtonConditionActionList {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Option<Self>> {
        let size = reader.read_u16()?;
        if size == 0 {
            Ok(None)
        } else {
            let mut action_list_reader = reader.slice(size as usize);
            let condition = ButtonCondition::read(&mut action_list_reader)?;
            let actions = ActionList::read_to_end(&mut action_list_reader)?;
            Ok(Some(Self { condition, actions }))
        }
    }

    pub fn read_list(reader: &mut SwfSliceReader) -> Result<Vec<Self>> {
        let mut actions = Vec::new();
        while let Some(action) = Self::read(reader)? {
            actions.push(action);
        }
        Ok(actions)
    }
}
