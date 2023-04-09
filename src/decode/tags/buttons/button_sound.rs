use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::sounds::sound_info::SoundInfo;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonSound {
    pub sound_id: u16,
    pub sound_info: SoundInfo,
}

impl ButtonSound {
    pub fn read<R: Read>(reader: &mut R) -> Result<Option<Self>> {
        let sound_id = reader.read_u16()?;
        Ok(if sound_id == 0 {
            None
        } else {
            let sound_info = SoundInfo::read(reader)?;
            Some(Self {
                sound_id,
                sound_info,
            })
        })
    }
}
