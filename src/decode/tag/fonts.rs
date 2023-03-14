use crate::ast::fonts::DefineFontTag;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::shapes::read_shape;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_define_font_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DefineFontTag> {
    let font_id = reader.read_u16()?;
    let end_offset = reader.remaining();
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
        glyph_shapes.push(read_shape(&mut reader.with_max_length(length))?)
    }
    Ok(DefineFontTag {
        font_id,
        glyph_shapes,
    })
}
