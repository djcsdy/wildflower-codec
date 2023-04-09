use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::buttons::button_record_flags::ButtonRecordFlags;
use crate::decode::tags::common::matrix::Matrix;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord {
    pub flags: ButtonRecordFlags,
    pub character_id: u16,
    pub place_depth: u16,
    pub place_matrix: Matrix,
}

impl ButtonRecord {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Option<Self>> {
        let flags = ButtonRecordFlags::read(reader)?;
        Ok(if flags.is_empty() {
            None
        } else {
            let character_id = reader.read_u16()?;
            let place_depth = reader.read_u16()?;
            let place_matrix = Matrix::read(reader)?;
            Some(Self {
                flags,
                character_id,
                place_depth,
                place_matrix,
            })
        })
    }

    pub fn read_list(reader: &mut SwfSliceReader) -> Result<Vec<Self>> {
        let mut list = Vec::new();
        while let Some(record) = ButtonRecord::read(reader)? {
            list.push(record);
        }
        Ok(list)
    }
}
