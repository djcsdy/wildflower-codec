use crate::ast::common::{
    ColorTransform, ColorTransformWithAlpha, Fixed16, Fixed8, Matrix, Rectangle, Rgb, Rgba,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_rgb<R: Read>(reader: &mut R) -> Result<Rgb> {
    let mut buf = [0u8; 3];
    reader.read_u8_into(&mut buf)?;
    Ok(Rgb {
        red: buf[0],
        green: buf[1],
        blue: buf[2],
    })
}

pub fn read_rgba<R: Read>(reader: &mut R) -> Result<Rgba> {
    let mut buf = [0u8; 4];
    reader.read_u8_into(&mut buf)?;
    Ok(Rgba {
        red: buf[0],
        green: buf[1],
        blue: buf[2],
        alpha: buf[3],
    })
}

pub fn read_argb<R: Read>(reader: &mut R) -> Result<Rgba> {
    let mut buf = [0u8; 4];
    reader.read_u8_into(&mut buf)?;
    Ok(Rgba {
        red: buf[1],
        green: buf[2],
        blue: buf[3],
        alpha: buf[0],
    })
}

pub fn read_rectangle<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Rectangle> {
    reader.align_byte();
    let bits = reader.read_ub8(5)?;
    let x_min = reader.read_sb(bits)?;
    let x_max = reader.read_sb(bits)?;
    let y_min = reader.read_sb(bits)?;
    let y_max = reader.read_sb(bits)?;
    Ok(Rectangle {
        x_min,
        x_max,
        y_min,
        y_max,
    })
}

pub fn read_matrix<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Matrix> {
    reader.align_byte();
    let has_scale = reader.read_bit()?;
    let scale_bits = if has_scale { reader.read_ub8(5)? } else { 0 };
    let scale_x = if has_scale {
        reader.read_fixed16_bits(scale_bits)?
    } else {
        Fixed16::ONE
    };
    let scale_y = if has_scale {
        reader.read_fixed16_bits(scale_bits)?
    } else {
        Fixed16::ONE
    };
    let has_rotate = reader.read_bit()?;
    let rotate_bits = if has_rotate { reader.read_ub8(5)? } else { 0 };
    let rotate_skew_0 = if has_rotate {
        reader.read_fixed16_bits(rotate_bits)?
    } else {
        Fixed16::ZERO
    };
    let rotate_skew_1 = if has_rotate {
        reader.read_fixed16_bits(rotate_bits)?
    } else {
        Fixed16::ZERO
    };
    let translate_bits = reader.read_ub8(5)?;
    let translate_x = reader.read_sb(translate_bits)?;
    let translate_y = reader.read_sb(translate_bits)?;
    Ok(Matrix {
        scale_x,
        scale_y,
        rotate_skew_0,
        rotate_skew_1,
        translate_x,
        translate_y,
    })
}

pub fn read_color_transform<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ColorTransform> {
    let has_add_terms = reader.read_bit()?;
    let has_mult_terms = reader.read_bit()?;
    let bits = reader.read_ub8(4)?;
    let red_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
    } else {
        Fixed8::ONE
    };
    let green_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
    } else {
        Fixed8::ONE
    };
    let blue_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
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

pub fn read_color_transform_with_alpha<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<ColorTransformWithAlpha> {
    let has_add_terms = reader.read_bit()?;
    let has_mult_terms = reader.read_bit()?;
    let bits = reader.read_ub8(4)?;
    let red_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
    } else {
        Fixed8::ONE
    };
    let green_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
    } else {
        Fixed8::ONE
    };
    let blue_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
    } else {
        Fixed8::ONE
    };
    let alpha_multiplication_term = if has_mult_terms {
        reader.read_fixed8_bits(bits)?
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
