use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub enum RegisterParam {
    Register(u8),
    Name(String),
}
