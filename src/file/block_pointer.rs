/// A pointer to a specific byte within a [SwfBlock][super::block::SwfBlock].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfBlockPointer(pub(super) u32);

impl From<SwfBlockPointer> for usize {
    fn from(value: SwfBlockPointer) -> Self {
        value.0 as usize
    }
}
