use crate::decode::tags::buttons::button_condition_action_list::ButtonConditionActionList;
use crate::decode::tags::buttons::button_record_2::ButtonRecord2;
use crate::decode::tags::buttons::define_button_2_flags::DefineButton2Flags;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButton2Tag {
    pub button_id: u16,
    pub flags: DefineButton2Flags,
    pub characters: Vec<ButtonRecord2>,
    pub actions: Vec<ButtonConditionActionList>,
}
