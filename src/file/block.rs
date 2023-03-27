use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub(super) const BLOCK_SIZE: usize = 1 << 15;

/// A 64k block of opaque SWF data.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub(super) struct SwfBlock([u8; BLOCK_SIZE]);

impl SwfBlock {
    pub(super) fn new(buffer: [u8; BLOCK_SIZE]) -> Self {
        Self(buffer)
    }

    pub(super) fn buffer(&self) -> &[u8; BLOCK_SIZE] {
        &self.0
    }
}

impl Index<usize> for SwfBlock {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Range<usize>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeInclusive<usize>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeFrom<usize>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeTo<usize>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeToInclusive<usize>> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeFull> for SwfBlock {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}
