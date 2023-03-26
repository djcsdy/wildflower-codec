use crate::buffer::block::SwfBlock;
use crate::buffer::offset::SwfOffset;
use crate::buffer::pointer::SwfPointer;
use std::sync::Arc;

pub struct SwfSlice {
    start_pointer: SwfPointer,
    blocks: Vec<Arc<SwfBlock>>,
    start_offset: SwfOffset,
    end_offset: SwfOffset,
}
