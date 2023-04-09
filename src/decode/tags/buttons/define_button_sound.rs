use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::buttons::button_sound::ButtonSound;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonSoundTag {
    pub button_id: u16,
    pub button_sound_0: Option<ButtonSound>,
    pub button_sound_1: Option<ButtonSound>,
    pub button_sound_2: Option<ButtonSound>,
    pub button_sound_3: Option<ButtonSound>,
}

impl DefineButtonSoundTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let button_id = reader.read_u16()?;
        let button_sound_0 = ButtonSound::read(reader)?;
        let button_sound_1 = ButtonSound::read(reader)?;
        let button_sound_2 = ButtonSound::read(reader)?;
        let button_sound_3 = ButtonSound::read(reader)?;
        Ok(Self {
            button_id,
            button_sound_0,
            button_sound_1,
            button_sound_2,
            button_sound_3,
        })
    }
}
