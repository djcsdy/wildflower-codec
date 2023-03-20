use super::common::*;

pub mod align_zone_record;
pub mod code_table;
pub mod code_table_and_layout;
pub mod csm_table_hint;
pub mod define_font;
pub mod define_font_2;
pub mod define_font_3;
pub mod define_font_align_zones;
pub mod define_font_flags;
pub mod define_font_info;
pub mod define_font_info_2;
pub mod define_font_info_flags;
pub mod glyph_shape_table;
pub mod glyphs_and_code_table_and_layout;
pub mod kerning;
pub mod language_code;
pub mod layout;
pub mod zone_data;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontNameTag {
    pub font_id: u16,
    pub font_name: String,
    pub font_copyright: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFont4Tag {
    pub font_id: u16,
    pub italic: bool,
    pub bold: bool,
    pub font_name: String,
    pub font_data: Vec<u8>,
}
