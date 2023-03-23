use crate::decode::tags::sounds::SoundInfo;

#[derive(Clone, PartialEq, Debug)]
pub struct StartSoundTag {
    pub sound_id: u16,
    pub sound_info: SoundInfo,
}
