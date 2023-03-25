use crate::decode::tags::buttons::button_record_2::ButtonRecord2;
use crate::decode::tags::buttons::ButtonConditionActionList;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButton2Tag {
    pub button_id: u16,
    pub track_as_menu: bool,
    pub characters: Vec<ButtonRecord2>,
    pub actions: Vec<ButtonConditionActionList>,
}
