use crate::decode::read_ext::SwfTypesReadExt;
use crate::file::block_pointer::SwfBlockPointer;
use std::io::{Read, Result};
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub(super) const BLOCK_SIZE: usize = 1 << 15;

/// A 64k block of opaque SWF data.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub(super) struct SwfBlock([u8; BLOCK_SIZE]);

impl SwfBlock {
    pub(super) const EMPTY: Self = Self([0u8; BLOCK_SIZE]);

    pub(super) fn new(buffer: [u8; BLOCK_SIZE]) -> Self {
        Self(buffer)
    }

    pub(super) fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; BLOCK_SIZE];
        reader.read_u8_into(&mut buffer)?;
        Ok(Self(buffer))
    }

    pub(super) fn buffer(&self) -> &[u8; BLOCK_SIZE] {
        &self.0
    }
}

impl Index<SwfBlockPointer> for SwfBlock {
    type Output = u8;

    fn index(&self, index: SwfBlockPointer) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl Index<Range<SwfBlockPointer>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: Range<SwfBlockPointer>) -> &Self::Output {
        &self.0[index.start.0 as usize..index.end.0 as usize]
    }
}

impl Index<RangeInclusive<SwfBlockPointer>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<SwfBlockPointer>) -> &Self::Output {
        &self.0[index.start().0 as usize..=index.end().0 as usize]
    }
}

impl Index<RangeFrom<SwfBlockPointer>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeFrom<SwfBlockPointer>) -> &Self::Output {
        &self.0[index.start.0 as usize..]
    }
}

impl Index<RangeTo<SwfBlockPointer>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeTo<SwfBlockPointer>) -> &Self::Output {
        &self.0[..index.end.0 as usize]
    }
}

impl Index<RangeToInclusive<SwfBlockPointer>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<SwfBlockPointer>) -> &Self::Output {
        &self.0[..=index.end.0 as usize]
    }
}

impl Index<RangeFull> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}
