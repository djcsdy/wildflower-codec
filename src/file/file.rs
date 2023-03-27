use crate::decode::tags::header::Header;

pub struct SwfFile {
    header: Header,
    payload: Vec<u8>,
}
