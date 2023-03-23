use crate::decode::tags::sounds::sampling_bits::SamplingBits;
use crate::decode::tags::sounds::sampling_rate::SamplingRate;
use crate::decode::tags::sounds::sound_channels::SoundChannels;
use crate::decode::tags::sounds::sound_format::SoundFormat;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundStreamHead2Tag {
    pub playback_sound_rate: SamplingRate,
    pub playback_bits: SamplingBits,
    pub playback_channels: SoundChannels,
    pub stream_sound_compression: SoundFormat,
    pub stream_sound_rate: SamplingRate,
    pub stream_sound_bits: SamplingBits,
    pub stream_sound_channels: SoundChannels,
    pub stream_sound_sample_count: u16,
    pub latency_seek: i16,
}
