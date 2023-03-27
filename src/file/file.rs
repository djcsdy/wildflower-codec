use crate::decode::header::Header;

pub struct SwfFile {
    header: Header,
    payload: Vec<u8>,
}
