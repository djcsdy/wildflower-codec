use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct KerningRecord<CharacterCode> {
    pub left_character_code: CharacterCode,
    pub right_character_code: CharacterCode,
    pub kerning_adjustment: i16,
}

impl<CharacterCode> KerningRecord<CharacterCode> {
    pub fn read<R: Read, ReadCharacterCode: Fn(&mut R) -> Result<CharacterCode>>(
        reader: &mut R,
        read_character_code: ReadCharacterCode,
    ) -> Result<Self> {
        let left_character_code = read_character_code(reader)?;
        let right_character_code = read_character_code(reader)?;
        let kerning_adjustment = reader.read_i16()?;
        Ok(Self {
            left_character_code,
            right_character_code,
            kerning_adjustment,
        })
    }
}
