use crate::ast::header::{Compression, Header};
use crate::decode::bit_reader::SwfBitReader;
use crate::decode::decompressing_reader::DecompressingReader;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::ErrorKind::InvalidData;
use std::io::{BufRead, Error, Result};

pub fn read_header<R: BufRead>(
    mut reader: R,
) -> Result<(Header, SwfBitReader<DecompressingReader<R>>)> {
    let mut uncompressed_bit_reader = SwfBitReader::new(reader);

    let mut signature = [0u8; 3];
    uncompressed_bit_reader.read_u8_into(&mut signature)?;

    let compression = match signature {
        [0x46, 0x57, 0x54] => Ok(Compression::None),
        [0x43, 0x57, 0x54] => Ok(Compression::Zlib),
        [0x5a, 0x57, 0x54] => Ok(Compression::Lzma),
        _ => Err(Error::from(InvalidData)),
    }?;

    let version = uncompressed_bit_reader.read_u8()?;
    let file_length = uncompressed_bit_reader.read_u32()?;

    reader = uncompressed_bit_reader.into_inner();
    let mut compressed_bit_reader = SwfBitReader::new(match compression {
        Compression::None => DecompressingReader::uncompressed(reader),
        Compression::Zlib => DecompressingReader::deflate(reader),
        Compression::Lzma => return Err(Error::from(InvalidData)),
    });

    let frame_size = compressed_bit_reader.read_rectangle()?;
    let frame_rate = compressed_bit_reader.read_fixed8()?; // FIXME May use a different byte order than Fixed8
    let frame_count = compressed_bit_reader.read_u16()?;

    Ok((
        Header {
            compression,
            version,
            file_length,
            frame_size,
            frame_rate,
            frame_count,
        },
        compressed_bit_reader,
    ))
}
