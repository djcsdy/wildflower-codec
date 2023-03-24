use crate::decode::tags::actions::ActionRecord;
use crate::decode::tags::buttons::button_record::ButtonRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonTag {
    pub button_id: u16,
    pub characters: Vec<ButtonRecord>,
    pub actions: Vec<ActionRecord>,
}
