use crate::buffer::block::SwfBlock;
use crate::buffer::block_index::SwfBlockIndex;
use crate::buffer::offset::SwfOffset;
use std::sync::Arc;

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
}
