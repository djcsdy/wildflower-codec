use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

/// Sets the tab ordering of the character at the specified depth.
#[derive(Clone, PartialEq, Debug)]
pub struct SetTabIndexTag {
    pub depth: u16,
    pub tab_index: u16,
}

impl SetTabIndexTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let depth = reader.read_u16()?;
        let tab_index = reader.read_u16()?;
        Ok(Self { depth, tab_index })
    }
}
