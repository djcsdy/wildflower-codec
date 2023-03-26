use std::ops::Add;

/// A byte-offset from a [SwfPointer][super::pointer::SwfPointer].
pub struct SwfOffset(i32);

impl Add for SwfOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
