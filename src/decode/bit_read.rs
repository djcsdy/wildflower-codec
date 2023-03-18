use crate::ast::common::{Fixed16, Fixed8};
use byteorder::{ByteOrder, LittleEndian};
use std::io::Result;

pub trait BitRead {
    fn align_byte(&mut self);

    fn read_bit(&mut self) -> Result<bool> {
        Ok(self.read_ub(1)? == 1)
    }

    fn read_ub8(&mut self, bits: u8) -> Result<u8> {
        if bits > 8 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u8)
    }

    fn read_ub16(&mut self, bits: u8) -> Result<u16> {
        if bits > 16 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u16)
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32>;

    fn read_sb16(&mut self, bits: u8) -> Result<i16> {
        if bits > 16 {
            panic!();
        }

        Ok(self.read_sb(bits)? as i16)
    }

    fn read_sb(&mut self, bits: u8) -> Result<i32> {
        Ok(extend_sign(self.read_ub(bits)?, bits))
    }

    fn read_fixed16_bits(&mut self, bits: u8) -> Result<Fixed16> {
        let mut buf = [0u8; 4];
        LittleEndian::write_u32(&mut buf, self.read_ub(bits)?);
        Ok(Fixed16::from_bytes(&buf))
    }

    fn read_fixed8_bits(&mut self, bits: u8) -> Result<Fixed8> {
        let mut buf = [0u8; 2];
        LittleEndian::write_u16(&mut buf, self.read_ub16(bits)?);
        Ok(Fixed8::from_bytes(&buf))
    }
}

pub fn extend_sign(value: u32, bits: u8) -> i32 {
    let sign = 1u32 << (bits - 1);
    if (value & sign) == 0 {
        value as i32
    } else {
        (value as i32) | -(sign as i32)
    }
}
