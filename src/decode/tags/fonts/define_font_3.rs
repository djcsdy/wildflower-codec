use crate::decode::tags::common::string::String;
use crate::decode::tags::fonts::define_font_2::DefineFont2Tag;
use crate::decode::tags::fonts::define_font_flags::DefineFontFlags;
use crate::decode::tags::fonts::language_code::LanguageCode;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont3Tag {
    pub font_id: u16,
    pub flags: DefineFontFlags,
    pub language_code: LanguageCode,
    pub font_name: String,
    pub num_glyphs: u16,
    pub glyphs_and_code_table_and_layout: Vec<u8>,
}

impl DefineFont3Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        DefineFont2Tag::read(reader).and_then(
            |DefineFont2Tag {
                 font_id,
                 flags,
                 language_code,
                 font_name,
                 num_glyphs,
                 glyphs_and_code_table_and_layout,
             }| {
                language_code
                    .ok_or_else(|| Error::from(InvalidData))
                    .map(|language_code| Self {
                        font_id,
                        flags,
                        language_code,
                        font_name,
                        num_glyphs,
                        glyphs_and_code_table_and_layout,
                    })
            },
        )
    }
}
