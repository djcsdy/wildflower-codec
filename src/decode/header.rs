use crate::decode::bit_reader::BitReader;
use crate::decode::compression::Compression;
use crate::decode::decompressing_reader::DecompressingReader;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::swf_version::SwfVersion;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rectangle::Rectangle;
use std::io::ErrorKind::InvalidData;
use std::io::{BufRead, Error, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Header {
    pub compression: Compression,
    pub version: SwfVersion,
    pub file_length: u32,
    pub frame_size: Rectangle,
    pub frame_rate: Fixed8,
    pub frame_count: u16,
}

impl Header {
    pub fn read<R: BufRead>(mut reader: R) -> Result<(Self, DecompressingReader<R>)> {
        let mut signature = [0u8; 3];
        reader.read_u8_into(&mut signature)?;

        let compression = match signature {
            [0x46, 0x57, 0x54] => Ok(Compression::None),
            [0x43, 0x57, 0x54] => Ok(Compression::Zlib),
            [0x5a, 0x57, 0x54] => Ok(Compression::Lzma),
            _ => Err(Error::from(InvalidData)),
        }?;

        let version = SwfVersion::read(&mut reader)?;
        let file_length = reader.read_u32()?;

        let mut bit_reader = BitReader::new(match compression {
            Compression::None => DecompressingReader::uncompressed(reader),
            Compression::Zlib => DecompressingReader::deflate(reader),
            Compression::Lzma => return Err(Error::from(InvalidData)),
        });

        let frame_size = Rectangle::read(&mut bit_reader)?;
        let frame_rate = Fixed8::read(&mut bit_reader)?;
        let frame_count = bit_reader.read_u16()?;

        Ok((
            Self {
                compression,
                version,
                file_length,
                frame_size,
                frame_rate,
                frame_count,
            },
            bit_reader.into_inner(),
        ))
    }
}
