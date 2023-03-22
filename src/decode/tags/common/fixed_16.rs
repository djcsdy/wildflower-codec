use crate::decode::read_ext::SwfTypesReadExt;
use byteorder::{ByteOrder, LittleEndian};
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Result};

/// A fixed-point number consisting of a 16-bit whole part plus a 16-bit
/// fractional part.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fixed16(i32);

impl Fixed16 {
    pub const ZERO: Fixed16 = Fixed16(0);
    pub const ONE: Fixed16 = Fixed16(0x10000);

    pub fn from_bytes(buf: &[u8; 4]) -> Fixed16 {
        Fixed16(LittleEndian::read_i32(buf))
    }

    pub fn read<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_u8_into(&mut buf)?;
        Ok(Self::from_bytes(&buf))
    }
}

impl Display for Fixed16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let whole = self.0 >> 16;
        let mut fraction = self.0 & 0xffff;
        write!(f, "{}", whole)?;
        if let Some(precision) = f.precision() {
            write!(f, ".")?;
            for _ in 0..precision {
                fraction *= 10;
                write!(f, "{}", fraction >> 16)?;
                fraction &= 0xffff;
            }
        } else if fraction > 0 {
            write!(f, ".")?;
            while fraction > 0 {
                fraction *= 10;
                write!(f, "{}", fraction >> 16)?;
                fraction &= 0xffff;
            }
        }
        Ok(())
    }
}

impl Debug for Fixed16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
