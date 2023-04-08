use crate::decode::bit_read::BitRead;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct CurvedEdgeRecord {
    pub control_delta_x: i32,
    pub control_delta_y: i32,
    pub anchor_delta_x: i32,
    pub anchor_delta_y: i32,
}

impl CurvedEdgeRecord {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let num_bits = reader.read_ub8(4)? + 2;
        let control_delta_x = reader.read_sb(num_bits)?;
        let control_delta_y = reader.read_sb(num_bits)?;
        let anchor_delta_x = reader.read_sb(num_bits)?;
        let anchor_delta_y = reader.read_sb(num_bits)?;
        Ok(Self {
            control_delta_x,
            control_delta_y,
            anchor_delta_x,
            anchor_delta_y,
        })
    }
}
