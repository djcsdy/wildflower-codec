use crate::decode::tags::fonts::glyph_table::GlyphTable;
use crate::decode::tags::fonts::layout::FontLayout;

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphsAndLayout<'define_font, Offset: Copy + Into<usize>, CharacterCode> {
    pub glyph_table: GlyphTable<'define_font, Offset>,
    pub code_table: Vec<CharacterCode>,
    pub layout: Option<FontLayout<CharacterCode>>,
}
