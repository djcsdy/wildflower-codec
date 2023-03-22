use crate::decode::tags::common::String;
use crate::decode::tags::fonts::define_font_4_flags::DefineFont4Flags;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont4Tag {
    pub font_id: u16,
    pub flags: DefineFont4Flags,
    pub font_name: String,
    pub font_data: Vec<u8>,
}
