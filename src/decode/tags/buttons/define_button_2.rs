use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_read::SliceRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::buttons::button_condition_action_list::ButtonConditionActionList;
use crate::decode::tags::buttons::button_record_2::ButtonRecord2;
use crate::decode::tags::buttons::define_button_2_flags::DefineButton2Flags;
use std::io::Result;
use std::mem::size_of;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButton2Tag {
    pub button_id: u16,
    pub flags: DefineButton2Flags,
    pub characters: Vec<ButtonRecord2>,
    pub actions: Vec<ButtonConditionActionList>,
}

impl DefineButton2Tag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let button_id = reader.read_u16()?;
        let flags = DefineButton2Flags::read(reader)?;
        let action_offset = reader.read_u16()? as usize - size_of::<u16>();
        let characters = ButtonRecord2::read_list(&mut reader.slice(action_offset))?;
        let actions = ButtonConditionActionList::read_list(reader)?;
        Ok(Self {
            button_id,
            flags,
            characters,
            actions,
        })
    }
}
