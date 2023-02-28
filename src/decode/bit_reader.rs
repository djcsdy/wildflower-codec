use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct SwfBitReader<R: Read> {
    inner: BufReader<R>,
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
        }
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.inner.read_i8()
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        self.inner.read_i16::<LittleEndian>()
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.inner.read_i32::<LittleEndian>()
    }

    pub fn read_i8_into(&mut self, buf: &mut [i8]) -> Result<()> {
        self.inner.read_i8_into(buf)
    }

    pub fn read_i16_into(&mut self, buf: &mut [i16]) -> Result<()> {
        self.inner.read_i16_into::<LittleEndian>(buf)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.inner.read_u8()
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.inner.read_u16::<LittleEndian>()
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.inner.read_u32::<LittleEndian>()
    }

    pub fn read_u8_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner.read_exact(buf)
    }

    pub fn read_u16_into(&mut self, buf: &mut [u16]) -> Result<()> {
        self.inner.read_u16_into::<LittleEndian>(buf)
    }

    pub fn read_u24_into(&mut self, buf: &mut [u32]) -> Result<()> {
        for i in 0..buf.len() {
            let mut buf2 = [0u8; 3];
            self.inner.read_exact(&mut buf2)?;
            buf[i] = (buf2[0] as u32) & ((buf2[1] as u32) << 8) & ((buf2[2] as u32) << 16);
        }
        Ok(())
    }
}

impl<R: Read> From<R> for SwfBitReader<R> {
    fn from(value: R) -> Self {
        SwfBitReader::new(value)
    }
}
