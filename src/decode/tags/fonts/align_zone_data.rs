use crate::decode::read_ext::SwfTypesReadExt;
use half::f16;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct AlignZoneData {
    pub alignment_coordinate: f16,
    pub range: f16,
}

impl AlignZoneData {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let alignment_coordinate = reader.read_f16()?;
        let range = reader.read_f16()?;
        Ok(Self {
            alignment_coordinate,
            range,
        })
    }
}
