use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::buttons::button_condition::ButtonCondition;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonConditionActionList {
    pub condition: ButtonCondition,
    pub actions: ActionList<Vec<u8>>,
}
