use crate::ast::common::{
    ColorTransform, ColorTransformWithAlpha, Fixed16, Fixed8, Matrix, Rectangle, Rgb, Rgba, String,
};
use crate::decode::max_length_reader::MaxLengthReader;
use crate::decode::read_ext::SwfTypesReadExt;
use byteorder::{ByteOrder, LittleEndian};
use std::io::{IoSliceMut, Read, Result};

pub struct SwfFieldReader<R: Read> {
    inner: MaxLengthReader<R>,
    partial_byte: u8,
    partial_bits: u8,
}

impl<R: Read> SwfFieldReader<R> {
    pub fn new(inner: R, max_length: usize) -> SwfFieldReader<R> {
        SwfFieldReader {
            inner: MaxLengthReader::new(inner, max_length),
            partial_byte: 0,
            partial_bits: 0,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }

    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    pub fn skip_to_end(&mut self) -> Result<()> {
        let mut buf = [0; 4096];
        while self.remaining() > 0 {
            self.read(&mut buf)?;
        }
        Ok(())
    }

    pub fn align_byte(&mut self) {
        self.partial_byte = 0;
        self.partial_bits = 0;
    }

    pub fn read_bit(&mut self) -> Result<bool> {
        Ok(self.read_ub(1)? == 1)
    }

    pub fn read_ub8(&mut self, bits: u8) -> Result<u8> {
        if bits > 8 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u8)
    }

    pub fn read_ub16(&mut self, bits: u8) -> Result<u16> {
        if bits > 16 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u16)
    }

    pub fn read_ub(&mut self, bits: u8) -> Result<u32> {
        if bits > 32 {
            panic!();
        }

        if bits <= self.partial_bits {
            self.partial_bits = self.partial_bits - bits;
            Ok((self.partial_byte as u32) >> self.partial_bits)
        } else {
            let mut result = self.partial_byte as u32;
            let mut bits_remaining = bits - self.partial_bits;
            while bits_remaining > 8 {
                result = (result << 8) | self.read_u8()? as u32;
                bits_remaining = bits_remaining - 8;
            }

            self.partial_byte = self.read_u8()?;
            self.partial_bits = 8 - bits_remaining;

            Ok((result << bits_remaining) | ((self.partial_byte as u32) >> self.partial_bits))
        }
    }

    pub fn read_sb16(&mut self, bits: u8) -> Result<i16> {
        if bits > 16 {
            panic!();
        }

        Ok(self.read_sb(bits)? as i16)
    }

    pub fn read_sb(&mut self, bits: u8) -> Result<i32> {
        Ok(Self::extend_sign(self.read_ub(bits)?, bits))
    }

    fn extend_sign(value: u32, bits: u8) -> i32 {
        let sign = 1u32 << (bits - 1);
        if (value & sign) == 0 {
            value as i32
        } else {
            (value as i32) | -(sign as i32)
        }
    }

    pub fn read_fixed16_bits(&mut self, bits: u8) -> Result<Fixed16> {
        let mut buf = [0u8; 4];
        LittleEndian::write_u32(&mut buf, self.read_ub(bits)?);
        Ok(Fixed16::from_bytes(&buf))
    }

    pub fn read_fixed8_bits(&mut self, bits: u8) -> Result<Fixed8> {
        let mut buf = [0u8; 2];
        LittleEndian::write_u16(&mut buf, self.read_ub16(bits)?);
        Ok(Fixed8::from_bytes(&buf))
    }

    pub fn read_string(&mut self) -> Result<String> {
        let mut bytes = Vec::new();
        let mut byte = self.read_u8()?;
        while byte != 0 {
            bytes.push(byte);
            byte = self.read_u8()?;
        }
        Ok(String::from_bytes(bytes))
    }

    pub fn read_rgb(&mut self) -> Result<Rgb> {
        let mut buf = [0u8; 3];
        self.read_u8_into(&mut buf)?;
        Ok(Rgb {
            red: buf[0],
            green: buf[1],
            blue: buf[2],
        })
    }

    pub fn read_rgba(&mut self) -> Result<Rgba> {
        let mut buf = [0u8; 4];
        self.read_u8_into(&mut buf)?;
        Ok(Rgba {
            red: buf[0],
            green: buf[1],
            blue: buf[2],
            alpha: buf[3],
        })
    }

    pub fn read_argb(&mut self) -> Result<Rgba> {
        let mut buf = [0u8; 4];
        self.read_u8_into(&mut buf)?;
        Ok(Rgba {
            red: buf[1],
            green: buf[2],
            blue: buf[3],
            alpha: buf[0],
        })
    }

    pub fn read_rectangle(&mut self) -> Result<Rectangle> {
        self.align_byte();
        let bits = self.read_ub8(5)?;
        let x_min = self.read_sb(bits)?;
        let x_max = self.read_sb(bits)?;
        let y_min = self.read_sb(bits)?;
        let y_max = self.read_sb(bits)?;
        Ok(Rectangle {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }

    pub fn read_matrix(&mut self) -> Result<Matrix> {
        self.align_byte();
        let has_scale = self.read_bit()?;
        let scale_bits = if has_scale { self.read_ub8(5)? } else { 0 };
        let scale_x = if has_scale {
            self.read_fixed16_bits(scale_bits)?
        } else {
            Fixed16::ONE
        };
        let scale_y = if has_scale {
            self.read_fixed16_bits(scale_bits)?
        } else {
            Fixed16::ONE
        };
        let has_rotate = self.read_bit()?;
        let rotate_bits = if has_rotate { self.read_ub8(5)? } else { 0 };
        let rotate_skew_0 = if has_rotate {
            self.read_fixed16_bits(rotate_bits)?
        } else {
            Fixed16::ZERO
        };
        let rotate_skew_1 = if has_rotate {
            self.read_fixed16_bits(rotate_bits)?
        } else {
            Fixed16::ZERO
        };
        let translate_bits = self.read_ub8(5)?;
        let translate_x = self.read_sb(translate_bits)?;
        let translate_y = self.read_sb(translate_bits)?;
        Ok(Matrix {
            scale_x,
            scale_y,
            rotate_skew_0,
            rotate_skew_1,
            translate_x,
            translate_y,
        })
    }

    pub fn read_color_transform(&mut self) -> Result<ColorTransform> {
        let has_add_terms = self.read_bit()?;
        let has_mult_terms = self.read_bit()?;
        let bits = self.read_ub8(4)?;
        let red_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let green_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let blue_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let red_addition_term = if has_add_terms {
            self.read_sb16(bits)?
        } else {
            0
        };
        let green_addition_term = if has_add_terms {
            self.read_sb16(bits)?
        } else {
            0
        };
        let blue_addition_term = if has_add_terms {
            self.read_sb16(bits)?
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

    pub fn read_color_transform_with_alpha(&mut self) -> Result<ColorTransformWithAlpha> {
        let has_add_terms = self.read_bit()?;
        let has_mult_terms = self.read_bit()?;
        let bits = self.read_ub8(4)?;
        let red_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let green_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let blue_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let alpha_multiplication_term = if has_mult_terms {
            self.read_fixed8_bits(bits)?
        } else {
            Fixed8::ONE
        };
        let red_addition_term = if has_add_terms {
            self.read_sb16(bits)?
        } else {
            0
        };
        let green_addition_term = if has_add_terms {
            self.read_sb16(bits)?
        } else {
            0
        };
        let blue_addition_term = if has_add_terms {
            self.read_sb16(bits)?
        } else {
            0
        };
        let alpha_addition_term = if has_add_terms {
            self.read_sb16(bits)?
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
}

impl<R: Read> Read for SwfFieldReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.align_byte();
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        self.align_byte();
        self.inner.read_vectored(bufs)
    }
}
