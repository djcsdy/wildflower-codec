use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::shapes::shape::Shape;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontTag {
    pub font_id: u16,
    pub glyph_shapes: Vec<Shape<(), ()>>,
}

impl DefineFontTag {
    pub fn read<R: SliceRead + SizedRead + BitRead>(reader: &mut R) -> Result<Self> {
        let font_id = reader.read_u16()?;
        let end_offset = reader.remaining_bytes();
        let first_offset = reader.read_u16()? as usize;
        let num_glyphs = first_offset / 2;
        let mut length_table = Vec::with_capacity(num_glyphs);
        let mut prev_offset = first_offset;
        for _ in 1..num_glyphs {
            let offset = reader.read_u16()? as usize;
            length_table.push(offset - prev_offset);
            prev_offset = offset;
        }
        length_table.push(end_offset - prev_offset);
        let mut glyph_shapes = Vec::with_capacity(num_glyphs);
        for length in length_table {
            let mut glyph_reader = reader.slice(length);
            glyph_shapes.push(Shape::read(&mut glyph_reader)?);
        }
        Ok(Self {
            font_id,
            glyph_shapes,
        })
    }
}
