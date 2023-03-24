use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct GoToLabel {
    pub label: String,
}
