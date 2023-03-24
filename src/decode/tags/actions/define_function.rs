use crate::decode::tags::actions::ActionRecord;
use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction {
    pub function_name: String,
    pub params: Vec<String>,
    pub body: Vec<ActionRecord>,
}
