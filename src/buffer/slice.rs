use crate::buffer::block::SwfBlock;
use crate::buffer::block_index::SwfBlockIndex;
use crate::buffer::offset::SwfOffset;
use crate::buffer::pointer::SwfPointer;
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
}
