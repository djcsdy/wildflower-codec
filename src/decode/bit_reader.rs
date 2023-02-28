use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct SwfBitReader<R: Read> {
    inner: BufReader<R>,
}

impl SwfBitReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfBitReader<File>> {
        File::open(path).map(|file| SwfBitReader::new(file))
    }
}

impl<R: Read> SwfBitReader<R> {
    pub fn new(inner: R) -> SwfBitReader<R> {
        SwfBitReader {
            inner: BufReader::new(inner),
        }
    }

    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner.read_exact(buf)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl<R: Read> From<R> for SwfBitReader<R> {
    fn from(value: R) -> Self {
        SwfBitReader::new(value)
    }
}
