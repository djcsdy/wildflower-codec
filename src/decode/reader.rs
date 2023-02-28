use decode::bit_reader::SwfBitReader;
use std::io::Read;

pub struct SwfReader<R: Read> {
    bit_reader: SwfBitReader<R>,
}

impl<R: Read> SwfReader<R> {
    pub fn new<I: Into<SwfBitReader<R>>>(bit_reader: I) -> SwfReader<R> {
        SwfReader { bit_reader }
    }
}
