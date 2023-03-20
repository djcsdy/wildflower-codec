use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use half::f16;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ZoneData {
    pub alignment_coordinate: f16,
    pub range: f16,
}

impl ZoneData {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let alignment_coordinate = reader.read_f16()?;
        let range = reader.read_f16()?;
        Ok(Self {
            alignment_coordinate,
            range,
        })
    }
}
