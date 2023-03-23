use crate::decode::tags::common::string::String;
use crate::decode::tags::sounds::sound_info::SoundInfo;

#[derive(Clone, PartialEq, Debug)]
pub struct StartSound2Tag {
    pub sound_class_name: String,
    pub sound_info: SoundInfo,
}
