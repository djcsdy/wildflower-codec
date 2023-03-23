use crate::decode::tags::sounds::sampling_bits::SamplingBits;
use crate::decode::tags::sounds::sampling_rate::SamplingRate;
use crate::decode::tags::sounds::sound_format::SoundFormat;
use crate::decode::tags::sounds::SoundChannels;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSoundTag {
    pub sound_id: u16,
    pub sound_format: SoundFormat,
    pub sound_rate: SamplingRate,
    pub sound_bits: SamplingBits,
    pub sound_channels: SoundChannels,
    pub sound_data: Vec<u8>,
}
