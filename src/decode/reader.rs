use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct SwfReader<R: Read> {
    inner: BufReader<R>,
}

impl SwfReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader<File>> {
        File::open(path).map(|file| SwfReader::new(file))
    }
}

impl<R: Read> SwfReader<R> {
    pub fn new(inner: R) -> SwfReader<R> {
        SwfReader {
            inner: BufReader::new(inner),
        }
    }
}

impl<R: Read> From<R> for SwfReader<R> {
    fn from(value: R) -> Self {
        SwfReader::new(value)
    }
}
