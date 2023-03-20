use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::shapes::read_shape;
use crate::decode::tags::shapes::Shape;
use std::io::Result;

pub struct GlyphTable<'define_font, Offset: Copy + Into<usize>> {
    pub swf_version: u8,
    pub offset_table: &'define_font [Offset],
    pub shape_table: &'define_font [u8],
    pub index: usize,
}

impl<'define_font, Offset: Copy + Into<usize>> Iterator for GlyphTable<'define_font, Offset> {
    type Item = Result<Shape<(), ()>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.offset_table.len() - 1 {
            None
        } else {
            let start_offset = self.offset_table[self.index].into();
            let end_offset = self.offset_table[self.index + 1].into();
            self.index += 1;
            Some(read_shape(&mut SwfSliceReader::new(
                &self.shape_table[start_offset..end_offset],
                self.swf_version,
            )))
        }
    }
}
