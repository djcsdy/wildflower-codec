use crate::buffer::offset::SwfOffset;
use std::ops::Add;

/// A pointer to a specific byte within a SWF file.
///
/// [ZERO][SwfPointer::ZERO] refers to the first byte of the SWF file after
/// the [Header][crate::decode::tags::header::Header].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfPointer(u32);

impl SwfPointer {
    /// A pointer to the first byte of a SWF file after the
    /// [Header][crate::decode::tags::header::Header].
    pub const ZERO: Self = Self(0);
}

impl Add<SwfOffset> for SwfPointer {
    type Output = Self;

    fn add(self, rhs: SwfOffset) -> Self {
        Self(self.0.checked_add_signed(rhs.0).unwrap())
    }
}
