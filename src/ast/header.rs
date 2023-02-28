use ast::common::{Fixed8, Rectangle};

pub enum Compression {
    None,
    Zlib,
    Lzma,
}

pub struct Header {
    pub compression: Compression,
    pub version: u8,
    pub frame_size: Rectangle,
    pub frame_rate: Fixed8,
    pub frame_count: u16,
}
