/// A pointer to a specific byte within a [SwfBlock][super::block::SwfBlock].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfBlockPointer(pub(super) u32);

impl Into<usize> for SwfBlockPointer {
    fn into(self) -> usize {
        self.0 as usize
    }
}
