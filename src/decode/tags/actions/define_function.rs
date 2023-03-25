use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction {
    pub function_name: String,
    pub params: Vec<String>,
    pub body: ActionList<Vec<u8>>,
}
