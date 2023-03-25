use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::buttons::button_record::ButtonRecord;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonTag {
    pub button_id: u16,
    pub characters: Vec<ButtonRecord>,
    pub actions: ActionList<Vec<u8>>,
}

impl DefineButtonTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let button_id = reader.read_u16()?;
        let characters = ButtonRecord::read_list(reader)?;
        let actions = ActionList::read_to_end(reader)?;
        Ok(Self {
            button_id,
            characters,
            actions,
        })
    }
}
