use crate::buffer::block::BLOCK_SIZE;
use crate::buffer::pointer::SwfPointer;

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
}
