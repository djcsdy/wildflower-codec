use crate::decode::tags::common::string;

#[derive(Clone, PartialEq, Debug)]
pub struct GoToLabel {
    pub label: string::String,
}
