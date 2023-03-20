use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::shapes::read_shape;
use crate::decode::tags::shapes::Shape;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphTable<'define_font, Offset: Copy + Into<usize>> {
    pub swf_version: u8,
    pub offset_table: &'define_font [Offset],
    pub shape_table: &'define_font [u8],
}

impl<'define_font, Offset: Copy + Into<usize>> GlyphTable<'define_font, Offset> {
    pub fn num_glyphs(&self) -> usize {
        self.offset_table.len() - 1
    }

    pub fn iter<'glyph_table>(
        &'glyph_table self,
    ) -> GlyphTableIterator<'glyph_table, 'define_font, Offset> {
        GlyphTableIterator {
            table: self,
            index: 0,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphTableIterator<'glyph_table, 'define_font, Offset: Copy + Into<usize>> {
    pub table: &'glyph_table GlyphTable<'define_font, Offset>,
    pub index: usize,
}

impl<'glyph_table, 'define_font, Offset: Copy + Into<usize>> Iterator
for GlyphTableIterator<'glyph_table, 'define_font, Offset>
{
    type Item = Result<Shape<(), ()>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.table.num_glyphs() {
            None
        } else {
            let start_offset = self.table.offset_table[self.index].into();
            let end_offset = self.table.offset_table[self.index + 1].into();
            self.index += 1;
            Some(read_shape(&mut SwfSliceReader::new(
                &self.table.shape_table[start_offset..end_offset],
                self.table.swf_version,
            )))
        }
    }
}
