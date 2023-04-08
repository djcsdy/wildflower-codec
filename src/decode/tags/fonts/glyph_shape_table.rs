use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::shapes::shape::Shape;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphShapeTable<'define_font> {
    pub offset_table: Vec<usize>,
    pub shape_table: &'define_font [u8],
}

impl<'define_font> GlyphShapeTable<'define_font> {
    pub fn num_glyphs(&self) -> usize {
        self.offset_table.len() - 1
    }

    pub fn iter<'glyph_table>(
        &'glyph_table self,
    ) -> GlyphTableIterator<'glyph_table, 'define_font> {
        GlyphTableIterator {
            table: self,
            index: 0,
        }
    }
}

impl<'glyph_table, 'define_font> IntoIterator for &'glyph_table GlyphShapeTable<'define_font> {
    type Item = Result<Shape<(), ()>>;
    type IntoIter = GlyphTableIterator<'glyph_table, 'define_font>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphTableIterator<'glyph_table, 'define_font> {
    pub table: &'glyph_table GlyphShapeTable<'define_font>,
    pub index: usize,
}

impl<'glyph_table, 'define_font> Iterator for GlyphTableIterator<'glyph_table, 'define_font> {
    type Item = Result<Shape<(), ()>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.table.num_glyphs() {
            None
        } else {
            let start_offset = self.table.offset_table[self.index].into();
            let end_offset = self.table.offset_table[self.index + 1].into();
            self.index += 1;
            let reader =
                &mut SwfSliceReader::new(&self.table.shape_table[start_offset..end_offset]);
            Some(Shape::read(reader))
        }
    }
}
