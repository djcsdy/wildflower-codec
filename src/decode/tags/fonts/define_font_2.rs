use crate::decode::tags::common;
use crate::decode::tags::fonts::define_font_2_flags::DefineFont2Flags;
use crate::decode::tags::fonts::LanguageCode;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont2Tag {
    pub font_id: u16,
    pub flags: DefineFont2Flags,
    pub language_code: Option<LanguageCode>,
    pub font_name: common::String,
    pub num_glyphs: u16,
    pub glyphs_and_layout: Vec<u8>,
}
