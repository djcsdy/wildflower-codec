use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ColorMatrixFilter {
    pub matrix: [f32; 20],
}

impl ColorMatrixFilter {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut matrix = [0f32; 20];
        reader.read_f32_into(&mut matrix)?;
        Ok(Self { matrix })
    }
}
