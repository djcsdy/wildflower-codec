use crate::ast::header::Header;
use crate::decode::bit_reader::SwfBitReader;
use crate::decode::header::read_header;
use crate::decode::transparent_decompressing_reader::TransparentDecompressingReader;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub struct SwfReader<R: BufRead> {
    header: Header,
    bit_reader: SwfBitReader<TransparentDecompressingReader<R>>,
}

impl SwfReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader<BufReader<File>>> {
        SwfReader::read_from(BufReader::new(File::open(path)?))
    }
}

impl<R: BufRead> SwfReader<R> {
    pub fn read_from(bit_reader: R) -> Result<SwfReader<R>> {
        read_header(bit_reader).map(|(header, bit_reader)| SwfReader { header, bit_reader })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}
