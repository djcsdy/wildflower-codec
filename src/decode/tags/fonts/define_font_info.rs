use crate::decode::tags::common::String;
use crate::decode::tags::fonts::CodeTable;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontInfoTag {
    pub font_id: u16,
    pub font_name: String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub code_table: CodeTable,
}
