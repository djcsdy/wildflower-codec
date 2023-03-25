use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::actions::register_param::RegisterParam;

#[derive(Clone, PartialEq, Debug)]
pub struct Try {
    pub catch_parameter: RegisterParam,
    pub try_body: ActionList<Vec<u8>>,
    pub catch_body: Option<ActionList<Vec<u8>>>,
    pub finally_body: Option<ActionList<Vec<u8>>>,
}
