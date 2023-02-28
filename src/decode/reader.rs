use crate::decode::bit_reader::SwfBitReader;
use std::fs::File;
use std::io::{Read, Result};
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
        SwfReader { bit_reader: bit_reader.into() }
    }
}
