use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct SetTarget {
    pub target_name: String,
}
