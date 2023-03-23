use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::fixed_16::Fixed16;
use std::io::Result;

/// A 2×3 matrix, used for 2D affine transformations.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Matrix {
    pub scale_x: Fixed16,
    pub scale_y: Fixed16,
    pub rotate_skew_0: Fixed16,
    pub rotate_skew_1: Fixed16,
    pub translate_x: i32,
    pub translate_y: i32,
}

impl Matrix {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        reader.align_byte();
        let has_scale = reader.read_bit()?;
        let scale_bits = if has_scale { reader.read_ub8(5)? } else { 0 };
        let scale_x = if has_scale {
            Fixed16::read_bits(reader, scale_bits)?
        } else {
            Fixed16::ONE
        };
        let scale_y = if has_scale {
            Fixed16::read_bits(reader, scale_bits)?
        } else {
            Fixed16::ONE
        };
        let has_rotate = reader.read_bit()?;
        let rotate_bits = if has_rotate { reader.read_ub8(5)? } else { 0 };
        let rotate_skew_0 = if has_rotate {
            Fixed16::read_bits(reader, rotate_bits)?
        } else {
            Fixed16::ZERO
        };
        let rotate_skew_1 = if has_rotate {
            Fixed16::read_bits(reader, rotate_bits)?
        } else {
            Fixed16::ZERO
        };
        let translate_bits = reader.read_ub8(5)?;
        let translate_x = reader.read_sb(translate_bits)?;
        let translate_y = reader.read_sb(translate_bits)?;
        Ok(Self {
            scale_x,
            scale_y,
            rotate_skew_0,
            rotate_skew_1,
            translate_x,
            translate_y,
        })
    }
}
