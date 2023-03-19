use crate::decode::tags::common;
use crate::decode::tags::fonts::{DefineFont2Flags, LanguageCode};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont2Tag {
    pub font_id: u16,
    pub flags: DefineFont2Flags,
    pub language_code: Option<LanguageCode>,
    pub font_name: common::String,
    pub num_glyphs: u16,
    pub glyphs_and_layout: Vec<u8>,
}
