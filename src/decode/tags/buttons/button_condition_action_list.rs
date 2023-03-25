use crate::decode::tags::actions::action_list::ActionList;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonConditionActionList {
    pub condition_idle_to_over_down: bool,
    pub condition_out_down_to_idle: bool,
    pub condition_out_down_to_over_down: bool,
    pub condition_over_down_to_out_down: bool,
    pub condition_over_down_to_over_up: bool,
    pub condition_over_up_to_over_down: bool,
    pub condition_over_up_to_idle: bool,
    pub condition_idle_to_over_up: bool,
    pub condition_key_press: u8,
    pub condition_over_down_to_idle: bool,
    pub actions: ActionList<Vec<u8>>,
}
