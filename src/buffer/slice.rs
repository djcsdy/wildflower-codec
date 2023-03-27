use crate::buffer::block::{SwfBlock, BLOCK_SIZE};
use crate::buffer::block_index::SwfBlockIndex;
use crate::buffer::offset::SwfOffset;
use crate::buffer::pointer::SwfPointer;
use std::cmp::max;
use std::io::{Read, Result};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SwfSlice {
    first_block_index: SwfBlockIndex,
    blocks: Vec<Arc<SwfBlock>>,
    start_offset: SwfOffset,
    end_offset: SwfOffset,
    read_offset: SwfOffset,
}

impl SwfSlice {
    pub(super) fn new(
        first_block_index: SwfBlockIndex,
        blocks: Vec<Arc<SwfBlock>>,
        start_offset: SwfOffset,
        end_offset: SwfOffset,
    ) -> Self {
        Self {
            first_block_index,
            blocks,
            start_offset,
            end_offset,
            read_offset: start_offset,
        }
    }

    /// Returns a [SwfPointer] to the start of this slice within the SWF file.
    pub fn start_pointer(&self) -> SwfPointer {
        self.first_block_index.as_pointer() + self.start_offset
    }

    /// Returns a [SwfPointer] to the byte after the end of this slice within
    /// the SWF file.
    pub fn end_pointer(&self) -> SwfPointer {
        self.first_block_index.as_pointer() + self.end_offset
    }

    /// Returns the current read position of this slice as a [SwfPointer].
    ///
    /// The pointer points to the next byte to be read within the SWF file.
    pub fn position_pointer(&self) -> SwfPointer {
        self.first_block_index.as_pointer() + self.read_offset
    }

    /// Returns the current read position of this slice as a [SwfOffset] from
    /// the start of the slice.
    pub fn position(&self) -> SwfOffset {
        self.read_offset
    }

    /// Returns the length of this slice as a [SwfOffset] from the start of
    /// the slice to the byte after the end of the slice.
    pub fn length(&self) -> SwfOffset {
        self.end_offset - self.start_offset
    }

    /// Returns the remaining number of readable bytes in this slice as a
    /// [SwfOffset] from the current read position to the byte after the end
    /// of the slice.
    pub fn remaining(&self) -> SwfOffset {
        self.end_offset - self.read_offset
    }
}

impl Read for SwfSlice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let length = max(buf.len(), self.remaining().into());
        let mut block_index = self.read_offset.0 as usize / BLOCK_SIZE;
        let block = &self.blocks[block_index];
        let block_position = self.read_offset.0 as usize % BLOCK_SIZE;
        let block_remaining = BLOCK_SIZE - block_position;
        if block_remaining > length {
            buf[..length].copy_from_slice(&block[block_position..block_position + length]);
            return Ok(length);
        } else {
            buf[..block_remaining].copy_from_slice(&block[block_position..]);
            let mut pos = block_remaining;
            loop {
                block_index += 1;
                let remaining = pos - length;
                let block = &self.blocks[block_index];
                if BLOCK_SIZE > remaining {
                    buf[pos..length].copy_from_slice(&block[..remaining]);
                    return Ok(length);
                } else {
                    buf[pos..pos + BLOCK_SIZE].copy_from_slice(block.buffer());
                    pos += BLOCK_SIZE
                }
            }
        }
    }
}
