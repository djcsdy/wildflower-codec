use crate::decode::header::Header;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{BufRead, Result};

pub struct SwfFile {
    header: Header,
    pub(super) payload: Vec<u8>,
}

impl SwfFile {
    pub fn read<R: BufRead>(reader: R) -> Result<Self> {
        let (header, mut reader) = Header::read(reader)?;
        let payload = reader.read_u8_to_end()?;
        Ok(Self { header, payload })
    }
}
