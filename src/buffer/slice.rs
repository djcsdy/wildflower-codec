use crate::buffer::block::{SwfBlock, BLOCK_SIZE};
use crate::buffer::block_index::SwfBlockIndex;
use crate::buffer::offset::SwfOffset;
use crate::buffer::pointer::SwfPointer;
use std::cmp::max;
use std::io::{Read, Result};
use std::sync::Arc;
use crate::decode::bit_read::{bit_read, BitRead, BitReadOptions, BitReadState};
use crate::decode::read_ext::SwfTypesReadExt;

#[derive(Clone, Debug)]
pub struct SwfSlice {
    first_block_index: SwfBlockIndex,
    blocks: Vec<Arc<SwfBlock>>,
    start_offset: SwfOffset,
    end_offset: SwfOffset,
    read_offset: SwfOffset,
    partial_byte: u8,
    partial_bit_count: u8,
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
            partial_byte: 0,
            partial_bit_count: 0,
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
        self.align_byte();
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

impl BitRead for SwfSlice {
    fn align_byte(&mut self) {
        self.partial_byte = 0;
        self.partial_bit_count = 0;
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32> {
        let state = BitReadState {
            partial_byte: self.partial_byte,
            partial_bit_count: self.partial_bit_count,
        };
        let (new_state, result) = bit_read(&mut BitReadOptions {
            read_byte: || self.read_u8(),
            state,
            bits,
        });
        self.partial_byte = new_state.partial_byte;
        self.partial_bit_count = new_state.partial_bit_count;
        result
    }
}