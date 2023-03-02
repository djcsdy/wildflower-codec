use crate::ast::header::{Compression, Header};
use crate::decode::decompressing_reader::DecompressingReader;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{BufRead, Error, Result};

pub fn read_header<R: BufRead>(mut reader: R) -> Result<(Header, DecompressingReader<R>)> {
    let mut signature = [0u8; 3];
    reader.read_u8_into(&mut signature)?;

    let compression = match signature {
        [0x46, 0x57, 0x54] => Ok(Compression::None),
        [0x43, 0x57, 0x54] => Ok(Compression::Zlib),
        [0x5a, 0x57, 0x54] => Ok(Compression::Lzma),
        _ => Err(Error::from(InvalidData)),
    }?;

    let version = reader.read_u8()?;
    let file_length = reader.read_u32()?;

    let mut decompressing_tag_body_reader = SwfTagBodyReader::new(
        match compression {
            Compression::None => DecompressingReader::uncompressed(reader),
            Compression::Zlib => DecompressingReader::deflate(reader),
            Compression::Lzma => return Err(Error::from(InvalidData)),
        },
        29,
    );

    let frame_size = decompressing_tag_body_reader.read_rectangle()?;
    let frame_rate = decompressing_tag_body_reader.read_fixed8()?; // FIXME May use a different byte order than Fixed8
    let frame_count = decompressing_tag_body_reader.read_u16()?;

    Ok((
        Header {
            compression,
            version,
            file_length,
            frame_size,
            frame_rate,
            frame_count,
        },
        decompressing_tag_body_reader.into_inner(),
    ))
}
