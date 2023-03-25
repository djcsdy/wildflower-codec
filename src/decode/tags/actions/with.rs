use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct With {
    pub body: ActionList<Vec<u8>>,
}

impl With {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let size = reader.read_u16()?;
        let body = ActionList::read(reader, size as usize)?;
        Ok(Self { body })
    }
}
