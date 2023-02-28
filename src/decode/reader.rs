use decode::bit_reader::SwfBitReader;
use std::io::Read;

pub struct SwfReader<R: Read> {
    bit_reader: SwfBitReader<R>,
}
