/// A fixed-point number consisting of a 16-bit whole part plus a 16-bit
/// fractional part.
struct Fixed16(i32);

/// A fixed-point number consisting of a 8-bit whole part plus an 8-bit
/// fractional part.
struct Fixed8(i16);

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
struct String([u8]);