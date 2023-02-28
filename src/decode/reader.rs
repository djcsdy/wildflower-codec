use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub struct SwfReader<R: Read> {
    inner: R,
}

impl SwfReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader<File>> {
        File::open(path).map(|inner| SwfReader { inner })
    }
}
