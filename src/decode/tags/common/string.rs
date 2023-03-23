use std::fmt::{Debug, Formatter};

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
    pub const EMPTY: Self = Self(vec![]);

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
