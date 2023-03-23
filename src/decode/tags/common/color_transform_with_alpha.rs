use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::fixed_8::Fixed8;
use std::io::Result;

/// A simple transformation of an RGBA color-with-alpha.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ColorTransformWithAlpha {
    pub red_multiplication_term: Fixed8,
    pub green_multiplication_term: Fixed8,
    pub blue_multiplication_term: Fixed8,
    pub alpha_multiplication_term: Fixed8,
    pub red_addition_term: i16,
    pub green_addition_term: i16,
    pub blue_addition_term: i16,
    pub alpha_addition_term: i16,
}

impl ColorTransformWithAlpha {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let has_add_terms = reader.read_bit()?;
        let has_mult_terms = reader.read_bit()?;
        let bits = reader.read_ub8(4)?;
        let red_multiplication_term = if has_mult_terms {
            Fixed8::read_bits(reader, bits)?
        } else {
            Fixed8::ONE
        };
        let green_multiplication_term = if has_mult_terms {
            Fixed8::read_bits(reader, bits)?
        } else {
            Fixed8::ONE
        };
        let blue_multiplication_term = if has_mult_terms {
            Fixed8::read_bits(reader, bits)?
        } else {
            Fixed8::ONE
        };
        let alpha_multiplication_term = if has_mult_terms {
            Fixed8::read_bits(reader, bits)?
        } else {
            Fixed8::ONE
        };
        let red_addition_term = if has_add_terms {
            reader.read_sb16(bits)?
        } else {
            0
        };
        let green_addition_term = if has_add_terms {
            reader.read_sb16(bits)?
        } else {
            0
        };
        let blue_addition_term = if has_add_terms {
            reader.read_sb16(bits)?
        } else {
            0
        };
        let alpha_addition_term = if has_add_terms {
            reader.read_sb16(bits)?
        } else {
            0
        };
        Ok(Self {
            red_multiplication_term,
            green_multiplication_term,
            blue_multiplication_term,
            alpha_multiplication_term,
            red_addition_term,
            green_addition_term,
            blue_addition_term,
            alpha_addition_term,
        })
    }
}
