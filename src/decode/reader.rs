use std::fs::File;
use std::io::Result;
use std::path::Path;

pub struct SwfReader {
    file: File,
}

impl SwfReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfReader> {
        File::open(path).map(|file| SwfReader { file })
    }
}
