const BLOCK_SIZE: usize = 1 << 15;

/// A 64k block of opaque SWF data.
///
/// This block size is chosen because it is the next larger power of 2
/// than the maximum offset of an AVM1 branch instruction.
pub struct SwfBlock {
    pub bytes: [u8; BLOCK_SIZE],
}
