use byteorder::{ByteOrder, LittleEndian};
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Result};

/// A fixed-point number consisting of a 8-bit whole part plus an 8-bit
/// fractional part.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fixed8(i16);

impl Fixed8 {
    pub const ZERO: Fixed8 = Fixed8(0);
    pub const ONE: Fixed8 = Fixed8(0x100);

    pub fn from_bytes(buf: &[u8; 2]) -> Fixed8 {
        Fixed8(LittleEndian::read_i16(buf))
    }

    pub fn read<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(Self::from_bytes(&buf))
    }
}

impl Display for Fixed8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let whole = self.0 >> 8;
        let mut fraction = self.0 & 0xff;
        write!(f, "{}", whole)?;
        if let Some(precision) = f.precision() {
            write!(f, ".")?;
            for _ in 0..precision {
                fraction *= 10;
                write!(f, "{}", fraction >> 8)?;
                fraction &= 0xff;
            }
        } else if fraction > 0 {
            write!(f, ".")?;
            while fraction > 0 {
                fraction *= 10;
                write!(f, "{}", fraction >> 8)?;
                fraction &= 0xff;
            }
        }
        Ok(())
    }
}

impl Debug for Fixed8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
