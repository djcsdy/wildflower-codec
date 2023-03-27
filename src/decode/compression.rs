#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Compression {
    None,
    Zlib,
    Lzma,
}
