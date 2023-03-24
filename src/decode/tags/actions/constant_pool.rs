use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct ConstantPool {
    pub constant_pool: Vec<String>,
}
