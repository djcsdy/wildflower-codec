use super::common::*;
use super::shapes::Shape;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontTag {
    pub font_id: u16,
    pub glyph_shapes: Vec<Shape<(), ()>>,
}

pub struct DefineFont2Tag {
    pub font_id: u16,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub language_code: Option<LanguageCode>,
    pub font_name: String,
    pub glyph_shapes: Vec<Shape<(), ()>>,
    pub code_table: CodeTableWithKernings,
    pub ascent: u16,
    pub descent: u16,
    pub leading: i16,
    pub advances: Vec<i16>,
    pub bounds: Vec<Rectangle>,
}

pub struct DefineFont3Tag {
    pub font_id: u16,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub language_code: LanguageCode,
    pub font_name: String,
    pub glyph_shapes: Vec<Shape<(), ()>>,
    pub code_table: CodeTableWithKernings,
    pub ascent: u16,
    pub descent: u16,
    pub leading: i16,
    pub advances: Vec<i16>,
    pub bounds: Vec<Rectangle>,
}

pub struct DefineFontInfoTag {
    pub font_id: u16,
    pub font_name: String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub code_table: CodeTable,
}

pub struct DefineFontInfo2Tag {
    pub font_id: u16,
    pub font_name: String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub language_code: LanguageCode,
    pub code_table: CodeTable,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LanguageCode {
    Latin = 1,
    Japanese = 2,
    Korean = 3,
    SimplifiedChinese = 4,
    TraditionalChinese = 5,
}

pub enum CodeTable {
    Windows1252(Vec<u8>),
    JisX0201(Vec<u8>),
    ShiftJis(Vec<u16>),
    Ucs2(Vec<u16>),
}

pub enum CodeTableWithKernings {
    Windows1252 {
        character_codes: Vec<u8>,
        kernings: Vec<Kerning<u8>>,
    },
    JisX0201 {
        character_codes: Vec<u8>,
        kernings: Vec<Kerning<u8>>,
    },
    ShiftJis {
        character_codes: Vec<u16>,
        kernings: Vec<Kerning<u16>>,
    },
    Ucs2 {
        character_codes: Vec<u16>,
        kernings: Vec<Kerning<u16>>,
    },
}

pub struct Kerning<T> {
    pub left_character_code: T,
    pub right_character_code: T,
    pub kerning_adjustment: i16,
}

pub struct DefineFontAlignZonesTag {
    pub font_id: u16,
    pub csm_table_hint: CsmTableHint,
    pub zones: Vec<AlignZoneRecord>,
}

pub enum CsmTableHint {
    Thin = 0,
    Medium = 1,
    Thick = 2,
}

pub struct AlignZoneRecord {
    pub zone_data: Vec<ZoneData>,
    pub zone_mask_y: bool,
    pub zone_mask_x: bool,
}

pub struct ZoneData {
    pub alignment_coordinate: Float16,
    pub range: Float16,
}

pub struct DefineFontNameTag {
    pub font_id: u16,
    pub font_name: String,
    pub font_copyright: String,
}

pub struct DefineFont4Tag {
    pub font_id: u16,
    pub italic: bool,
    pub bold: bool,
    pub font_name: String,
    pub font_data: Vec<u8>,
}
