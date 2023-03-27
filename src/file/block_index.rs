use crate::file::block::BLOCK_SIZE;
use crate::file::pointer::SwfPointer;
use std::ops::{AddAssign, Range};

/// An index into the list of [SwfBlock][super::block::SwfBlock]s contained by
/// a SWF file.
///
/// [ZERO][SwfBlockIndex::ZERO] refers to the first SwfBlock in a SWF file.
///
/// The first SwfBlock starts at the first byte after the end of the
/// [Header][crate::decode::tags::header::Header].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub(super) struct SwfBlockIndex(pub(super) u32);

impl SwfBlockIndex {
    /// The index of the first [SwfBlock][super::block::SwfBlock] in a SWF
    /// file.
    ///
    /// The first SwfBlock starts at the first byte after the end of the
    /// [Header][crate::decode::tags::header::Header].
    pub(super) const ZERO: Self = Self(0);

    /// Returns a [SwfPointer] to the first byte contained by this block.
    pub(super) fn as_pointer(&self) -> SwfPointer {
        SwfPointer(self.0 * BLOCK_SIZE as u32)
    }

    /// Returns the [SwfBlockIndex] of the [SwfBlock][super::block::SwfBlock]
    /// that contains the byte pointed to by the specified [SwfPointer].
    pub(super) fn of_pointer(pointer: SwfPointer) -> SwfBlockIndex {
        SwfBlockIndex((usize::from(pointer) / BLOCK_SIZE) as u32)
    }

    pub(super) fn iterate(range: Range<Self>) -> SwfBlockIndexRangeIterator {
        SwfBlockIndexRangeIterator {
            position: range.start,
            end: range.end,
        }
    }
}

impl AddAssign<u32> for SwfBlockIndex {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

impl From<SwfBlockIndex> for usize {
    fn from(value: SwfBlockIndex) -> Self {
        value.0 as usize
    }
}

pub(super) struct SwfBlockIndexRangeIterator {
    position: SwfBlockIndex,
    end: SwfBlockIndex,
}

impl Iterator for SwfBlockIndexRangeIterator {
    type Item = SwfBlockIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.end {
            let item = self.position;
            self.position += 1;
            Some(item)
        } else {
            None
        }
    }
}
