use crate::decode::bit_read::BitRead;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct StraightEdgeRecord {
    pub delta_x: i32,
    pub delta_y: i32,
}

impl StraightEdgeRecord {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<StraightEdgeRecord> {
        let num_bits = reader.read_ub8(4)? + 2;
        let is_general_line = reader.read_bit()?;
        let is_vertical_line = if is_general_line {
            false
        } else {
            reader.read_bit()?
        };
        let delta_x = if is_general_line || !is_vertical_line {
            reader.read_sb(num_bits)?
        } else {
            0
        };
        let delta_y = if is_general_line || is_vertical_line {
            reader.read_sb(num_bits)?
        } else {
            0
        };
        Ok(StraightEdgeRecord { delta_x, delta_y })
    }
}
