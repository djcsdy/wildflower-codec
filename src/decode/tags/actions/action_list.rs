use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_pointer::ActionPointer;
use crate::decode::tags::actions::action_record::ActionRecord;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ActionList<Buffer: AsRef<[u8]>> {
    pub buffer: Buffer,
}

impl ActionList<Vec<u8>> {
    pub fn read<R: Read>(reader: &mut R, length: usize) -> Result<Self> {
        let mut buffer = vec![0u8; length];
        reader.read_u8_into(&mut buffer)?;
        Ok(Self { buffer })
    }

    pub fn read_to_end<R: Read>(reader: &mut R) -> Result<Self> {
        let buffer = reader.read_u8_to_end()?;
        Ok(Self { buffer })
    }
}

impl<Buffer: AsRef<[u8]>> ActionList<Buffer> {
    pub fn read_action(&self, pointer: ActionPointer) -> Result<(ActionRecord, ActionPointer)> {
        let mut reader = SwfSliceReader::new(pointer.index_buffer(&self.buffer));
        let action_record = ActionRecord::read(&mut reader)?;
        Ok((action_record, pointer + reader.position().into()))
    }
}
