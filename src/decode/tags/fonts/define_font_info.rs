use crate::decode::tags::common;
use crate::decode::tags::fonts::CodeTable;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontInfoTag {
    pub font_id: u16,
    pub font_name: common::String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub code_table: CodeTable,
}
