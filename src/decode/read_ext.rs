use byteorder::{LittleEndian, ReadBytesExt};
use half::f16;
use std::io::{Read, Result};

pub trait SwfTypesReadExt {
    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_i8_into(&mut self, buf: &mut [i8]) -> Result<()>;
    fn read_i16_into(&mut self, buf: &mut [i16]) -> Result<()>;
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u8_into(&mut self, buf: &mut [u8]) -> Result<()>;
    fn read_u16_into(&mut self, buf: &mut [u16]) -> Result<()>;
    fn read_u24_into(&mut self, buf: &mut [u32]) -> Result<()>;
    fn read_u32_into(&mut self, buf: &mut [u32]) -> Result<()>;
    fn read_u64_into(&mut self, buf: &mut [u64]) -> Result<()>;
    fn read_f16(&mut self) -> Result<f16>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;
    fn read_f32_into(&mut self, buf: &mut [f32]) -> Result<()>;
    fn read_encoded_u32(&mut self) -> Result<u32>;
}

impl<R: Read + ?Sized> SwfTypesReadExt for R {
    fn read_i8(&mut self) -> Result<i8> {
        ReadBytesExt::read_i8(self)
    }

    fn read_i16(&mut self) -> Result<i16> {
        ReadBytesExt::read_i16::<LittleEndian>(self)
    }

    fn read_i32(&mut self) -> Result<i32> {
        ReadBytesExt::read_i32::<LittleEndian>(self)
    }

    fn read_i8_into(&mut self, buf: &mut [i8]) -> Result<()> {
        ReadBytesExt::read_i8_into(self, buf)
    }

    fn read_i16_into(&mut self, buf: &mut [i16]) -> Result<()> {
        ReadBytesExt::read_i16_into::<LittleEndian>(self, buf)
    }

    fn read_u8(&mut self) -> Result<u8> {
        ReadBytesExt::read_u8(self)
    }

    fn read_u16(&mut self) -> Result<u16> {
        ReadBytesExt::read_u16::<LittleEndian>(self)
    }

    fn read_u32(&mut self) -> Result<u32> {
        ReadBytesExt::read_u32::<LittleEndian>(self)
    }

    fn read_u8_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)
    }

    fn read_u16_into(&mut self, buf: &mut [u16]) -> Result<()> {
        ReadBytesExt::read_u16_into::<LittleEndian>(self, buf)
    }

    fn read_u24_into(&mut self, buf: &mut [u32]) -> Result<()> {
        for i in 0..buf.len() {
            let mut buf2 = [0u8; 3];
            self.read_exact(&mut buf2)?;
            buf[i] = (buf2[0] as u32) | ((buf2[1] as u32) << 8) | ((buf2[2] as u32) << 16);
        }
        Ok(())
    }

    fn read_u32_into(&mut self, buf: &mut [u32]) -> Result<()> {
        ReadBytesExt::read_u32_into::<LittleEndian>(self, buf)
    }

    fn read_u64_into(&mut self, buf: &mut [u64]) -> Result<()> {
        ReadBytesExt::read_u64_into::<LittleEndian>(self, buf)
    }

    fn read_f16(&mut self) -> Result<f16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(f16::from_le_bytes(buf))
    }

    fn read_f32(&mut self) -> Result<f32> {
        ReadBytesExt::read_f32::<LittleEndian>(self)
    }

    fn read_f64(&mut self) -> Result<f64> {
        ReadBytesExt::read_f64::<LittleEndian>(self)
    }

    fn read_f32_into(&mut self, buf: &mut [f32]) -> Result<()> {
        ReadBytesExt::read_f32_into::<LittleEndian>(self, buf)
    }

    fn read_encoded_u32(&mut self) -> Result<u32> {
        let mut count = 1;
        let mut byte = ReadBytesExt::read_u8(self)?;
        let mut value = (byte as u32) & 0x7f;
        while count < 5 && (byte & 0x80) == 0x80 {
            byte = ReadBytesExt::read_u8(self)?;
            value = value | (((byte as u32) & 0x7f).wrapping_shl(7 * count));
            count = count + 1
        }
        Ok(value)
    }
}
