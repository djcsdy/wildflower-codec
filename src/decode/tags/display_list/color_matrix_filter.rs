use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ColorMatrixFilter {
    pub matrix: [f32; 20],
}

impl ColorMatrixFilter {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let mut matrix = [0f32; 20];
        reader.read_f32_into(&mut matrix)?;
        Ok(Self { matrix })
    }
}
