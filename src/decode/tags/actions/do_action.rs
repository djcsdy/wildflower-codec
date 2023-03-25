use crate::decode::tags::actions::action_list::ActionList;

#[derive(Clone, PartialEq, Debug)]
pub struct DoActionTag {
    pub actions: ActionList<Vec<u8>>,
}
