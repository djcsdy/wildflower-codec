use crate::decode::tags::actions::action_list::ActionList;

#[derive(Clone, PartialEq, Debug)]
pub struct DoInitActionTag {
    pub sprite_id: u16,
    pub actions: ActionList<Vec<u8>>,
}
