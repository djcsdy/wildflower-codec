use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

/// Associates an ActionScript 3 class with a character.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassRecord {
    /// The character ID to be associated.
    pub character_id: u16,

    /// The fully-qualified name of the ActionScript 3 class to be associated.
    pub class_name: String,
}

impl SymbolClassRecord {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let class_name = String::read(reader)?;
        Ok(Self {
            character_id,
            class_name,
        })
    }
}
