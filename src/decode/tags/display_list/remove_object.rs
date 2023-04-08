use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

/// Removes the specified character at the specified depth from the display
/// list.
#[derive(Clone, PartialEq, Debug)]
pub struct RemoveObjectTag {
    /// ID of character to remove.
    pub character_id: u16,

    /// Depth of character to remove.
    pub depth: u16,
}

impl RemoveObjectTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let depth = reader.read_u16()?;
        Ok(Self {
            character_id,
            depth,
        })
    }
}
