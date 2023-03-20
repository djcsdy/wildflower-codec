use crate::decode::tags::common;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontNameTag {
    pub font_id: u16,
    pub font_name: common::String,
    pub font_copyright: common::String,
}
