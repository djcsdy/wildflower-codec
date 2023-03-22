use crate::decode::tags::common::String;

#[derive(Clone, PartialEq, Debug)]
pub struct DoAbcTag {
    pub flags: u32,
    pub name: String,
    pub abc_data: Vec<u8>,
}
