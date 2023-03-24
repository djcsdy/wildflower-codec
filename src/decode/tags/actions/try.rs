use crate::decode::tags::actions::register_param::RegisterParam;
use crate::decode::tags::actions::ActionRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct Try {
    pub catch_parameter: RegisterParam,
    pub try_body: Vec<ActionRecord>,
    pub catch_body: Option<Vec<ActionRecord>>,
    pub finally_body: Option<Vec<ActionRecord>>,
}
