use crate::decode::bit_read::BitRead;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::shapes::shape_record::{ReadShapeRecordArrayOptions, ShapeRecord};
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct Shape<Color, LineStyle> {
    pub shape_records: Vec<ShapeRecord<Color, LineStyle>>,
}

impl Shape<(), ()> {
    pub fn read<R: BitRead + SizedRead>(reader: &mut R) -> Result<Self> {
        let num_fill_bits = reader.read_ub8(4)?;
        let num_line_bits = reader.read_ub8(4)?;
        let shape_records = ShapeRecord::read_array(ReadShapeRecordArrayOptions {
            reader,
            num_fill_bits,
            num_line_bits,
            read_line_style_array: &|_| Ok(vec![]),
            read_fill_style_array: &|_| Ok(vec![]),
        })?;
        Ok(Self { shape_records })
    }
}
