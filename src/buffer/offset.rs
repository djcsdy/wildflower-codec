use std::ops::{Add, Sub};

/// A byte-offset from a [SwfPointer][super::pointer::SwfPointer].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfOffset(pub(super) i32);

impl Add for SwfOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for SwfOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
