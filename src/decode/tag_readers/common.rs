use crate::decode::bit_read::BitRead;
use crate::decode::tags::common::color_transform::ColorTransform;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::ColorTransformWithAlpha;
use std::io::Result;

pub fn read_color_transform<R: BitRead>(reader: &mut R) -> Result<ColorTransform> {
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
    Ok(ColorTransform {
        red_multiplication_term,
        green_multiplication_term,
        blue_multiplication_term,
        red_addition_term,
        green_addition_term,
        blue_addition_term,
    })
}

pub fn read_color_transform_with_alpha<R: BitRead>(
    reader: &mut R,
) -> Result<ColorTransformWithAlpha> {
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
    Ok(ColorTransformWithAlpha {
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
