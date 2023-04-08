use crate::decode::bit_read::BitRead;
use crate::decode::tags::styles::gradient_record::GradientRecord;
use crate::decode::tags::styles::interpolation_mode::InterpolationMode;
use crate::decode::tags::styles::spread_mode::SpreadMode;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient<Color> {
    pub spread_mode: SpreadMode,
    pub interpolation_mode: InterpolationMode,
    pub gradient_records: Vec<GradientRecord<Color>>,
}

impl<Color> Gradient<Color> {
    pub fn read<Read: BitRead, ReadColor: Fn(&mut Read) -> Result<Color>>(
        reader: &mut Read,
        read_color: &ReadColor,
    ) -> Result<Self> {
        let spread_mode = reader
            .read_ub8(2)?
            .try_into()
            .map_err(|_| Error::from(InvalidData))?;
        let interpolation_mode = reader
            .read_ub8(2)?
            .try_into()
            .map_err(|_| Error::from(InvalidData))?;
        let num_gradients = reader.read_ub8(4)?;
        let mut gradient_records = Vec::with_capacity(num_gradients as usize);
        for _ in 0..num_gradients {
            gradient_records.push(GradientRecord::read(reader, &read_color)?);
        }
        Ok(Self {
            spread_mode,
            interpolation_mode,
            gradient_records,
        })
    }
}
