use crate::ast::actions::PushValue::Float;
use crate::ast::common::{Fixed16, Fixed8, Float16, Rgb, Rgba, String};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct SwfBitReader<R: Read> {
    inner: BufReader<R>,
    partial_byte: u8,
    partial_bits: u8,
}

impl SwfBitReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfBitReader<File>> {
        File::open(path).map(|file| SwfBitReader::new(file))
    }
}

impl<R: Read> SwfBitReader<R> {
    pub fn new(inner: R) -> SwfBitReader<R> {
        SwfBitReader {
            inner: BufReader::new(inner),
            partial_byte: 0,
            partial_bits: 0,
        }
    }

    pub fn align_byte(&mut self) {
        self.partial_byte = 0;
        self.partial_bits = 0;
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.align_byte();
        self.inner.read_i8()
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        self.align_byte();
        self.inner.read_i16::<LittleEndian>()
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.align_byte();
        self.inner.read_i32::<LittleEndian>()
    }

    pub fn read_i8_into(&mut self, buf: &mut [i8]) -> Result<()> {
        self.align_byte();
        self.inner.read_i8_into(buf)
    }

    pub fn read_i16_into(&mut self, buf: &mut [i16]) -> Result<()> {
        self.align_byte();
        self.inner.read_i16_into::<LittleEndian>(buf)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.align_byte();
        self.inner.read_u8()
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.align_byte();
        self.inner.read_u16::<LittleEndian>()
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.align_byte();
        self.inner.read_u32::<LittleEndian>()
    }

    pub fn read_u8_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.align_byte();
        self.inner.read_exact(buf)
    }

    pub fn read_u16_into(&mut self, buf: &mut [u16]) -> Result<()> {
        self.align_byte();
        self.inner.read_u16_into::<LittleEndian>(buf)
    }

    pub fn read_u24_into(&mut self, buf: &mut [u32]) -> Result<()> {
        self.align_byte();
        for i in 0..buf.len() {
            let mut buf2 = [0u8; 3];
            self.inner.read_exact(&mut buf2)?;
            buf[i] = (buf2[0] as u32) & ((buf2[1] as u32) << 8) & ((buf2[2] as u32) << 16);
        }
        Ok(())
    }

    pub fn read_u32_into(&mut self, buf: &mut [u32]) -> Result<()> {
        self.align_byte();
        self.inner.read_u32_into::<LittleEndian>(buf)
    }

    pub fn read_u64_into(&mut self, buf: &mut [u64]) -> Result<()> {
        self.align_byte();
        self.inner.read_u64_into::<LittleEndian>(buf)
    }

    pub fn read_fixed16(&mut self) -> Result<Fixed16> {
        self.align_byte();
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(Fixed16::from_bytes(&buf))
    }

    pub fn read_fixed8(&mut self) -> Result<Fixed8> {
        self.align_byte();
        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(Fixed8::from_bytes(&buf))
    }

    pub fn read_float16(&mut self) -> Result<Float16> {
        self.align_byte();
        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(Float16::from_bytes(&buf))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        self.align_byte();
        self.inner.read_f32::<LittleEndian>()
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        self.align_byte();
        self.inner.read_f64::<LittleEndian>()
    }

    pub fn read_encoded_u32(&mut self) -> Result<u32> {
        self.align_byte();
        let mut count = 1;
        let mut byte = self.read_u8()?;
        let mut value = (byte as u32) & 0x7f;
        while count < 5 && (byte & 0x80) == 0x80 {
            byte = self.read_u8()?;
            value = value | (((byte as u32) & 0x7f).wrapping_shl(7 * count));
            count = count + 1
        }
        Ok(value)
    }

    pub fn read_ub8(&mut self, bits: u8) -> Result<u8> {
        if bits > 8 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u8)
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
}

impl<R: Read> From<R> for SwfBitReader<R> {
    fn from(value: R) -> Self {
        SwfBitReader::new(value)
    }
}
