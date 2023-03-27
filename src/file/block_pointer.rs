/// A pointer to a specific byte within a [SwfBlock][super::block::SwfBlock].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfBlockPointer(pub(super) u32);
