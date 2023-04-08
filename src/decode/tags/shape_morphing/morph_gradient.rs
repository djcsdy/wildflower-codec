use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::shape_morphing::morph_gradient_record::MorphGradientRecord;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
}

impl MorphGradient {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let num_gradients = reader.read_u8()? as usize;
        let mut gradient_records = Vec::with_capacity(num_gradients);
        for _ in 0..num_gradients {
            gradient_records.push(MorphGradientRecord::read(reader)?);
        }
        Ok(Self { gradient_records })
    }
}
