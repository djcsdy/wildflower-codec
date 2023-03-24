use crate::decode::tags::common::string;

#[derive(Clone, PartialEq, Debug)]
pub struct SetTarget {
    pub target_name: string::String,
}
