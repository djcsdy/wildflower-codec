use crate::decode::tags::common::String;
use crate::decode::tags::fonts::language_code::LanguageCode;
use crate::decode::tags::fonts::CodeTable;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontInfo2Tag {
    pub font_id: u16,
    pub font_name: String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub language_code: LanguageCode,
    pub code_table: CodeTable,
}
