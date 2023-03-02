use crate::ast::header::Header;
use crate::decode::bit_reader::SwfBitReader;
use crate::decode::header::read_header;
use std::fs::File;
use std::io::{Read, Result};
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
        read_header(bit_reader).map(|(header, bit_reader)| SwfReader { header, bit_reader })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}
