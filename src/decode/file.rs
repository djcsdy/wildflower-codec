use std::fs::File;
use std::io::Result;
use std::path::Path;

pub struct SwfFile {
    file: File,
}

impl SwfFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SwfFile> {
        File::open(path).map(|file| SwfFile { file })
    }
}
