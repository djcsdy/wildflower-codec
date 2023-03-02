use crate::ast::header::{Compression, Header};
use crate::decode::bit_reader::SwfBitReader;
use std::fs::File;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};
use std::path::Path;

pub struct SwfReader<R: Read> {
    bit_reader: SwfBitReader<R>,
}

impl SwfReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader<File>> {
        SwfBitReader::open(path).map(|bit_reader| SwfReader::new(bit_reader))
    }
}

impl<R: Read> SwfReader<R> {
    pub fn new<I: Into<SwfBitReader<R>>>(bit_reader: I) -> SwfReader<R> {
        SwfReader {
            bit_reader: bit_reader.into(),
        }
    }

    pub fn read_header(&mut self) -> Result<Header> {
        let mut signature = [0u8; 3];
        self.bit_reader.read_u8_into(&mut signature)?;

        let compression = match signature {
            [0x46, 0x57, 0x54] => Ok(Compression::None),
            [0x43, 0x57, 0x54] => Ok(Compression::Zlib),
            [0x5a, 0x57, 0x54] => Ok(Compression::Lzma),
            _ => Err(Error::from(InvalidData)),
        }?;

        let version = self.bit_reader.read_u8()?;
        let frame_size = self.bit_reader.read_rectangle()?;
        let frame_rate = self.bit_reader.read_fixed8()?; // FIXME May use a different byte order than Fixed8
        let frame_count = self.bit_reader.read_u16()?;

        Ok(Header {
            compression,
            version,
            frame_size,
            frame_rate,
            frame_count,
        })
    }
}
