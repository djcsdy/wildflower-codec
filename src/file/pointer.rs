use crate::file::block::BLOCK_SIZE;
use crate::file::block_index::SwfBlockIndex;
use crate::file::block_pointer::SwfBlockPointer;
use crate::file::offset::SwfOffset;
use std::ops::{Add, Sub};

/// A pointer to a specific byte within a SWF file.
///
/// [ZERO][SwfPointer::ZERO] refers to the first byte of the SWF file after
/// the [Header][crate::decode::tags::header::Header].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfPointer(pub(super) u32);

impl SwfPointer {
    /// A pointer to the first byte of a SWF file after the
    /// [Header][crate::decode::tags::header::Header].
    pub const ZERO: Self = Self(0);

    pub(super) fn as_block_index_and_pointer(self) -> (SwfBlockIndex, SwfBlockPointer) {
        (
            SwfBlockIndex::of_pointer(self),
            SwfBlockPointer((self.0 as usize % BLOCK_SIZE) as u16),
        )
    }
}

impl Add<SwfOffset> for SwfPointer {
    type Output = Self;

    fn add(self, rhs: SwfOffset) -> Self {
        Self(self.0.checked_add_signed(rhs.0).unwrap())
    }
}

impl Sub<SwfOffset> for SwfPointer {
    type Output = Self;

    fn sub(self, rhs: SwfOffset) -> Self {
        Self(self.0.checked_add_signed(-rhs.0).unwrap())
    }
}

impl Sub for SwfPointer {
    type Output = SwfOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        SwfOffset(self.0.wrapping_sub(rhs.0) as i32)
    }
}
