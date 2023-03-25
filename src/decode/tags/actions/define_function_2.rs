use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::actions::register_param::RegisterParam;
use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction2 {
    pub function_name: String,
    pub register_count: u8,
    pub preload_parent: bool,
    pub preload_root: bool,
    pub suppress_super: bool,
    pub preload_super: bool,
    pub suppress_arguments: bool,
    pub preload_arguments: bool,
    pub suppress_this: bool,
    pub preload_this: bool,
    pub preload_global: bool,
    pub parameters: Vec<RegisterParam>,
    pub body: ActionList<Vec<u8>>,
}
