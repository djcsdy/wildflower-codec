use crate::file::block::{SwfBlock, BLOCK_SIZE};
use crate::file::block_index::SwfBlockIndex;
use crate::file::pointer::SwfPointer;
use std::io::{Read, Result};
use std::ops::Range;

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

    pub fn read_byte_range(&self, range: Range<SwfPointer>) -> Vec<u8> {
        if range.start >= range.end {
            Vec::new()
        } else {
            let start_pointer = range.start;
            let end_pointer = range.end;

            let (start_block_index, start_block_pointer) =
                start_pointer.as_block_index_and_pointer();
            let (end_block_index, end_block_pointer) = end_pointer.as_block_index_and_pointer();

            let start_block = &self.0[usize::from(start_block_index)];

            if start_block_index == end_block_index {
                start_block[start_block_pointer..end_block_pointer].to_vec()
            } else {
                let mut buffer = Vec::with_capacity((end_pointer - start_pointer).0 as usize);
                let mut buffer_position = BLOCK_SIZE - usize::from(start_block_pointer);
                buffer[..buffer_position].copy_from_slice(&start_block[start_block_pointer..]);

                for block_index in
                SwfBlockIndex::iterate(start_block_index + 1..end_block_index - 1)
                {
                    let block = &self.0[usize::from(block_index)];
                    let new_buffer_position = buffer_position + BLOCK_SIZE;
                    buffer[buffer_position..new_buffer_position].copy_from_slice(&block[..]);
                    buffer_position = new_buffer_position;
                }

                let end_block = &self.0[usize::from(end_block_index - 1)];
                buffer[buffer_position..].copy_from_slice(&end_block[..end_block_pointer]);

                buffer
            }
        }
    }
}
