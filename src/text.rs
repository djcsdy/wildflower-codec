use super::shapes::Shape

pub struct DefineFontTag {
    pub font_id: u16,
    pub glyph_shapes: Vec<Shape<(), ()>>
}

pub struct DefineFontInfoTag {
    pub font_id: u16,
    pub font_name: String,
    pub small_text: bool,
    pub italic: bool,
    pub bold: bool,
    pub code_table: CodeTable
}

pub enum CodeTable {
    Windows1252(Vec<u8>),
    JisX0201(Vec<u8>),
    ShiftJis(Vec<u16>),
    Ucs2(Vec<u16>)
}