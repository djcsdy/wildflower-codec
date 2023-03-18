use byteorder::{ByteOrder, LittleEndian};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Twips<N>(N);

impl<N> Twips<N> {
    pub fn new(twips: N) -> Twips<N> {
        Twips(twips)
    }
}

impl<N: Display> Display for Twips<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}twpx", self.0)
    }
}

impl<N: Display> Debug for Twips<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

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

/// A sequence of bytes representing a character string.
///
/// In SWF 6 and later, the string is encoded using UTF-8.
///
/// In SWF 5 and earlier, the string is encoded using either Windows-1252 or
/// Windows-932. The encoding used is not specified, so Wildflower must guess.
/// (Flash Player guesses the encoding based on the user's locale. If the
/// locale is Japanese, then Flash Player guesses Windows-932. Otherwise, it
/// guesses Windows-1252.)
///
/// No matter the encoding, the sequence of bytes are not guaranteed to be
/// valid according to that encoding.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct String(Vec<u8>);

impl String {
    pub fn from_bytes<I: Into<Vec<u8>>>(buf: I) -> String {
        String(buf.into())
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for c in &self.0 {
            if *c == '\\' as u8 {
                write!(f, "\\\\")?;
            } else if c.is_ascii() && !c.is_ascii_control() {
                write!(f, "{}", c)?;
            } else {
                write!(f, "\\x{:X}", c)?;
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}

/// An RGB color.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// An RGB color with an alpha component.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// An axis-aligned rectangle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rectangle {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

/// A 2×3 matrix, used for 2D affine transformations.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Matrix {
    pub scale_x: Fixed16,
    pub scale_y: Fixed16,
    pub rotate_skew_0: Fixed16,
    pub rotate_skew_1: Fixed16,
    pub translate_x: i32,
    pub translate_y: i32,
}

/// A simple color transformation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ColorTransform {
    pub red_multiplication_term: Fixed8,
    pub green_multiplication_term: Fixed8,
    pub blue_multiplication_term: Fixed8,
    pub red_addition_term: i16,
    pub green_addition_term: i16,
    pub blue_addition_term: i16,
}

/// A simple transformation of an RGBA color-with-alpha.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ColorTransformWithAlpha {
    pub red_multiplication_term: Fixed8,
    pub green_multiplication_term: Fixed8,
    pub blue_multiplication_term: Fixed8,
    pub alpha_multiplication_term: Fixed8,
    pub red_addition_term: i16,
    pub green_addition_term: i16,
    pub blue_addition_term: i16,
    pub alpha_addition_term: i16,
}
