use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub enum PushValue {
    String(String),
    Float(f32),
    Null,
    Undefined,
    RegisterNumber(u8),
    Boolean(bool),
    Double(f64),
    Integer(u32),
    Constant(u16),
}
