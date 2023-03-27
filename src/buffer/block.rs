use std::ops::{Index, Range, RangeFrom, RangeInclusive, RangeTo};

pub(super) const BLOCK_SIZE: usize = 1 << 15;

/// A 64k block of opaque SWF data.
///
/// This block size is chosen because it is the next larger power of 2
/// than the maximum offset of an AVM1 branch instruction.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub(super) struct SwfBlock(pub(super) [u8; BLOCK_SIZE]);

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
