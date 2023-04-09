use crate::decode::tags::common::string::String;
use crate::decode::tags::sounds::sound_info::SoundInfo;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct StartSound2Tag {
    pub sound_class_name: String,
    pub sound_info: SoundInfo,
}

impl StartSound2Tag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let sound_class_name = String::read(reader)?;
        let sound_info = SoundInfo::read(reader)?;
        Ok(Self {
            sound_class_name,
            sound_info,
        })
    }
}
