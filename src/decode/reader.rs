use crate::ast::header::{Compression, Header};
use crate::decode::bit_reader::SwfBitReader;
use std::fs::File;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};
use std::path::Path;

pub struct SwfReader<R: Read> {
    header: Header,
    bit_reader: SwfBitReader<R>,
}

impl SwfReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader<File>> {
        SwfBitReader::open(path).and_then(|bit_reader| SwfReader::read_from(bit_reader))
    }
}

impl<R: Read> SwfReader<R> {
    pub fn read_from<I: Into<SwfBitReader<R>>>(bit_reader: I) -> Result<SwfReader<R>> {
        let mut r = bit_reader.into();
        Self::read_header(&mut r).map(|header| SwfReader {
            header,
            bit_reader: r,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    fn read_header(bit_reader: &mut SwfBitReader<R>) -> Result<Header> {
        let mut signature = [0u8; 3];
        bit_reader.read_u8_into(&mut signature)?;

        let compression = match signature {
            [0x46, 0x57, 0x54] => Ok(Compression::None),
            [0x43, 0x57, 0x54] => Ok(Compression::Zlib),
            [0x5a, 0x57, 0x54] => Ok(Compression::Lzma),
            _ => Err(Error::from(InvalidData)),
        }?;

        let version = bit_reader.read_u8()?;
        let frame_size = bit_reader.read_rectangle()?;
        let frame_rate = bit_reader.read_fixed8()?; // FIXME May use a different byte order than Fixed8
        let frame_count = bit_reader.read_u16()?;

        Ok(Header {
            compression,
            version,
            frame_size,
            frame_rate,
            frame_count,
        })
    }
}
