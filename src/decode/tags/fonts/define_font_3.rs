use crate::decode::tags::common;
use crate::decode::tags::common::Rectangle;
use crate::decode::tags::fonts::define_font_2_flags::DefineFont2Flags;
use crate::decode::tags::fonts::language_code::LanguageCode;
use crate::decode::tags::fonts::CodeTableWithKernings;
use crate::decode::tags::shapes::Shape;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont3Tag {
    pub font_id: u16,
    pub flags: DefineFont2Flags,
    pub language_code: LanguageCode,
    pub font_name: common::String,
    pub glyph_shapes: Vec<Shape<(), ()>>,
    pub code_table: CodeTableWithKernings,
    pub ascent: u16,
    pub descent: u16,
    pub leading: i16,
    pub advances: Vec<i16>,
    pub bounds: Vec<Rectangle>,
}
