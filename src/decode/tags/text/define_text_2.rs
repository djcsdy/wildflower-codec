use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::common::read_matrix;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::Matrix;
use crate::decode::tags::text::text_record::{ReadTextRecordOptions, TextRecord};
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineText2Tag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgba>>,
}

impl DefineText2Tag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let text_bounds = Rectangle::read(reader)?;
        let text_matrix = read_matrix(reader)?;
        let glyph_bits = reader.read_u8()?;
        let advance_bits = reader.read_u8()?;
        let text_records = TextRecord::read_all(&mut ReadTextRecordOptions {
            reader,
            glyph_bits,
            advance_bits,
            read_color: &Rgba::read,
        })?;
        Ok(Self {
            character_id,
            text_bounds,
            text_matrix,
            text_records,
        })
    }
}
