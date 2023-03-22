use crate::decode::bit_read::BitRead;
use std::io::Result;

/// An axis-aligned rectangle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rectangle {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

impl Rectangle {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        reader.align_byte();
        let bits = reader.read_ub8(5)?;
        let x_min = reader.read_sb(bits)?;
        let x_max = reader.read_sb(bits)?;
        let y_min = reader.read_sb(bits)?;
        let y_max = reader.read_sb(bits)?;
        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}
