use crate::decode::header::Header;
use crate::file::block_list::SwfBlockList;
use crate::file::offset::SwfOffset;
use crate::file::pointer::SwfPointer;
use crate::file::slice::SwfSlice;
use std::collections::Bound;
use std::io::{BufRead, Result};
use std::ops::RangeBounds;

pub struct SwfFile {
    header: Header,
    pub(super) payload: SwfBlockList,
}

impl SwfFile {
    pub fn read<R: BufRead>(reader: &mut R) -> Result<Self> {
        let (header, mut reader) = Header::read(reader)?;
        let payload = SwfBlockList::read(&mut reader, header.file_length)?;
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
            Bound::Unbounded => SwfPointer(self.header.file_length),
        };
        SwfSlice::new(&self, start_pointer, end_pointer)
    }
}
