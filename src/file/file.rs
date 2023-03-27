use std::collections::Bound;
use crate::decode::header::Header;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{BufRead, Result};
use std::ops::RangeBounds;
use crate::file::offset::SwfOffset;
use crate::file::pointer::SwfPointer;
use crate::file::slice::SwfSlice;

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

    pub fn slice<Range: RangeBounds<SwfPointer>>(&self, range: Range) -> SwfSlice {
        let start_pointer = match range.start_bound() {
            Bound::Included(&pointer) => pointer,
            Bound::Excluded(&pointer) => pointer + SwfOffset(1),
            Bound::Unbounded => SwfPointer::ZERO,
        };
        let end_pointer = match range.end_bound() {
            Bound::Included(&pointer) => pointer + SwfOffset(1),
            Bound::Excluded(&pointer) => pointer,
            Bound::Unbounded => SwfPointer(self.payload.len() as u32),
        };
        SwfSlice::new(&self, start_pointer, end_pointer)
    }
}
