use crate::decode::tags::common::String;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont4Tag {
    pub font_id: u16,
    pub italic: bool,
    pub bold: bool,
    pub font_name: String,
    pub font_data: Vec<u8>,
}
