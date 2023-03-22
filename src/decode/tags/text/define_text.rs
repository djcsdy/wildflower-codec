use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::common::{read_matrix, read_rectangle};
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::common::Matrix;
use crate::decode::tags::text::text_record::{ReadTextRecordOptions, TextRecord};
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineTextTag {
    pub character_id: u16,
    pub text_bounds: Rectangle,
    pub text_matrix: Matrix,
    pub text_records: Vec<TextRecord<Rgb>>,
}

impl DefineTextTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let text_bounds = read_rectangle(reader)?;
        let text_matrix = read_matrix(reader)?;
        let glyph_bits = reader.read_u8()?;
        let advance_bits = reader.read_u8()?;
        let text_records = TextRecord::read_all(&mut ReadTextRecordOptions {
            reader,
            glyph_bits,
            advance_bits,
            read_color: &Rgb::read,
        })?;
        Ok(Self {
            character_id,
            text_bounds,
            text_matrix,
            text_records,
        })
    }
}
