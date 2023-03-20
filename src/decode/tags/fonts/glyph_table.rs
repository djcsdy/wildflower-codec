pub struct GlyphTable<'define_font, Offset: Copy + Into<usize>> {
    pub swf_version: u8,
    pub offset_table: &'define_font [Offset],
    pub shape_table: &'define_font [u8],
    pub index: usize,
}
