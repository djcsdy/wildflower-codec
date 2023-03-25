use crate::decode::tags::actions::action_list::ActionList;

#[derive(Clone, PartialEq, Debug)]
pub struct With {
    pub body: ActionList<Vec<u8>>,
}
