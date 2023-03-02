use crate::ast::header::{Compression, Header};
use crate::decode::bit_reader::SwfBitReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_header<R: Read, I: Into<SwfBitReader<R>>>(
    bit_reader: I,
) -> Result<(Header, SwfBitReader<R>)> {
    let mut r = bit_reader.into();

    let mut signature = [0u8; 3];
    r.read_u8_into(&mut signature)?;

    let compression = match signature {
        [0x46, 0x57, 0x54] => Ok(Compression::None),
        [0x43, 0x57, 0x54] => Ok(Compression::Zlib),
        [0x5a, 0x57, 0x54] => Ok(Compression::Lzma),
        _ => Err(Error::from(InvalidData)),
    }?;

    let version = r.read_u8()?;
    let file_length = r.read_u32()?;
    let frame_size = r.read_rectangle()?;
    let frame_rate = r.read_fixed8()?; // FIXME May use a different byte order than Fixed8
    let frame_count = r.read_u16()?;

    Ok((
        Header {
            compression,
            version,
            file_length,
            frame_size,
            frame_rate,
            frame_count,
        },
        r,
    ))
}
