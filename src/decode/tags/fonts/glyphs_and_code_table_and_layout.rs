use crate::decode::tags::fonts::code_table_and_layout::CodeTableAndLayout;
use crate::decode::tags::fonts::glyph_shape_table::GlyphShapeTable;

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphsAndCodeTableAndLayout<'define_font> {
    pub shape_table: GlyphShapeTable<'define_font>,
    pub code_table_and_layout: CodeTableAndLayout,
}
