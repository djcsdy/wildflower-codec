use crate::decode::tags::fonts::glyph_table::GlyphTable;
use crate::decode::tags::fonts::layout::FontLayout;

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphsAndLayout<'define_font> {
    pub glyph_table: GlyphTable<'define_font>,
    pub code_table_and_layout: CodeTableAndLayout,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CodeTableAndLayout {
    Windows1252 {
        character_codes: Vec<u8>,
        layout: Option<FontLayout<u8>>,
    },
    JisX0201 {
        character_codes: Vec<u8>,
        layout: Option<FontLayout<u8>>,
    },
    ShiftJis {
        character_codes: Vec<u16>,
        layout: Option<FontLayout<u16>>,
    },
    Ucs2 {
        character_codes: Vec<u16>,
        layout: Option<FontLayout<u16>>,
    },
}
