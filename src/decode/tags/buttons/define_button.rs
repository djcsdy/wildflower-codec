use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::buttons::button_record::ButtonRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonTag {
    pub button_id: u16,
    pub characters: Vec<ButtonRecord>,
    pub actions: ActionList<Vec<u8>>,
}
