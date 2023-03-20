use crate::decode::tags::common;
use crate::decode::tags::fonts::define_font_flags::DefineFontFlags;
use crate::decode::tags::fonts::language_code::LanguageCode;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont3Tag {
    pub font_id: u16,
    pub flags: DefineFontFlags,
    pub language_code: LanguageCode,
    pub font_name: common::String,
    pub num_glyphs: u16,
    pub glyphs_and_code_table_and_layout: Vec<u8>,
}
