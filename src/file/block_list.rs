use crate::file::block::{SwfBlock, BLOCK_SIZE};
use std::io::{Read, Result};

#[derive(Clone, Debug)]
pub(super) struct SwfBlockList(Vec<SwfBlock>);

impl SwfBlockList {
    pub fn read<R: Read>(reader: &mut R, length: u32) -> Result<Self> {
        let block_count = length as usize / BLOCK_SIZE;
        let mut blocks = Vec::with_capacity(block_count);
        for _ in 0..block_count {
            blocks.push(SwfBlock::read(reader)?);
        }
        Ok(Self(blocks))
    }
}
