use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
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
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let text_bounds = Rectangle::read(reader)?;
        let text_matrix = Matrix::read(reader)?;
        let glyph_bits = reader.read_u8()?;
        let advance_bits = reader.read_u8()?;
        let text_records = TextRecord::read_all(ReadTextRecordOptions {
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
