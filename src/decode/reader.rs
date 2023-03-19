use crate::decode::decompressing_reader::DecompressingReader;
use crate::decode::header::read_header;
use crate::decode::tags::header::Header;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub struct SwfReader<R: BufRead> {
    header: Header,
    inner: DecompressingReader<R>,
}

impl SwfReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader<BufReader<File>>> {
        SwfReader::read_from(BufReader::new(File::open(path)?))
    }
}

impl<R: BufRead> SwfReader<R> {
    pub fn read_from(reader: R) -> Result<SwfReader<R>> {
        read_header(reader).map(|(header, inner)| SwfReader { header, inner })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}
