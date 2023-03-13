use super::common::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Compression {
    None,
    Zlib,
    Lzma,
}

pub struct Header {
    pub compression: Compression,
    pub version: u8,
    pub file_length: u32,
    pub frame_size: Rectangle,
    pub frame_rate: Fixed8,
    pub frame_count: u16,
}
