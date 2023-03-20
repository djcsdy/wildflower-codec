use super::common::*;
use half::f16;

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

#[derive(Clone, PartialEq, Debug)]
pub struct AlignZoneRecord {
    pub zone_data: Vec<ZoneData>,
    pub zone_mask_y: bool,
    pub zone_mask_x: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ZoneData {
    pub alignment_coordinate: f16,
    pub range: f16,
}

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
