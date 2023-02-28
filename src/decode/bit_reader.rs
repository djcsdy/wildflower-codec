use crate::ast::actions::PushValue::Float;
use crate::ast::common::{Fixed16, Fixed8, Float16};
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
}

impl<R: Read> From<R> for SwfBitReader<R> {
    fn from(value: R) -> Self {
        SwfBitReader::new(value)
    }
}
