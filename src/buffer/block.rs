use std::ops::Index;

pub(super) const BLOCK_SIZE: usize = 1 << 15;

/// A 64k block of opaque SWF data.
///
/// This block size is chosen because it is the next larger power of 2
/// than the maximum offset of an AVM1 branch instruction.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SwfBlock {
    pub(super) bytes: [u8; BLOCK_SIZE],
}

impl Index<usize> for SwfBlock {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}
