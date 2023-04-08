use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

/// Removes the character at the specified depth from the display list.
#[derive(Clone, PartialEq, Debug)]
pub struct RemoveObject2Tag {
    /// Depth of character to remove.
    pub depth: u16,
}

impl RemoveObject2Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<RemoveObject2Tag> {
        let depth = reader.read_u16()?;
        Ok(RemoveObject2Tag { depth })
    }
}
